use abi_stable::{std_types::*, StableAbi};
use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};

use super::types::*;

#[derive(Default, Debug, Serialize, Copy, Clone, PartialEq)]
#[repr(u32)]
pub enum Class {
    #[default]
    Unknown = 0,
    WarriorMale = 101,
    Berserker = 102,
    Destroyer = 103,
    Gunlancer = 104,
    Paladin = 105,
    WarriorFemale = 111,
    Slayer = 112,
    Mage = 201,
    Arcanist = 202,
    Summoner = 203,
    Bard = 204,
    Sorceress = 205,
    MartialArtistFemale = 301,
    Wardancer = 302,
    Scrapper = 303,
    Soulfist = 304,
    Glaivier = 305,
    MartialArtistMale = 311,
    Striker = 312,
    Breaker = 313,
    Assassin = 401,
    Deathblade = 402,
    Shadowhunter = 403,
    Reaper = 404,
    Souleater = 405,
    GunnerMale = 501,
    Sharpshooter = 502,
    Deadeye = 503,
    Artillerist = 504,
    Machinist = 505,
    GunnerFemale = 511,
    Gunslinger = 512,
    Specialist = 601,
    Artist = 602,
    Aeromancer = 603,
    Wildsoul = 604,
}

#[derive(StableAbi, Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
#[repr(i32)]
pub enum HitOption {
    #[default]
    None = 0,
    BackAttack = 1,
    FrontalAttack = 2,
    FlankAttack = 3,
    Max = 4,
}

#[derive(StableAbi, Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
#[repr(u32)]
pub enum HitFlag {
    #[default]
    Normal = 0,
    Critical = 1,
    Miss = 2,
    Invincible = 3,
    DamageOverTime = 4,
    Immune = 5,
    ImmuneSilenced = 6,
    FontSilence = 7,
    DamageOverTimeCritical = 8,
    Dodge = 9,
    Reflect = 10,
    DamageShare = 11,
    DodgeHit = 12,
    Max = 13,
}


#[repr(C)]
#[derive(StableAbi, Debug, Default, Serialize, Deserialize, Clone)]
pub struct StatusEffectData {
    pub source_id: EntityId,
    pub status_effect_id: StatusEffectId,
    pub status_effect_instance_id: StatusEffectInstanceId,
    pub value: RVec<u8>,
    pub total_time: f32,
    pub stack_count: u8,
    pub end_tick: u64
}

#[repr(C)]
#[derive(StableAbi, Debug, Serialize, Deserialize, Default, Clone)]
pub struct SkillMoveOptionData {
    pub down_time: ROption<f32>,
    pub stand_up_time: ROption<f32>,
    pub move_time: ROption<f32>,
}

#[repr(C)]
#[derive(StableAbi, Debug, Default, Serialize, Deserialize, Clone)]
pub struct StatPair {
    pub stat_type: u8,
    pub value: i64
}

#[repr(C)]
#[derive(StableAbi, Debug, Default, Serialize, Deserialize, Clone)]
pub struct EquipItemData {

}

#[repr(C)]
#[derive(StableAbi, Debug, Serialize, Deserialize, Default, Clone, Copy)]
pub struct TripodIndex {
    pub first: u8,
    pub second: u8,
    pub third: u8,
}

#[repr(C)]
#[derive(StableAbi, Debug, Serialize, Deserialize, Default, Clone, Copy)]
pub struct TripodLevel {
    pub first: u16,
    pub second: u16,
    pub third: u16,
}

#[repr(C)]
#[derive(StableAbi, Debug, Default, Serialize, Deserialize, Clone)]
pub struct SkillDamageEvent {
    pub target_id: EntityId,
    pub damage: i64,
    pub hit_option: HitOption,
    pub hit_flag: HitFlag,
    pub cur_hp: i64,
    pub max_hp: i64
}

#[repr(C)]
#[derive(StableAbi, Debug, Serialize, Deserialize, Default, Clone)]
pub struct PartyMember {
    pub name: RString,
    pub class_id: ClassId,
    pub character_id: CharacterId,
    pub gear_level: GearLevel,
}

#[repr(C)]
#[derive(StableAbi, Debug, Serialize, Deserialize, Default, Clone)]
pub struct SkillDamageAbnormalMoveDetails {
    pub event: SkillDamageEvent,
    pub skill_move_option_data: SkillMoveOptionData
}

#[repr(C)]
#[derive(StableAbi, Debug, Serialize, Deserialize, Default, Clone)]
pub struct SkillDamage {
    pub source_id: EntityId,
    pub skill_damage_events: RVec<SkillDamageEvent>,
    pub skill_id: SkillId,
    pub skill_effect_id: ROption<SkillEffectId>,
}

