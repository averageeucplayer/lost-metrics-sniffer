use abi_stable::std_types::ROption::{self, RSome};
use rand::{distr::Alphanumeric, rng, rngs::ThreadRng, Rng};
use lost_metrics_sniffer::{models::{Packet, SkillDamageEvent, SkillMoveOptionData}, packet_capture::PacketCapture};
use anyhow::*;

pub struct FakePacketCapturer;

impl PacketCapture for FakePacketCapturer {

    fn start(&mut self, port: u16) -> Result<()> {
        Ok(())
    }

    fn recv(&mut self) -> Result<Vec<u8>> {
        let mut rng = rng();
        let action = rng.random_range(1..20);

        let packet = match action {
            1 => Packet::CounterAttack { source_id: rng.random_range(1..100) },
            2 => Packet::Death { target_id: rng.random_range(1..100) },
            3 => Packet::IdentityGaugeChange { 
                player_id: rng.random_range(1..100),
                identity_gauge1: rng.random_range(1..100),
                identity_gauge2: rng.random_range(1..100),
                identity_gauge3: rng.random_range(1..100)
            },
            4 => Packet::InitEnv { player_id: rng.random_range(1..100) },
            5 => Packet::NewPC { 
                player_id: rng.random_range(1..100),
                name: self.random_nickname().into(),
                class_id: rng.random_range(1..100),
                max_item_level: rng.random_range(1660.0..1710.0),
                character_id: rng.random_range(1..100),
                stat_pairs: vec![].into(),
                equip_item_datas: vec![].into(),
                status_effect_datas: vec![].into()
            },
            6 => Packet::InitPC { 
                player_id: rng.random_range(1..100),
                name: self.random_nickname().into(),
                character_id: rng.random_range(1..100),
                class_id: rng.random_range(1..100),
                gear_level: rng.random_range(1660.0..1710.0),
                stat_pairs: vec![].into(),
                status_effect_datas: vec![].into()
            },
            7 => Packet::NewNpc { 
                object_id: rng.random_range(1..100),
                type_id: rng.random_range(1..100),
                level: rng.random_range(1..100),
                balance_level: RSome(rng.random_range(1..100)),
                stat_pairs: vec![].into(),
                status_effect_datas: vec![].into()
            },
            _ => Packet::SkillDamage { 
                event: SkillDamageEvent { 
                    target_id: rng.random_range(1..100),
                    damage: rng.random_range(1..100),
                    modifier: rng.random_range(1..100),
                    cur_hp: rng.random_range(1..100),
                    max_hp: rng.random_range(1..100),
                    damage_attr: RSome(rng.random_range(1..100)),
                    damage_type: rng.random_range(1..100),
                },
                skill_move_option_data: SkillMoveOptionData {
                    down_time: ROption::RNone,
                    stand_up_time: ROption::RNone,
                    move_time: ROption::RNone,
                }
            }
        };

        let data = serde_json::to_vec(&packet)?;

        Ok(data)
    }

    fn close(&mut self) -> Result<()> {
        Ok(())
    }
}

impl FakePacketCapturer {
    fn random_nickname(&self) -> String {
        let mut rng = rng();
        let distribution = rand::distr::Uniform::new_inclusive(b'A', b'Z').unwrap();
        let first_letter = rng.sample(distribution);

        let rest: String = (5..10) 
            .map(|_| rng.sample(Alphanumeric) as char)
            .filter(|c| c.is_alphabetic())
            .map(|c| c.to_ascii_lowercase())
            .collect();
        
        format!("{}{}", first_letter, rest)
    }

    pub fn new() -> Self {
        Self {}
    }
}