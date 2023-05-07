use super::*;
pub trait AI {
    fn get_action(
        &self,
        battle_instance: &BattleInstance,
        battle_settings: &BattleSettings,
        creatures: &[Vec<Creature>; 2],
        creature_instances: &[Vec<CreatureInstance>; 2],
        actioner: bool,
    ) -> CombatAction;
    fn get_forced_switch(&self, creature_instances: &Vec<CreatureInstance>) -> usize;
}

pub struct RandomAI {}

impl AI for RandomAI {
    fn get_action(
        &self,
        battle_instance: &BattleInstance,
        battle_settings: &BattleSettings,
        creatures: &[Vec<Creature>; 2],
        creature_instances: &[Vec<CreatureInstance>; 2],
        actioner: bool,
    ) -> CombatAction {
        CombatAction::Attack(rand::random::<u8>() % 4)
    }
    /// Assumption that if all fainted we don't force switch
    fn get_forced_switch(&self, creature_instances: &Vec<CreatureInstance>) -> usize {
        for switch_to in 0..creature_instances.len() {
            if !creature_instances[switch_to].is_fainted() {
                return switch_to;
            }
        }
        panic!("Assumption that if all fainted we don't force switch");
    }
}

pub struct StrongestAttackAI {}

impl AI for StrongestAttackAI {
    fn get_action(
        &self,
        battle_instance: &BattleInstance,
        battle_settings: &BattleSettings,
        creatures: &[Vec<Creature>; 2],
        creature_instances: &[Vec<CreatureInstance>; 2],
        actioner: bool,
    ) -> CombatAction {
        CombatAction::Attack(
            battle_instance
                .get_highest_damage_move(&battle_settings, &creatures, actioner)
                .0 as u8,
        )
    }
    /// Assumption that if all fainted we don't force switch
    fn get_forced_switch(&self, creature_instances: &Vec<CreatureInstance>) -> usize {
        for switch_to in 0..creature_instances.len() {
            if !creature_instances[switch_to].is_fainted() {
                return switch_to;
            }
        }
        panic!("Assumption that if all fainted we don't force switch");
    }
}

pub fn get_points(
    battle_instance: &BattleInstance,
    battle_settings: &BattleSettings,
    actioner: bool,
    creatures: &[Vec<Creature>; 2],
    creature_instances: &[Vec<CreatureInstance>; 2],
) -> f32 {
    let active_side_turns_to_ko = battle_instance.get_turns_to_ko_with_highest_damage_move(
        battle_settings,
        creatures,
        &creature_instances,
        actioner,
    );
    let passive_side_turns_to_ko = battle_instance.get_turns_to_ko_with_highest_damage_move(
        battle_settings,
        creatures,
        &creature_instances,
        !actioner,
    );
    passive_side_turns_to_ko - active_side_turns_to_ko
}
