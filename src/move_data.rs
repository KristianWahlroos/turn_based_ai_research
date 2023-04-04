use crate::{Effect, Move, MoveUnit, Type, VolatileStatus};
#[derive(Debug, Clone, PartialEq)]
pub enum MoveID {
    DamageLow,
    DamageMed,
    DamageHigh,
    MissLow,
    MissMed,
    MissHigh,
    StatsUp(VolatileStatus),
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
            _ => Move {
                id: MoveID::DamageLow,
                move_type: Type::Normal,
                chance_of_success: Some(1.0),
                pp: 35,
                priority: 0,
                units: vec![MoveUnit {
                    chance_of_success: 1.0,
                    power: Some(40),
                    effect: Effect::PhysicalAttack,
                    needs_target: true,
                    target_self: false,
                    continues_previous_unit: true,
                }],
            },
        }
    }
}
