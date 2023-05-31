use std::vec;

use turn_based_ai_research::ai::*;
use turn_based_ai_research::*;

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
fn strongest_attack_ai() {
    let mut creature_generator = CreatureGenerator::default();
    creature_generator.move_generation_settings.stats_mod_chance = 0;
    creature_generator.move_generation_settings.missable_ratio = 0;
    let mut times_strongest_ai_won_twice = 0;
    for i in 0..100000 {
        let (mut creature_instances, creatures, mut battle_instance, battle_settings) =
            setup(&creature_generator.clone(), 3);
        let strongest_attack_ai = StrongestAttackAI {};
        let random_ai = RandomAI {};
        let mut creature_instances_2 = creature_instances.clone();
        let mut battle_instance_2 = battle_instance.clone();

        let mut combat_action_1 = strongest_attack_ai.get_action(
            &battle_instance,
            &battle_settings,
            &creatures,
            &creature_instances,
            false,
        );
        let mut combat_action_2 = random_ai.get_action(
            &battle_instance,
            &battle_settings,
            &creatures,
            &creature_instances,
            true,
        );
        let mut strongest_ai_won = false;
        let mut combat_action_list = vec![];

        for i in 0..500 {
            combat_action_list.push(combat_action_1.clone());
            combat_action_list.push(combat_action_2.clone());
            let interrupt_opt = battle_instance.turn(
                &battle_settings,
                &creatures,
                &mut creature_instances,
                &[combat_action_1.clone(), combat_action_2.clone()],
            );
            match battle_instance.handle_interrupts(
                &battle_settings,
                &creatures,
                &mut creature_instances,
                interrupt_opt,
                &strongest_attack_ai,
                &random_ai,
            ) {
                Ok(_) => (),
                Err(interrupt) => match interrupt {
                    Interrupt::AWon => {
                        strongest_ai_won = true;
                        break;
                    }
                    Interrupt::BWon => {
                        strongest_ai_won = false;
                        break;
                    }
                    _ => panic!("faints should be handled already"),
                },
            }
            combat_action_1 = strongest_attack_ai.get_action(
                &battle_instance,
                &battle_settings,
                &creatures,
                &creature_instances,
                false,
            );
            combat_action_2 = random_ai.get_action(
                &battle_instance,
                &battle_settings,
                &creatures,
                &creature_instances,
                true,
            );
        }
        combat_action_1 = random_ai.get_action(
            &battle_instance_2,
            &battle_settings,
            &creatures,
            &creature_instances,
            false,
        );
        combat_action_2 = strongest_attack_ai.get_action(
            &battle_instance_2,
            &battle_settings,
            &creatures,
            &creature_instances,
            true,
        );
        combat_action_list.push(CombatAction::Switch(0));
        for i in 0..1000 {
            combat_action_list.push(combat_action_1.clone());
            combat_action_list.push(combat_action_2.clone());
            let interrupt_opt = battle_instance_2.turn(
                &battle_settings,
                &creatures,
                &mut creature_instances_2,
                &[combat_action_1.clone(), combat_action_2.clone()],
            );
            match battle_instance_2.handle_interrupts(
                &battle_settings,
                &creatures,
                &mut creature_instances_2,
                interrupt_opt,
                &random_ai,
                &strongest_attack_ai,
            ) {
                Ok(_) => (),
                Err(interrupt) => match interrupt {
                    Interrupt::AWon => {
                        assert!(strongest_ai_won);
                        break;
                    }
                    Interrupt::BWon => {
                        if strongest_ai_won {
                            times_strongest_ai_won_twice += 1
                        }
                        break;
                    }
                    _ => panic!("faints should be handled already"),
                },
            }
            if i == 999 {
                panic!("should not be 999 when there are only attacks");
            }
            combat_action_1 = random_ai.get_action(
                &battle_instance_2,
                &battle_settings,
                &creatures,
                &creature_instances,
                false,
            );
            combat_action_2 = strongest_attack_ai.get_action(
                &battle_instance_2,
                &battle_settings,
                &creatures,
                &creature_instances,
                true,
            );
        }
    }
    println!(
        "Times strongest AI won both sides against random AI: {}",
        times_strongest_ai_won_twice
    )
}
#[test]
fn strongest_attack_ai_with_forced_switch() {
    let mut creature_generator = CreatureGenerator::default();
    creature_generator.move_generation_settings.stats_mod_chance = 0;
    creature_generator.move_generation_settings.missable_ratio = 0;
    let mut times_better_switch_won_twice = 0;
    let mut times_worse_switch_won_twice = 0;
    for i in 0..6000 {
        let (mut creature_instances, creatures, mut battle_instance, battle_settings) =
            setup(&creature_generator.clone(), 6);
        let strongest_attack_ai_forced_switch = StrongestAttackAIWithBetterSwitching {};
        let stronged_attack_ai_bad_switch = StrongestAttackAI {};
        let mut creature_instances_2 = creature_instances.clone();
        let mut battle_instance_2 = battle_instance.clone();

        let mut combat_action_1 = strongest_attack_ai_forced_switch.get_action(
            &battle_instance,
            &battle_settings,
            &creatures,
            &creature_instances,
            false,
        );
        let mut combat_action_2 = stronged_attack_ai_bad_switch.get_action(
            &battle_instance,
            &battle_settings,
            &creatures,
            &creature_instances,
            true,
        );
        let mut better_switch_ai_won = false;
        let mut combat_action_list = vec![];

        for i in 0..500 {
            combat_action_list.push(combat_action_1.clone());
            combat_action_list.push(combat_action_2.clone());
            let interrupt_opt = battle_instance.turn(
                &battle_settings,
                &creatures,
                &mut creature_instances,
                &[combat_action_1.clone(), combat_action_2.clone()],
            );
            match battle_instance.handle_interrupts(
                &battle_settings,
                &creatures,
                &mut creature_instances,
                interrupt_opt,
                &strongest_attack_ai_forced_switch,
                &stronged_attack_ai_bad_switch,
            ) {
                Ok(_) => (),
                Err(interrupt) => match interrupt {
                    Interrupt::AWon => {
                        better_switch_ai_won = true;
                        break;
                    }
                    Interrupt::BWon => {
                        better_switch_ai_won = false;
                        break;
                    }
                    _ => panic!("faints should be handled already"),
                },
            }
            combat_action_1 = strongest_attack_ai_forced_switch.get_action(
                &battle_instance,
                &battle_settings,
                &creatures,
                &creature_instances,
                false,
            );
            combat_action_2 = stronged_attack_ai_bad_switch.get_action(
                &battle_instance,
                &battle_settings,
                &creatures,
                &creature_instances,
                true,
            );
        }
        combat_action_1 = stronged_attack_ai_bad_switch.get_action(
            &battle_instance_2,
            &battle_settings,
            &creatures,
            &creature_instances,
            false,
        );
        combat_action_2 = strongest_attack_ai_forced_switch.get_action(
            &battle_instance_2,
            &battle_settings,
            &creatures,
            &creature_instances,
            true,
        );
        combat_action_list.push(CombatAction::Switch(0));
        for i in 0..1000 {
            combat_action_list.push(combat_action_1.clone());
            combat_action_list.push(combat_action_2.clone());
            let interrupt_opt = battle_instance_2.turn(
                &battle_settings,
                &creatures,
                &mut creature_instances_2,
                &[combat_action_1.clone(), combat_action_2.clone()],
            );
            match battle_instance_2.handle_interrupts(
                &battle_settings,
                &creatures,
                &mut creature_instances_2,
                interrupt_opt,
                &stronged_attack_ai_bad_switch,
                &strongest_attack_ai_forced_switch,
            ) {
                Ok(_) => (),
                Err(interrupt) => match interrupt {
                    Interrupt::AWon => {
                        if !better_switch_ai_won {
                            times_worse_switch_won_twice += 1
                        }
                        break;
                    }
                    Interrupt::BWon => {
                        if better_switch_ai_won {
                            times_better_switch_won_twice += 1
                        }
                        break;
                    }
                    _ => panic!("faints should be handled already"),
                },
            }
            if i == 999 {
                panic!("should not be 999 when there are only attacks");
            }
            combat_action_1 = stronged_attack_ai_bad_switch.get_action(
                &battle_instance_2,
                &battle_settings,
                &creatures,
                &creature_instances,
                false,
            );
            combat_action_2 = strongest_attack_ai_forced_switch.get_action(
                &battle_instance_2,
                &battle_settings,
                &creatures,
                &creature_instances,
                true,
            );
        }
    }
    println!(
        "Times better forced switch AI won both sides against random AI: {},Times worse forced switch AI won both sides against random AI: {}",
        times_better_switch_won_twice,
        times_worse_switch_won_twice
    );
    assert!(times_better_switch_won_twice * 4 > times_worse_switch_won_twice);
}

