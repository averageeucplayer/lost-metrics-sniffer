use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};

use super::types::*;

#[repr(C)]
#[derive(Debug, Default, Serialize, Deserialize, Encode, Decode, Clone)]
pub struct StatusEffectData {
    pub source_id: EntityId,
    pub status_effect_id: StatusEffectId,
    pub status_effect_instance_id: StatusEffectInstanceId,
    pub value: Option<Vec<u8>>,
    pub total_time: f32,
    pub stack_count: u8,
    pub end_tick: u64
}

#[repr(C)]
#[derive(Debug, Serialize, Deserialize, Default, Encode, Decode, Clone)]
pub struct SkillMoveOptionData {
    pub down_time: Option<f32>,
    pub stand_up_time: Option<f32>,
    pub move_time: Option<f32>,
}

#[repr(C)]
#[derive(Debug, Default, Serialize, Deserialize, Encode, Decode, Clone)]
pub struct StatPair {
    pub stat_type: u8,
    pub value: i64
}

#[repr(C)]
#[derive(Debug, Default, Serialize, Deserialize, Encode, Decode, Clone)]
pub struct EquipItemData {

}

#[repr(C)]
#[derive(Debug, Serialize, Deserialize, Default, Encode, Decode, Clone, Copy)]
pub struct TripodIndex {
    pub first: u8,
    pub second: u8,
    pub third: u8,
}

#[repr(C)]
#[derive(Debug, Serialize, Deserialize, Default, Encode, Decode, Clone, Copy)]
pub struct TripodLevel {
    pub first: u16,
    pub second: u16,
    pub third: u16,
}

#[repr(C)]
#[derive(Debug, Default, Serialize, Deserialize, Encode, Decode, Clone)]
pub struct SkillDamageEvent {
    pub target_id: EntityId,
    pub damage: i64,
    pub modifier: i32,
    pub cur_hp: i64,
    pub max_hp: i64,
    pub damage_attr: Option<u8>,
    pub damage_type: u8,
}

#[repr(C)]
#[derive(Debug, Serialize, Deserialize, Default, Encode, Decode, Clone)]
pub struct PartyMember {
    pub name: String,
    pub class_id: ClassId,
    pub character_id: CharacterId,
    pub gear_level: GearLevel,
}

#[repr(C)]
#[derive(Debug, Serialize, Deserialize, Default, Encode, Decode, Clone)]
pub struct SkillDamageAbnormalMoveDetails {
    pub event: SkillDamageEvent,
    pub skill_move_option_data: SkillMoveOptionData
}

#[repr(C)]
#[derive(Debug, Serialize, Deserialize, Default, Encode, Decode, Clone)]
pub struct SkillDamage {
    pub source_id: EntityId,
    pub skill_damage_events: Vec<SkillDamageEvent>,
    pub skill_id: SkillId,
    pub skill_effect_id: Option<SkillEffectId>,
}

