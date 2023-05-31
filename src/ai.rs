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
    fn get_forced_switch(
        &self,
        battle_instance: &BattleInstance,
        battle_settings: &BattleSettings,
        creatures: &[Vec<Creature>; 2],
        creature_instances: &[Vec<CreatureInstance>; 2],
        actioner: bool,
    ) -> usize;
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
    fn get_forced_switch(
        &self,
        battle_instance: &BattleInstance,
        battle_settings: &BattleSettings,
        creatures: &[Vec<Creature>; 2],
        creature_instances: &[Vec<CreatureInstance>; 2],
        side: bool,
    ) -> usize {
        for switch_to in 0..creature_instances[side as usize].len() {
            if !creature_instances[side as usize][switch_to].is_fainted() {
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
    fn get_forced_switch(
        &self,
        battle_instance: &BattleInstance,
        battle_settings: &BattleSettings,
        creatures: &[Vec<Creature>; 2],
        creature_instances: &[Vec<CreatureInstance>; 2],
        side: bool,
    ) -> usize {
        for switch_to in 0..creature_instances[side as usize].len() {
            if !creature_instances[side as usize][switch_to].is_fainted() {
                return switch_to;
            }
        }
        panic!("Assumption that if all fainted we don't force switch");
    }
}

pub struct StrongestAttackAIWithBetterSwitching {}

impl AI for StrongestAttackAIWithBetterSwitching {
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
    fn get_forced_switch(
        &self,
        battle_instance: &BattleInstance,
        battle_settings: &BattleSettings,
        creatures: &[Vec<Creature>; 2],
        creature_instances: &[Vec<CreatureInstance>; 2],
        side: bool,
    ) -> usize {
        let health_percentages =
            battle_instance.get_health_percentages(creature_instances, creatures);
        let matchup_matrix = &battle_instance
            .get_matchup_matrix_with_highest_damage_move(battle_settings, creatures);
        battle_instance
            .get_strongest_forced_switch(
                matchup_matrix,
                &health_percentages,
                side,
                battle_instance.battler_ids[!side as usize],
            )
            .expect("that if all fainted we don't force switch")
            .1
    }
}

pub struct StrongestAttackAIWithBettererSwitching {}

impl AI for StrongestAttackAIWithBettererSwitching {
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
    fn get_forced_switch(
        &self,
        battle_instance: &BattleInstance,
        battle_settings: &BattleSettings,
        creatures: &[Vec<Creature>; 2],
        creature_instances: &[Vec<CreatureInstance>; 2],
        side: bool,
    ) -> usize {
        let health_percentages =
            battle_instance.get_health_percentages(creature_instances, creatures);
        let matchup_matrix = &battle_instance
            .get_matchup_matrix_with_highest_damage_move(battle_settings, creatures);
        battle_instance.get_forced_switch_with_min_maxing_setup(
            matchup_matrix,
            &health_percentages,
            side,
        )
    }
}
pub struct MinMaxMovesAI {
    pub depth: u8,
}

impl AI for MinMaxMovesAI {
    fn get_action(
        &self,
        battle_instance: &BattleInstance,
        battle_settings: &BattleSettings,
        creatures: &[Vec<Creature>; 2],
        creature_instances: &[Vec<CreatureInstance>; 2],
        actioner: bool,
    ) -> CombatAction {
        CombatAction::Attack(
            (min_max(
                battle_instance,
                battle_settings,
                creature_instances,
                creatures,
                actioner,
                0,
                self.depth,
            )
            .1 / 4) as u8,
        )
    }
    /// Assumption that if all fainted we don't force switch
    fn get_forced_switch(
        &self,
        battle_instance: &BattleInstance,
        battle_settings: &BattleSettings,
        creatures: &[Vec<Creature>; 2],
        creature_instances: &[Vec<CreatureInstance>; 2],
        side: bool,
    ) -> usize {
        for switch_to in 0..creature_instances[side as usize].len() {
            if !creature_instances[side as usize][switch_to].is_fainted() {
                return switch_to;
            }
        }
        panic!("Assumption that if all fainted we don't force switch");
    }
}
fn min_max(
    battle_instance: &BattleInstance,
    battle_settings: &BattleSettings,
    creature_instances: &[Vec<CreatureInstance>; 2],
    creatures: &[Vec<Creature>; 2],
    actioner: bool,
    depth: u8,
    max_depth: u8,
) -> (f32, usize) {
    let ai_for_forced_switch = RandomAI {};
    let mut points = vec![];
    for active_move_id in 0..4 {
        for passive_move_id in 0..4 {
            let mut battle_instance_cloned = battle_instance.clone();
            let mut creature_instances_cloned = creature_instances.clone();
            let combat_action_active = CombatAction::Attack(active_move_id);
            let combat_action_passive = CombatAction::Attack(passive_move_id);
            let combat_actions = if actioner {
                [combat_action_passive, combat_action_active]
            } else {
                [combat_action_active, combat_action_passive]
            };
            let interrupt_opt = battle_instance_cloned.turn(
                &battle_settings,
                &creatures,
                &mut creature_instances_cloned,
                &combat_actions,
            );
            match battle_instance_cloned.handle_interrupts(
                &battle_settings,
                creatures,
                &mut creature_instances_cloned,
                interrupt_opt,
                &ai_for_forced_switch,
                &ai_for_forced_switch,
            ) {
                Ok(_) => (),
                Err(interrupt) => match interrupt {
                    Interrupt::AWon => {
                        points.push(get_points(
                            &battle_instance_cloned,
                            battle_settings,
                            actioner,
                            creatures,
                            &creature_instances_cloned,
                        ));
                        continue;
                    }
                    Interrupt::BWon => {
                        points.push(get_points(
                            &battle_instance_cloned,
                            battle_settings,
                            actioner,
                            creatures,
                            &creature_instances_cloned,
                        ));
                        continue;
                    }
                    _ => panic!("faints should be handled already"),
                },
            }
            if depth == max_depth {
                points.push(
                    min_max(
                        &battle_instance_cloned,
                        battle_settings,
                        &creature_instances_cloned,
                        creatures,
                        actioner,
                        depth + 1,
                        max_depth,
                    )
                    .0,
                );
            } else {
                points.push(get_points(
                    &battle_instance_cloned,
                    battle_settings,
                    actioner,
                    creatures,
                    &creature_instances_cloned,
                ));
            }
        }
    }
    let mut best_points_for_active = None;
    let mut active = 0;
    for player_move_id in 0..4 {
        let mut best_points_for_passive = points[player_move_id * 4];
        let mut passive_id = 0;
        for enemy_move_id in 1..4 {
            if points[(player_move_id * 4) + enemy_move_id] < best_points_for_passive {
                best_points_for_passive = points[(player_move_id * 4) + enemy_move_id];
                passive_id = enemy_move_id;
            }
        }
        if best_points_for_active.is_none()
            || best_points_for_passive > best_points_for_active.unwrap()
        {
            active = (player_move_id * 4) + passive_id;
            best_points_for_active = Some(best_points_for_passive);
        }
    }
    return (best_points_for_active.unwrap(), active);
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