#[test]
fn strongest_attack_ai_with_minimax_forced_switch() {
    let mut creature_generator = CreatureGenerator::default();
    creature_generator.move_generation_settings.stats_mod_chance = 0;
    creature_generator.move_generation_settings.missable_ratio = 0;
    let mut times_better_switch_won_twice = 0;
    let mut times_worse_switch_won_twice = 0;
    for i in 0..1000 {
        let (mut creature_instances, creatures, mut battle_instance, battle_settings) =
            setup(&creature_generator.clone(), 4);
        let strong_attack_ai_forced_switch = StrongestAttackAIWithBettererSwitching {};
        let strong_attack_ai = StrongestAttackAI {};
        let mut creature_instances_2 = creature_instances.clone();
        let mut battle_instance_2 = battle_instance.clone();

        let mut combat_action_1 = strong_attack_ai_forced_switch.get_action(
            &battle_instance,
            &battle_settings,
            &creatures,
            &creature_instances,
            false,
        );
        let mut combat_action_2 = strong_attack_ai.get_action(
            &battle_instance,
            &battle_settings,
            &creatures,
            &creature_instances,
            true,
        );
        let mut better_switch_ai_won = false;
        let mut combat_action_list = vec![];

        for i in 0..500 {
            combat_action_list.push(combat_action_1.clone());
            combat_action_list.push(combat_action_2.clone());
            let interrupt_opt = battle_instance.turn(
                &battle_settings,
                &creatures,
                &mut creature_instances,
                &[combat_action_1.clone(), combat_action_2.clone()],
            );
            match battle_instance.handle_interrupts(
                &battle_settings,
                &creatures,
                &mut creature_instances,
                interrupt_opt,
                &strong_attack_ai_forced_switch,
                &strong_attack_ai,
            ) {
                Ok(_) => (),
                Err(interrupt) => match interrupt {
                    Interrupt::AWon => {
                        better_switch_ai_won = true;
                        break;
                    }
                    Interrupt::BWon => {
                        better_switch_ai_won = false;
                        break;
                    }
                    _ => panic!("faints should be handled already"),
                },
            }
            combat_action_1 = strong_attack_ai_forced_switch.get_action(
                &battle_instance,
                &battle_settings,
                &creatures,
                &creature_instances,
                false,
            );
            combat_action_2 = strong_attack_ai.get_action(
                &battle_instance,
                &battle_settings,
                &creatures,
                &creature_instances,
                true,
            );
        }
        combat_action_1 = strong_attack_ai.get_action(
            &battle_instance_2,
            &battle_settings,
            &creatures,
            &creature_instances,
            false,
        );
        combat_action_2 = strong_attack_ai_forced_switch.get_action(
            &battle_instance_2,
            &battle_settings,
            &creatures,
            &creature_instances,
            true,
        );
        combat_action_list.push(CombatAction::Switch(0));
        for i in 0..1000 {
            combat_action_list.push(combat_action_1.clone());
            combat_action_list.push(combat_action_2.clone());
            let interrupt_opt = battle_instance_2.turn(
                &battle_settings,
                &creatures,
                &mut creature_instances_2,
                &[combat_action_1.clone(), combat_action_2.clone()],
            );
            match battle_instance_2.handle_interrupts(
                &battle_settings,
                &creatures,
                &mut creature_instances_2,
                interrupt_opt,
                &strong_attack_ai,
                &strong_attack_ai_forced_switch,
            ) {
                Ok(_) => (),
                Err(interrupt) => match interrupt {
                    Interrupt::AWon => {
                        if !better_switch_ai_won {
                            println!("{:?}", combat_action_list);
                            println!("{:?}", creatures);
                            times_worse_switch_won_twice += 1;
                        }
                        break;
                    }
                    Interrupt::BWon => {
                        if better_switch_ai_won {
                            times_better_switch_won_twice += 1
                        }
                        break;
                    }
                    _ => panic!("faints should be handled already"),
                },
            }
            if i == 999 {
                panic!("should not be 999 when there are only attacks");
            }
            combat_action_1 = strong_attack_ai.get_action(
                &battle_instance_2,
                &battle_settings,
                &creatures,
                &creature_instances,
                false,
            );
            combat_action_2 = strong_attack_ai_forced_switch.get_action(
                &battle_instance_2,
                &battle_settings,
                &creatures,
                &creature_instances,
                true,
            );
        }
    }
    println!(
        "Times better forced switch won both sides against random forced switch: {},Times random forced switch won both sides against better forced switch: {}",
        times_better_switch_won_twice,
        times_worse_switch_won_twice
    );
    assert_eq!(times_worse_switch_won_twice, 0);
}
#[test]
fn min_max_ai() {
    let mut creature_generator = CreatureGenerator::default();
    creature_generator.move_generation_settings.stats_mod_chance = 0;
    creature_generator.move_generation_settings.missable_ratio = 0;
    let mut times_min_max_ai_won_twice = 0;
    for i in 0..10000 {
        let (mut creature_instances, creatures, mut battle_instance, battle_settings) =
            setup(&creature_generator.clone(), 2);
        let min_max_ai = MinMaxMovesAI { depth: 2 };
        let strongest_ai = StrongestAttackAI {};
        let mut creature_instances_2 = creature_instances.clone();
        let mut battle_instance_2 = battle_instance.clone();

        let mut combat_action_1 = min_max_ai.get_action(
            &battle_instance,
            &battle_settings,
            &creatures,
            &creature_instances,
            false,
        );
        let mut combat_action_2 = strongest_ai.get_action(
            &battle_instance,
            &battle_settings,
            &creatures,
            &creature_instances,
            true,
        );
        let mut min_max_ai_won = false;
        let mut combat_action_list = vec![];

        for i in 0..500 {
            combat_action_list.push(combat_action_1.clone());
            combat_action_list.push(combat_action_2.clone());
            let interrupt_opt = battle_instance.turn(
                &battle_settings,
                &creatures,
                &mut creature_instances,
                &[combat_action_1.clone(), combat_action_2.clone()],
            );
            match battle_instance.handle_interrupts(
                &battle_settings,
                &creatures,
                &mut creature_instances,
                interrupt_opt,
                &min_max_ai,
                &strongest_ai,
            ) {
                Ok(_) => (),
                Err(interrupt) => match interrupt {
                    Interrupt::AWon => {
                        min_max_ai_won = true;
                        break;
                    }
                    Interrupt::BWon => {
                        min_max_ai_won = false;
                        break;
                    }
                    _ => panic!("faints should be handled already"),
                },
            }
            combat_action_1 = min_max_ai.get_action(
                &battle_instance,
                &battle_settings,
                &creatures,
                &creature_instances,
                false,
            );
            combat_action_2 = strongest_ai.get_action(
                &battle_instance,
                &battle_settings,
                &creatures,
                &creature_instances,
                true,
            );
        }
        combat_action_list.push(CombatAction::Switch(0));
        combat_action_1 = strongest_ai.get_action(
            &battle_instance_2,
            &battle_settings,
            &creatures,
            &creature_instances_2,
            false,
        );
        combat_action_2 = min_max_ai.get_action(
            &battle_instance_2,
            &battle_settings,
            &creatures,
            &creature_instances_2,
            true,
        );
        for i in 0..1000 {
            combat_action_list.push(combat_action_1.clone());
            combat_action_list.push(combat_action_2.clone());
            let interrupt_opt = battle_instance_2.turn(
                &battle_settings,
                &creatures,
                &mut creature_instances_2,
                &[combat_action_1.clone(), combat_action_2.clone()],
            );
            match battle_instance_2.handle_interrupts(
                &battle_settings,
                &creatures,
                &mut creature_instances_2,
                interrupt_opt,
                &strongest_ai,
                &min_max_ai,
            ) {
                Ok(_) => (),
                Err(interrupt) => match interrupt {
                    Interrupt::AWon => {
                        if !min_max_ai_won {
                            println!("{:?}", combat_action_list);

                            println!("This is bad");
                            panic!("min max should always win strongest move");
                        }
                        break;
                    }
                    Interrupt::BWon => {
                        if min_max_ai_won {
                            times_min_max_ai_won_twice += 1
                        }
                        break;
                    }
                    _ => panic!("faints should be handled already"),
                },
            }
            combat_action_1 = strongest_ai.get_action(
                &battle_instance_2,
                &battle_settings,
                &creatures,
                &creature_instances_2,
                false,
            );
            combat_action_2 = min_max_ai.get_action(
                &battle_instance_2,
                &battle_settings,
                &creatures,
                &creature_instances_2,
                true,
            );
        }
    }
    println!(
        "Times min max AI won both sides against strongest AI: {}",
        times_min_max_ai_won_twice
    )
}

