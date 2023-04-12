use env_logger::{Env, Logger};
use log::{debug, error, info, log_enabled, warn, Level};
mod move_data;
use move_data::*;
use rand::rngs::ThreadRng;
use rand::Rng;
use serde::Deserialize;
use serde::Serialize;
use std::fs;
use std::ops::Mul;

fn main() {
    env_logger::builder().format_timestamp(None).init();
}

fn get_stat_stage_multiplier(stage: i32) -> f32 {
    if stage == 0 {
        1.0
    } else if stage > 0 {
        (2 + stage) as f32 / 2.0
    } else {
        2.0 / (2 - stage) as f32
    }
}

fn get_acc_stage_multiplier(acc_stage: i32, eva_stage: i32) -> f32 {
    let mut stage = acc_stage - eva_stage;
    if stage == 0 {
        1.0
    } else if stage > 0 {
        if stage > 6 {
            stage = 6;
        }
        (100.0 + ((stage as f32 * 100.0) / 3.0)) / 100.0
    } else if stage == -1 {
        0.75
    } else if stage == -2 {
        0.6
    } else if stage == -3 {
        0.5
    } else if stage == -4 {
        0.43
    } else if stage == -5 {
        0.36
    } else {
        0.33
    }
}

pub fn get_offensive_stage_multiplier(stage: i32, is_crit: bool) -> f32 {
    if is_crit && stage <= 0 {
        1.0
    } else {
        get_stat_stage_multiplier(stage)
    }
}

pub fn get_defensive_stage_multiplier(stage: i32, is_crit: bool) -> f32 {
    if is_crit && stage >= 0 {
        1.0
    } else {
        get_stat_stage_multiplier(stage)
    }
}

pub fn get_crit_chance(stage: i32) -> f32 {
    match stage {
        0 => 0.0625,
        1 => 0.125,
        2 => 0.25,
        3 => 1.0 / 3.0,
        _ => 0.5,
    }
}

/// [damage formula](https://bulbapedia.bulbagarden.net/wiki/Damage#Generation_IV)
pub fn calculate_damage(
    power: i32,
    attack: i32,
    defense: i32,
    level: i32,
    attack_stage: i32,
    defense_stage: i32,
    stab_bonus: f32,
    type_effectiviness: f32,
    chance_bonuses: f32,
    is_crit: bool,
) -> f32 {
    (((((2.0 * level as f32) / 5.0) + 2.0)
        * (power as f32)
        * (attack as f32 / defense as f32)
        * (get_offensive_stage_multiplier(attack_stage, is_crit)
            / get_defensive_stage_multiplier(defense_stage, is_crit))
        / 50.0)
        + 2.0)
        * stab_bonus
        * type_effectiviness
        * chance_bonuses
}

pub trait AI {
    fn get_action(&self) -> CombatAction;
    fn get_forced_switch(&self, creature_instances: &Vec<CreatureInstance>) -> usize;
}

pub struct RandomAI {}

impl AI for RandomAI {
    fn get_action(&self) -> CombatAction {
        CombatAction::Attack(rand::random::<u8>() % 4)
    }
    /// Assumption that if all fainted we don't force switch
    fn get_forced_switch(&self, creature_instances: &Vec<CreatureInstance>) -> usize {
        loop {
            // TODO random ai doesn't strategizes, so for loop might be better
            let switch_to = rand::random::<usize>() % creature_instances.len();
            if !creature_instances[switch_to].is_fainted() {
                return switch_to;
            }
        }
    }
}

fn has_team_fainted(creature_instances: &[Vec<CreatureInstance>; 2], side: usize) -> bool {
    for instance in &creature_instances[side] {
        if !instance.is_fainted() {
            return false;
        }
    }
    true
}

#[derive(Clone, Debug)]
pub enum CombatAction {
    Attack(u8),
    Switch(u8),
}

impl CombatAction {
    pub fn try_get_move_id(&self, moves: &Vec<Move>) -> Option<MoveID> {
        match self {
            CombatAction::Attack(id) => Some(moves[(*id) as usize].id.clone()),
            CombatAction::Switch(_) => None,
        }
    }
}
#[derive(Debug)]
pub enum Interrupt {
    AFainted,
    BFainted,
    AWon,
    BWon,
}
enum Roll {
    RandomRoll,
    HighRoll,
    AverageRoll,
}

struct BattleSettings {
    crit_enabled: bool,
    always_hits: bool,
    roll: Roll,
}
impl BattleSettings {
    fn new(crit_enabled: bool, always_hits: bool, roll: Roll) -> Self {
        BattleSettings {
            crit_enabled,
            always_hits,
            roll,
        }
    }
}
impl Default for BattleSettings {
    fn default() -> Self {
        BattleSettings {
            crit_enabled: false,
            always_hits: false,
            roll: Roll::HighRoll,
        }
    }
}

// cloneable
struct BattleInstance {
    battler_ids: [usize; 2],
    volatile_statuses: [Vec<(VolatileStatus, i32)>; 2],
    current_turn: i32, // Current turn is used for weather mostly as turn order might by have to take into account when using it in things with side
}
impl Default for BattleInstance {
    fn default() -> Self {
        BattleInstance {
            battler_ids: [0, 0],
            volatile_statuses: [vec![], vec![]],
            current_turn: 0,
        }
    }
}

// When both faints -> faster switches in first

// PP is wasted, even if target doesn't exist

impl BattleInstance {
    /// Main method
    /// Try to faint check outside of the methods that are inside fn turn
    pub fn turn(
        &mut self,
        battle_settings: &BattleSettings,
        creatures: &[Vec<Creature>; 2],
        creature_instances: &mut [Vec<CreatureInstance>; 2],
        combat_actions: &[CombatAction; 2],
    ) -> Option<Interrupt> {
        let first_faster = self.is_first_faster(creatures, combat_actions);
        self.do_action(
            battle_settings,
            creatures,
            creature_instances,
            combat_actions,
            !first_faster,
        );

        if !creature_instances[first_faster as usize][self.battler_ids[first_faster as usize]]
            .is_fainted()
        {
            self.do_action(
                battle_settings,
                creatures,
                creature_instances,
                combat_actions,
                first_faster,
            );
        }
        self.current_turn += 1;

        if creature_instances[0][self.battler_ids[0]].is_fainted() {
            if has_team_fainted(creature_instances, 0) {
                Some(Interrupt::BWon)
            } else {
                Some(Interrupt::AFainted)
            }
        } else if creature_instances[1][self.battler_ids[1]].is_fainted() {
            if has_team_fainted(creature_instances, 1) {
                Some(Interrupt::AWon)
            } else {
                Some(Interrupt::BFainted)
            }
        } else {
            None
        }
    }

    fn handle_interrupts<AiA: AI, AiB: AI>(
        &mut self,
        creature_instances: &mut [Vec<CreatureInstance>; 2],
        interrupt_opt: Option<Interrupt>,
        ai_a: &AiA,
        ai_b: &AiB,
    ) -> Result<(), Interrupt> {
        Ok(match interrupt_opt {
            Some(Interrupt::AFainted) => self.switch(
                creature_instances,
                ai_a.get_forced_switch(&creature_instances[0]),
                0,
            ),
            Some(Interrupt::BFainted) => self.switch(
                creature_instances,
                ai_b.get_forced_switch(&creature_instances[1]),
                1,
            ),
            Some(Interrupt::AWon) | Some(Interrupt::BWon) => {
                return Err(interrupt_opt.unwrap());
            }
            None => (),
        })
    }

    fn has_then_try_remove_value_volatile_status(
        &mut self,
        actioner: usize,
        value_volatile_status: &VolatileStatus,
    ) -> bool {
        for i in 0..self.volatile_statuses[actioner].len() {
            if &self.volatile_statuses[actioner][i].0 == value_volatile_status {
                self.volatile_statuses[actioner].remove(i);
                return true;
            }
        }
        false
    }

    fn get_value_from_value_volatile_status(
        &self,
        actioner: usize,
        volatile_status: VolatileStatus,
    ) -> Option<i32> {
        for ps in &self.volatile_statuses[actioner] {
            if ps.0 == volatile_status {
                return Some(ps.1);
            }
        }
        None
    }

    /// Shortcut for [get_value_from_value_volatile_status] in a way
    fn get_stage_from_value_volatile_status(
        &self,
        side: usize,
        volatile_status: VolatileStatus,
    ) -> i32 {
        for ps in &self.volatile_statuses[side] {
            if ps.0 == volatile_status {
                return ps.1;
            }
        }
        0
    }

    fn do_action(
        &mut self,
        battle_settings: &BattleSettings,
        creatures: &[Vec<Creature>; 2],
        creature_instances: &mut [Vec<CreatureInstance>; 2],
        combat_actions: &[CombatAction; 2],
        actioner: bool,
    ) {
        match combat_actions[actioner as usize] {
            CombatAction::Attack(move_id) => self.use_move(
                battle_settings,
                creatures,
                creature_instances,
                move_id as usize,
                actioner,
            ),
            CombatAction::Switch(switch_to_id) => {
                self.switch(creature_instances, switch_to_id as usize, actioner as usize)
            }
        };
    }
    fn get_move_id(
        &self,
        creatures: &[Vec<Creature>; 2],
        move_id: usize,
        actioner: usize,
    ) -> MoveID {
        creatures[actioner][self.battler_ids[actioner]].moves[move_id]
            .id
            .clone()
    }
    fn switch(
        &mut self,
        creature_instances: &mut [Vec<CreatureInstance>; 2],
        switch_to_id: usize,
        actioner: usize,
    ) {
        if creature_instances[actioner][switch_to_id].is_fainted() {
            panic!("Can't switch to a fainted");
        }
        self.volatile_statuses[actioner] = vec![];
        self.battler_ids[actioner] = switch_to_id;
        // TODO consider add move spikes here
    }

    fn take_damage(
        &mut self,
        creature_instances: &mut [Vec<CreatureInstance>; 2],
        damage_taker: usize,
        damage: i32,
    ) {
        if damage < 0 {
            panic!("Damage shouldnt' be negative");
        }
        creature_instances[damage_taker][self.battler_ids[damage_taker]].current_health -= damage;
    }

    pub fn get_base_chance_of_success(
        &self,
        battle_settings: &BattleSettings,
        attacker: &Creature,
        actioner: bool,
        move_id: usize,
    ) -> Option<f32> {
        match self.get_chance_of_success(attacker, actioner, move_id) {
            Some(chance) => {
                if !battle_settings.always_hits {
                    let random = rand::random();
                    assert!(random <= 1.0);
                    assert!(random >= 0.0);
                    if chance < random {
                        return None;
                    }
                }
                Some(chance)
            }
            None => Some(1.0),
        }
    }

