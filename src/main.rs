use env_logger::{Env, Logger};
use log::{debug, error, info, log_enabled, warn, Level};
mod move_data;
use move_data::*;
use rand::Rng;
use serde::Deserialize;
use serde::Serialize;
use std::fs;
use std::ops::Mul;

/// This is some early prototyping and should not be looked at as reference
/// Having volatile statuses in [BattleInstance] as vecs was a mistake as it turns out for me that in original Pokemon the volatile status order matters a lot.
/// Probably should remove a lot of Pokemon-specific features and focusing on the research related to switching.
fn main() {
    env_logger::builder().format_timestamp(None).init();

    /////////////////////////////////////////////////////////////////////////////////////////////////

    let the_file =
        fs::read_to_string("moves.json").expect("Should have been able to read the file");
    let contents: Vec<NotFinalMove> =
        serde_json::from_str(&the_file).expect("JSON was not well-formatted");
    let all_the_moves: Vec<Move> = contents.into_iter().map(|x| x.into()).collect();
    for a_move in all_the_moves {
        if a_move.units[0].effect == Effect::Unimplemented {
            print!("{:?} | ", a_move.id);
        }
    }

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
    burn_screen: f32,
    weather_bonus: f32,
    stab_bonus: f32,
    type_effectiviness: f32,
    chance_bonuses: f32,
    is_crit: bool,
) -> f32 {
    (((((2.0 * level as f32) / 5.0) + 2.0)
        * (power as f32)
        * (attack as f32 / defense as f32)
        * (get_offensive_stage_multiplier(attack_stage, is_crit) / get_defensive_stage_multiplier(defense_stage, is_crit))
        / 50.0)
        * burn_screen
        * weather_bonus
        + 2.0)
        * stab_bonus
        * type_effectiviness
        // Could use conditional compilation on random rolls and crits
        * chance_bonuses

    // TODO -> PACK sames together? -> Random chance stuff at the end so they can be slightly easier to remove
}

// TODO add validity checks and loop here instead or at least somewhere around the AI methods
// TODO Multihit moves should be different for AI (RockBlast is used)
// TODO Sleep should also track the start of sleep so AI can determine
pub trait AI {
    fn get_action(&self, move_count: u8) -> CombatAction;
    fn get_forced_switch(&self, creature_instances: &[CreatureInstance; 6]) -> CombatAction;
}

pub struct RandomAI {}

impl AI for RandomAI {
    fn get_action(&self, move_count: u8) -> CombatAction {
        CombatAction::Attack(rand::random::<u8>() % move_count)
    }
    fn get_forced_switch(&self, creature_instances: &[CreatureInstance; 6]) -> CombatAction {
        loop {
            let switch_to = rand::random::<usize>() % 5;
            // TODO Separate fainted check from get_forced_switch
            if creature_instances[switch_to].status != Some(Status::Fainted) {
                return CombatAction::Switch(switch_to as u8);
            }
        }
    }
}

fn is_team_fainted(creature_instances: &[[CreatureInstance; 6]; 2], side: bool) -> bool {
    for instance in &creature_instances[side as usize] {
        if instance.status != Some(Status::Fainted) {
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
    always_hits: bool, // TODO
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
            always_hits: false, // TODO check tests after this
            roll: Roll::HighRoll,
        }
    }
}