#[test]
fn team_generation_test() {
    let mut creature_generator = CreatureGenerator::default();
    creature_generator.move_generation_settings.stats_mod_chance = 0;
    creature_generator.move_generation_settings.missable_ratio = 0;
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
            setup(&creature_generator, 3);
        let random_ai_a = RandomAI {};
        let random_ai_b = RandomAI {};
        let mut combat_action_1 = random_ai_a.get_action(
            &battle_instance,
            &battle_settings,
            &creatures,
            &creature_instances,
            false,
        );
        let mut combat_action_2 = random_ai_b.get_action(
            &battle_instance,
            &battle_settings,
            &creatures,
            &creature_instances,
            true,
        );
        let mut combat_action_list = vec![];
        for i in 0..1000 {
            combat_action_list.push(combat_action_1.clone());
            combat_action_list.push(combat_action_2.clone());
            if i == 100 {
                println!("{:?}", combat_action_list);
                panic!("Not should happen")
            }
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
                &battle_settings,
                &creatures,
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
                                        won_with_specific_base_move[a_move.id.get_as_index()] += 1;
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
                                        lost_with_specific_base_move[a_move.id.get_as_index()] += 1;
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
                                        won_with_specific_base_move[a_move.id.get_as_index()] += 1;
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
                                        lost_with_specific_base_move[a_move.id.get_as_index()] += 1;
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
            combat_action_1 = random_ai_a.get_action(
                &battle_instance,
                &battle_settings,
                &creatures,
                &creature_instances,
                false,
            );
            combat_action_2 = random_ai_b.get_action(
                &battle_instance,
                &battle_settings,
                &creatures,
                &creature_instances,
                true,
            );
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