    fn get_chance_of_success(
        &self,
        creature: &Creature,
        actioner: bool,
        move_id: usize,
    ) -> Option<f32> {
        match creature.moves[move_id].chance_of_success {
            Some(base_chance) => {
                let acc_stage = self.get_stage_from_value_volatile_status(
                    actioner as usize,
                    VolatileStatus::AccStage,
                );
                let eva_stage = self.get_stage_from_value_volatile_status(
                    !actioner as usize,
                    VolatileStatus::EvaStage,
                );
                let uncapped_chance = get_acc_stage_multiplier(acc_stage, eva_stage) * base_chance;
                if uncapped_chance > 1.0 {
                    Some(1.0)
                } else {
                    Some(uncapped_chance)
                }
            }
            None => None,
        }
    }

    /// Only accurate with moves with only one move effect and will automatically test with optimistic BattleSettings currently
    /// Some naughty repeating :(
    fn check_move_damage(
        &mut self,
        battle_settings: &BattleSettings,
        creatures: &[Vec<Creature>; 2],
        move_id: usize,
        actioner: bool,
    ) -> i32 {
        let mut damage = 0;
        let attacker = &creatures[actioner as usize][self.battler_ids[actioner as usize]];
        let mut success = true;
        let base_hit_chance =
            match self.get_base_chance_of_success(battle_settings, attacker, actioner, move_id) {
                Some(base_chance) => base_chance,
                None => return 0,
            };
        for unit in &attacker.moves[move_id].units {
            if !success {
                if unit.continues_previous_unit {
                    break;
                }
            }
            if unit.chance_of_success < rand::random() {
                success = false;
                continue;
            }
            let damage_taker = ((!actioner) ^ unit.target_self) as usize;
            damage += match unit.effect {
                Effect::PhysicalAttack => self.attack_damage(
                    battle_settings,
                    attacker,
                    &creatures[damage_taker][self.battler_ids[damage_taker]],
                    actioner as usize,
                    damage_taker,
                    true,
                    unit.power.unwrap(),
                    attacker.level as i32,
                    &attacker.moves[move_id].move_type,
                    base_hit_chance,
                ),
                Effect::SpecialAttack => self.attack_damage(
                    battle_settings,
                    attacker,
                    &creatures[damage_taker][self.battler_ids[damage_taker]],
                    actioner as usize,
                    damage_taker,
                    false,
                    unit.power.unwrap(),
                    attacker.level as i32,
                    &attacker.moves[move_id].move_type,
                    base_hit_chance,
                ),
                Effect::Unimplemented => unimplemented!(
                    "Not yet implemented the unit for {:?}",
                    &attacker.moves[move_id]
                ),
                Effect::ValueVolatileStatusChange(_) => 0,
            };
        }
        damage
    }

    fn use_move(
        &mut self,
        battle_settings: &BattleSettings,
        creatures: &[Vec<Creature>; 2],
        creature_instances: &mut [Vec<CreatureInstance>; 2],
        move_id: usize,
        actioner: bool,
    ) {
        let attacker = &creatures[actioner as usize][self.battler_ids[actioner as usize]];
        let mut success = true;
        let base_hit_chance =
            match self.get_base_chance_of_success(battle_settings, attacker, actioner, move_id) {
                Some(base_chance) => base_chance,
                None => return,
            };
        for unit in &attacker.moves[move_id].units {
            // check did the last loop succeed
            if !success {
                if unit.continues_previous_unit {
                    // TODO check if there is any move where we only skip units in the middle
                    break;
                }
            }
            if unit.chance_of_success < rand::random() {
                success = false;
                continue;
            }
            // TODO check is thsi success implemented right
            success = match unit.effect {
                Effect::PhysicalAttack => self.attack(
                    battle_settings,
                    creatures,
                    creature_instances,
                    true,
                    unit.power.unwrap(),
                    attacker.level as i32,
                    &attacker.moves[move_id].move_type,
                    actioner as usize,
                    ((!actioner) ^ unit.target_self) as usize,
                    base_hit_chance,
                ),
                Effect::SpecialAttack => self.attack(
                    battle_settings,
                    creatures,
                    creature_instances,
                    false,
                    unit.power.unwrap(),
                    attacker.level as i32,
                    &attacker.moves[move_id].move_type,
                    actioner as usize,
                    ((!actioner) ^ unit.target_self) as usize,
                    base_hit_chance,
                ),
                Effect::Unimplemented => unimplemented!(
                    "Not yet implemented the unit for {:?}",
                    &attacker.moves[move_id]
                ),
                // Set or addition
                Effect::ValueVolatileStatusChange(ref volatile_status) => self
                    .value_volatile_status_change(
                        volatile_status,
                        &unit.power.unwrap(),
                        ((!actioner) ^ unit.target_self) as usize,
                    ),
            };
        }
    }

    fn volatile_status_with_turn_range(
        &mut self,
        target: usize,
        value_volatile_status: &VolatileStatus,
        min_turns: i32,
        max_turns: i32,
    ) -> bool {
        if self
            .get_value_from_value_volatile_status(target, value_volatile_status.clone())
            .is_some()
        {
            return false;
        }
        let turns_remaining = (rand::random::<i32>() % min_turns) + (max_turns - min_turns); // TODO tweak this // TODO end turn might lead to bugs as end turn is dependant who starts faster and who is targeted
        self.volatile_statuses[target].push((value_volatile_status.clone(), turns_remaining));
        true
    }

    /// Currently limited to -6 and 6. Only for staged changes most likely
    fn value_volatile_status_change(
        &mut self,
        volatile_status: &VolatileStatus,
        change: &i32,
        side: usize,
    ) -> bool {
        for ps in &mut self.volatile_statuses[side] {
            if &ps.0 == volatile_status {
                if ps.1 + change > 6 {
                    ps.1 = 6;
                } else if ps.1 + change < -6 {
                    ps.1 = -6;
                } else {
                    ps.1 += change
                }
                return true; // TODO consider if it is a fail to add to already maxed one
            }
        }
        self.volatile_statuses[side].push((volatile_status.clone(), *change));
        true
    }

    fn attack_damage(
        &self,
        battle_settings: &BattleSettings,
        attacker: &Creature,
        damage_taker: &Creature,
        attacker_side: usize,
        damage_taker_side: usize,
        physical: bool,
        power: i32,
        level: i32,
        attack_type: &Type,
        base_hit_chance: f32,
    ) -> i32 {
        let (attack, defense, attack_stage, defense_stage) = if physical {
            (
                attacker.stats.atk,
                damage_taker.stats.def,
                self.get_stage_from_value_volatile_status(attacker_side, VolatileStatus::AtkStage),
                self.get_stage_from_value_volatile_status(
                    damage_taker_side,
                    VolatileStatus::DefStage,
                ),
            )
        } else {
            (
                attacker.stats.spa,
                damage_taker.stats.spd,
                self.get_stage_from_value_volatile_status(attacker_side, VolatileStatus::SpaStage),
                self.get_stage_from_value_volatile_status(
                    damage_taker_side,
                    VolatileStatus::SpdStage,
                ),
            )
        };
        let crit_stage =
            self.get_stage_from_value_volatile_status(attacker_side, VolatileStatus::CrtStage);
        let crit_chance = get_crit_chance(crit_stage);
        let (is_crit, bonus_crit_power) = if battle_settings.crit_enabled {
            if rand::random::<f32>() < crit_chance {
                (true, 2.0)
            } else {
                (false, 1.0)
            }
        } else {
            (false, 1.0 + crit_chance)
        };
        let bonus_power = if battle_settings.always_hits {
            bonus_crit_power * base_hit_chance
        } else {
            bonus_crit_power
        };
        let chance_bonuses = match battle_settings.roll {
            Roll::RandomRoll => rand::thread_rng().gen_range(0.85..1.0),
            Roll::HighRoll => 1.0,
            Roll::AverageRoll => 0.925,
        } * bonus_power;
        let stab_bonus = attacker.get_stab_modifier(attack_type);
        let type_effectiviness = damage_taker.effectiviness_when_attacked(attack_type);

        calculate_damage(
            power,
            attack,
            defense,
            level,
            attack_stage,
            defense_stage,
            stab_bonus,
            type_effectiviness.clone().into(),
            chance_bonuses,
            is_crit,
        ) as i32
    }

    fn attack(
        &mut self,
        battle_settings: &BattleSettings,
        creatures: &[Vec<Creature>; 2],
        creature_instances: &mut [Vec<CreatureInstance>; 2],
        physical: bool,
        power: i32,
        level: i32,
        attack_type: &Type,
        attacker: usize,
        damage_taker: usize,
        base_hit_chance: f32,
    ) -> bool {
        let damage = self.attack_damage(
            battle_settings,
            &creatures[attacker][self.battler_ids[attacker]],
            &creatures[damage_taker][self.battler_ids[damage_taker]],
            attacker,
            damage_taker,
            physical,
            power,
            level,
            attack_type,
            base_hit_chance,
        );
        self.take_damage(creature_instances, damage_taker, damage);
        true
    }

    // TODO add volatile status checks
    fn is_first_faster(
        &self,
        creatures: &[Vec<Creature>; 2],
        combat_actions: &[CombatAction; 2],
    ) -> bool {
        let first_priority = match combat_actions[0] {
            CombatAction::Attack(attack_id) => {
                self.get_battler(0, creatures).moves[attack_id as usize].priority
            }
            CombatAction::Switch(_) => -6,
        };
        let second_priority = match combat_actions[1] {
            CombatAction::Attack(attack_id) => {
                self.get_battler(1, creatures).moves[attack_id as usize].priority
            }
            CombatAction::Switch(_) => -6,
        };
        if first_priority == second_priority {
            let speed_0 = (self.get_battler(0, creatures).stats.spe as f32
                * get_stat_stage_multiplier(
                    self.get_stage_from_value_volatile_status(0, VolatileStatus::SpeStage),
                )) as i32;
            let speed_1 = (self.get_battler(1, creatures).stats.spe as f32
                * get_stat_stage_multiplier(
                    self.get_stage_from_value_volatile_status(1, VolatileStatus::SpeStage),
                )) as i32;
            if speed_0 == speed_1 {
                random_roll()
            } else if speed_0 > speed_1 {
                true
            } else {
                false
            }
        } else if first_priority > second_priority {
            true
        } else {
            false
        }
    }
    fn get_battler<'a>(&'a self, side: usize, creatures: &'a [Vec<Creature>; 2]) -> &Creature {
        &creatures[side][self.battler_ids[side]]
    }
}
fn random_roll() -> bool {
    rand::random()
}

