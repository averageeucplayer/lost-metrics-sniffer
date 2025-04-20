use abi_stable::StableAbi;
use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};
use super::structures::*;
use super::types::*;

#[derive(Serialize, Encode, Decode, Debug, Clone)]
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
        name: String,
        character_id: CharacterId,
        class_id: ClassId,
        gear_level: GearLevel,
        stat_pairs: Vec<StatPair>,
        status_effect_datas: Vec<StatusEffectData>,
    }, 
    NewPC {
        player_id: EntityId,
        name: String,
        class_id: ClassId,
        max_item_level: GearLevel,
        character_id: CharacterId,
        stat_pairs: Vec<StatPair>,
        equip_item_datas: Vec<EquipItemData>,
        status_effect_datas: Vec<StatusEffectData>
    },
    NewNpc {
        object_id: EntityId,
        type_id: NpcId,
        level: u16,
        balance_level: Option<u16>,
        stat_pairs: Vec<StatPair>,
        status_effect_datas: Vec<StatusEffectData>
    },
    NewNpcSummon {
        owner_id: EntityId,
        object_id: EntityId,
        type_id: NpcId,
        level: u16,
        balance_level: Option<u16>,
        stat_pairs: Vec<StatPair>,
        status_effect_datas: Vec<StatusEffectData>
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
        unpublished_objects: Vec<EntityId>
    },
    SkillCast {
        source_id: EntityId,
        skill_id: SkillId,
    },
    SkillStart {
        source_id: EntityId,
        skill_id: SkillId,
        tripod_index: Option<TripodIndex>,
        tripod_level: Option<TripodLevel>,
    },
    SkillDamageAbnormalMove {
        source_id: EntityId,
        events: Vec<SkillDamageAbnormalMoveDetails>,
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
        members: Vec<PartyMember>
    },
    PartyLeaveResult {
        party_instance_id: PartyInstanceId,
        name: String
    },
    PartyStatusEffectAdd {
        character_id: CharacterId,
        status_effect_datas: Vec<StatusEffectData>
    },
    PartyStatusEffectRemove {
        character_id: CharacterId,
        status_effect_instance_ids: Vec<StatusEffectInstanceId>,
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
        status_effect_instance_ids: Vec<StatusEffectInstanceId>,
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
        status_effect_datas: Vec<StatusEffectData>,
    },
    NewTransit {
        channel_id: u32
    }
}