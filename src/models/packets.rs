use abi_stable::{std_types::{ROption, RString, RVec}, StableAbi};
use serde::{Deserialize, Serialize};
use super::structures::*;
use super::types::*;

#[repr(u32)]
#[derive(Debug, Serialize, Deserialize, StableAbi, Clone)]
pub enum Packet {
    CounterAttack {
        source_id: EntityId
    },
    Death {
        target_id: EntityId
    },
    IdentityGaugeChange {
        player_id: EntityId,
        identity_gauge1: u32,
        identity_gauge2: u32,
        identity_gauge3: u32
    },
    InitEnv {
        player_id: EntityId
    },
    InitPC {
        player_id: EntityId,
        name: RString,
        character_id: CharacterId,
        class_id: ClassId,
        gear_level: GearLevel,
        stat_pairs: RVec<StatPair>,
        status_effect_datas: RVec<StatusEffectData>,
    }, 
    NewPC {
        player_id: EntityId,
        name: RString,
        class_id: ClassId,
        max_item_level: GearLevel,
        character_id: CharacterId,
        stat_pairs: RVec<StatPair>,
        equip_item_datas: RVec<EquipItemData>,
        status_effect_datas: RVec<StatusEffectData>
    },
    NewNpc {
        object_id: EntityId,
        type_id: NpcId,
        level: u16,
        balance_level: ROption<u16>,
        stat_pairs: RVec<StatPair>,
        status_effect_datas: RVec<StatusEffectData>
    },
    NewNpcSummon {
        owner_id: EntityId,
        object_id: EntityId,
        type_id: NpcId,
        level: u16,
        balance_level: ROption<u16>,
        stat_pairs: RVec<StatPair>,
        status_effect_datas: RVec<StatusEffectData>
    },
    NewProjectile {
        projectile_id: EntityId,
        owner_id: EntityId,
        skill_id: SkillId,
        skill_effect: SkillEffectId,
    },
    NewTrap {
        object_id: EntityId,
        owner_id: EntityId,
        skill_id: SkillId,
        skill_effect: SkillEffectId
    },
    RaidBegin {
        raid_id: u32,
    },
    RaidBossKill,
    RaidResult,
    RemoveObject {
        unpublished_objects: RVec<EntityId>
    },
    SkillCast {
        source_id: EntityId,
        skill_id: SkillId,
    },
    SkillStart {
        source_id: EntityId,
        skill_id: SkillId,
        tripod_index: ROption<TripodIndex>,
        tripod_level: ROption<TripodLevel>,
    },
    SkillDamageAbnormalMove {
        source_id: EntityId,
        events: RVec<SkillDamageAbnormalMoveDetails>,
        skill_id: SkillId,
        skill_effect_id: SkillEffectId,
    },
    SkillDamage {
        event: SkillDamageEvent,
        skill_move_option_data: SkillMoveOptionData
    },
    PartyInfo {
        party_instance_id: PartyInstanceId,
        raid_instance_id: RaidInstanceId,
        members: RVec<PartyMember>
    },
    PartyLeaveResult {
        party_instance_id: PartyInstanceId,
        name: RString
    },
    PartyStatusEffectAdd {
        character_id: CharacterId,
        status_effect_datas: RVec<StatusEffectData>
    },
    PartyStatusEffectRemove {
        character_id: CharacterId,
        status_effect_instance_ids: RVec<StatusEffectInstanceId>,
        reason: u8
    },
    PartyStatusEffectResult {
        raid_instance_id: RaidInstanceId,
        party_instance_id: PartyInstanceId,
        character_id: CharacterId
    },
    StatusEffectAdd {
        object_id: EntityId,
        status_effect_data: StatusEffectData
    },
    StatusEffectRemove {
        object_id: EntityId,
        character_id: CharacterId,
        status_effect_instance_ids: RVec<StatusEffectInstanceId>,
        reason: u8
    },
    TriggerBossBattleStatus,
    TriggerStart {
        signal: u32,
    },
    ZoneMemberLoadStatus {
        zone_id: u32,
        zone_level: u32
    },
    ZoneObjectUnpublish {
        object_id: u64
    },
    StatusEffectSyncData {
        object_id: EntityId,
        status_effect_instance_id: StatusEffectInstanceId,
        character_id: CharacterId,
        value: u64,
    },
    TroopMemberUpdateMin {
        character_id: u64,
        cur_hp: i64,
        max_hp: i64,
        status_effect_datas: RVec<StatusEffectData>,
    },
    NewTransit {
        channel_id: u32
    }
}