struct MoveManager {
    moves: Vec<Move>,
}

#[derive(Debug, Clone)]
pub struct Move {
    pub id: MoveID,
    pub move_type: Type,
    pub chance_of_success: Option<f32>,
    pub pp: u8,
    pub priority: i8,
    pub units: Vec<MoveUnit>,
}

#[derive(Debug, Clone)]
pub struct MoveUnit {
    chance_of_success: f32,
    power: Option<i32>,
    effect: Effect,
    needs_target: bool,
    target_self: bool,
    continues_previous_unit: bool,
}

#[derive(Debug, Clone, PartialEq)]
enum Effect {
    PhysicalAttack,
    SpecialAttack,
    ValueVolatileStatusChange(VolatileStatus),
    Unimplemented,
}
#[derive(PartialEq, Clone)]
pub enum Effectiviness {
    Neutral,
    Double,
    Half,
    Immune,
    Quadruple,
    Quarter,
}

impl Mul for Effectiviness {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        match self {
            Effectiviness::Neutral => match rhs {
                Effectiviness::Neutral => Effectiviness::Neutral,
                Effectiviness::Double => Effectiviness::Double,
                Effectiviness::Half => Effectiviness::Half,
                Effectiviness::Immune => Effectiviness::Immune,
                _ => panic!(),
            },
            Effectiviness::Double => match rhs {
                Effectiviness::Neutral => Effectiviness::Double,
                Effectiviness::Double => Effectiviness::Quadruple,
                Effectiviness::Half => Effectiviness::Neutral,
                Effectiviness::Immune => Effectiviness::Immune,
                _ => panic!(),
            },
            Effectiviness::Half => match rhs {
                Effectiviness::Neutral => Effectiviness::Half,
                Effectiviness::Double => Effectiviness::Neutral,
                Effectiviness::Half => Effectiviness::Quarter,
                Effectiviness::Immune => Effectiviness::Immune,
                _ => panic!(),
            },
            Effectiviness::Immune => Effectiviness::Immune,
            _ => panic!(),
        }
    }
}

