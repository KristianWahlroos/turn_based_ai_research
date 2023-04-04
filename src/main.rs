use env_logger::{Env, Logger};
use log::{debug, error, info, log_enabled, warn, Level};
mod move_data;
use move_data::*;
use rand::Rng;
use serde::Deserialize;
use serde::Serialize;
use std::fs;
use std::ops::Mul;

fn main() {
    env_logger::builder().format_timestamp(None).init();

    /////////////////////////////////////////////////////////////////////////////////////////////////

    // let the_file =
    //     fs::read_to_string("moves.json").expect("Should have been able to read the file");
    // let contents: Vec<NotFinalMove> =
    //     serde_json::from_str(&the_file).expect("JSON was not well-formatted");
    // let all_the_moves: Vec<Move> = contents.into_iter().map(|x| x.into()).collect();
    // for a_move in all_the_moves {
    //     if a_move.units[0].effect == Effect::Unimplemented {
    //         print!("{:?} | ", a_move.id);
    //     }
    // }

    /////////////////////////////////////////////////////

    // let the_file = fs::read_to_string("teams.txt").expect("to be able to read the file");
    // let contents: TeamManager =
    //     serde_json::from_str(&the_file).expect("JSON was not well-formatted");
    // let creature: Creature = (&contents.teams[0].team[0]).into();
    // warn!("{:?}", creature);

    //////////////////////////////////////////////////////

    // let team = Team {
    //     team: [
    //         Person {
    //             species: "Magnezone".to_owned(),
    //             level: 80,
    //             moves: [
    //                 "explosion".to_owned(),
    //                 "flashcannon".to_owned(),
    //                 "hiddenpowerice".to_owned(),
    //                 "thunderbolt".to_owned(),
    //             ],
    //             stats: Stats {
    //                 hp: 243,
    //                 atk: 157,
    //                 def: 229,
    //                 spa: 254,
    //                 spd: 190,
    //                 spe: 142,
    //             },
    //         },
    //         Person {
    //             species: "Placeholder".to_owned(),
    //             level: 80,
    //             moves: [
    //                 "Placeholder1".to_owned(),
    //                 "Placeholder2".to_owned(),
    //                 "Placeholder3".to_owned(),
    //                 "Placeholder4".to_owned(),
    //             ],
    //             stats: Stats {
    //                 hp: 20,
    //                 atk: 20,
    //                 def: 30,
    //                 spa: 40,
    //                 spd: 50,
    //                 spe: 50,
    //             },
    //         },
    //         Person {
    //             species: "Placeholder".to_owned(),
    //             level: 80,
    //             moves: [
    //                 "Placeholder1".to_owned(),
    //                 "Placeholder2".to_owned(),
    //                 "Placeholder3".to_owned(),
    //                 "Placeholder4".to_owned(),
    //             ],
    //             stats: Stats {
    //                 hp: 20,
    //                 atk: 20,
    //                 def: 30,
    //                 spa: 40,
    //                 spd: 50,
    //                 spe: 50,
    //             },
    //         },
    //         Person {
    //             species: "Placeholder".to_owned(),
    //             level: 80,
    //             moves: [
    //                 "Placeholder1".to_owned(),
    //                 "Placeholder2".to_owned(),
    //                 "Placeholder3".to_owned(),
    //                 "Placeholder4".to_owned(),
    //             ],
    //             stats: Stats {
    //                 hp: 20,
    //                 atk: 20,
    //                 def: 30,
    //                 spa: 40,
    //                 spd: 50,
    //                 spe: 50,
    //             },
    //         },
    //         Person {
    //             species: "Placeholder".to_owned(),
    //             level: 80,
    //             moves: [
    //                 "Placeholder1".to_owned(),
    //                 "Placeholder2".to_owned(),
    //                 "Placeholder3".to_owned(),
    //                 "Placeholder4".to_owned(),
    //             ],
    //             stats: Stats {
    //                 hp: 20,
    //                 atk: 20,
    //                 def: 30,
    //                 spa: 40,
    //                 spd: 50,
    //                 spe: 50,
    //             },
    //         },
    //         Person {
    //             species: "Placeholder".to_owned(),
    //             level: 80,
    //             moves: [
    //                 "Placeholder1".to_owned(),
    //                 "Placeholder2".to_owned(),
    //                 "Placeholder3".to_owned(),
    //                 "Placeholder4".to_owned(),
    //             ],
    //             stats: Stats {
    //                 hp: 20,
    //                 atk: 20,
    //                 def: 30,
    //                 spa: 40,
    //                 spd: 50,
    //                 spe: 50,
    //             },
    //         },
    //     ],
    // };
    // let team_manager = TeamManager {
    //     teams: vec![team.clone(), team.clone(), team.clone(), team.clone()],
    // };
    // let team_manager_as_str = serde_json::to_string(&team_manager).unwrap();
    // let contents: TeamManager = serde_json::from_str(&team_manager_as_str).expect("JSON was not well-formatted");
    // println!("{}", team_manager_as_str);
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

// TODO add random rolls
/// [damage formula](https://bulbapedia.bulbagarden.net/wiki/Damage#Generation_IV)
/// [MoveID::MeFirst] not yet implemented as it seems not to appear in the datasets. Also [MoveID::Metronome] won't trigger Me First
pub fn calculate_damage(
    // creatures: &[[Creature; 6]; 2],
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
    fn get_action(&self, move_count: u8) -> CombatAction;
    fn get_forced_switch(&self, creature_instances: &[CreatureInstance; 6]) -> CombatAction;
}

pub struct RandomAI {}

impl AI for RandomAI {
    fn get_action(&self, move_count: u8) -> CombatAction {
        CombatAction::Attack(rand::random::<u8>() % move_count)
    }
    /// Assumption that if all fainted we don't force switch
    fn get_forced_switch(&self, creature_instances: &[CreatureInstance; 6]) -> CombatAction {
        loop {
            let switch_to = rand::random::<usize>() % 5;
            if !creature_instances[switch_to].is_fainted() {
                return CombatAction::Switch(switch_to as u8);
            }
        }
    }
}

fn is_team_fainted(creature_instances: &[[CreatureInstance; 6]; 2], side: bool) -> bool {
    for instance in &creature_instances[side as usize] {
        if !instance.is_fainted() {
            return false;
        }
    }
    true
}

#[derive(Clone)]
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

enum Interrupt {
    PlayerFainted,
    EnemyFainted,
    BothFainted,
    Condition,
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
        creatures: &[[Creature; 6]; 2],
        creature_instances: &mut [[CreatureInstance; 6]; 2],
        combat_actions: &[CombatAction; 2],
    ) {
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
        } else {
            // win check here as well
            println!("hello!");
        }
        self.current_turn += 1;

        // win check here as well

        // weather check faster
        // weather check slower
        // forced switch faster
        // forced switch slower

        // win check now?
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
        creatures: &[[Creature; 6]; 2],
        creature_instances: &mut [[CreatureInstance; 6]; 2],
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
        creatures: &[[Creature; 6]; 2],
        move_id: usize,
        actioner: usize,
    ) -> MoveID {
        creatures[actioner][self.battler_ids[actioner]].moves[move_id]
            .id
            .clone()
    }
    fn switch(
        &mut self,
        creature_instances: &mut [[CreatureInstance; 6]; 2],
        switch_to_id: usize,
        actioner: usize,
    ) {
        if creature_instances[actioner][switch_to_id].is_fainted() {
            panic!("Can't switch to a fainted");
        }
        self.volatile_statuses[actioner] = vec![];
        self.battler_ids[actioner] = switch_to_id;
        // TODO add spikes behaviour
    }

    fn take_damage(
        &mut self,
        creature_instances: &mut [[CreatureInstance; 6]; 2],
        damage_taker: usize,
        damage: i32,
    ) {
        if damage < 0 {
            panic!("Damage shouldnt' be negative");
        }
        creature_instances[damage_taker][self.battler_ids[damage_taker]].current_health -= damage;
    }

    fn get_chance_of_success(
        &mut self,
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

    fn use_move(
        &mut self,
        battle_settings: &BattleSettings,
        creatures: &[[Creature; 6]; 2],
        creature_instances: &mut [[CreatureInstance; 6]; 2],
        move_id: usize,
        actioner: bool,
    ) {
        let attacker = &creatures[actioner as usize][self.battler_ids[actioner as usize]];
        // TODO immunities should cause failure here
        let mut success = true;
        // Reduce PP // or we shouldn't do it here because of sleep talk???
        let base_hit_chance = match self.get_chance_of_success(attacker, actioner, move_id) {
            Some(chance) => {
                if !battle_settings.always_hits {
                    let random = rand::random();
                    assert!(random <= 1.0);
                    assert!(random >= 0.0);
                    if chance < random {
                        return; // TODO self destruct does stuff even if the main attack misses
                    }
                }
                chance
            }
            None => 1.0,
        };
        for unit in &attacker.moves[move_id].units {
            // check did the last loop succeed
            if !success {
                if unit.continues_previous_unit {
                    // TODO check if there is any move where we only skip units in the middle
                    break;
                }
            }
            let random = rand::random();
            assert!(random <= 1.0);
            assert!(random >= 0.0);
            if unit.chance_of_success < random {
                success = false;
                continue;
            }
            // TODO check is thsi success implemented right
            success = match unit.effect {
                Effect::PhysicalAttack => self.attack(
                    battle_settings,
                    creatures,
                    creature_instances,
                    &attacker.moves[move_id].id,
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
                    &attacker.moves[move_id].id,
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

    fn attack(
        &mut self,
        battle_settings: &BattleSettings,
        creatures: &[[Creature; 6]; 2],
        creature_instances: &mut [[CreatureInstance; 6]; 2],
        move_id: &MoveID, // TODO consider is this right solution
        physical: bool,
        power: i32,
        level: i32,
        attack_type: &Type,
        attacker: usize,
        damage_taker: usize,
        base_hit_chance: f32,
        // target_self: bool,
    ) -> bool {
        let (attack, defense, attack_stage, defense_stage) = if physical {
            (
                creatures[attacker][self.battler_ids[attacker]].stats.atk,
                creatures[damage_taker][self.battler_ids[damage_taker]]
                    .stats
                    .def,
                self.get_stage_from_value_volatile_status(attacker, VolatileStatus::AtkStage),
                self.get_stage_from_value_volatile_status(damage_taker, VolatileStatus::DefStage),
            )
        } else {
            (
                creatures[attacker][self.battler_ids[attacker]].stats.spa,
                creatures[damage_taker][self.battler_ids[damage_taker]]
                    .stats
                    .spd,
                self.get_stage_from_value_volatile_status(
                    attacker as usize,
                    VolatileStatus::SpaStage,
                ),
                self.get_stage_from_value_volatile_status(
                    damage_taker as usize,
                    VolatileStatus::SpdStage,
                ),
            )
        };
        let crit_stage =
            self.get_stage_from_value_volatile_status(attacker, VolatileStatus::CrtStage);
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
        let stab_bonus =
            creatures[attacker][self.battler_ids[attacker]].get_stab_modifier(attack_type);
        let type_effectiviness = creatures[damage_taker][self.battler_ids[damage_taker]]
            .effectiviness_when_attacked(attack_type);

        let damage = calculate_damage(
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
        ) as i32;
        self.take_damage(creature_instances, damage_taker, damage);
        true
    }

    // TODO add volatile status checks
    fn is_first_faster(
        &self,
        creatures: &[[Creature; 6]; 2],
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
    fn get_battler<'a>(&'a self, side: usize, creatures: &'a [[Creature; 6]; 2]) -> &Creature {
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

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Normal,
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

fn get_fainted_interrupt(any_faints: [bool; 2]) -> Option<Interrupt> {
    match any_faints {
        [true, true] => Some(Interrupt::BothFainted),
        [true, false] => Some(Interrupt::EnemyFainted),
        [false, true] => Some(Interrupt::PlayerFainted),
        [false, false] => None,
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
    pub moves: Vec<Move>,
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
}

impl Creature {
    pub fn get_stab_modifier(&self, attack_type: &Type) -> f32 {
        for creature_type in &self.types {
            if creature_type == attack_type {
                return 1.5;
            }
        }
        1.0
    }
}

#[derive(Clone)]
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
    AtkStage,
    DefStage,
    SpaStage,
    SpdStage,
    SpeStage,
    EvaStage,
    AccStage,
    CrtStage,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_basic_setup() -> (
        [[Creature; 6]; 2],
        [[CreatureInstance; 6]; 2],
        BattleInstance,
        BattleSettings,
        CombatAction,
        CombatAction,
    ) {
        (
            get_placeholder_creatures(),
            get_placeholder_creature_instances(),
            BattleInstance::default(),
            BattleSettings::default(),
            CombatAction::Attack(0),
            CombatAction::Attack(0),
        )
    }

    fn get_placeholder_creatures() -> [[Creature; 6]; 2] {
        [
            [
                Creature {
                    species: "Placeholder".to_string(),
                    level: 100,
                    moves: vec![(&MoveID::DamageLow).into()],
                    stats: Stats {
                        hp: 300,
                        atk: 100,
                        def: 100,
                        spa: 100,
                        spd: 100,
                        spe: 300,
                    },
                    types: vec![Type::Normal],
                },
                Creature {
                    species: "Placeholder".to_string(),
                    level: 100,
                    moves: vec![(&MoveID::DamageLow).into()],
                    stats: Stats::default(),
                    types: vec![Type::Normal],
                },
                Creature {
                    species: "Placeholder".to_string(),
                    level: 100,
                    moves: vec![(&MoveID::DamageLow).into()],
                    stats: Stats::default(),
                    types: vec![Type::Normal],
                },
                Creature {
                    species: "Placeholder".to_string(),
                    level: 100,
                    moves: vec![(&MoveID::DamageLow).into()],
                    stats: Stats::default(),
                    types: vec![Type::Normal],
                },
                Creature {
                    species: "Placeholder".to_string(),
                    level: 100,
                    moves: vec![(&MoveID::DamageLow).into()],
                    stats: Stats::default(),
                    types: vec![Type::Normal],
                },
                Creature {
                    species: "Placeholder".to_string(),
                    level: 100,
                    moves: vec![(&MoveID::DamageLow).into()],
                    stats: Stats::default(),
                    types: vec![Type::Normal],
                },
            ],
            [
                Creature {
                    species: "Placeholder".to_string(),
                    level: 100,
                    moves: vec![(&MoveID::DamageLow).into()],
                    stats: Stats::default(),
                    types: vec![Type::Normal],
                },
                Creature {
                    species: "Placeholder".to_string(),
                    level: 100,
                    moves: vec![(&MoveID::DamageLow).into()],
                    stats: Stats::default(),
                    types: vec![Type::Normal],
                },
                Creature {
                    species: "Placeholder".to_string(),
                    level: 100,
                    moves: vec![(&MoveID::DamageLow).into()],
                    stats: Stats::default(),
                    types: vec![Type::Normal],
                },
                Creature {
                    species: "Placeholder".to_string(),
                    level: 100,
                    moves: vec![(&MoveID::DamageLow).into()],
                    stats: Stats::default(),
                    types: vec![Type::Normal],
                },
                Creature {
                    species: "Placeholder".to_string(),
                    level: 100,
                    moves: vec![(&MoveID::DamageLow).into()],
                    stats: Stats::default(),
                    types: vec![Type::Normal],
                },
                Creature {
                    species: "Placeholder".to_string(),
                    level: 100,
                    moves: vec![(&MoveID::DamageLow).into()],
                    stats: Stats::default(),
                    types: vec![Type::Normal],
                },
            ],
        ]
    }

    fn get_placeholder_creature_instances() -> [[CreatureInstance; 6]; 2] {
        [
            [
                CreatureInstance {
                    current_health: 300,
                },
                CreatureInstance {
                    current_health: 300,
                },
                CreatureInstance {
                    current_health: 300,
                },
                CreatureInstance {
                    current_health: 300,
                },
                CreatureInstance {
                    current_health: 300,
                },
                CreatureInstance {
                    current_health: 300,
                },
            ],
            [
                CreatureInstance {
                    current_health: 300,
                },
                CreatureInstance {
                    current_health: 300,
                },
                CreatureInstance {
                    current_health: 300,
                },
                CreatureInstance {
                    current_health: 300,
                },
                CreatureInstance {
                    current_health: 300,
                },
                CreatureInstance {
                    current_health: 300,
                },
            ],
        ]
    }
}