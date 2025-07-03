use std::collections::HashMap;

use lost_metrics_sniffer::models::*;
use serde_json;
use rand::Rng;

pub struct PacketRunner {
    packets: Vec<Packet>,
    cursor: usize,
}

pub struct PacketBuilder {
    packets: Vec<Packet>,
    next_id: u64,
    next_party_id: u64,
    raid_instance_id: u64,
    players: HashMap<String, PlayerInfo>,
    boss_id: Option<u64>,
    boss_hp: Option<i64>,
}

struct PlayerInfo {
    player_id: EntityId,
    character_id: CharacterId,
    class_id: ClassId,
    gear_level: GearLevel,
}

impl PacketBuilder {
    pub fn new() -> Self {
        let mut rng = rand::rng();
        Self {
            packets: Vec::new(),
            next_id: rng.random_range(1..=10_000),
            next_party_id: 1,
            raid_instance_id: 1,
            players: HashMap::new(),
            boss_id: None,
            boss_hp: None
        }
    }

    fn gen_id(&mut self) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        id
    }

    pub fn add_init_pc(mut self, name: &str) -> Self {

        if let Some(player) = self.players.get(name) {
            self.packets.push(Packet::InitPC {
                player_id: player.player_id,
                name: name.into(),
                character_id: player.character_id,
                class_id: player.class_id,
                gear_level: player.gear_level,
                stat_pairs: vec![].into(),
                status_effect_datas: vec![].into(),
            });
        }

        self
    }

    pub fn add_new_pc(mut self, name: &str, class_id: ClassId, gear_level: GearLevel) -> Self {
        let player_id = self.gen_id();
        let character_id = self.gen_id();

        self.players.insert(
            name.to_string(),
            PlayerInfo {
                player_id,
                character_id,
                class_id,
                gear_level,
            },
        );

        self.packets.push(Packet::NewPC {
            player_id,
            name: name.into(),
            class_id,
            character_id,
            max_item_level: gear_level,
            stat_pairs: vec![].into(),
            equip_item_datas: vec![].into(),
            status_effect_datas: vec![].into(),
        });

        self
    }

    pub fn add_new_npc(mut self, type_id: NpcId, level: u16, hp: i64) -> Self {
        let object_id = self.gen_id();

        self.boss_id = Some(object_id);
        self.boss_hp = Some(hp);

        self.packets.push(Packet::NewNpc {
            object_id,
            type_id,
            level,
            balance_level: Some(level).into(),
            stat_pairs: vec![
                StatPair { stat_type: 1, value: hp },
                StatPair { stat_type: 27, value: hp },
            ].into(),
            status_effect_datas: vec![].into(),
        });

        self
    }

    pub fn add_party_info(mut self, names: Vec<&str>) -> Self {
        let members = names
            .into_iter()
            .filter_map(|name| {
                self.players.get(name).map(|p| PartyMember {
                    character_id: p.character_id,
                    class_id: p.class_id,
                    gear_level: p.gear_level,
                    name: name.into(),
                })
            })
            .collect::<Vec<_>>();

        let party_id = self.next_party_id;
        self.next_party_id += 1;

        self.packets.push(Packet::PartyInfo {
            party_instance_id: party_id as u32,
            raid_instance_id: self.raid_instance_id as u32,
            members: members.into(),
        });

        self
    }

    pub fn add_raid_begin(mut self, raid_id: u32) -> Self {
        self.packets.push(Packet::RaidBegin { raid_id });
        self
    }

    pub fn add_skill_damage(mut self, min_damage: i64, max_damage: i64) -> Self {
        let mut rng = rand::rng();

        let boss_id = match self.boss_id {
            Some(id) => id,
            None => return self,
        };

        let mut boss_hp = self.boss_hp.unwrap_or(0);
        let max_boss_hp = boss_hp;

        let player_ids: Vec<_> = self.players.values().map(|p| p.player_id).collect();
        if player_ids.is_empty() {
            return self;
        }

        let mut player_iter = player_ids.iter().cycle();

        while boss_hp > 0 {
            let damage = rng.random_range(min_damage..=max_damage).min(boss_hp);
            boss_hp -= damage;

            let attacker = player_iter.next().unwrap();

            self.packets.push(Packet::SkillDamage {
                source_id: *attacker,
                event: SkillDamageEvent {
                    target_id: boss_id,
                    hit_flag: Self::random_hit_flag(),
                    hit_option: Self::random_hit_option(),
                    damage,
                    cur_hp: boss_hp.max(0),
                    max_hp: max_boss_hp,
                },
                skill_move_option_data: SkillMoveOptionData {
                    down_time: None.into(),
                    stand_up_time: None.into(),
                    move_time: None.into(),
                },
            });
        }

        self
    }

    fn random_hit_option() -> HitOption {
        match rand::rng().random_range(0..=3) {
            0 => HitOption::FlankAttack,
            1 => HitOption::FrontalAttack,
            2 => HitOption::BackAttack,
            3 => HitOption::Max,
            _ => unreachable!(),
        }
    }

    fn random_hit_flag() -> HitFlag {
        match rand::rng().random_range(0..=3) {
            0 => HitFlag::Normal,
            1 => HitFlag::DamageOverTime,
            2 => HitFlag::Critical,
            3 => HitFlag::DamageOverTimeCritical,
            _ => unreachable!(),
        }
    }

    pub fn build(self) -> PacketRunner {
        PacketRunner {
            packets: self.packets,
            cursor: 0,
        }
    }
}

impl PacketRunner {
    pub fn next_packet(&mut self) -> Option<Vec<u8>> {
        if self.cursor < self.packets.len() {
            let packet = &self.packets[self.cursor];
            self.cursor += 1;
            Some(serde_json::to_vec(packet).unwrap())
        } else {
            None
        }
    }
}