impl From<Effectiviness> for f32 {
    fn from(item: Effectiviness) -> f32 {
        match item {
            Effectiviness::Neutral => 1.0,
            Effectiviness::Double => 2.0,
            Effectiviness::Half => 0.5,
            Effectiviness::Immune => 0.0,
            Effectiviness::Quadruple => 4.0,
            Effectiviness::Quarter => 0.25,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum Type {
    Normal = 0,
    Fighting,
    Flying,
    Poison,
    Ground,
    Rock,
    Bug,
    Ghost,
    Steel,
    Fire,
    Water,
    Grass,
    Electric,
    Psychic,
    Ice,
    Dragon,
    Dark,
}

impl From<&str> for Type {
    fn from(item: &str) -> Self {
        match item {
            "Normal" => Type::Normal,
            "Fighting" => Type::Fighting,
            "Flying" => Type::Flying,
            "Poison" => Type::Poison,
            "Ground" => Type::Ground,
            "Rock" => Type::Rock,
            "Bug" => Type::Bug,
            "Ghost" => Type::Ghost,
            "Steel" => Type::Steel,
            "Fire" => Type::Fire,
            "Water" => Type::Water,
            "Grass" => Type::Grass,
            "Electric" => Type::Electric,
            "Psychic" => Type::Psychic,
            "Ice" => Type::Ice,
            "Dragon" => Type::Dragon,
            "Dark" => Type::Dark,
            _ => unimplemented!("Type not implemented {}", item),
        }
    }
}

impl From<i32> for Type {
    fn from(item: i32) -> Self {
        match item {
            0 => Type::Normal,
            1 => Type::Fighting,
            2 => Type::Flying,
            3 => Type::Poison,
            4 => Type::Ground,
            5 => Type::Rock,
            6 => Type::Bug,
            7 => Type::Ghost,
            8 => Type::Steel,
            9 => Type::Fire,
            10 => Type::Water,
            11 => Type::Grass,
            12 => Type::Electric,
            13 => Type::Psychic,
            14 => Type::Ice,
            15 => Type::Dragon,
            16 => Type::Dark,
            _ => unimplemented!("Type not implemented {}", item),
        }
    }
}

impl Type {
    // TODO triple check these
    pub fn effectiviness_when_attacked(&self, attacked_by: &Type) -> Effectiviness {
        match self {
            Type::Normal => match attacked_by {
                Type::Normal => Effectiviness::Neutral,
                Type::Fighting => Effectiviness::Double,
                Type::Flying => Effectiviness::Neutral,
                Type::Poison => Effectiviness::Neutral,
                Type::Ground => Effectiviness::Neutral,
                Type::Rock => Effectiviness::Neutral,
                Type::Bug => Effectiviness::Neutral,
                Type::Ghost => Effectiviness::Immune,
                Type::Steel => Effectiviness::Neutral,
                Type::Fire => Effectiviness::Neutral,
                Type::Water => Effectiviness::Neutral,
                Type::Grass => Effectiviness::Neutral,
                Type::Electric => Effectiviness::Neutral,
                Type::Psychic => Effectiviness::Neutral,
                Type::Ice => Effectiviness::Neutral,
                Type::Dragon => Effectiviness::Neutral,
                Type::Dark => Effectiviness::Neutral,
            },
            Type::Fighting => match attacked_by {
                Type::Normal => Effectiviness::Neutral,
                Type::Fighting => Effectiviness::Neutral,
                Type::Flying => Effectiviness::Double,
                Type::Poison => Effectiviness::Neutral,
                Type::Ground => Effectiviness::Neutral,
                Type::Rock => Effectiviness::Half,
                Type::Bug => Effectiviness::Half,
                Type::Ghost => Effectiviness::Neutral,
                Type::Steel => Effectiviness::Neutral,
                Type::Fire => Effectiviness::Neutral,
                Type::Water => Effectiviness::Neutral,
                Type::Grass => Effectiviness::Neutral,
                Type::Electric => Effectiviness::Neutral,
                Type::Psychic => Effectiviness::Double,
                Type::Ice => Effectiviness::Neutral,
                Type::Dragon => Effectiviness::Neutral,
                Type::Dark => Effectiviness::Half,
            },
            Type::Flying => match attacked_by {
                Type::Normal => Effectiviness::Neutral,
                Type::Fighting => Effectiviness::Half,
                Type::Flying => Effectiviness::Neutral,
                Type::Poison => Effectiviness::Neutral,
                Type::Ground => Effectiviness::Immune,
                Type::Rock => Effectiviness::Double,
                Type::Bug => Effectiviness::Half,
                Type::Ghost => Effectiviness::Neutral,
                Type::Steel => Effectiviness::Neutral,
                Type::Fire => Effectiviness::Neutral,
                Type::Water => Effectiviness::Neutral,
                Type::Grass => Effectiviness::Half,
                Type::Electric => Effectiviness::Double,
                Type::Psychic => Effectiviness::Neutral,
                Type::Ice => Effectiviness::Double,
                Type::Dragon => Effectiviness::Neutral,
                Type::Dark => Effectiviness::Neutral,
            },
            Type::Poison => match attacked_by {
                Type::Normal => Effectiviness::Neutral,
                Type::Fighting => Effectiviness::Half,
                Type::Flying => Effectiviness::Neutral,
                Type::Poison => Effectiviness::Half,
                Type::Ground => Effectiviness::Double,
                Type::Rock => Effectiviness::Neutral,
                Type::Bug => Effectiviness::Neutral,
                Type::Ghost => Effectiviness::Neutral,
                Type::Steel => Effectiviness::Neutral,
                Type::Fire => Effectiviness::Neutral,
                Type::Water => Effectiviness::Neutral,
                Type::Grass => Effectiviness::Half,
                Type::Electric => Effectiviness::Neutral,
                Type::Psychic => Effectiviness::Double,
                Type::Ice => Effectiviness::Neutral,
                Type::Dragon => Effectiviness::Neutral,
                Type::Dark => Effectiviness::Neutral,
            },
            Type::Ground => match attacked_by {
                Type::Normal => Effectiviness::Neutral,
                Type::Fighting => Effectiviness::Neutral,
                Type::Flying => Effectiviness::Neutral,
                Type::Poison => Effectiviness::Neutral,
                Type::Ground => Effectiviness::Half,
                Type::Rock => Effectiviness::Neutral,
                Type::Bug => Effectiviness::Half,
                Type::Ghost => Effectiviness::Neutral,
                Type::Steel => Effectiviness::Neutral,
                Type::Fire => Effectiviness::Neutral,
                Type::Water => Effectiviness::Double,
                Type::Grass => Effectiviness::Double,
                Type::Electric => Effectiviness::Immune,
                Type::Psychic => Effectiviness::Neutral,
                Type::Ice => Effectiviness::Double,
                Type::Dragon => Effectiviness::Neutral,
                Type::Dark => Effectiviness::Neutral,
            },
            Type::Rock => match attacked_by {
                Type::Normal => Effectiviness::Half,
                Type::Fighting => Effectiviness::Double,
                Type::Flying => Effectiviness::Half,
                Type::Poison => Effectiviness::Half,
                Type::Ground => Effectiviness::Double,
                Type::Rock => Effectiviness::Neutral,
                Type::Bug => Effectiviness::Neutral,
                Type::Ghost => Effectiviness::Neutral,
                Type::Steel => Effectiviness::Double,
                Type::Fire => Effectiviness::Half,
                Type::Water => Effectiviness::Double,
                Type::Grass => Effectiviness::Double,
                Type::Electric => Effectiviness::Neutral,
                Type::Psychic => Effectiviness::Neutral,
                Type::Ice => Effectiviness::Neutral,
                Type::Dragon => Effectiviness::Neutral,
                Type::Dark => Effectiviness::Neutral,
            },
            Type::Bug => match attacked_by {
                Type::Normal => Effectiviness::Neutral,
                Type::Fighting => Effectiviness::Half,
                Type::Flying => Effectiviness::Double,
                Type::Poison => Effectiviness::Neutral,
                Type::Ground => Effectiviness::Half,
                Type::Rock => Effectiviness::Double,
                Type::Bug => Effectiviness::Neutral,
                Type::Ghost => Effectiviness::Neutral,
                Type::Steel => Effectiviness::Neutral,
                Type::Fire => Effectiviness::Double,
                Type::Water => Effectiviness::Neutral,
                Type::Grass => Effectiviness::Half,
                Type::Electric => Effectiviness::Neutral,
                Type::Psychic => Effectiviness::Neutral,
                Type::Ice => Effectiviness::Neutral,
                Type::Dragon => Effectiviness::Neutral,
                Type::Dark => Effectiviness::Neutral,
            },
            Type::Ghost => match attacked_by {
                Type::Normal => Effectiviness::Immune,
                Type::Fighting => Effectiviness::Immune,
                Type::Flying => Effectiviness::Neutral,
                Type::Poison => Effectiviness::Half,
                Type::Ground => Effectiviness::Neutral,
                Type::Rock => Effectiviness::Neutral,
                Type::Bug => Effectiviness::Half,
                Type::Ghost => Effectiviness::Double,
                Type::Steel => Effectiviness::Neutral,
                Type::Fire => Effectiviness::Neutral,
                Type::Water => Effectiviness::Neutral,
                Type::Grass => Effectiviness::Neutral,
                Type::Electric => Effectiviness::Neutral,
                Type::Psychic => Effectiviness::Neutral,
                Type::Ice => Effectiviness::Neutral,
                Type::Dragon => Effectiviness::Neutral,
                Type::Dark => Effectiviness::Double,
            },
            Type::Steel => match attacked_by {
                Type::Normal => Effectiviness::Half,
                Type::Fighting => Effectiviness::Double,
                Type::Flying => Effectiviness::Half,
                Type::Poison => Effectiviness::Immune,
                Type::Ground => Effectiviness::Double,
                Type::Rock => Effectiviness::Half,
                Type::Bug => Effectiviness::Half,
                Type::Ghost => Effectiviness::Half,
                Type::Steel => Effectiviness::Half,
                Type::Fire => Effectiviness::Double,
                Type::Water => Effectiviness::Neutral,
                Type::Grass => Effectiviness::Half,
                Type::Electric => Effectiviness::Neutral,
                Type::Psychic => Effectiviness::Half,
                Type::Ice => Effectiviness::Half,
                Type::Dragon => Effectiviness::Half,
                Type::Dark => Effectiviness::Half,
            },
            Type::Fire => match attacked_by {
                Type::Normal => Effectiviness::Neutral,
                Type::Fighting => Effectiviness::Neutral,
                Type::Flying => Effectiviness::Neutral,
                Type::Poison => Effectiviness::Neutral,
                Type::Ground => Effectiviness::Double,
                Type::Rock => Effectiviness::Double,
                Type::Bug => Effectiviness::Half,
                Type::Ghost => Effectiviness::Neutral,
                Type::Steel => Effectiviness::Half,
                Type::Fire => Effectiviness::Half,
                Type::Water => Effectiviness::Double,
                Type::Grass => Effectiviness::Half,
                Type::Electric => Effectiviness::Neutral,
                Type::Psychic => Effectiviness::Neutral,
                Type::Ice => Effectiviness::Half,
                Type::Dragon => Effectiviness::Neutral,
                Type::Dark => Effectiviness::Neutral,
            },
            Type::Water => match attacked_by {
                Type::Normal => Effectiviness::Neutral,
                Type::Fighting => Effectiviness::Neutral,
                Type::Flying => Effectiviness::Neutral,
                Type::Poison => Effectiviness::Neutral,
                Type::Ground => Effectiviness::Neutral,
                Type::Rock => Effectiviness::Neutral,
                Type::Bug => Effectiviness::Neutral,
                Type::Ghost => Effectiviness::Neutral,
                Type::Steel => Effectiviness::Half,
                Type::Fire => Effectiviness::Half,
                Type::Water => Effectiviness::Half,
                Type::Grass => Effectiviness::Double,
                Type::Electric => Effectiviness::Double,
                Type::Psychic => Effectiviness::Neutral,
                Type::Ice => Effectiviness::Half,
                Type::Dragon => Effectiviness::Neutral,
                Type::Dark => Effectiviness::Neutral,
            },
            Type::Grass => match attacked_by {
                Type::Normal => Effectiviness::Neutral,
                Type::Fighting => Effectiviness::Neutral,
                Type::Flying => Effectiviness::Double,
                Type::Poison => Effectiviness::Double,
                Type::Ground => Effectiviness::Half,
                Type::Rock => Effectiviness::Neutral,
                Type::Bug => Effectiviness::Double,
                Type::Ghost => Effectiviness::Neutral,
                Type::Steel => Effectiviness::Neutral,
                Type::Fire => Effectiviness::Double,
                Type::Water => Effectiviness::Half,
                Type::Grass => Effectiviness::Half,
                Type::Electric => Effectiviness::Half,
                Type::Psychic => Effectiviness::Neutral,
                Type::Ice => Effectiviness::Double,
                Type::Dragon => Effectiviness::Neutral,
                Type::Dark => Effectiviness::Neutral,
            },
            Type::Electric => match attacked_by {
                Type::Normal => Effectiviness::Neutral,
                Type::Fighting => Effectiviness::Neutral,
                Type::Flying => Effectiviness::Half,
                Type::Poison => Effectiviness::Neutral,
                Type::Ground => Effectiviness::Double,
                Type::Rock => Effectiviness::Neutral,
                Type::Bug => Effectiviness::Neutral,
                Type::Ghost => Effectiviness::Neutral,
                Type::Steel => Effectiviness::Half,
                Type::Fire => Effectiviness::Neutral,
                Type::Water => Effectiviness::Neutral,
                Type::Grass => Effectiviness::Neutral,
                Type::Electric => Effectiviness::Half,
                Type::Psychic => Effectiviness::Neutral,
                Type::Ice => Effectiviness::Neutral,
                Type::Dragon => Effectiviness::Neutral,
                Type::Dark => Effectiviness::Neutral,
            },
            Type::Psychic => match attacked_by {
                Type::Normal => Effectiviness::Neutral,
                Type::Fighting => Effectiviness::Half,
                Type::Flying => Effectiviness::Neutral,
                Type::Poison => Effectiviness::Neutral,
                Type::Ground => Effectiviness::Neutral,
                Type::Rock => Effectiviness::Neutral,
                Type::Bug => Effectiviness::Double,
                Type::Ghost => Effectiviness::Double,
                Type::Steel => Effectiviness::Neutral,
                Type::Fire => Effectiviness::Neutral,
                Type::Water => Effectiviness::Neutral,
                Type::Grass => Effectiviness::Neutral,
                Type::Electric => Effectiviness::Neutral,
                Type::Psychic => Effectiviness::Half,
                Type::Ice => Effectiviness::Neutral,
                Type::Dragon => Effectiviness::Neutral,
                Type::Dark => Effectiviness::Double,
            },
            Type::Ice => match attacked_by {
                Type::Normal => Effectiviness::Neutral,
                Type::Fighting => Effectiviness::Double,
                Type::Flying => Effectiviness::Neutral,
                Type::Poison => Effectiviness::Neutral,
                Type::Ground => Effectiviness::Neutral,
                Type::Rock => Effectiviness::Double,
                Type::Bug => Effectiviness::Neutral,
                Type::Ghost => Effectiviness::Neutral,
                Type::Steel => Effectiviness::Double,
                Type::Fire => Effectiviness::Double,
                Type::Water => Effectiviness::Neutral,
                Type::Grass => Effectiviness::Neutral,
                Type::Electric => Effectiviness::Neutral,
                Type::Psychic => Effectiviness::Neutral,
                Type::Ice => Effectiviness::Half,
                Type::Dragon => Effectiviness::Neutral,
                Type::Dark => Effectiviness::Neutral,
            },
            Type::Dragon => match attacked_by {
                Type::Normal => Effectiviness::Neutral,
                Type::Fighting => Effectiviness::Neutral,
                Type::Flying => Effectiviness::Neutral,
                Type::Poison => Effectiviness::Neutral,
                Type::Ground => Effectiviness::Neutral,
                Type::Rock => Effectiviness::Neutral,
                Type::Bug => Effectiviness::Neutral,
                Type::Ghost => Effectiviness::Neutral,
                Type::Steel => Effectiviness::Neutral,
                Type::Fire => Effectiviness::Half,
                Type::Water => Effectiviness::Half,
                Type::Grass => Effectiviness::Half,
                Type::Electric => Effectiviness::Half,
                Type::Psychic => Effectiviness::Neutral,
                Type::Ice => Effectiviness::Double,
                Type::Dragon => Effectiviness::Double,
                Type::Dark => Effectiviness::Neutral,
            },
            Type::Dark => match attacked_by {
                Type::Normal => Effectiviness::Neutral,
                Type::Fighting => Effectiviness::Double,
                Type::Flying => Effectiviness::Neutral,
                Type::Poison => Effectiviness::Neutral,
                Type::Ground => Effectiviness::Neutral,
                Type::Rock => Effectiviness::Neutral,
                Type::Bug => Effectiviness::Double,
                Type::Ghost => Effectiviness::Half,
                Type::Steel => Effectiviness::Neutral,
                Type::Fire => Effectiviness::Neutral,
                Type::Water => Effectiviness::Neutral,
                Type::Grass => Effectiviness::Neutral,
                Type::Electric => Effectiviness::Neutral,
                Type::Psychic => Effectiviness::Immune,
                Type::Ice => Effectiviness::Neutral,
                Type::Dragon => Effectiviness::Neutral,
                Type::Dark => Effectiviness::Half,
            },
        }
    }
}

#[derive(Debug, Clone)]
// #[serde(rename_all = "PascalCase")]
pub struct Creature {
    pub species: String,
    pub level: i32,
    pub moves: [Move; 4],
    pub stats: Stats,
    pub types: Vec<Type>,
}

impl Creature {
    pub fn effectiviness_when_attacked(&self, attacked_by: &Type) -> Effectiviness {
        match self.types.len() {
            1 => self.types[0].effectiviness_when_attacked(attacked_by),
            2 => {
                self.types[0].effectiviness_when_attacked(attacked_by)
                    * self.types[1].effectiviness_when_attacked(attacked_by)
            }
            _ => panic!("Creature has not valid amount of types"),
        }
    }

    pub fn generate_creature(creature_generator: &CreatureGenerator) -> Creature {
        let (base_stats, sum) = creature_generator
            .base_stats_generation
            .get_base_stats_with_sum();
        let level = creature_generator.level_generation.get_level(sum);
        let stats = Stats::new(base_stats, level);
        let types =
            get_creature_types(creature_generator.dual_type_chance, &mut rand::thread_rng());
        let moves = creature_generator
            .move_generation_settings
            .generate_move_set(&mut types.clone());

        // let level = creature_generator.
        Creature {
            species: "generated".to_string(),
            level,
            moves,
            stats,
            types,
        }
    }

    pub fn get_stab_modifier(&self, attack_type: &Type) -> f32 {
        for creature_type in &self.types {
            if creature_type == attack_type {
                return 1.5;
            }
        }
        1.0
    }

    // When equal favors earlier stats
    pub fn estimate_lowest_and_highest_base_stat_id(&self) -> (usize, usize) {
        let mut lowest = Stats::estimate_stat_from_hp(self.stats.hp, self.level);
        let mut highest = Stats::estimate_stat_from_hp(self.stats.hp, self.level);
        let mut lowest_index = 0;
        let mut highest_index = 0;
        let stats: [i32; 5] = self.stats.into();
        for i in 0..5 {
            if stats[i] < lowest {
                lowest = stats[i];
                lowest_index = i + 1;
            } else if stats[i] > highest {
                highest = stats[i];
                highest_index = i + 1;
            }
        }
        (lowest_index, highest_index)
    }
}

#[derive(Clone, Debug)]
pub struct CreatureInstance {
    pub current_health: i32,
}

impl CreatureInstance {
    fn is_fainted(&self) -> bool {
        self.current_health <= 0
    }
}

#[derive(Serialize, Debug, Deserialize, Clone, Copy)]
pub struct Stats {
    pub hp: i32,
    pub atk: i32,
    pub def: i32,
    pub spa: i32,
    pub spd: i32,
    pub spe: i32,
}

pub enum LevelGeneration {
    From80To89Uniform,
    From80To89BalancedLinearyLowRange,
    From80To89BalancedLinearyMedRange,
    From80To89BalancedLinearyHighRange,
    Only88,
}

impl LevelGeneration {
    pub fn get_level(&self, base_stat_sum: i32) -> i32 {
        match self {
            LevelGeneration::From80To89Uniform => rand::thread_rng().gen_range(80..=89),
            LevelGeneration::From80To89BalancedLinearyLowRange => {
                LevelGeneration::get_balanced_level(base_stat_sum, 73, 12)
            }
            LevelGeneration::From80To89BalancedLinearyMedRange => {
                LevelGeneration::get_balanced_level(base_stat_sum, 63, 24)
            }
            LevelGeneration::From80To89BalancedLinearyHighRange => {
                LevelGeneration::get_balanced_level(base_stat_sum, 53, 36)
            }
            LevelGeneration::Only88 => 88,
        }
    }

    fn get_balanced_level(base_stat_sum: i32, min_value: i32, increment: i32) -> i32 {
        let value = 9 - ((base_stat_sum - (min_value * 6)) / increment);
        80 + if base_stat_sum - (min_value * 6) == increment * 10 {
            0 // perfect base_stats are moved to one category lower
        } else {
            value
        }
    }
}

/// Base stats' sum about 500 chosen for now
pub enum BaseStatsGeneration {
    Average,
    SpreadLow,
    SpreadMed,
    SpreadHigh,
}

impl BaseStatsGeneration {
    pub fn get_base_stat(&self) -> i32 {
        match self {
            BaseStatsGeneration::Average => 83,
            BaseStatsGeneration::SpreadLow => rand::thread_rng().gen_range(73..=93),
            BaseStatsGeneration::SpreadMed => rand::thread_rng().gen_range(63..=103),
            BaseStatsGeneration::SpreadHigh => rand::thread_rng().gen_range(53..=113),
        }
    }

    pub fn get_base_stats_with_sum(&self) -> ([i32; 6], i32) {
        let mut base_stats = [0; 6];
        let mut sum = 0;
        for i in 0..6 {
            let base_stat = self.get_base_stat();
            sum += base_stat;
            base_stats[i] = base_stat;
        }
        (base_stats, sum)
    }
}

pub struct CreatureGenerator {
    pub move_generation_settings: MoveGenerationSettings,
    pub base_stats_generation: BaseStatsGeneration,
    pub level_generation: LevelGeneration,
    pub dual_type_chance: f64, // Adding Type Generations enable custom type chances
    pub has_speed_tie_removal: bool,
}

impl Default for CreatureGenerator {
    fn default() -> Self {
        CreatureGenerator {
            move_generation_settings: MoveGenerationSettings::default(),
            base_stats_generation: BaseStatsGeneration::SpreadHigh,
            level_generation: LevelGeneration::From80To89BalancedLinearyHighRange,
            dual_type_chance: 0.9,
            has_speed_tie_removal: true,
        }
    }
}
impl CreatureGenerator {
    fn no_random() -> Self {
        CreatureGenerator {
            move_generation_settings: MoveGenerationSettings::new(0, 0, 1, 1, 0, 0, 0, 0, 0, 1, 0),
            base_stats_generation: BaseStatsGeneration::Average,
            level_generation: LevelGeneration::Only88,
            dual_type_chance: 0.0,
            has_speed_tie_removal: true, // When false more even, but less deterministic battles
        }
    }
}

/// TODOS:
/// Currently always two stab moves
/// No setting for favoring physical or special
/// Could have bool for duplicate type removal
/// Should prevent duplicate stats modifier skills
/// Should add ratios to stats modifier stat
pub struct MoveGenerationSettings {
    low_attack_ratio: i32,
    med_attack_ratio: i32,
    high_attack_ratio: i32,
    attack_ratio: i32, // attack ratio should be low as there is already two attacks that are not rolled
    stats_mod_chance: i32,
    one_step_stat_ratio: i32,
    two_step_stat_ratio: i32,
    buff_ratio: i32,
    debuff_ratio: i32,
    always_hit_ratio: i32,
    missable_ratio: i32,
}

impl Default for MoveGenerationSettings {
    fn default() -> Self {
        MoveGenerationSettings {
            low_attack_ratio: 1,
            med_attack_ratio: 1,
            high_attack_ratio: 1,
            attack_ratio: 1,
            stats_mod_chance: 1,
            one_step_stat_ratio: 1,
            two_step_stat_ratio: 1,
            buff_ratio: 1,
            debuff_ratio: 1,
            always_hit_ratio: 1,
            missable_ratio: 1,
        }
    }
}

impl MoveGenerationSettings {
    pub fn new(
        low_attack_ratio: i32,
        med_attack_ratio: i32,
        high_attack_ratio: i32,
        attack_ratio: i32,
        stats_mod_chance: i32,
        one_step_stat_ratio: i32,
        two_step_stat_ratio: i32,
        buff_ratio: i32,
        debuff_ratio: i32,
        always_hit_ratio: i32,
        missable_ratio: i32,
    ) -> Self {
        MoveGenerationSettings {
            low_attack_ratio,
            med_attack_ratio,
            high_attack_ratio,
            attack_ratio,
            stats_mod_chance,
            one_step_stat_ratio,
            two_step_stat_ratio,
            buff_ratio,
            debuff_ratio,
            always_hit_ratio,
            missable_ratio,
        }
    }

    /// for attacks: 17 types * 2 effects * 6 different base moves = 204
    /// for stat modifiers:
    pub fn generate_move_set(&self, types_used: &mut Vec<Type>) -> [Move; 4] {
        let mut rng = rand::thread_rng();
        let mut moves = vec![];
        for i in 0..types_used.len() {
            moves.push(self.get_base_attack(&mut rng, types_used[i]));
        }
        for _ in 0..(4 - types_used.len()) {
            let is_attack_value = rng.gen_range(0..(self.attack_ratio + self.stats_mod_chance));
            if is_attack_value < self.attack_ratio {
                let new_type = MoveGenerationSettings::get_new_type(&mut rng, &types_used);
                types_used.push(new_type);
                moves.push(self.get_base_attack(&mut rng, new_type));
            } else {
                moves.push(self.get_stat_modifier_move(&mut rng));
            }
        }
        // This is bit lazy .. sorry
        [
            moves[0].clone(),
            moves[1].clone(),
            moves[2].clone(),
            moves[3].clone(),
        ]
    }

    pub fn get_stat_modifier_move(&self, rng: &mut ThreadRng) -> Move {
        let is_buff_value = rng.gen_range(0..(self.buff_ratio + self.debuff_ratio));
        let is_one_step_value =
            rng.gen_range(0..(self.one_step_stat_ratio + self.two_step_stat_ratio));
        if is_buff_value < self.buff_ratio {
            if is_one_step_value < self.one_step_stat_ratio {
                (&MoveID::StatsUp(self.get_stat_category(rng))).into()
            } else {
                (&MoveID::StatsUpDouble(self.get_stat_category(rng))).into()
            }
        } else {
            if is_one_step_value < self.one_step_stat_ratio {
                (&MoveID::StatsDown(self.get_stat_category(rng))).into()
            } else {
                (&MoveID::StatsDownDouble(self.get_stat_category(rng))).into()
            }
        }
    }

    /// TODO add ratios here and more rules described above
    pub fn get_stat_category(&self, rng: &mut ThreadRng) -> VolatileStatus {
        VolatileStatus::from(rng.gen_range(0..8))
    }

    pub fn get_new_type(rng: &mut ThreadRng, types_used: &Vec<Type>) -> Type {
        loop {
            let new_type = Type::from(rng.gen_range(0..17));
            let mut duplicate = false;
            for old_type in types_used {
                if old_type == &new_type {
                    duplicate = true;
                }
            }
            if !duplicate {
                return new_type;
            }
        }
    }

    pub fn get_base_attack(&self, rng: &mut ThreadRng, move_type: Type) -> Move {
        let miss_value = rng.gen_range(0..(self.always_hit_ratio + self.missable_ratio));
        let strenght_value = rng
            .gen_range(0..(self.low_attack_ratio + self.med_attack_ratio + self.high_attack_ratio));
        let physical = rand::random::<bool>();
        if miss_value < self.always_hit_ratio {
            if strenght_value < self.low_attack_ratio {
                (&MoveID::DamageLow(physical, move_type)).into()
            } else if strenght_value < self.low_attack_ratio + self.med_attack_ratio {
                (&MoveID::DamageMed(physical, move_type)).into()
            } else {
                (&MoveID::DamageHigh(physical, move_type)).into()
            }
        } else {
            if strenght_value < self.low_attack_ratio {
                // Low value skill, because high miss rate
                (&MoveID::MissHigh(physical, move_type)).into()
            } else if strenght_value < self.low_attack_ratio + self.med_attack_ratio {
                (&MoveID::MissMed(physical, move_type)).into()
            } else {
                (&MoveID::MissLow(physical, move_type)).into()
            }
        }
    }
}

pub fn get_creature_types(dual_type_chance: f64, rng: &mut ThreadRng) -> Vec<Type> {
    let mut types = vec![Type::from(rng.gen_range(0..17))];
    if dual_type_chance == 0.0 || !rng.gen_bool(dual_type_chance) {
        return types;
    }
    loop {
        let second_type = Type::from(rng.gen_range(0..17));
        if types[0] != second_type {
            types.push(second_type);
            return types;
        }
    }
}

impl From<Stats> for [i32; 6] {
    fn from(value: Stats) -> Self {
        [
            value.hp, value.atk, value.def, value.spa, value.spd, value.spe,
        ]
    }
}
impl From<Stats> for [i32; 5] {
    fn from(value: Stats) -> Self {
        [value.atk, value.def, value.spa, value.spd, value.spe]
    }
}

impl Stats {
    pub fn new(base_stats: [i32; 6], level: i32) -> Stats {
        Stats {
            hp: Stats::calculate_hp(base_stats[0], level),
            atk: Stats::calculate_stat(base_stats[1], level),
            def: Stats::calculate_stat(base_stats[2], level),
            spa: Stats::calculate_stat(base_stats[3], level),
            spd: Stats::calculate_stat(base_stats[4], level),
            spe: Stats::calculate_stat(base_stats[5], level),
        }
    }

    pub fn get_slow_tank() -> Stats {
        let level = 88;
        Stats {
            hp: Stats::calculate_hp(146, level),
            atk: Stats::calculate_stat(28, level),
            def: Stats::calculate_stat(28, level),
            spa: Stats::calculate_stat(28, level),
            spd: Stats::calculate_stat(28, level),
            spe: Stats::calculate_stat(28, level),
        }
    }
    pub fn get_fast_tank() -> Stats {
        let level = 88;
        Stats {
            hp: Stats::calculate_hp(146, level),
            atk: Stats::calculate_stat(28, level),
            def: Stats::calculate_stat(28, level),
            spa: Stats::calculate_stat(28, level),
            spd: Stats::calculate_stat(28, level),
            spe: Stats::calculate_stat(146, level),
        }
    }

    /// IV = 31
    /// EV = floor(85/4) = 21
    fn calculate_hp(base_stat: i32, level: i32) -> i32 {
        (((2 * base_stat + 31 + 21) * level) / 100) + level + 10
    }
    fn estimate_base_stat_from_hp(hp: i32, level: i32) -> i32 {
        (((hp - level - 10) * 100 / level) - 31 - 21) / 2
    }
    fn estimate_stat_from_hp(hp: i32, level: i32) -> i32 {
        Stats::calculate_stat(Stats::estimate_base_stat_from_hp(hp, level), level)
    }
    fn calculate_stat(base_stat: i32, level: i32) -> i32 {
        (((2 * base_stat + 31 + 21) * level) / 100) + 5
    }
}

impl Default for Stats {
    fn default() -> Self {
        Stats {
            hp: 300,
            atk: 100,
            def: 100,
            spa: 100,
            spd: 100,
            spe: 100,
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum VolatileStatus {
    AtkStage = 0,
    DefStage,
    SpaStage,
    SpdStage,
    SpeStage,
    EvaStage,
    AccStage,
    CrtStage,
}

impl From<i32> for VolatileStatus {
    fn from(item: i32) -> Self {
        match item {
            0 => VolatileStatus::AtkStage,
            1 => VolatileStatus::DefStage,
            2 => VolatileStatus::SpaStage,
            3 => VolatileStatus::SpdStage,
            4 => VolatileStatus::SpeStage,
            5 => VolatileStatus::EvaStage,
            6 => VolatileStatus::AccStage,
            7 => VolatileStatus::CrtStage,
            _ => panic!("There is no volatile status for this value"),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use rand::random;

    use super::*;

    fn setup(
        creature_generator: &CreatureGenerator,
        team_size: usize,
    ) -> (
        [Vec<CreatureInstance>; 2],
        [Vec<Creature>; 2],
        BattleInstance,
        BattleSettings,
    ) {
        let creatures = get_team(creature_generator, team_size);
        (
            get_full_health_team(&creatures),
            creatures,
            BattleInstance::default(),
            BattleSettings::default(),
        )
    }

    fn get_team(creature_generator: &CreatureGenerator, size: usize) -> [Vec<Creature>; 2] {
        let mut first_team = vec![];
        let mut second_team = vec![];

        for _ in 0..size {
            first_team.push(Creature::generate_creature(creature_generator));
            second_team.push(Creature::generate_creature(creature_generator));
        }
        if creature_generator.has_speed_tie_removal {
            loop {
                let mut ties_removed = true;
                for i in 0..size {
                    for j in 0..size {
                        if first_team[i].stats.spe == second_team[j].stats.spe {
                            // in case of equal speed stats. Have deterministic fights.
                            // Don't favor just one side by letting in ties one side always start.
                            // Instead do some rerolling to both direction and check if we have created speed ties again
                            if rand::random::<bool>() {
                                second_team[j].stats.spe += 1;
                            } else {
                                second_team[j].stats.spe -= 1;
                            }
                            ties_removed = false;
                        }
                    }
                }
                if ties_removed {
                    break;
                }
            }
        }

        [first_team, second_team]
    }

    fn get_full_health_team(creatures: &[Vec<Creature>; 2]) -> [Vec<CreatureInstance>; 2] {
        let mut first_team = vec![];
        let mut second_team = vec![];
        for i in 0..creatures[0].len() {
            first_team.push(CreatureInstance {
                current_health: creatures[0][i].stats.hp,
            });
            second_team.push(CreatureInstance {
                current_health: creatures[1][i].stats.hp,
            });
        }
        [first_team, second_team]
    }

    #[test]
    fn convert_hp_to_base_stat_test() {
        let mut inaccuracy = 0;
        for base_stat in 53..=113 {
            for level in 80..90 {
                inaccuracy += (Stats::estimate_base_stat_from_hp(
                    Stats::calculate_hp(base_stat, level),
                    level,
                ) - base_stat)
                    * (Stats::estimate_base_stat_from_hp(
                        Stats::calculate_hp(base_stat, level),
                        level,
                    ) - base_stat);
            }
        }
        assert!(inaccuracy < 600);
    }

    #[test]
    fn level_generation() {
        let mut highest_value_tested = 0;
        for i in 0..10 {
            for j in 0..36 {
                highest_value_tested = 318 + j + i * 36;
                assert_eq!(
                    89 - i,
                    LevelGeneration::get_balanced_level(highest_value_tested, 53, 36),
                    "i: {} and j: {} and highest_value_tested: {}",
                    i,
                    j,
                    highest_value_tested
                );
            }
        }
        assert_eq!(highest_value_tested, 677);
        assert_eq!(80, LevelGeneration::get_balanced_level(678, 53, 36));
    }

    #[test]
    fn stat_generation_test() {
        // get_slow_tank uses 146 base stat for hp and 28 for others. level is 88
        let tanky_stats = Stats::get_slow_tank();
        assert_eq!(tanky_stats.atk, 100);
        assert_eq!(tanky_stats.hp, 400);
    }

    #[test]
    fn team_generation_test() {
        let mut average_turns = 0;
        let mut type_won = [0i32; 17];
        let mut type_lost = [0i32; 17];
        let mut won_with_highest_base_stats = [0i32; 6];
        let mut won_with_lowest_base_stats = [0i32; 6];
        let mut lost_with_highest_base_stats = [0i32; 6];
        let mut lost_with_lowest_base_stats = [0i32; 6];
        let mut won_with_specific_volatile_status = [0i32; 8];
        let mut lost_with_specific_volatile_status = [0i32; 8];
        let mut won_with_specific_base_move = [0i32; 10];
        let mut lost_with_specific_base_move = [0i32; 10];
        let mut turns_alive_for_type = [0i32; 17];
        let mut won_with_non_stab_type_move = [0i32; 17];
        let mut lost_with_non_stab_type_move = [0i32; 17];
        let mut won_level_distribution = [0i32; 10];
        let mut lost_level_distribution = [0i32; 10];
        for i in 0..100000 {
            let (mut creature_instances, creatures, mut battle_instance, battle_settings) =
                setup(&CreatureGenerator::default(), 3);
            let random_ai_a = RandomAI {};
            let random_ai_b = RandomAI {};
            let mut combat_action_1 = random_ai_a.get_action();
            let mut combat_action_2 = random_ai_b.get_action();
            for i in 0..1000 {
                for side in 0..2 {
                    turns_alive_for_type
                        [creatures[side][battle_instance.battler_ids[side]].types[0] as usize] += 1;
                }
                let interrupt_opt = battle_instance.turn(
                    &battle_settings,
                    &creatures,
                    &mut creature_instances,
                    &[combat_action_1.clone(), combat_action_2.clone()],
                );
                match battle_instance.handle_interrupts(
                    &mut creature_instances,
                    interrupt_opt,
                    &random_ai_a,
                    &random_ai_b,
                ) {
                    Ok(_) => (),
                    Err(interrupt) => {
                        match interrupt {
                            Interrupt::AWon => {
                                for creature in &creatures[0] {
                                    for i in 0..creature.types.len() {
                                        type_won[creature.types[i] as usize] += 1;
                                        won_level_distribution[creature.level as usize - 80] += 1;
                                        let (lowest_stat, highest_stat) =
                                            creature.estimate_lowest_and_highest_base_stat_id();
                                        won_with_lowest_base_stats[lowest_stat] += 1;
                                        won_with_highest_base_stats[highest_stat] += 1;
                                        for a_move in &creature.moves {
                                            won_with_specific_base_move
                                                [a_move.id.get_as_index()] += 1;
                                            match a_move.id.get_volatile_status() {
                                                Some(volatile_status) => {
                                                    won_with_specific_volatile_status
                                                        [volatile_status as usize] += 1
                                                }
                                                None => (),
                                            }
                                            if creature.types[i] != (*a_move).move_type {
                                                won_with_non_stab_type_move
                                                    [(*a_move).move_type as usize] += 1;
                                            }
                                        }
                                    }
                                }
                                for creature in &creatures[1] {
                                    for i in 0..creature.types.len() {
                                        lost_level_distribution[creature.level as usize - 80] += 1;
                                        type_lost[creature.types[i] as usize] += 1;
                                        let (lowest_stat, highest_stat) =
                                            creature.estimate_lowest_and_highest_base_stat_id();
                                        lost_with_lowest_base_stats[lowest_stat] += 1;
                                        lost_with_highest_base_stats[highest_stat] += 1;
                                        for a_move in &creature.moves {
                                            lost_with_specific_base_move
                                                [a_move.id.get_as_index()] += 1;
                                            match a_move.id.get_volatile_status() {
                                                Some(volatile_status) => {
                                                    lost_with_specific_volatile_status
                                                        [volatile_status as usize] += 1
                                                }
                                                None => (),
                                            }
                                            if creature.types[i] != (*a_move).move_type {
                                                lost_with_non_stab_type_move
                                                    [(*a_move).move_type as usize] += 1;
                                            }
                                        }
                                    }
                                }
                            }
                            Interrupt::BWon => {
                                for creature in &creatures[1] {
                                    for i in 0..creature.types.len() {
                                        won_level_distribution[creature.level as usize - 80] += 1;
                                        type_won[creature.types[i] as usize] += 1;
                                        let (lowest_stat, highest_stat) =
                                            creature.estimate_lowest_and_highest_base_stat_id();
                                        won_with_lowest_base_stats[lowest_stat] += 1;
                                        won_with_highest_base_stats[highest_stat] += 1;
                                        for a_move in &creature.moves {
                                            won_with_specific_base_move
                                                [a_move.id.get_as_index()] += 1;
                                            match a_move.id.get_volatile_status() {
                                                Some(volatile_status) => {
                                                    won_with_specific_volatile_status
                                                        [volatile_status as usize] += 1
                                                }
                                                None => (),
                                            }
                                            if creature.types[i] != (*a_move).move_type {
                                                won_with_non_stab_type_move
                                                    [(*a_move).move_type as usize] += 1;
                                            }
                                        }
                                    }
                                }
                                for creature in &creatures[0] {
                                    for i in 0..creature.types.len() {
                                        lost_level_distribution[creature.level as usize - 80] += 1;
                                        type_lost[creature.types[i] as usize] += 1;
                                        let (lowest_stat, highest_stat) =
                                            creature.estimate_lowest_and_highest_base_stat_id();
                                        lost_with_lowest_base_stats[lowest_stat] += 1;
                                        lost_with_highest_base_stats[highest_stat] += 1;
                                        for a_move in &creature.moves {
                                            lost_with_specific_base_move
                                                [a_move.id.get_as_index()] += 1;
                                            match a_move.id.get_volatile_status() {
                                                Some(volatile_status) => {
                                                    lost_with_specific_volatile_status
                                                        [volatile_status as usize] += 1
                                                }
                                                None => (),
                                            }
                                            if creature.types[i] != (*a_move).move_type {
                                                lost_with_non_stab_type_move
                                                    [(*a_move).move_type as usize] += 1;
                                            }
                                        }
                                    }
                                }
                            }
                            _ => panic!("faints should be handled in handle_interrupts"),
                        }
                        average_turns += i + 1;
                        break;
                    }
                }
                combat_action_1 = random_ai_a.get_action();
                combat_action_2 = random_ai_b.get_action();
            }
        }
        println!(
            "
        Normal won: {}
        Fighting won: {}
        Flying won: {}
        Poison won: {}
        Ground won: {}
        Rock won: {}
        Bug won: {}
        Ghost won: {}
        Steel won: {}
        Fire won: {}
        Water won: {}
        Grass won: {}
        Electric won: {}
        Psychic won: {}
        Ice won: {}
        Dragon won: {}
        Dark won: {}
        Normal lost: {}
        Fighting lost: {}
        Flying lost: {}
        Poison lost: {}
        Ground lost: {}
        Rock lost: {}
        Bug lost: {}
        Ghost lost: {}
        Steel lost: {}
        Fire lost: {}
        Water lost: {}
        Grass lost: {}
        Electric lost: {}
        Psychic lost: {}
        Ice lost: {}
        Dragon lost: {}
        Dark lost: {}
        ",
            type_won[0],
            type_won[1],
            type_won[2],
            type_won[3],
            type_won[4],
            type_won[5],
            type_won[6],
            type_won[7],
            type_won[8],
            type_won[9],
            type_won[10],
            type_won[11],
            type_won[12],
            type_won[13],
            type_won[14],
            type_won[15],
            type_won[16],
            type_lost[0],
            type_lost[1],
            type_lost[2],
            type_lost[3],
            type_lost[4],
            type_lost[5],
            type_lost[6],
            type_lost[7],
            type_lost[8],
            type_lost[9],
            type_lost[10],
            type_lost[11],
            type_lost[12],
            type_lost[13],
            type_lost[14],
            type_lost[15],
            type_lost[16],
        );
        println!(
            "
        Normal winrate: {}%
        Fighting winrate: {}%
        Flying winrate: {}%
        Poison winrate: {}%
        Ground winrate: {}%
        Rock winrate: {}%
        Bug winrate: {}%
        Ghost winrate: {}%
        Steel winrate: {}%
        Fire winrate: {}%
        Water winrate: {}%
        Grass winrate: {}%
        Electric winrate: {}%
        Psychic winrate: {}%
        Ice winrate: {}%
        Dragon winrate: {}%
        Dark winrate: {}%",
            100.0 * type_won[0] as f32 / (type_won[0] + type_lost[0]) as f32,
            100.0 * type_won[1] as f32 / (type_won[1] + type_lost[1]) as f32,
            100.0 * type_won[2] as f32 / (type_won[2] + type_lost[2]) as f32,
            100.0 * type_won[3] as f32 / (type_won[3] + type_lost[3]) as f32,
            100.0 * type_won[4] as f32 / (type_won[4] + type_lost[4]) as f32,
            100.0 * type_won[5] as f32 / (type_won[5] + type_lost[5]) as f32,
            100.0 * type_won[6] as f32 / (type_won[6] + type_lost[6]) as f32,
            100.0 * type_won[7] as f32 / (type_won[7] + type_lost[7]) as f32,
            100.0 * type_won[8] as f32 / (type_won[8] + type_lost[8]) as f32,
            100.0 * type_won[9] as f32 / (type_won[9] + type_lost[9]) as f32,
            100.0 * type_won[10] as f32 / (type_won[10] + type_lost[10]) as f32,
            100.0 * type_won[11] as f32 / (type_won[11] + type_lost[11]) as f32,
            100.0 * type_won[12] as f32 / (type_won[12] + type_lost[12]) as f32,
            100.0 * type_won[13] as f32 / (type_won[13] + type_lost[13]) as f32,
            100.0 * type_won[14] as f32 / (type_won[14] + type_lost[14]) as f32,
            100.0 * type_won[15] as f32 / (type_won[15] + type_lost[15]) as f32,
            100.0 * type_won[16] as f32 / (type_won[16] + type_lost[16]) as f32,
        );
        println!(
            "
        Normal non-STAB move's winrate: {}%
        Fighting non-STAB move's winrate: {}%
        Flying non-STAB move's winrate: {}%
        Poison non-STAB move's winrate: {}%
        Ground non-STAB move's winrate: {}%
        Rock non-STAB move's winrate: {}%
        Bug non-STAB move's winrate: {}%
        Ghost non-STAB move's winrate: {}%
        Steel non-STAB move's winrate: {}%
        Fire non-STAB move's winrate: {}%
        Water non-STAB move's winrate: {}%
        Grass non-STAB move's winrate: {}%
        Electric non-STAB move's winrate: {}%
        Psychic non-STAB move's winrate: {}%
        Ice non-STAB move's winrate: {}%
        Dragon non-STAB move's winrate: {}%
        Dark non-STAB move's winrate: {}%",
            100.0 * won_with_non_stab_type_move[0] as f32
                / (won_with_non_stab_type_move[0] + lost_with_non_stab_type_move[0]) as f32,
            100.0 * won_with_non_stab_type_move[1] as f32
                / (won_with_non_stab_type_move[1] + lost_with_non_stab_type_move[1]) as f32,
            100.0 * won_with_non_stab_type_move[2] as f32
                / (won_with_non_stab_type_move[2] + lost_with_non_stab_type_move[2]) as f32,
            100.0 * won_with_non_stab_type_move[3] as f32
                / (won_with_non_stab_type_move[3] + lost_with_non_stab_type_move[3]) as f32,
            100.0 * won_with_non_stab_type_move[4] as f32
                / (won_with_non_stab_type_move[4] + lost_with_non_stab_type_move[4]) as f32,
            100.0 * won_with_non_stab_type_move[5] as f32
                / (won_with_non_stab_type_move[5] + lost_with_non_stab_type_move[5]) as f32,
            100.0 * won_with_non_stab_type_move[6] as f32
                / (won_with_non_stab_type_move[6] + lost_with_non_stab_type_move[6]) as f32,
            100.0 * won_with_non_stab_type_move[7] as f32
                / (won_with_non_stab_type_move[7] + lost_with_non_stab_type_move[7]) as f32,
            100.0 * won_with_non_stab_type_move[8] as f32
                / (won_with_non_stab_type_move[8] + lost_with_non_stab_type_move[8]) as f32,
            100.0 * won_with_non_stab_type_move[9] as f32
                / (won_with_non_stab_type_move[9] + lost_with_non_stab_type_move[9]) as f32,
            100.0 * won_with_non_stab_type_move[10] as f32
                / (won_with_non_stab_type_move[10] + lost_with_non_stab_type_move[10]) as f32,
            100.0 * won_with_non_stab_type_move[11] as f32
                / (won_with_non_stab_type_move[11] + lost_with_non_stab_type_move[11]) as f32,
            100.0 * won_with_non_stab_type_move[12] as f32
                / (won_with_non_stab_type_move[12] + lost_with_non_stab_type_move[12]) as f32,
            100.0 * won_with_non_stab_type_move[13] as f32
                / (won_with_non_stab_type_move[13] + lost_with_non_stab_type_move[13]) as f32,
            100.0 * won_with_non_stab_type_move[14] as f32
                / (won_with_non_stab_type_move[14] + lost_with_non_stab_type_move[14]) as f32,
            100.0 * won_with_non_stab_type_move[15] as f32
                / (won_with_non_stab_type_move[15] + lost_with_non_stab_type_move[15]) as f32,
            100.0 * won_with_non_stab_type_move[16] as f32
                / (won_with_non_stab_type_move[16] + lost_with_non_stab_type_move[16]) as f32,
        );
        println!(
            "
        Normal's average lifespan: {}
        Fighting's average lifespan: {}
        Flying's average lifespan: {}
        Poison's average lifespan: {}
        Ground's average lifespan: {}
        Rock's average lifespan: {}
        Bug's average lifespan: {}
        Ghost's average lifespan: {}
        Steel's average lifespan: {}
        Fire's average lifespan: {}
        Water's average lifespan: {}
        Grass's average lifespan: {}
        Electric's average lifespan: {}
        Psychic's average lifespan: {}
        Ice's average lifespan: {}
        Dragon's average lifespan: {}
        Dark's average lifespan: {}
        ",
            turns_alive_for_type[0] as f32 / (type_won[0] as f32 + type_lost[0] as f32),
            turns_alive_for_type[1] as f32 / (type_won[1] as f32 + type_lost[1] as f32),
            turns_alive_for_type[2] as f32 / (type_won[2] as f32 + type_lost[2] as f32),
            turns_alive_for_type[3] as f32 / (type_won[3] as f32 + type_lost[3] as f32),
            turns_alive_for_type[4] as f32 / (type_won[4] as f32 + type_lost[4] as f32),
            turns_alive_for_type[5] as f32 / (type_won[5] as f32 + type_lost[5] as f32),
            turns_alive_for_type[6] as f32 / (type_won[6] as f32 + type_lost[6] as f32),
            turns_alive_for_type[7] as f32 / (type_won[7] as f32 + type_lost[7] as f32),
            turns_alive_for_type[8] as f32 / (type_won[8] as f32 + type_lost[8] as f32),
            turns_alive_for_type[9] as f32 / (type_won[9] as f32 + type_lost[9] as f32),
            turns_alive_for_type[10] as f32 / (type_won[10] as f32 + type_lost[10] as f32),
            turns_alive_for_type[11] as f32 / (type_won[11] as f32 + type_lost[11] as f32),
            turns_alive_for_type[12] as f32 / (type_won[12] as f32 + type_lost[12] as f32),
            turns_alive_for_type[13] as f32 / (type_won[13] as f32 + type_lost[13] as f32),
            turns_alive_for_type[14] as f32 / (type_won[14] as f32 + type_lost[14] as f32),
            turns_alive_for_type[15] as f32 / (type_won[15] as f32 + type_lost[15] as f32),
            turns_alive_for_type[16] as f32 / (type_won[16] as f32 + type_lost[16] as f32),
        );
        println!("average turns: {}", average_turns as f32 / 100000.0);

        println!(
            "
        DamageLow's winrate: {}%
        DamageMed's winrate: {}%
        DamageHigh's winrate: {}%
        MissLow's winrate: {}%
        MissMed's winrate: {}%
        MissHigh's winrate: {}%
        StatsUp's winrate: {}%
        StatsUpDouble's winrate: {}%
        StatsDown's winrate: {}%
        StatsDownDouble's winrate: {}%
        ",
            100.0 * won_with_specific_base_move[0] as f32
                / (lost_with_specific_base_move[0] as f32 + won_with_specific_base_move[0] as f32),
            100.0 * won_with_specific_base_move[1] as f32
                / (lost_with_specific_base_move[1] as f32 + won_with_specific_base_move[1] as f32),
            100.0 * won_with_specific_base_move[2] as f32
                / (lost_with_specific_base_move[2] as f32 + won_with_specific_base_move[2] as f32),
            100.0 * won_with_specific_base_move[3] as f32
                / (lost_with_specific_base_move[3] as f32 + won_with_specific_base_move[3] as f32),
            100.0 * won_with_specific_base_move[4] as f32
                / (lost_with_specific_base_move[4] as f32 + won_with_specific_base_move[4] as f32),
            100.0 * won_with_specific_base_move[5] as f32
                / (lost_with_specific_base_move[5] as f32 + won_with_specific_base_move[5] as f32),
            100.0 * won_with_specific_base_move[6] as f32
                / (lost_with_specific_base_move[6] as f32 + won_with_specific_base_move[6] as f32),
            100.0 * won_with_specific_base_move[7] as f32
                / (lost_with_specific_base_move[7] as f32 + won_with_specific_base_move[7] as f32),
            100.0 * won_with_specific_base_move[8] as f32
                / (lost_with_specific_base_move[8] as f32 + won_with_specific_base_move[8] as f32),
            100.0 * won_with_specific_base_move[9] as f32
                / (lost_with_specific_base_move[9] as f32 + won_with_specific_base_move[9] as f32),
        );
        println!(
            "
        stat modifier's AtkStage winrate: {}%
        stat modifier's DefStage winrate: {}%
        stat modifier's SpaStage winrate: {}%
        stat modifier's SpdStage winrate: {}%
        stat modifier's SpeStage winrate: {}%
        stat modifier's EvaStage winrate: {}%
        stat modifier's AccStage winrate: {}%
        stat modifier's CrtStage winrate: {}%
    ",
            100.0 * won_with_specific_volatile_status[0] as f32
                / (lost_with_specific_volatile_status[0] as f32
                    + won_with_specific_volatile_status[0] as f32),
            100.0 * won_with_specific_volatile_status[1] as f32
                / (lost_with_specific_volatile_status[1] as f32
                    + won_with_specific_volatile_status[1] as f32),
            100.0 * won_with_specific_volatile_status[2] as f32
                / (lost_with_specific_volatile_status[2] as f32
                    + won_with_specific_volatile_status[2] as f32),
            100.0 * won_with_specific_volatile_status[3] as f32
                / (lost_with_specific_volatile_status[3] as f32
                    + won_with_specific_volatile_status[3] as f32),
            100.0 * won_with_specific_volatile_status[4] as f32
                / (lost_with_specific_volatile_status[4] as f32
                    + won_with_specific_volatile_status[4] as f32),
            100.0 * won_with_specific_volatile_status[5] as f32
                / (lost_with_specific_volatile_status[5] as f32
                    + won_with_specific_volatile_status[5] as f32),
            100.0 * won_with_specific_volatile_status[6] as f32
                / (lost_with_specific_volatile_status[6] as f32
                    + won_with_specific_volatile_status[6] as f32),
            100.0 * won_with_specific_volatile_status[7] as f32
                / (lost_with_specific_volatile_status[7] as f32
                    + won_with_specific_volatile_status[7] as f32),
        );
        println!(
            "winrate by level:
            level 80 {}%,
             level 81 {}%,
              level 82 {}%,
               level 83 {}%,
                level 84 {}%,
                 level 85 {}%,
                  level 86 {}%,
                   level 87 {}%,
                    level 88 {}%,
                     level 89 {}%",
            100.0 * won_level_distribution[0] as f32
                / (won_level_distribution[0] + lost_level_distribution[0]) as f32,
            100.0 * won_level_distribution[1] as f32
                / (won_level_distribution[1] + lost_level_distribution[1]) as f32,
            100.0 * won_level_distribution[2] as f32
                / (won_level_distribution[2] + lost_level_distribution[2]) as f32,
            100.0 * won_level_distribution[3] as f32
                / (won_level_distribution[3] + lost_level_distribution[3]) as f32,
            100.0 * won_level_distribution[4] as f32
                / (won_level_distribution[4] + lost_level_distribution[4]) as f32,
            100.0 * won_level_distribution[5] as f32
                / (won_level_distribution[5] + lost_level_distribution[5]) as f32,
            100.0 * won_level_distribution[6] as f32
                / (won_level_distribution[6] + lost_level_distribution[6]) as f32,
            100.0 * won_level_distribution[7] as f32
                / (won_level_distribution[7] + lost_level_distribution[7]) as f32,
            100.0 * won_level_distribution[8] as f32
                / (won_level_distribution[8] + lost_level_distribution[8]) as f32,
            100.0 * won_level_distribution[9] as f32
                / (won_level_distribution[9] + lost_level_distribution[9]) as f32,
        );
        println!(
            "
        Highest base stat is hp: winrate {}%
        Highest base stat is attack: winrate {}%
        Highest base stat is defence: winrate {}%
        Highest base stat is special attack: winrate {}%
        Highest base stat is special defense: winrate {}%
        Highest base stat is speed: winrate {}%
        Lowest base stat is hp: winrate {}%
        Lowest base stat is attack: winrate {}%
        Lowest base stat is defence: winrate {}%
        Lowest base stat is special attack: winrate {}%
        Lowest base stat is special defense: winrate {}%
        Lowest base stat is speed: winrate {}%
        ",
            100.0 * won_with_highest_base_stats[0] as f32
                / (won_with_highest_base_stats[0] as f32 + lost_with_highest_base_stats[0] as f32),
            100.0 * won_with_highest_base_stats[1] as f32
                / (won_with_highest_base_stats[1] as f32 + lost_with_highest_base_stats[1] as f32),
            100.0 * won_with_highest_base_stats[2] as f32
                / (won_with_highest_base_stats[2] as f32 + lost_with_highest_base_stats[2] as f32),
            100.0 * won_with_highest_base_stats[3] as f32
                / (won_with_highest_base_stats[3] as f32 + lost_with_highest_base_stats[3] as f32),
            100.0 * won_with_highest_base_stats[4] as f32
                / (won_with_highest_base_stats[4] as f32 + lost_with_highest_base_stats[4] as f32),
            100.0 * won_with_highest_base_stats[5] as f32
                / (won_with_highest_base_stats[5] as f32 + lost_with_highest_base_stats[5] as f32),
            100.0 * won_with_lowest_base_stats[0] as f32
                / (won_with_lowest_base_stats[0] as f32 + lost_with_lowest_base_stats[0] as f32),
            100.0 * won_with_lowest_base_stats[1] as f32
                / (won_with_lowest_base_stats[1] as f32 + lost_with_lowest_base_stats[1] as f32),
            100.0 * won_with_lowest_base_stats[2] as f32
                / (won_with_lowest_base_stats[2] as f32 + lost_with_lowest_base_stats[2] as f32),
            100.0 * won_with_lowest_base_stats[3] as f32
                / (won_with_lowest_base_stats[3] as f32 + lost_with_lowest_base_stats[3] as f32),
            100.0 * won_with_lowest_base_stats[4] as f32
                / (won_with_lowest_base_stats[4] as f32 + lost_with_lowest_base_stats[4] as f32),
            100.0 * won_with_lowest_base_stats[5] as f32
                / (won_with_lowest_base_stats[5] as f32 + lost_with_lowest_base_stats[5] as f32),
        );
    }
}
    }
}
