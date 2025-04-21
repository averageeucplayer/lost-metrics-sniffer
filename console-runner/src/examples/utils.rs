use std::{thread::JoinHandle, time::Duration};

use anyhow::*;
use log::*;
use lost_metrics_sniffer::{models::*, FakeSender};
use rand::{rng, Rng};
use tokio::{runtime::Runtime, time::sleep};

pub fn random_packet() -> Packet {
    let mut rng = rng();

    let variant_count = 32;

    let index = rng.random_range(0..variant_count);

    match index {
        0 => Packet::CounterAttack {
            source_id: rng.random(),
        },
        1 => Packet::Death {
            target_id: rng.random(),
        },
        2 => Packet::IdentityGaugeChange {
            player_id: rng.random(),
            identity_gauge1: rng.random(),
            identity_gauge2: rng.random(),
            identity_gauge3: rng.random(),
        },
        3 => Packet::InitEnv {
            player_id: rng.random(),
        },
        4 => Packet::InitPC {
            player_id: rng.random(),
            name: "Player1".to_string(),
            character_id: rng.random(),
            class_id: rng.random(),
            gear_level: rng.random(),
            stat_pairs: vec![
                StatPair { stat_type: rng.random(), value: rng.random() },
                StatPair { stat_type: rng.random(), value: rng.random() },
            ],
            status_effect_datas: vec![
                StatusEffectData {
                    source_id: rng.random(),
                    status_effect_id: rng.random(),
                    status_effect_instance_id: rng.random(),
                    value: Some(vec![rng.random_range(0..=255)]),
                    total_time: rng.random_range(0.0..=10.0),
                    stack_count: rng.random_range(1..=5),
                    end_tick: rng.random(),
                },
                StatusEffectData {
                    source_id: rng.random(),
                    status_effect_id: rng.random(),
                    status_effect_instance_id: rng.random(),
                    value: Some(vec![rng.random_range(0..=255)]),
                    total_time: rng.random_range(0.0..=10.0),
                    stack_count: rng.random_range(1..=5),
                    end_tick: rng.random(),
                },
            ],
        },
        5 => Packet::NewPC {
            player_id: rng.random(),
            name: "NewPC".to_string(),
            class_id: rng.random(),
            max_item_level: rng.random(),
            character_id: rng.random(),
            stat_pairs: vec![
                StatPair { stat_type: rng.random(), value: rng.random() },
                StatPair { stat_type: rng.random(), value: rng.random() },
            ],
            equip_item_datas: vec![
                EquipItemData { },
                EquipItemData { },
            ],
            status_effect_datas: vec![
                StatusEffectData {
                    source_id: rng.random(),
                    status_effect_id: rng.random(),
                    status_effect_instance_id: rng.random(),
                    value: Some(vec![rng.random_range(0..=255)]),
                    total_time: rng.random_range(0.0..=10.0),
                    stack_count: rng.random_range(1..=5),
                    end_tick: rng.random(),
                },
                StatusEffectData {
                    source_id: rng.random(),
                    status_effect_id: rng.random(),
                    status_effect_instance_id: rng.random(),
                    value: Some(vec![rng.random_range(0..=255)]),
                    total_time: rng.random_range(0.0..=10.0),
                    stack_count: rng.random_range(1..=5),
                    end_tick: rng.random(),
                },
            ],
        },
        6 => Packet::NewNpc {
            object_id: rng.random(),
            type_id: rng.random(),
            level: rng.random(),
            balance_level: Some(rng.random()),
            stat_pairs: vec![
                StatPair { stat_type: rng.random(), value: rng.random() },
                StatPair { stat_type: rng.random(), value: rng.random() },
            ],
            status_effect_datas: vec![
                StatusEffectData {
                    source_id: rng.random(),
                    status_effect_id: rng.random(),
                    status_effect_instance_id: rng.random(),
                    value: Some(vec![rng.random_range(0..=255)]),
                    total_time: rng.random_range(0.0..=10.0),
                    stack_count: rng.random_range(1..=5),
                    end_tick: rng.random(),
                },
                StatusEffectData {
                    source_id: rng.random(),
                    status_effect_id: rng.random(),
                    status_effect_instance_id: rng.random(),
                    value: Some(vec![rng.random_range(0..=255)]),
                    total_time: rng.random_range(0.0..=10.0),
                    stack_count: rng.random_range(1..=5),
                    end_tick: rng.random(),
                },
            ],
        },
        7 => Packet::NewNpcSummon {
            owner_id: rng.random(),
            object_id: rng.random(),
            type_id: rng.random(),
            level: rng.random(),
            balance_level: Some(rng.random()),
            stat_pairs: vec![
                StatPair { stat_type: rng.random(), value: rng.random() },
                StatPair { stat_type: rng.random(), value: rng.random() },
            ],
            status_effect_datas: vec![
                StatusEffectData {
                    source_id: rng.random(),
                    status_effect_id: rng.random(),
                    status_effect_instance_id: rng.random(),
                    value: Some(vec![rng.random_range(0..=255)]),
                    total_time: rng.random_range(0.0..=10.0),
                    stack_count: rng.random_range(1..=5),
                    end_tick: rng.random(),
                },
                StatusEffectData {
                    source_id: rng.random(),
                    status_effect_id: rng.random(),
                    status_effect_instance_id: rng.random(),
                    value: Some(vec![rng.random_range(0..=255)]),
                    total_time: rng.random_range(0.0..=10.0),
                    stack_count: rng.random_range(1..=5),
                    end_tick: rng.random(),
                },
            ],
        },
        8 => Packet::NewProjectile {
            projectile_id: rng.random(),
            owner_id: rng.random(),
            skill_id: rng.random(),
            skill_effect: rng.random(),
        },
        9 => Packet::NewTrap {
            object_id: rng.random(),
            owner_id: rng.random(),
            skill_id: rng.random(),
            skill_effect: rng.random(),
        },
        10 => Packet::RaidBegin {
            raid_id: rng.random(),
        },
        11 => Packet::RaidBossKill,
        12 => Packet::RaidResult,
        13 => Packet::RemoveObject {
            unpublished_objects: vec![rng.random(), rng.random()],
        },
        14 => Packet::SkillCast {
            source_id: rng.random(),
            skill_id: rng.random(),
        },
        15 => Packet::SkillStart {
            source_id: rng.random(),
            skill_id: rng.random(),
            tripod_index: Some(TripodIndex { 
                first: rng.random(),
                second: rng.random(),
                third: rng.random()
            }),
            tripod_level: Some(TripodLevel { 
                first: rng.random(),
                second: rng.random(),
                third: rng.random()
            }),
        },
        16 => Packet::SkillDamageAbnormalMove {
            source_id: rng.random(),
            events: vec![
                SkillDamageAbnormalMoveDetails {
                    event: SkillDamageEvent {
                        target_id: rng.random(),
                        damage: rng.random(),
                        modifier: rng.random(),
                        cur_hp: rng.random(),
                        max_hp: rng.random(),
                        damage_attr: Some(rng.random()),
                        damage_type: rng.random()
                    },
                    skill_move_option_data: SkillMoveOptionData {
                        down_time: Some(rng.random()),
                        stand_up_time: Some(rng.random()),
                        move_time: Some(rng.random()),
                    }
                },
                SkillDamageAbnormalMoveDetails {
                    event: SkillDamageEvent {
                        target_id: rng.random(),
                        damage: rng.random(),
                        modifier: rng.random(),
                        cur_hp: rng.random(),
                        max_hp: rng.random(),
                        damage_attr: Some(rng.random()),
                        damage_type: rng.random()
                    },
                    skill_move_option_data: SkillMoveOptionData {
                        down_time: Some(rng.random()),
                        stand_up_time: Some(rng.random()),
                        move_time: Some(rng.random()),
                    }
                },
            ],
            skill_id: rng.random(),
            skill_effect_id: rng.random(),
        },
        17 => Packet::SkillDamage {
            event: SkillDamageEvent::default(),
            skill_move_option_data: SkillMoveOptionData::default(),
        },
        18 => Packet::PartyInfo {
            party_instance_id: rng.random(),
            raid_instance_id: rng.random(),
            members: vec![
                PartyMember {
                    name: format!("Player{}", rng.random_range(0..=100)),
                    class_id: rng.random(),
                    character_id: rng.random(),
                    gear_level: rng.random_range(1660.0..=1700.0),
                },
                PartyMember {
                    name: format!("Player{}", rng.random_range(0..=100)),
                    class_id: rng.random(),
                    character_id: rng.random(),
                    gear_level: rng.random_range(1660.0..=1700.0),
                }
            ],
        },
        19 => Packet::PartyLeaveResult {
            party_instance_id: rng.random(),
            name: "PlayerLeft".to_string(),
        },
        20 => Packet::PartyStatusEffectAdd {
            character_id: rng.random(),
            status_effect_datas: vec![
                StatusEffectData {
                    source_id: rng.random(),
                    status_effect_id: rng.random(),
                    status_effect_instance_id: rng.random(),
                    value: Some(vec![rng.random_range(0..=255)]),
                    total_time: rng.random_range(0.0..=10.0),
                    stack_count: rng.random_range(1..=5),
                    end_tick: rng.random(),
                },
                StatusEffectData {
                    source_id: rng.random(),
                    status_effect_id: rng.random(),
                    status_effect_instance_id: rng.random(),
                    value: Some(vec![rng.random_range(0..=255)]),
                    total_time: rng.random_range(0.0..=10.0),
                    stack_count: rng.random_range(1..=5),
                    end_tick: rng.random(),
                },
            ],
        },
        21 => Packet::PartyStatusEffectRemove {
            character_id: rng.random(),
            status_effect_instance_ids: vec![rng.random(), rng.random()],
            reason: rng.random(),
        },
        22 => Packet::PartyStatusEffectResult {
            raid_instance_id: rng.random(),
            party_instance_id: rng.random(),
            character_id: rng.random(),
        },
        23 => Packet::StatusEffectAdd {
            object_id: rng.random(),
            status_effect_data:  StatusEffectData {
                source_id: rng.random(),
                status_effect_id: rng.random(),
                status_effect_instance_id: rng.random(),
                value: Some(vec![rng.random_range(0..=255)]),
                total_time: rng.random_range(0.0..=10.0),
                stack_count: rng.random_range(1..=5),
                end_tick: rng.random(),
            },
        },
        24 => Packet::StatusEffectRemove {
            object_id: rng.random(),
            character_id: rng.random(),
            status_effect_instance_ids: vec![rng.random(), rng.random()],
            reason: rng.random(),
        },
        25 => Packet::TriggerBossBattleStatus,
        26 => Packet::TriggerStart {
            signal: rng.random(),
        },
        27 => Packet::ZoneMemberLoadStatus {
            zone_id: rng.random(),
            zone_level: rng.random(),
        },
        28 => Packet::ZoneObjectUnpublish {
            object_id: rng.random(),
        },
        29 => Packet::StatusEffectSyncData {
            object_id: rng.random(),
            status_effect_instance_id: rng.random(),
            character_id: rng.random(),
            value: rng.random(),
        },
        30 => Packet::TroopMemberUpdateMin {
            character_id: rng.random(),
            cur_hp: rng.random(),
            max_hp: rng.random(),
            status_effect_datas: vec![
                StatusEffectData {
                    source_id: rng.random(),
                    status_effect_id: rng.random(),
                    status_effect_instance_id: rng.random(),
                    value: Some(vec![rng.random_range(0..=255)]),
                    total_time: rng.random_range(0.0..=10.0),
                    stack_count: rng.random_range(1..=5),
                    end_tick: rng.random(),
                },
                StatusEffectData {
                    source_id: rng.random(),
                    status_effect_id: rng.random(),
                    status_effect_instance_id: rng.random(),
                    value: Some(vec![rng.random_range(0..=255)]),
                    total_time: rng.random_range(0.0..=10.0),
                    stack_count: rng.random_range(1..=5),
                    end_tick: rng.random(),
                },
            ],
        },
        31 => Packet::NewTransit {
            channel_id: rng.random(),
        },
        _ => unreachable!(),
    }
}

pub fn get_type_name<T: std::fmt::Debug>(value: T) -> &'static str {
    std::any::type_name::<T>()
}

pub async fn send_to_fake_server() -> Result<()> {

    let mut sender = FakeSender::new();
    sender.open().await?;
    let config = bincode::config::standard();

    loop {
        let packet = random_packet();
        let data = bincode::encode_to_vec(&packet, config)?;
 
        info!("Sending... {}", get_type_name(packet));
        sender.send(&data).await?;
        sleep(Duration::from_secs(1)).await;
    }

    anyhow::Ok(())
}

pub fn separate_thread() -> JoinHandle<anyhow::Result<()>> {
    std::thread::spawn(move || {
        let rt = Runtime::new().unwrap();
        
        rt.block_on(async {
            send_to_fake_server().await?;
            anyhow::Ok(())
        })?;

        Ok(())
    })
}
