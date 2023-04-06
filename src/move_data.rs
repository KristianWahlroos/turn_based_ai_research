use crate::{Effect, Move, MoveUnit, Type, VolatileStatus};
#[derive(Debug, Clone, PartialEq)]
pub enum MoveID {
    DamageLow(bool, Type),
    DamageMed(bool, Type),
    DamageHigh(bool, Type),
    MissLow(bool, Type),
    MissMed(bool, Type),
    MissHigh(bool, Type),
    StatsUp(VolatileStatus),
    StatsUpDouble(VolatileStatus),
    StatsDown(VolatileStatus),
    StatsDownDouble(VolatileStatus),
}

impl MoveID {
    pub fn get_as_index(&self) -> usize {
        match self {
            MoveID::DamageLow(_, _) => 0,
            MoveID::DamageMed(_, _) => 1,
            MoveID::DamageHigh(_, _) => 2,
            MoveID::MissLow(_, _) => 3,
            MoveID::MissMed(_, _) => 4,
            MoveID::MissHigh(_, _) => 5,
            MoveID::StatsUp(_) => 6,
            MoveID::StatsUpDouble(_) => 7,
            MoveID::StatsDown(_) => 8,
            MoveID::StatsDownDouble(_) => 9,
        }
    }

    pub fn get_volatile_status(&self) -> Option<VolatileStatus> {
        match self {
            MoveID::StatsUp(volatile_status)
            | MoveID::StatsUpDouble(volatile_status)
            | MoveID::StatsDown(volatile_status)
            | MoveID::StatsDownDouble(volatile_status) => Some(volatile_status.clone()),
            _ => None,
        }
    }
}

impl From<&MoveID> for Move {
    fn from(item: &MoveID) -> Self {
        match item {
            MoveID::StatsUp(stat) => Move {
                id: MoveID::StatsUp(stat.clone()),
                move_type: Type::Normal,
                chance_of_success: None,
                pp: 30,
                priority: 0,
                units: vec![MoveUnit {
                    chance_of_success: 1.0,
                    power: Some(1),
                    effect: Effect::ValueVolatileStatusChange(stat.clone()),
                    needs_target: false,
                    target_self: true,
                    continues_previous_unit: false,
                }],
            },
            MoveID::StatsUpDouble(stat) => Move {
                id: MoveID::StatsUpDouble(stat.clone()),
                move_type: Type::Normal,
                chance_of_success: None,
                pp: 30,
                priority: 0,
                units: vec![MoveUnit {
                    chance_of_success: 1.0,
                    power: Some(2),
                    effect: Effect::ValueVolatileStatusChange(stat.clone()),
                    needs_target: false,
                    target_self: true,
                    continues_previous_unit: false,
                }],
            },
            MoveID::StatsDown(stat) => Move {
                id: MoveID::StatsDown(stat.clone()),
                move_type: Type::Normal,
                chance_of_success: None,
                pp: 30,
                priority: 0,
                units: vec![MoveUnit {
                    chance_of_success: 1.0,
                    power: Some(-1),
                    effect: Effect::ValueVolatileStatusChange(stat.clone()),
                    needs_target: true,
                    target_self: false,
                    continues_previous_unit: false,
                }],
            },
            MoveID::StatsDownDouble(stat) => Move {
                id: MoveID::StatsDownDouble(stat.clone()),
                move_type: Type::Normal,
                chance_of_success: None,
                pp: 30,
                priority: 0,
                units: vec![MoveUnit {
                    chance_of_success: 1.0,
                    power: Some(-2),
                    effect: Effect::ValueVolatileStatusChange(stat.clone()),
                    needs_target: true,
                    target_self: false,
                    continues_previous_unit: false,
                }],
            },
            MoveID::DamageLow(physical, move_type) => Move {
                id: MoveID::DamageLow(*physical, *move_type),
                move_type: *move_type,
                chance_of_success: Some(1.0),
                pp: 35,
                priority: 0,
                units: vec![MoveUnit {
                    chance_of_success: 1.0,
                    power: Some(70),
                    effect: if *physical {
                        Effect::PhysicalAttack
                    } else {
                        Effect::SpecialAttack
                    },
                    needs_target: true,
                    target_self: false,
                    continues_previous_unit: true,
                }],
            },
            MoveID::DamageMed(physical, move_type) => Move {
                id: MoveID::DamageMed(*physical, *move_type),
                move_type: *move_type,
                chance_of_success: Some(1.0),
                pp: 35,
                priority: 0,
                units: vec![MoveUnit {
                    chance_of_success: 1.0,
                    power: Some(80),
                    effect: if *physical {
                        Effect::PhysicalAttack
                    } else {
                        Effect::SpecialAttack
                    },
                    needs_target: true,
                    target_self: false,
                    continues_previous_unit: true,
                }],
            },
            MoveID::DamageHigh(physical, move_type) => Move {
                id: MoveID::DamageHigh(*physical, *move_type),
                move_type: *move_type,
                chance_of_success: Some(1.0),
                pp: 35,
                priority: 0,
                units: vec![MoveUnit {
                    chance_of_success: 1.0,
                    power: Some(90),
                    effect: if *physical {
                        Effect::PhysicalAttack
                    } else {
                        Effect::SpecialAttack
                    },
                    needs_target: true,
                    target_self: false,
                    continues_previous_unit: true,
                }],
            },
            MoveID::MissLow(physical, move_type) => Move {
                id: MoveID::MissLow(*physical, *move_type),
                move_type: *move_type,
                chance_of_success: Some(0.9),
                pp: 35,
                priority: 0,
                units: vec![MoveUnit {
                    chance_of_success: 1.0,
                    power: Some(100),
                    effect: if *physical {
                        Effect::PhysicalAttack
                    } else {
                        Effect::SpecialAttack
                    },
                    needs_target: true,
                    target_self: false,
                    continues_previous_unit: true,
                }],
            },
            MoveID::MissMed(physical, move_type) => Move {
                id: MoveID::MissMed(*physical, *move_type),
                move_type: *move_type,
                chance_of_success: Some(0.8),
                pp: 35,
                priority: 0,
                units: vec![MoveUnit {
                    chance_of_success: 1.0,
                    power: Some(100),
                    effect: if *physical {
                        Effect::PhysicalAttack
                    } else {
                        Effect::SpecialAttack
                    },
                    needs_target: true,
                    target_self: false,
                    continues_previous_unit: true,
                }],
            },
            MoveID::MissHigh(physical, move_type) => Move {
                id: MoveID::MissHigh(*physical, *move_type),
                move_type: *move_type,
                chance_of_success: Some(0.7),
                pp: 35,
                priority: 0,
                units: vec![MoveUnit {
                    chance_of_success: 1.0,
                    power: Some(100),
                    effect: if *physical {
                        Effect::PhysicalAttack
                    } else {
                        Effect::SpecialAttack
                    },
                    needs_target: true,
                    target_self: false,
                    continues_previous_unit: true,
                }],
            },
        }
    }
}