// cloneable
struct BattleInstance {
    battler_ids: [usize; 2],
    volatile_statuses: [Vec<VolatileStatus>; 2],
    value_volatile_statuses: [Vec<(VolatileStatusWithValue, i32)>; 2],
    weather: Weather,
    current_turn: i32, // Current turn is used for weather mostly as turn order might by have to take into account when using it in things with side
}
impl Default for BattleInstance {
    fn default() -> Self {
        BattleInstance {
            battler_ids: [0, 0],
            volatile_statuses: [vec![], vec![]],
            value_volatile_statuses: [vec![], vec![]],
            weather: Weather::default(),
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
        self.prepare_action(creatures, combat_actions, !first_faster as usize);
        self.prepare_action(creatures, combat_actions, first_faster as usize);
        self.do_action(
            battle_settings,
            creatures,
            creature_instances,
            combat_actions,
            !first_faster,
        );

        if creature_instances[first_faster as usize][self.battler_ids[first_faster as usize]].status
            != Some(Status::Fainted)
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
        self.prepare_for_next_turn(creatures, creature_instances)

        // win check here as well

        // weather check faster
        // weather check slower
        // forced switch faster
        // forced switch slower

        // win check now?
    }

    /// Currently only for [MoveID::Counter]
    fn prepare_action(
        &mut self,
        creatures: &[[Creature; 6]; 2],
        combat_actions: &[CombatAction; 2],
        actioner: usize,
    ) {
        match combat_actions[actioner] {
            CombatAction::Attack(move_id) => {
                match self.get_move_id(creatures, move_id as usize, actioner) {
                    MoveID::Counter => self.value_volatile_statuses[actioner]
                        .push((VolatileStatusWithValue::Counter, 0)),
                    _ => (),
                }
            }
            CombatAction::Switch(_) => (),
        }
    }
    pub fn start_weather(&mut self, weather_type: &WeatherType) -> bool {
        if self.weather.weather_type != Some(weather_type.clone()) {
            self.weather.weather_type = Some(weather_type.clone());
            self.weather.end_turn = self.current_turn + 4; // 5 turns == this + 4
            true
        } else {
            false // weather move fails, if the weather is already on the field
        }
    }
    fn prepare_for_next_turn(
        &mut self,
        creatures: &[[Creature; 6]; 2],
        creature_instances: &mut [[CreatureInstance; 6]; 2],
    ) {
        // WEATHER
        if self.weather.weather_type.is_some() {
            if self.current_turn == self.weather.end_turn {
                // Weather is cleared // separate method probably not needed for this
                self.weather.weather_type = None;
            }
        }
        for side in 0..2 {
            // STATUS DAMAGE
            let damage = (creatures[side][self.battler_ids[side]].stats.hp
                * match creature_instances[side][self.battler_ids[side]].status {
                    Some(Status::Burn) => 1,
                    Some(Status::Poison) => 2,
                    Some(Status::BadlyPoisoned) => {
                        creature_instances[side][self.battler_ids[side]].status_elapsed += 1;
                        creature_instances[side][self.battler_ids[side]].status_elapsed
                    }
                    _ => return,
                })
                / 16;
            let _ = self.take_damage(creature_instances, side, damage, None);

            // VOLATILE STATUSES Cleaning
            let volatile_statuses_count = self.volatile_statuses[side].len();
            let mut protect_used = false;
            for i in 0..volatile_statuses_count {
                let index = volatile_statuses_count - i - 1;
                if self.volatile_statuses[side][index] == VolatileStatus::Flinch {
                    self.volatile_statuses[side].remove(index);
                } else if self.volatile_statuses[side][index] == VolatileStatus::Protection {
                    self.volatile_statuses[side].remove(index);
                    protect_used = true;
                }
            }
            let value_volatile_statuses_count = self.value_volatile_statuses[side].len();
            for i in 0..value_volatile_statuses_count {
                let index = value_volatile_statuses_count - i - 1;
                if self.value_volatile_statuses[side][index].0 == VolatileStatusWithValue::Counter {
                    self.value_volatile_statuses[side].remove(index);
                } else if self.value_volatile_statuses[side][index].0
                    == VolatileStatusWithValue::ProtectionCounter
                {
                    if !protect_used {
                        self.value_volatile_statuses[side].remove(index);
                    }
                }
            }
        }

        self.current_turn += 1;
    }

    fn has_valueless_volatile_status(
        &self,
        actioner: usize,
        volatile_status: VolatileStatus,
    ) -> bool {
        for ps in &self.volatile_statuses[actioner] {
            if ps == &volatile_status {
                return true;
            }
        }
        false
    }

    fn has_then_try_remove_valueless_volatile_status(
        &mut self,
        actioner: usize,
        volatile_status: &VolatileStatus,
    ) -> bool {
        for i in 0..self.volatile_statuses[actioner].len() {
            if &self.volatile_statuses[actioner][i] == volatile_status {
                let _ = &self.volatile_statuses[actioner].remove(i);
                return true;
            }
        }
        false
    }
    fn has_then_try_remove_value_volatile_status(
        &mut self,
        actioner: usize,
        value_volatile_status: &VolatileStatusWithValue,
    ) -> bool {
        for i in 0..self.value_volatile_statuses[actioner].len() {
            if &self.value_volatile_statuses[actioner][i].0 == value_volatile_status {
                self.value_volatile_statuses[actioner].remove(i);
                return true;
            }
        }
        false
    }

    fn get_value_from_value_volatile_status(
        &self,
        actioner: usize,
        volatile_status: VolatileStatusWithValue,
    ) -> Option<i32> {
        for ps in &self.value_volatile_statuses[actioner] {
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
        volatile_status: VolatileStatusWithValue,
    ) -> i32 {
        for ps in &self.value_volatile_statuses[side] {
            if ps.0 == volatile_status {
                return ps.1;
            }
        }
        0
    }

    /// [Status::Burn] is not here. It is directly in a move calculation.
    /// These affected statuses are only checked before moves
    fn pre_move_status(
        &mut self,
        creature_instances: &mut [[CreatureInstance; 6]; 2],
        move_id: MoveID,
        actioner: usize,
    ) -> Option<Interrupt> {
        match creature_instances[actioner][self.battler_ids[actioner]].status {
            Some(Status::Sleep) => {
                assert!(
                    creature_instances[actioner][self.battler_ids[actioner]].status_elapsed > 0
                );
                creature_instances[actioner][self.battler_ids[actioner]].status_elapsed -= 1;
                if creature_instances[actioner][self.battler_ids[actioner]].status_elapsed == 0 {
                    creature_instances[actioner][self.battler_ids[actioner]].status = None;
                    None
                } else {
                    if move_id.is_sleep_move() {
                        None
                    } else {
                        Some(Interrupt::Condition)
                    }
                }
            }
            Some(Status::Freeze) => {
                if rand::random::<i32>() % 5 == 4 {
                    creature_instances[actioner][self.battler_ids[actioner]].status = None;
                    None
                } else {
                    if move_id.is_thaw_move() {
                        creature_instances[actioner][self.battler_ids[actioner]].status = None;
                        None
                    } else {
                        Some(Interrupt::Condition)
                    }
                }
            }
            Some(Status::Paralysis) => {
                if rand::random::<u8>() % 4 != 3 {
                    None
                } else {
                    Some(Interrupt::Condition)
                }
            }
            _ => None,
        }
    }
    fn pre_move_volatile_status(&mut self, actioner: usize) -> Option<Interrupt> {
        for volatile_status in &self.volatile_statuses[actioner] {
            match volatile_status {
                VolatileStatus::Flinch => return Some(Interrupt::Condition),
                _ => (),
            }
        }
        let last_index = self.value_volatile_statuses[actioner].len() - 1;
        for i in 0..self.value_volatile_statuses[actioner].len() {
            let index = last_index - i;
            match self.value_volatile_statuses[actioner][index].0 {
                VolatileStatusWithValue::Confused => {
                    if self.value_volatile_statuses[actioner][index].1 == 0 {
                        self.value_volatile_statuses[actioner].remove(index);
                    } else {
                        self.value_volatile_statuses[actioner][index].1 -= 1;
                        if rand::random::<bool>() {
                            // 50%
                            return Some(Interrupt::Condition);
                        }
                    }
                }
                _ => (),
            }
        }
        None
    }

    fn post_action_volatile_status(
        &mut self,
        creatures: &[[Creature; 6]; 2],
        creature_instances: &mut [[CreatureInstance; 6]; 2],
        actioner: usize,
    ) {
        //
        // let damage = (creatures[actioner][self.battler_ids[actioner]].stats.hp
        //     * match creature_instances[actioner][self.battler_ids[actioner]].status {
        //         Some(Status::Burn) => 1,
        //         Some(Status::Poison) => 2,
        //         Some(Status::BadlyPoisoned) => {
        //             creature_instances[actioner][self.battler_ids[actioner]].status_elapsed += 1;
        //             creature_instances[actioner][self.battler_ids[actioner]].status_elapsed
        //         }
        //         _ => return,
        //     })
        //     / 16;
        // let _ = self.take_damage(creature_instances, actioner, damage, None);
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
            CombatAction::Attack(move_id) => {
                // probably can combine [status_interrupt] with [volatile_interrupt]. Probably unnecessary optimisation though
                let status_interrupt = self.pre_move_status(
                    creature_instances,
                    self.get_move_id(creatures, move_id as usize, actioner as usize),
                    actioner as usize,
                );
                let volatile_interrupt = self.pre_move_volatile_status(actioner as usize);
                if volatile_interrupt.is_none() && status_interrupt.is_none() {
                    self.use_move(
                        battle_settings,
                        creatures,
                        creature_instances,
                        move_id as usize,
                        actioner,
                    )
                }
            }
            CombatAction::Switch(switch_to_id) => {
                self.switch(creature_instances, switch_to_id as usize, actioner as usize)
            }
        };
        // self.post_action_status(); TODO
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
        if creature_instances[actioner][switch_to_id].status == Some(Status::Fainted) {
            panic!("Can't switch to a fainted");
        } else if creature_instances[actioner][switch_to_id].status == Some(Status::BadlyPoisoned) {
            creature_instances[actioner][switch_to_id].status_elapsed = 0;
        }
        self.volatile_statuses[actioner] = vec![];
        self.value_volatile_statuses[actioner] = vec![];
        self.battler_ids[actioner] = switch_to_id;
        // TODO add spikes behaviour
    }
    fn sleep(
        &self,
        is_rest: bool,
        creature_instances: &mut [[CreatureInstance; 6]; 2],
        status_taker: usize,
    ) {
        creature_instances[status_taker][self.battler_ids[status_taker]].status =
            Some(Status::Sleep);
        creature_instances[status_taker][self.battler_ids[status_taker]].status_elapsed = if is_rest
        {
            3
        } else {
            (rand::random::<i32>() % 5) + 1 // 1 to 5 turns
        }
    }

    fn inflict_status(
        &self,
        creature_instances: &mut [[CreatureInstance; 6]; 2],
        status_taker: usize,
    ) {
        creature_instances[status_taker][self.battler_ids[status_taker]].status =
            Some(Status::BadlyPoisoned);
    }

    /// returns damage_taken for purpose of drain
    fn take_damage(
        &mut self,
        creature_instances: &mut [[CreatureInstance; 6]; 2],
        damage_taker: usize,
        damage: i32,
        physical: Option<bool>,
    ) -> i32 {
        if damage < 0 {
            panic!("Damage shouldnt' be negative");
        }
        if creature_instances[damage_taker][self.battler_ids[damage_taker]].current_health - damage
            <= 0
        {
            self.faint(creature_instances, damage_taker);
            creature_instances[damage_taker][self.battler_ids[damage_taker]].current_health
        } else {
            creature_instances[damage_taker][self.battler_ids[damage_taker]].current_health -=
                damage;
            match physical {
                // Counter
                Some(true) => {
                    for value_volatile_status in &mut self.value_volatile_statuses[damage_taker] {
                        if value_volatile_status.0 == VolatileStatusWithValue::Counter {
                            value_volatile_status.1 += damage as i32;
                        }
                    }
                }
                Some(false) => (),
                None => (),
            }
            damage
        }
    }
    fn heal(
        &mut self,
        creatures: &[[Creature; 6]; 2],
        creature_instances: &mut [[CreatureInstance; 6]; 2],
        heal_taker: usize,
        heal: i32,
    ) {
        if heal < 0 {
            panic!("Heal shouldnt' be negative");
        }
        if creatures[heal_taker][self.battler_ids[heal_taker]].stats.hp
            <= creature_instances[heal_taker][self.battler_ids[heal_taker]].current_health + heal
        {
            creature_instances[heal_taker][self.battler_ids[heal_taker]].current_health =
                creatures[heal_taker][self.battler_ids[heal_taker]].stats.hp;
        } else {
            creature_instances[heal_taker][self.battler_ids[heal_taker]].current_health += heal;
        }
    }

    /// Faint faints the creature.
    /// Returns [Interrupt]
    /// Has own method so one can check destiny bonds and returns an [Interrupt]
    // TODO destiny bond
    fn faint(&self, creature_instances: &mut [[CreatureInstance; 6]; 2], fainter: usize) {
        creature_instances[fainter][self.battler_ids[fainter]].status = Some(Status::Fainted);
    }

    fn get_chance_of_success(
        &mut self,
        creature: &Creature,
        actioner: bool,
        move_id: usize,
    ) -> Option<f32> {
        match creature.moves[move_id].chance_of_success {
            Some(base_chance) => {
                // Here we look at expections
                let chance = match self.weather.weather_type {
                    Some(ref weather_type) => {
                        if creature.moves[move_id].id == MoveID::Thunder {
                            if weather_type == &WeatherType::Rainy {
                                1.0
                            } else {
                                0.5
                            }
                        } else {
                            base_chance
                        }
                    }
                    None => base_chance,
                };
                let acc_stage = self.get_stage_from_value_volatile_status(
                    actioner as usize,
                    VolatileStatusWithValue::AccStage,
                );
                let eva_stage = self.get_stage_from_value_volatile_status(
                    !actioner as usize,
                    VolatileStatusWithValue::EvaStage,
                );
                let uncapped_chance = chance * get_acc_stage_multiplier(acc_stage, eva_stage);
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
            if unit.chance_of_success < random
                || (!unit.target_self
                    && self.has_valueless_volatile_status(
                        !actioner as usize,
                        VolatileStatus::Protection,
                    ))
            {
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
                    None,
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
                    None,
                    unit.power.unwrap(),
                    attacker.level as i32,
                    &attacker.moves[move_id].move_type,
                    actioner as usize,
                    ((!actioner) ^ unit.target_self) as usize,
                    base_hit_chance,
                ),
                Effect::PhysicalDrainAttack(drain) => self.attack(
                    battle_settings,
                    creatures,
                    creature_instances,
                    &attacker.moves[move_id].id,
                    true,
                    Some(drain),
                    unit.power.unwrap(),
                    attacker.level as i32,
                    &attacker.moves[move_id].move_type,
                    actioner as usize,
                    ((!actioner) ^ unit.target_self) as usize,
                    base_hit_chance,
                ),
                Effect::SpecialDrainAttack(drain) => self.attack(
                    battle_settings,
                    creatures,
                    creature_instances,
                    &attacker.moves[move_id].id,
                    false,
                    Some(drain),
                    unit.power.unwrap(),
                    attacker.level as i32,
                    &attacker.moves[move_id].move_type,
                    actioner as usize,
                    ((!actioner) ^ unit.target_self) as usize,
                    base_hit_chance,
                ),
                Effect::Status(ref status) => self.cause_status(
                    creature_instances,
                    ((!actioner) ^ unit.target_self) as usize,
                    status,
                    unit.target_self,
                ),
                Effect::Unimplemented => unimplemented!(
                    "Not yet implemented the unit for {:?}",
                    &attacker.moves[move_id]
                ),
                Effect::Counter => self.use_counter(creature_instances, creatures, actioner), //are always targeted the same way
                // Set or addition
                Effect::ValueVolatileStatusChange(ref volatile_status) => self
                    .value_volatile_status_change(
                        volatile_status,
                        &unit.power.unwrap(),
                        ((!actioner) ^ unit.target_self) as usize,
                    ),
                Effect::Charge(ref charge_move) => {
                    // Charge moves are always self-targeted
                    match charge_move {
                        ChargeMove::RazorWind => self.charge_move(
                            VolatileStatus::Charge(ChargeMove::RazorWind),
                            actioner as usize,
                        ),
                        ChargeMove::HyperBeam => todo!(),
                        ChargeMove::SolarBeam => self.solar_beam(actioner as usize),
                    }
                }
                Effect::ForceSwitch => self.force_random_switch(
                    creatures,
                    creature_instances,
                    (!actioner) ^ unit.target_self,
                ),
                Effect::GenericVolatileStatus(ref volatile_status) => self
                    .inflict_valueless_volatile_status(
                        volatile_status,
                        ((!actioner) ^ unit.target_self) as usize,
                    ),
                Effect::LevelAsDamage(physical) => self.deal_level_as_damage(
                    creatures,
                    creature_instances,
                    &attacker.moves[move_id].move_type,
                    actioner, // LevelAsDamage moves are always self-targeted
                    physical,
                ),
                Effect::Percentile(percent, physical) => self.percentile_damage(
                    creatures,
                    creature_instances,
                    ((!actioner) ^ unit.target_self) as usize,
                    percent,
                    physical,
                ),
                Effect::SunnyHeal => self.sunny_heal(
                    creatures,
                    creature_instances,
                    ((!actioner) ^ unit.target_self) as usize,
                ),
                Effect::Weather(ref weather_type) => self.start_weather(weather_type),
                Effect::Protection => self.protection(actioner as usize),
                Effect::SleepRequirement => {
                    creature_instances[((!actioner) ^ unit.target_self) as usize]
                        [self.battler_ids[((!actioner) ^ unit.target_self) as usize]]
                        .status
                        == Some(Status::Sleep)
                }
                Effect::SleepTalk => {
                    self.sleep_talk(battle_settings, creatures, creature_instances, actioner)
                }
                Effect::TurnRangeVolatileStatus(ref volatile_status, min, max) => self
                    .volatile_status_with_turn_range(
                        ((!actioner) ^ unit.target_self) as usize,
                        volatile_status,
                        min,
                        max,
                    ),
                Effect::Confusion => self.volatile_status_with_turn_range(
                    ((!actioner) ^ unit.target_self) as usize,
                    &VolatileStatusWithValue::Confused,
                    1,
                    4,
                ),
            };
        }
    }

    fn volatile_status_with_turn_range(
        &mut self,
        target: usize,
        value_volatile_status: &VolatileStatusWithValue,
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
        self.value_volatile_statuses[target].push((value_volatile_status.clone(), turns_remaining));
        true
    }

    fn sleep_talk(
        &mut self,
        battle_settings: &BattleSettings,
        creatures: &[[Creature; 6]; 2],
        creature_instances: &mut [[CreatureInstance; 6]; 2],
        actioner: bool,
    ) -> bool {
        let attacker = &creatures[actioner as usize][self.battler_ids[actioner as usize]];
        let mut allowed_move_ids = vec![];
        for i in 0..attacker.moves.len() {
            if !attacker.moves[i].id.is_banned_sleep_talk_move() {
                allowed_move_ids.push(i);
            }
        }
        if allowed_move_ids.len() == 0 {
            return false;
        }
        let move_id = rand::random::<usize>() % allowed_move_ids.len();

        self.use_move(
            battle_settings,
            creatures,
            creature_instances,
            move_id,
            actioner,
        );
        true
    }

    fn protection(&mut self, actioner: usize) -> bool {
        let protection_counter = self.get_value_from_value_volatile_status(
            actioner,
            VolatileStatusWithValue::ProtectionCounter,
        );
        match protection_counter {
            None => {
                self.volatile_statuses[actioner].push(VolatileStatus::Protection);
                self.value_volatile_status_change(
                    &VolatileStatusWithValue::ProtectionCounter,
                    &1,
                    actioner,
                )
            }
            Some(number) => {
                let roll: f32 = rand::random();
                let successful_roll = if number == 1 {
                    roll <= 0.5
                } else if number == 2 {
                    roll <= 0.25
                } else {
                    roll <= 0.125
                };
                if successful_roll {
                    self.volatile_statuses[actioner].push(VolatileStatus::Protection);
                    self.value_volatile_status_change(
                        &VolatileStatusWithValue::ProtectionCounter,
                        &1,
                        actioner,
                    );
                    true
                } else {
                    self.has_then_try_remove_value_volatile_status(
                        actioner,
                        &VolatileStatusWithValue::ProtectionCounter,
                    );
                    false
                }
            }
        }
    }

    fn solar_beam(&mut self, actioner: usize) -> bool {
        match self.weather.weather_type {
            Some(WeatherType::Sunny) => {
                self.has_then_try_remove_valueless_volatile_status(
                    actioner,
                    &VolatileStatus::Charge(ChargeMove::SolarBeam),
                );
                true
            }
            Some(WeatherType::Rainy) => self.charge_move(
                VolatileStatus::Charge(ChargeMove::SolarBeam),
                actioner as usize,
            ),
            None => self.charge_move(
                VolatileStatus::Charge(ChargeMove::SolarBeam),
                actioner as usize,
            ),
        }
    }

    fn sunny_heal(
        &mut self,
        creatures: &[[Creature; 6]; 2],
        creature_instances: &mut [[CreatureInstance; 6]; 2],
        taker: usize,
    ) -> bool {
        match self.weather.weather_type {
            Some(WeatherType::Sunny) => {
                self.percentile_damage(creatures, creature_instances, taker, -2.0 / 3.0, false)
            }
            Some(WeatherType::Rainy) => {
                self.percentile_damage(creatures, creature_instances, taker, -0.25, false)
            }
            None => self.percentile_damage(creatures, creature_instances, taker, -0.5, false),
        }
    }

    fn percentile_damage(
        &mut self,
        creatures: &[[Creature; 6]; 2],
        creature_instances: &mut [[CreatureInstance; 6]; 2],
        taker: usize,
        percent: f32,
        physical: bool,
    ) -> bool {
        let damage = (creatures[taker][self.battler_ids[taker]].stats.hp as f32 * percent) as i32;

        if damage < 0 {
            // Here negative is healing
            self.heal(creatures, creature_instances, taker, -damage);
        } else {
            let _ = self.take_damage(creature_instances, taker, damage, Some(physical));
        }
        true
    }

    fn force_random_switch(
        &mut self,
        creatures: &[[Creature; 6]; 2],
        creature_instances: &mut [[CreatureInstance; 6]; 2],
        target: bool,
    ) -> bool {
        // TODO test this formula
        if rand::random::<f32>()
            > (creatures[target as usize][self.battler_ids[target as usize]].level / 4) as f32
                / (creatures[!target as usize][self.battler_ids[!target as usize]].level
                    + creatures[target as usize][self.battler_ids[target as usize]].level)
                    as f32
        {
            // random roll resulted in success
            for i in 0..6 {
                if self.battler_ids[target as usize] == i {
                    continue;
                } else {
                    if creature_instances[target as usize][i].status != Some(Status::Fainted) {
                        loop {
                            let potential_switch_in = rand::random::<usize>() % 6;
                            if potential_switch_in != self.battler_ids[target as usize] {
                                if creature_instances[target as usize][potential_switch_in].status
                                    != Some(Status::Fainted)
                                {
                                    self.switch(
                                        creature_instances,
                                        potential_switch_in,
                                        target as usize,
                                    );
                                    self.inflict_valueless_volatile_status(
                                        &VolatileStatus::Flinch,
                                        target as usize,
                                    ); // Just in case if enemy is slower
                                    return true;
                                }
                            }
                        }
                    }
                }
            }
            // All other teammates are fainted therefore the move fails
            false
        } else {
            // random roll resulted in fail
            false
        }
    }

    fn charge_move(&mut self, charge_type: VolatileStatus, actioner: usize) -> bool {
        if self.has_then_try_remove_valueless_volatile_status(actioner, &charge_type) {
            true
        } else {
            self.volatile_statuses[actioner].push(charge_type);
            false
        }
    }

    /// Only for generic cases
    fn inflict_valueless_volatile_status(
        &mut self,
        volatile_status: &VolatileStatus,
        side: usize,
    ) -> bool {
        for ps in &mut self.volatile_statuses[side] {
            if ps == volatile_status {
                return false;
            }
        }
        self.volatile_statuses[side].push(volatile_status.clone());
        true
    }

    /// Currently limited to -6 and 6. Only for staged changes most likely
    fn value_volatile_status_change(
        &mut self,
        volatile_status: &VolatileStatusWithValue,
        change: &i32,
        side: usize,
    ) -> bool {
        for ps in &mut self.value_volatile_statuses[side] {
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
        self.value_volatile_statuses[side].push((volatile_status.clone(), *change));
        true
    }

    fn use_counter(
        &mut self,
        creature_instances: &mut [[CreatureInstance; 6]; 2],
        creatures: &[[Creature; 6]; 2],
        actioner: bool,
    ) -> bool {
        if creatures[!actioner as usize][self.battler_ids[!actioner as usize]]
            .effectiviness_when_attacked(&Type::Fighting)
            == Effectiviness::Immune
        {
            false
        } else {
            let counter_damage = self
                .get_value_from_value_volatile_status(
                    actioner as usize,
                    VolatileStatusWithValue::Counter,
                )
                .expect("counter to be prepared")
                * 2;
            if counter_damage == 0 {
                false // Probably useless unless
            } else {
                self.take_damage(
                    creature_instances,
                    !actioner as usize,
                    counter_damage as i32,
                    None,
                );
                true
            }
        }
    }

    fn cause_status(
        &self,
        creature_instances: &mut [[CreatureInstance; 6]; 2],
        status_taker: usize,
        status_opt: &Option<Status>,
        target_self: bool,
    ) -> bool {
        match status_opt {
            Some(status) => {
                // Can't do status, if already has a status (this is circumvented in Rest)
                if creature_instances[status_taker][self.battler_ids[status_taker]].status == None {
                    match status {
                        Status::Sleep => self.sleep(target_self, creature_instances, status_taker),
                        Status::Fainted => {
                            panic!("Faint should only happen through damage taken or perish song")
                        }
                        Status::Freeze => {
                            if self.weather.weather_type == Some(WeatherType::Sunny) {
                                return false; // Can't freeze during sunlight
                            } else {
                                creature_instances[status_taker][self.battler_ids[status_taker]]
                                    .status = Some(status.clone())
                            }
                        }
                        Status::BadlyPoisoned => {
                            creature_instances[status_taker][self.battler_ids[status_taker]]
                                .status_elapsed = 0;
                            creature_instances[status_taker][self.battler_ids[status_taker]]
                                .status = Some(status.clone())
                        }
                        _ => {
                            creature_instances[status_taker][self.battler_ids[status_taker]]
                                .status = Some(status.clone())
                        }
                    }
                } else {
                    return false; // Already has a status, while trying to inflict status (Other than rest)
                }
            }
            None => creature_instances[status_taker][self.battler_ids[status_taker]].status = None,
        }
        true
    }

    fn attack(
        &mut self,
        battle_settings: &BattleSettings,
        creatures: &[[Creature; 6]; 2],
        creature_instances: &mut [[CreatureInstance; 6]; 2],
        move_id: &MoveID, // TODO consider is this right solution
        physical: bool,
        drain: Option<f32>,
        power: i32,
        level: i32,
        attack_type: &Type,
        attacker: usize,
        damage_taker: usize,
        base_hit_chance: f32,
        // target_self: bool,
    ) -> bool {
        let (attack, defense, attack_stage, defense_stage, burn_screen) = if physical {
            (
                creatures[attacker][self.battler_ids[attacker]].stats.atk,
                creatures[damage_taker][self.battler_ids[damage_taker]]
                    .stats
                    .def,
                self.get_stage_from_value_volatile_status(
                    attacker,
                    VolatileStatusWithValue::AtkStage,
                ),
                self.get_stage_from_value_volatile_status(
                    damage_taker,
                    VolatileStatusWithValue::DefStage,
                ),
                if creature_instances[attacker][self.battler_ids[attacker]].status
                    == Some(Status::Burn)
                {
                    0.5
                } else {
                    1.0
                }, // TODO BURN + DEF SCREEN HERE
            )
        } else {
            (
                creatures[attacker][self.battler_ids[attacker]].stats.spa,
                creatures[damage_taker][self.battler_ids[damage_taker]]
                    .stats
                    .spd,
                self.get_stage_from_value_volatile_status(
                    attacker as usize,
                    VolatileStatusWithValue::SpaStage,
                ),
                self.get_stage_from_value_volatile_status(
                    damage_taker as usize,
                    VolatileStatusWithValue::SpdStage,
                ),
                1.0, // TODO SPD SCREEN HERE
            )
        };
        let crit_stage =
            self.get_stage_from_value_volatile_status(attacker, VolatileStatusWithValue::CrtStage);
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
            burn_screen,
            self.weather.get_weather_bonus(move_id, attack_type),
            stab_bonus,
            type_effectiviness.clone().into(),
            chance_bonuses,
            is_crit,
        ) as i32;
        // // (!actioner) ^ target_self
        // // (NOT player) xor (target selfs)
        // // player heals (modifies player health) -> (true ^ true) -> false
        // // player attacks (modifies enemy health) -> (true ^ false) -> true
        // // enemy heals (modifies enemy health) -> (false ^ true) -> true
        // // enemy attacks (modifies player health) -> (false ^ false) -> false
        let damage_taken =
            self.take_damage(creature_instances, damage_taker, damage, Some(physical));
        match drain {
            Some(drain_amount) => {
                if drain_amount > 0.0 {
                    // heal
                    self.heal(
                        creatures,
                        creature_instances,
                        attacker,
                        (drain_amount * damage_taken as f32) as i32,
                    );
                } else {
                    let _ = self.take_damage(
                        creature_instances,
                        attacker,
                        (-drain_amount * damage_taken as f32) as i32,
                        None,
                    );
                }
            }
            None => (),
        };
        type_effectiviness != Effectiviness::Immune
    }

    fn deal_level_as_damage(
        &mut self,
        creatures: &[[Creature; 6]; 2],
        creature_instances: &mut [[CreatureInstance; 6]; 2],
        attack_type: &Type,
        actioner: bool,
        physical: bool,
    ) -> bool {
        let attacker = actioner as usize;
        let damage_taker = !actioner as usize;
        let type_effectiviness = creatures[damage_taker][self.battler_ids[damage_taker]]
            .effectiviness_when_attacked(attack_type);
        if type_effectiviness != Effectiviness::Immune {
            let damage = creatures[attacker as usize][self.battler_ids[attacker as usize]].level;
            let _ = self.take_damage(creature_instances, damage_taker, damage, Some(physical));
            true
        } else {
            false
        }
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
                    self.get_stage_from_value_volatile_status(0, VolatileStatusWithValue::SpeStage),
                )) as i32;
            let speed_1 = (self.get_battler(1, creatures).stats.spe as f32
                * get_stat_stage_multiplier(
                    self.get_stage_from_value_volatile_status(1, VolatileStatusWithValue::SpeStage),
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
    // fn get_battler<'a>(
    //     &'a self,
    //     side: usize,
    //     creatures: &'a [[CreatureInstance; 6]; 2],
    // ) -> &mut CreatureInstance {
    //     &creatures[side][self.battler_ids[side]]
    // }
}
fn random_roll() -> bool {
    rand::random()
}

#[derive(Serialize, Debug, Deserialize)]
struct TempMoveManager {
    moves: Vec<NotFinalMove>,
}
// #[derive(Debug)]
struct MoveManager {
    moves: Vec<Move>,
}
#[derive(Serialize, Debug, Deserialize)]
struct NotFinalMove {
    accuracy: Option<u8>,
    category: String,
    ename: String,
    id: u16,
    power: Option<u8>,
    pp: u8,
    move_type: String,
}

impl From<NotFinalMove> for Move {
    fn from(item: NotFinalMove) -> Self {
        Move {
            id: item.ename.as_str().into(),
            move_type: item.move_type.as_str().into(),
            chance_of_success: item.accuracy.map(|x| x as f32 / 100.0),
            pp: item.pp,
            priority: 0,
            units: vec![MoveUnit {
                chance_of_success: 1.0,
                power: item.power.map(|x| x as i32),
                effect: item.category.as_str().into(),
                needs_target: true,
                target_self: false,
                continues_previous_unit: true,
            }],
        }
    }
}

impl MoveID {
    pub fn is_sleep_move(&self) -> bool {
        match self {
            MoveID::Snore => true,
            MoveID::SleepTalk => true,
            _ => false,
        }
    }
    pub fn is_thaw_move(&self) -> bool {
        match self {
            MoveID::FlameWheel => true,
            MoveID::SacredFire => true,
            MoveID::FlareBlitz => true,
            _ => false,
        }
    }
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
/// [Effect::PhysicalAttack], [Effect::SpecialAttack], [Effect::PhysicalDrainAttack],[Effect::SpecialDrainAttack] could be inside the same enum
/// For [Effect::PhysicalDrainAttack],[Effect::SpecialDrainAttack] negative value = recoil, positive value = heal
enum Effect {
    PhysicalAttack,
    SpecialAttack,
    PhysicalDrainAttack(f32),
    SpecialDrainAttack(f32),
    Status(Option<Status>),
    ValueVolatileStatusChange(VolatileStatusWithValue),
    Counter,
    Charge(ChargeMove),
    ForceSwitch,
    GenericVolatileStatus(VolatileStatus),
    LevelAsDamage(bool), // bool = physical
    Weather(WeatherType),
    Percentile(f32, bool), // bool = physical // TODO probably should remove physical
    SunnyHeal,
    Protection,
    SleepRequirement, // Can make this into generic one, if has multiple uses
    SleepTalk,
    TurnRangeVolatileStatus(VolatileStatusWithValue, i32, i32),
    Confusion,
    Unimplemented,
}

impl From<&str> for Effect {
    fn from(item: &str) -> Self {
        match item {
            "physical" => Effect::PhysicalAttack,
            "special" => Effect::SpecialAttack,
            "status" => Effect::Unimplemented,
            _ => unimplemented!(),
        }
    }
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

#[derive(Serialize, Debug, Deserialize)]
// #[serde(rename_all = "PascalCase")]
struct TeamManager {
    teams: Vec<Team>,
}
#[derive(Serialize, Debug, Deserialize, Clone)]
// #[serde(rename_all = "PascalCase")]
struct Team {
    team: [NotFinalCreature; 6],
}

#[derive(Serialize, Debug, Deserialize, Clone)]
// #[serde(rename_all = "PascalCase")]
struct NotFinalCreature {
    pub species: String,
    pub level: i32,
    pub moves: Vec<String>,
    pub stats: Stats,
    pub types: Vec<String>,
}

impl From<&NotFinalCreature> for Creature {
    fn from(item: &NotFinalCreature) -> Self {
        Creature {
            species: item.species.clone(),
            level: item.level,
            moves: (&item.moves)
                .into_iter()
                .map(|m| (&<&str as Into<MoveID>>::into(m.as_str())).into())
                .collect(),
            stats: item.stats,
            types: (&item.types)
                .into_iter()
                .map(|t| t.as_str().into())
                .collect(),
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
    pub status_elapsed: i32, // for Sleep and Badly Poisoned
    pub status: Option<Status>,
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
    Placeh,
    Charge(ChargeMove),
    Flinch,
    Protection,
}

#[derive(Clone, PartialEq, Debug)]
pub enum ChargeMove {
    RazorWind,
    HyperBeam,
    SolarBeam,
}

#[derive(Clone, PartialEq, Debug)]
pub enum VolatileStatusWithValue {
    AtkStage,
    DefStage,
    SpaStage,
    SpdStage,
    SpeStage,
    EvaStage,
    AccStage,
    CrtStage,
    Counter,
    ProtectionCounter, // Endure not in data sets so we don't have to specify are we using Endure or Protect
    Taunted,
    Confused,
}

#[derive(Clone, PartialEq, Debug)]
pub enum Status {
    Burn,
    Freeze,
    Paralysis,
    Poison,
    BadlyPoisoned,
    Sleep,
    Fainted,
}

#[derive(Clone, PartialEq, Debug)]
pub enum WeatherType {
    Sunny,
    Rainy,
}

impl WeatherType {
    fn get_weather_bonus(&self, move_id: &MoveID, attack_type: &Type) -> f32 {
        match self {
            WeatherType::Sunny => {
                if attack_type == &Type::Fire {
                    1.5
                } else if attack_type == &Type::Water {
                    0.5
                } else {
                    1.0
                }
            }
            WeatherType::Rainy => {
                if attack_type == &Type::Water {
                    1.5
                } else if attack_type == &Type::Fire || move_id == &MoveID::SolarBeam {
                    0.5
                } else {
                    1.0
                }
            }
        }
    }
}

pub struct Weather {
    pub weather_type: Option<WeatherType>,
    pub end_turn: i32,
}

impl Weather {
    pub fn get_weather_bonus(&self, move_id: &MoveID, attack_type: &Type) -> f32 {
        match &self.weather_type {
            Some(weather) => weather.get_weather_bonus(move_id, attack_type),
            None => 1.0,
        }
    }
}

impl Default for Weather {
    fn default() -> Self {
        Weather {
            weather_type: None,
            end_turn: 0,
        }
    }
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
                    moves: vec![(&MoveID::Pound).into()],
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
                    moves: vec![(&MoveID::Pound).into()],
                    stats: Stats::default(),
                    types: vec![Type::Normal],
                },
                Creature {
                    species: "Placeholder".to_string(),
                    level: 100,
                    moves: vec![(&MoveID::Pound).into()],
                    stats: Stats::default(),
                    types: vec![Type::Normal],
                },
                Creature {
                    species: "Placeholder".to_string(),
                    level: 100,
                    moves: vec![(&MoveID::Pound).into()],
                    stats: Stats::default(),
                    types: vec![Type::Normal],
                },
                Creature {
                    species: "Placeholder".to_string(),
                    level: 100,
                    moves: vec![(&MoveID::Pound).into()],
                    stats: Stats::default(),
                    types: vec![Type::Normal],
                },
                Creature {
                    species: "Placeholder".to_string(),
                    level: 100,
                    moves: vec![(&MoveID::Pound).into()],
                    stats: Stats::default(),
                    types: vec![Type::Normal],
                },
            ],
            [
                Creature {
                    species: "Placeholder".to_string(),
                    level: 100,
                    moves: vec![(&MoveID::Pound).into()],
                    stats: Stats::default(),
                    types: vec![Type::Normal],
                },
                Creature {
                    species: "Placeholder".to_string(),
                    level: 100,
                    moves: vec![(&MoveID::Pound).into()],
                    stats: Stats::default(),
                    types: vec![Type::Normal],
                },
                Creature {
                    species: "Placeholder".to_string(),
                    level: 100,
                    moves: vec![(&MoveID::Pound).into()],
                    stats: Stats::default(),
                    types: vec![Type::Normal],
                },
                Creature {
                    species: "Placeholder".to_string(),
                    level: 100,
                    moves: vec![(&MoveID::Pound).into()],
                    stats: Stats::default(),
                    types: vec![Type::Normal],
                },
                Creature {
                    species: "Placeholder".to_string(),
                    level: 100,
                    moves: vec![(&MoveID::Pound).into()],
                    stats: Stats::default(),
                    types: vec![Type::Normal],
                },
                Creature {
                    species: "Placeholder".to_string(),
                    level: 100,
                    moves: vec![(&MoveID::Pound).into()],
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
                    status_elapsed: 0,
                    status: None,
                },
                CreatureInstance {
                    current_health: 300,
                    status_elapsed: 0,
                    status: None,
                },
                CreatureInstance {
                    current_health: 300,
                    status_elapsed: 0,
                    status: None,
                },
                CreatureInstance {
                    current_health: 300,
                    status_elapsed: 0,
                    status: None,
                },
                CreatureInstance {
                    current_health: 300,
                    status_elapsed: 0,
                    status: None,
                },
                CreatureInstance {
                    current_health: 300,
                    status_elapsed: 0,
                    status: None,
                },
            ],
            [
                CreatureInstance {
                    current_health: 300,
                    status_elapsed: 0,
                    status: None,
                },
                CreatureInstance {
                    current_health: 300,
                    status_elapsed: 0,
                    status: None,
                },
                CreatureInstance {
                    current_health: 300,
                    status_elapsed: 0,
                    status: None,
                },
                CreatureInstance {
                    current_health: 300,
                    status_elapsed: 0,
                    status: None,
                },
                CreatureInstance {
                    current_health: 300,
                    status_elapsed: 0,
                    status: None,
                },
                CreatureInstance {
                    current_health: 300,
                    status_elapsed: 0,
                    status: None,
                },
            ],
        ]
    }

    #[test]
    /// Tests always_hits as well
    fn multiple_hits_twice_375_then_twice_125() {
        let (
            mut creatures,
            mut creature_instances,
            mut battle_instance,
            mut battle_settings,
            combat_action1,
            combat_action2,
        ) = get_basic_setup();

        battle_settings.always_hits = true;
        creatures[0][0].moves[0] = (&MoveID::DoubleSlap).into();

        let mut probability_2 = 0;
        let mut probability_3 = 0;
        let mut probability_4 = 0;
        let mut probability_5 = 0;
        for _ in 0..1000 {
            battle_instance.turn(
                &battle_settings,
                &creatures,
                &mut creature_instances,
                &[combat_action1.clone(), combat_action2.clone()],
            );
            match 300 - creature_instances[1][0].current_health {
                38 => probability_2 += 1,
                57 => probability_3 += 1,
                76 => probability_4 += 1,
                95 => probability_5 += 1,
                value => println!("value: {}", value),
            }
            creature_instances[0][0].current_health = 300;
            creature_instances[1][0].current_health = 300;
        }
        println!(
            "2: {}, 3: {}, 4: {} 5: {}",
            probability_2, probability_3, probability_4, probability_5
        );
        assert!(probability_2 > 300);
        assert!(probability_3 > 300);
        assert!(probability_4 < 160);
        assert!(probability_5 < 160);
    }
    #[test]
    fn counter_works() {
        let (
            mut creatures,
            mut creature_instances,
            mut battle_instance,
            battle_settings,
            combat_action1,
            combat_action2,
        ) = get_basic_setup();
        creatures[0][0].moves[0] = (&MoveID::Counter).into();
        battle_instance.turn(
            &battle_settings,
            &creatures,
            &mut creature_instances,
            &[combat_action1.clone(), combat_action2.clone()],
        );
        assert_eq!(creature_instances[0][0].current_health, 244);
        assert_eq!(creature_instances[1][0].current_health, 188);
    }
    #[test]
    fn burn_freeze_x_280ish() {
        let (
            mut creatures,
            mut creature_instances,
            mut battle_instance,
            battle_settings,
            combat_action1,
            combat_action2,
        ) = get_basic_setup();

        creatures[0][0].moves[0] = (&MoveID::FirePunch).into();
        creatures[1][0].moves[0] = (&MoveID::IcePunch).into();

        let mut burns = 0;
        let mut freeze = 0;
        let mut frozen_on_second_turn = 0;
        let mut reduced_attack = 0;
        let mut no_attack = 0;

        for _ in 0..1000 {
            for _ in 0..2 {
                if creature_instances[0][0].status == Some(Status::Freeze) {
                    frozen_on_second_turn += 1;
                }
                battle_instance.turn(
                    &battle_settings,
                    &creatures,
                    &mut creature_instances,
                    &[combat_action1.clone(), combat_action2.clone()],
                );
                if creature_instances[1][0].status == Some(Status::Burn) {
                    // 0.1 chance to get burnt on the first turn
                    // 0.1828 chance to be burnt on the second turn (0.92 (0 not frozen) * 0.9 (1 not burnt) * 0.1 (burn chance))
                    // burn counter activated 0.2828 times on average
                    burns += 1;
                }
                if creature_instances[0][0].status == Some(Status::Freeze) {
                    // 0.1 chance to get frozen on first turn
                    // 0.08 chance to be frozen when attacking on second turn (0.1 * 0.8)
                    // 0.08 + 0.092 = 0.172 chance to be frozen on end of second turn.
                    // freeze counter activated 0.272 times on average
                    freeze += 1;
                }
                match 300 - creature_instances[0][0].current_health {
                    69 => (),
                    35 => reduced_attack += 1,
                    value => println!("value 0: {} {:?}", value, creature_instances[0][0].status),
                }
                match 300 - creature_instances[1][0].current_health {
                    69 => (),
                    87 => burns += 1,
                    0 => no_attack += 1,
                    18 => {
                        burns += 1;
                        no_attack += 1
                    }
                    value => println!("value 1: {} {:?}", value, creature_instances[1][0].status),
                }
                creature_instances[0][0].current_health = 300;
                creature_instances[1][0].current_health = 300;
            }
            creature_instances[0][0].status = None;
            creature_instances[1][0].status = None;
        }
        println!(
            "burns: {}, frozen: {}, frozen and trying to move {}, but couldn't move times {}, reduced by burn times {}",
            burns / 2,
            freeze,
            frozen_on_second_turn,
            no_attack,
            reduced_attack
        );
        assert_eq!(burns / 2, reduced_attack);
        assert!(burns / 2 > 230);
        assert!(burns / 2 < 330);
        assert!(freeze > 220);
        assert!(freeze < 320);
        assert!(no_attack as f32 <= frozen_on_second_turn as f32 * 0.95);
        assert!(no_attack as f32 >= frozen_on_second_turn as f32 * 0.6);
    }
    #[test]
    fn flinch_x_300() {
        let (
            mut creatures,
            mut creature_instances,
            mut battle_instance,
            battle_settings,
            combat_action1,
            combat_action2,
        ) = get_basic_setup();

        creatures[0][0].moves[0] = (&MoveID::Bite).into();

        let mut flinches = 0;
        for _ in 0..1000 {
            battle_instance.turn(
                &battle_settings,
                &creatures,
                &mut creature_instances,
                &[combat_action1.clone(), combat_action2.clone()],
            );
            match 300 - creature_instances[0][0].current_health {
                56 => (),
                0 => flinches += 1,
                value => println!("value: {}", value),
            }
            creature_instances[0][0].current_health = 300;
            creature_instances[1][0].current_health = 300;
            creature_instances[1][0].status = None;
        }
        println!("flinches: {}", flinches);
        assert!(flinches > 240);
        assert!(flinches < 375);
    }
    #[test]
    fn charge_test() {
        let (
            mut creatures,
            mut creature_instances,
            mut battle_instance,
            battle_settings,
            combat_action1,
            combat_action2,
        ) = get_basic_setup();

        creatures[0][0].moves[0] = (&MoveID::RazorWind).into();

        let mut no_hit_count = 0;
        let mut hit_count = 0;
        for _ in 0..10 {
            battle_instance.turn(
                &battle_settings,
                &creatures,
                &mut creature_instances,
                &[combat_action1.clone(), combat_action2.clone()],
            );
            match 300 - creature_instances[1][0].current_health {
                0 => no_hit_count += 1,
                116 => hit_count += 1,
                value => println!("value: {}", value),
            }
            creature_instances[0][0].current_health = 300;
            creature_instances[1][0].current_health = 300;
        }
        println!("charge turns: {}, hit turns: {}", no_hit_count, hit_count);
        assert_eq!(no_hit_count, 5);
    }
    #[test]
    fn crit_and_growl_and_paralysis_works() {
        let (
            mut creatures,
            mut creature_instances,
            mut battle_instance,
            mut battle_settings,
            combat_action1,
            combat_action2,
        ) = get_basic_setup();

        battle_settings.crit_enabled = true;
        creatures[0][0].moves[0] = (&MoveID::Growl).into();
        creatures[1][0].moves[0] = (&MoveID::KarateChop).into();
        creature_instances[1][0].status = Some(Status::Paralysis);

        let mut no_crit_count = 0;
        let mut crit_count = 0;
        let mut cant_move_count = 0;
        for _ in 0..1000 {
            battle_instance.turn(
                &battle_settings,
                &creatures,
                &mut creature_instances,
                &[combat_action1.clone(), combat_action2.clone()],
            );
            match 300 - creature_instances[0][0].current_health {
                60 => no_crit_count += 1,
                176 => crit_count += 1,
                0 => cant_move_count += 1,
                value => println!("value: {}", value),
            }
            creature_instances[0][0].current_health = 300;
            creature_instances[1][0].current_health = 300;
            battle_instance.value_volatile_statuses = [vec![], vec![]];
        }
        println!(
            "no crits: {}, crits: {}, couldn't move {}",
            no_crit_count, crit_count, cant_move_count
        );
        assert!(crit_count > 70);
        assert!(crit_count < 120);
        assert!(cant_move_count > 200);
        assert!(cant_move_count < 300);
    }

    #[test]
    fn random_roll_spread_test() {
        let (
            mut creatures,
            mut creature_instances,
            mut battle_instance,
            mut battle_settings,
            combat_action1,
            combat_action2,
        ) = get_basic_setup();

        battle_settings.roll = Roll::RandomRoll;
        creatures[0][0].moves[0] = (&MoveID::BodySlam).into();

        let mut random_roll_sums = 0;
        for _ in 0..1000 {
            battle_instance.turn(
                &battle_settings,
                &creatures,
                &mut creature_instances,
                &[combat_action1.clone(), combat_action2.clone()],
            );
            random_roll_sums += 300 - creature_instances[1][0].current_health;
            creature_instances[0][0].current_health = 300;
            creature_instances[1][0].current_health = 300;
        }
        println!("Average {}", random_roll_sums as f32 / 1000.0);
        assert!(random_roll_sums < 109000);
        assert!(random_roll_sums > 107000);
    }

    #[test]
    fn whirlwind_priority_test() {
        // TODO replace bodyslam
        let (
            mut creatures,
            mut creature_instances,
            mut battle_instance,
            battle_settings,
            combat_action1,
            combat_action2,
        ) = get_basic_setup();

        creatures[0][0].moves[0] = (&MoveID::Whirlwind).into();
        creatures[1][0].moves[0] = (&MoveID::BodySlam).into();
        creatures[1][2].moves[0] = (&MoveID::Scratch).into();
        creatures[1][2].stats.atk = 50;
        creatures[1][3].moves[0] = (&MoveID::HornAttack).into();
        creatures[1][4].moves[0] = (&MoveID::SwordsDance).into();
        creature_instances[1][4].status = Some(Status::Fainted); // Whirlwind won't change to fainted
        creatures[1][5].moves[0] = (&MoveID::KarateChop).into();
        let mut bodyslam_count = 0;
        let mut pound_count = 0;
        let mut scratch_count = 0;
        let mut hornattack_count = 0;
        let mut karatechop_count = 0;
        for _ in 0..1000 {
            battle_instance.turn(
                &battle_settings,
                &creatures,
                &mut creature_instances,
                &[combat_action1.clone(), combat_action2.clone()],
            );
            match 300 - creature_instances[0][0].current_health {
                116 => bodyslam_count += 1,
                56 => pound_count += 1,
                29 => scratch_count += 1,
                90 => hornattack_count += 1,
                99 => karatechop_count += 1,
                value => println!("value: {}", value),
            }
            creature_instances[0][0].current_health = 300;
            creature_instances[1][0].current_health = 300;
        }
        println!(
            "bodyslam count: {}, pound count: {}, scratch_count: {}, hornattack_count: {}, karatechop_count: {}",
            bodyslam_count, pound_count, scratch_count, hornattack_count, karatechop_count
        );
        assert!(bodyslam_count > 150);
        assert!(bodyslam_count < 250);
        assert!(scratch_count > 150);
        assert!(scratch_count < 250);
        assert!(hornattack_count > 150);
        assert!(hornattack_count > 150);
        assert!(karatechop_count > 150);
        assert!(karatechop_count < 250);
    }
    #[test]
    fn drain_test() {
        let (
            mut creatures,
            mut creature_instances,
            mut battle_instance,
            battle_settings,
            combat_action1_1,
            combat_action2,
        ) = get_basic_setup();

        let combat_action1_0 = CombatAction::Attack(1);
        creatures[0][0].moves[0] = (&MoveID::GigaDrain).into();
        creatures[0][0].moves.push((&MoveID::DoubleEdge).into());
        creatures[1][0].moves[0] = (&MoveID::Pound).into();

        battle_instance.turn(
            &battle_settings,
            &creatures,
            &mut creature_instances,
            &[combat_action1_0.clone(), combat_action2.clone()],
        );
        assert_eq!(300 - creature_instances[0][0].current_health, 110);
        assert_eq!(300 - creature_instances[1][0].current_health, 163);
        battle_instance.turn(
            &battle_settings,
            &creatures,
            &mut creature_instances,
            &[combat_action1_1.clone(), combat_action2.clone()],
        );
        assert_eq!(300 - creature_instances[0][0].current_health, 139);
        assert_eq!(300 - creature_instances[1][0].current_health, 218);
    }
    #[test]
    fn level_as_damage_test() {
        let (
            mut creatures,
            mut creature_instances,
            mut battle_instance,
            battle_settings,
            combat_action1,
            combat_action2,
        ) = get_basic_setup();

        creatures[0][0].moves[0] = (&MoveID::SeismicToss).into();
        creatures[1][0].moves[0] = (&MoveID::NightShade).into();

        battle_instance.turn(
            &battle_settings,
            &creatures,
            &mut creature_instances,
            &[combat_action1.clone(), combat_action2.clone()],
        );
        assert_eq!(300 - creature_instances[0][0].current_health, 0); // Immune to ghost
        assert_eq!(300 - creature_instances[1][0].current_health, 100);
    }
    #[test]
    fn percentile_test() {
        let (
            mut creatures,
            mut creature_instances,
            mut battle_instance,
            battle_settings,
            combat_action1,
            combat_action2,
        ) = get_basic_setup();

        creature_instances[1][0].current_health = 150;
        creatures[1][0].stats.hp = 150;
        creatures[0][0].moves[0] = (&MoveID::SeismicToss).into();
        creatures[1][0].moves[0] = (&MoveID::Recover).into();

        battle_instance.turn(
            &battle_settings,
            &creatures,
            &mut creature_instances,
            &[combat_action1.clone(), combat_action2.clone()],
        );
        assert_eq!(300 - creature_instances[0][0].current_health, 0);
        assert_eq!(150 - creature_instances[1][0].current_health, 25);
    }
    #[test]
    fn synthesis_test() {
        let (
            mut creatures,
            mut creature_instances,
            mut battle_instance,
            battle_settings,
            combat_action1,
            combat_action2,
        ) = get_basic_setup();

        creature_instances[0][0].current_health = 50;
        creatures[0][0].moves[0] = (&MoveID::Synthesis).into();
        creatures[1][0].moves[0] = (&MoveID::SunnyDay).into();
        // Harsh sunlight strengthens the power of Fire-type moves by 50% and weakens the power of Water-type moves by 50%. During harsh sunlight, no Pokémon can be frozen.
        // The recovery moves Synthesis, Morning Sun, and Moonlight restore more HP than usual in harsh sunlight, and less than usual in most other weather.
        // From Generation III onward, during no weather or strong winds they restore ½ total HP, during harsh sunlight they restore ⅔ total HP, and during other weather they restore ¼ total HP.
        // during harsh sunlight, Thunder only has 50% accuracy
        // Solar Beam and Solar Blade become one-turn moves in harsh sunlight,
        // Can't be frozen during sunlight

        battle_instance.turn(
            &battle_settings,
            &creatures,
            &mut creature_instances,
            &[combat_action1.clone(), combat_action2.clone()],
        );
        assert_eq!(300 - creature_instances[0][0].current_health, 100);
        creature_instances[0][0].current_health = 50;
        battle_instance.turn(
            &battle_settings,
            &creatures,
            &mut creature_instances,
            &[combat_action1.clone(), combat_action2.clone()],
        );
        assert_eq!(300 - creature_instances[0][0].current_health, 50); // Synthesis when sunny
        creature_instances[0][0].current_health = 50;
        battle_instance.weather.weather_type = Some(WeatherType::Rainy);
        battle_instance.turn(
            &battle_settings,
            &creatures,
            &mut creature_instances,
            &[combat_action1.clone(), combat_action2.clone()],
        );
        assert_eq!(300 - creature_instances[0][0].current_health, 175); // Synthesis when raining
    }
    #[test]
    fn solarbeam_test() {
        let (
            mut creatures,
            mut creature_instances,
            mut battle_instance,
            battle_settings,
            combat_action1,
            combat_action2,
        ) = get_basic_setup();
        creatures[0][0].moves[0] = (&MoveID::SolarBeam).into();
        creatures[1][0].moves[0] = (&MoveID::SunnyDay).into();
        // Harsh sunlight strengthens the power of Fire-type moves by 50% and weakens the power of Water-type moves by 50%. During harsh sunlight, no Pokémon can be frozen.
        // The recovery moves Synthesis, Morning Sun, and Moonlight restore more HP than usual in harsh sunlight, and less than usual in most other weather.
        // From Generation III onward, during no weather or strong winds they restore ½ total HP, during harsh sunlight they restore ⅔ total HP, and during other weather they restore ¼ total HP.
        // during harsh sunlight, Thunder only has 50% accuracy
        // Solar Beam and Solar Blade become one-turn moves in harsh sunlight,
        // Can't be frozen during sunlight
        for i in 0..5 {
            battle_instance.turn(
                &battle_settings,
                &creatures,
                &mut creature_instances,
                &[combat_action1.clone(), combat_action2.clone()],
            );
            if i == 0 || i == 3 {
                assert_eq!(300 - creature_instances[1][0].current_health, 0);
            } else if i == 1 || i == 2 {
                assert_eq!(300 - creature_instances[1][0].current_health, 109);
            } else if i == 4 {
                assert_eq!(300 - creature_instances[1][0].current_health, 55);
            }
            if i >= 2 {
                battle_instance.weather.weather_type = Some(WeatherType::Rainy)
            } // Turn 3 and 4 no sunlight
            if i == 2 {
                assert_eq!(battle_instance.weather.end_turn, 4)
            } else if i == 3 {
                // sunlight reseted so sunny day is successful and end turn updates
                assert_eq!(battle_instance.weather.end_turn, 7)
            }
            creature_instances[1][0].current_health = 300;
        }
    }
    #[test]
    fn thunder_icebeam_sunny_test() {
        let (
            mut creatures,
            mut creature_instances,
            mut battle_instance,
            battle_settings,
            combat_action1,
            combat_action2,
        ) = get_basic_setup();

        creatures[0][0].moves[0] = (&MoveID::IceBeam).into();
        creatures[1][0].moves[0] = (&MoveID::Thunder).into();
        // Harsh sunlight strengthens the power of Fire-type moves by 50% and weakens the power of Water-type moves by 50%. During harsh sunlight, no Pokémon can be frozen.
        // The recovery moves Synthesis, Morning Sun, and Moonlight restore more HP than usual in harsh sunlight, and less than usual in most other weather.
        // From Generation III onward, during no weather or strong winds they restore ½ total HP, during harsh sunlight they restore ⅔ total HP, and during other weather they restore ¼ total HP.
        // during harsh sunlight, Thunder only has 50% accuracy
        // Solar Beam and Solar Blade become one-turn moves in harsh sunlight,
        // Can't be frozen during sunlight

        let mut thunder_misses = 0;
        battle_instance.weather.weather_type = Some(WeatherType::Sunny);
        battle_instance.weather.end_turn = 999;
        for _ in 0..1000 {
            assert!(battle_instance.weather.weather_type == Some(WeatherType::Sunny));
            battle_instance.turn(
                &battle_settings,
                &creatures,
                &mut creature_instances,
                &[combat_action1.clone(), combat_action2.clone()],
            );
            if creature_instances[0][0].current_health == 300 {
                thunder_misses += 1;
            }
            creature_instances[0][0].current_health = 300;
            creature_instances[1][0].current_health = 300;
            creature_instances[0][0].status = None;
            assert!(creature_instances[1][0].status == None);
        }

        assert!(battle_instance.weather.weather_type == None);
        println!("thunder misses {}", thunder_misses);
        assert!(400 < thunder_misses);
        assert!(600 > thunder_misses);
    }
    #[test]
    fn thunder_rain_test() {
        let (
            mut creatures,
            mut creature_instances,
            mut battle_instance,
            battle_settings,
            combat_action1,
            combat_action2,
        ) = get_basic_setup();
        creatures[1][0].moves[0] = (&MoveID::Thunder).into();
        // Harsh sunlight strengthens the power of Fire-type moves by 50% and weakens the power of Water-type moves by 50%. During harsh sunlight, no Pokémon can be frozen.
        // The recovery moves Synthesis, Morning Sun, and Moonlight restore more HP than usual in harsh sunlight, and less than usual in most other weather.
        // From Generation III onward, during no weather or strong winds they restore ½ total HP, during harsh sunlight they restore ⅔ total HP, and during other weather they restore ¼ total HP.
        // during harsh sunlight, Thunder only has 50% accuracy
        // Solar Beam and Solar Blade become one-turn moves in harsh sunlight,
        // Can't be frozen during sunlight

        battle_instance.weather.weather_type = Some(WeatherType::Rainy);
        battle_instance.weather.end_turn = 999;
        for _ in 0..1000 {
            assert!(battle_instance.weather.weather_type == Some(WeatherType::Rainy));
            battle_instance.turn(
                &battle_settings,
                &creatures,
                &mut creature_instances,
                &[combat_action1.clone(), combat_action2.clone()],
            );
            assert_ne!(creature_instances[0][0].current_health, 300);

            creature_instances[0][0].current_health = 300;
            creature_instances[1][0].current_health = 300;
            creature_instances[0][0].status = None;
            assert!(creature_instances[1][0].status == None);
        }

        assert!(battle_instance.weather.weather_type == None);
    }

    #[test]
    fn protect_test() {
        let (
            mut creatures,
            mut creature_instances,
            mut battle_instance,
            battle_settings,
            combat_action1,
            combat_action2,
        ) = get_basic_setup();
        creatures[0][0].moves[0] = (&MoveID::Protect).into();
        let mut protect_worked = 0;
        let mut protect_0 = 0;
        let mut protect_1 = 0;
        let mut protect_2 = 0;
        let mut protect_3 = 0;
        let mut protect_4 = 0;
        for _ in 0..1000 {
            battle_instance.turn(
                &battle_settings,
                &creatures,
                &mut creature_instances,
                &[combat_action1.clone(), combat_action2.clone()],
            );
            if creature_instances[0][0].current_health == 300 {
                protect_worked += 1;
                match battle_instance.value_volatile_statuses[0][0].1 {
                    0 => panic!("should not be 0"),
                    1 => protect_0 += 1,
                    2 => protect_1 += 1,
                    3 => protect_2 += 1,
                    4 => protect_3 += 1,
                    _ => protect_4 += 1,
                };
                assert_eq!(battle_instance.value_volatile_statuses[0].len(), 1);
            } else {
                assert_eq!(battle_instance.value_volatile_statuses[0].len(), 0);
            }
            creature_instances[0][0].current_health = 300;
        }
        println!(
            "protected: {}, 1 {}, 2 {}, 3, {}, 4 {}, 5+ {}",
            protect_worked, protect_0, protect_1, protect_2, protect_3, protect_4
        );
        assert!(protect_worked > 540);
        assert!(protect_worked < 700);
        // TODO Protect should fail when last move // implement later
        // TODO check other effects of protect from bulbapedida
    }
    #[test]
    /// tests badly poisoned, rest, sleep talk, snore, giving status with someone already with a status
    fn rest_sleep_badly_poisoned_test() {
        let (
            mut creatures,
            mut creature_instances,
            mut battle_instance,
            battle_settings,
            combat_action1,
            combat_action2_0,
        ) = get_basic_setup();
        creature_instances[1][0].status = Some(Status::BadlyPoisoned);

        creatures[0][0].moves[0] = (&MoveID::ThunderWave).into();
        creatures[1][0].moves[0] = (&MoveID::Rest).into();
        creatures[1][0].moves.push((&MoveID::Snore).into());
        creatures[1][0].moves.push((&MoveID::SleepTalk).into());

        let combat_action2_1 = CombatAction::Attack(1);
        let combat_action2_2 = CombatAction::Attack(2);
        battle_instance.turn(
            &battle_settings,
            &creatures,
            &mut creature_instances,
            &[combat_action1.clone(), combat_action2_1.clone()],
        );
        assert_eq!(300 - creature_instances[1][0].current_health, 18);
        battle_instance.turn(
            &battle_settings,
            &creatures,
            &mut creature_instances,
            &[combat_action1.clone(), combat_action2_2.clone()],
        );
        assert_eq!(300 - creature_instances[1][0].current_health, 55);
        battle_instance.turn(
            &battle_settings,
            &creatures,
            &mut creature_instances,
            &[combat_action1.clone(), combat_action2_0.clone()],
        );
        assert_eq!(creature_instances[1][0].current_health, 300);
        battle_instance.turn(
            &battle_settings,
            &creatures,
            &mut creature_instances,
            &[combat_action1.clone(), combat_action2_1.clone()],
        );
        // Rest fails
        creature_instances[1][0].current_health = 50;
        battle_instance.turn(
            &battle_settings,
            &creatures,
            &mut creature_instances,
            &[combat_action1.clone(), combat_action2_0.clone()],
        );
        assert_eq!(creature_instances[1][0].current_health, 50);
        // Rest succeeds
        battle_instance.turn(
            &battle_settings,
            &creatures,
            &mut creature_instances,
            &[combat_action1.clone(), combat_action2_0.clone()],
        );
        assert_eq!(creature_instances[1][0].current_health, 300);
        // Sleep talk test
        creatures[1][0].moves[0] = (&MoveID::Pound).into(); // Snore should deal same damage as pound
        battle_instance.turn(
            &battle_settings,
            &creatures,
            &mut creature_instances,
            &[combat_action1.clone(), combat_action2_2.clone()],
        );
        assert_eq!(300 - creature_instances[0][0].current_health, 112); // Damage should be dealt from two 50 power moves
    }

    #[test]
    fn confusion_test() {
        let (
            mut creatures,
            mut creature_instances,
            mut battle_instance,
            battle_settings,
            combat_action1,
            combat_action2,
        ) = get_basic_setup();

        creatures[0][0].moves[0] = (&MoveID::ConfuseRay).into();
        for _ in 0..1000 {
            battle_instance.turn(
                &battle_settings,
                &creatures,
                &mut creature_instances,
                &[combat_action1.clone(), combat_action2.clone()],
            );
            assert_ne!(creature_instances[0][0].current_health, 300);

            creature_instances[0][0].current_health = 300;
            creature_instances[1][0].current_health = 300;
            creature_instances[0][0].status = None;
            assert!(creature_instances[1][0].status == None);
        }

        assert!(battle_instance.weather.weather_type == None);
    }

    // TODO test fire + water on weather
}
