use super::*;
use rand::rngs::ThreadRng;
use rand::Rng;
#[derive(Clone)]
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
#[derive(Clone)]
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

#[derive(Clone)]
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
#[derive(Clone)]
pub struct MoveGenerationSettings {
    pub low_attack_ratio: i32,
    pub med_attack_ratio: i32,
    pub high_attack_ratio: i32,
    pub attack_ratio: i32, // attack ratio should be low as there is already two attacks that are not rolled
    pub stats_mod_chance: i32,
    pub one_step_stat_ratio: i32,
    pub two_step_stat_ratio: i32,
    pub buff_ratio: i32,
    pub debuff_ratio: i32,
    pub always_hit_ratio: i32,
    pub missable_ratio: i32,
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
