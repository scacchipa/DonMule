use std::{collections::HashMap, fs::File, io::{BufReader, Cursor}};

use bytes::Bytes;

use crate::traits::buf_reader_ext::ByteReader;

pub struct ServerEntry {
    pub ip: Option<[u8; 4]>,
    pub port: Option<u16>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub users: Option<u32>,
    pub files: Option<u32>,
    pub ping: Option<u32>,
    pub fails: Option<u32>,
    pub priority: Option<u8>,
    pub preferences: Option<u32>,
    pub dns: Option<String>,
    pub max_users: Option<u32>,
    pub soft_files: Option<u32>,
    pub hard_files: Option<u32>,
    pub last_ping: Option<u32>,
    pub version: Option<String>,
    pub udp_flag: Option<u32>,
    pub auxiliary_port_list: Option<String>,
    pub low_id_clients: Option<u32>,
    pub udp_key: Option<u32>,
    pub udp_key_ip: Option<u32>,
    pub tcp_port_obfuscation: Option<u32>,
    pub upd_port_obfuscation: Option<u32>,
    pub others: HashMap<String, String>,
}

impl ServerEntry {
    pub fn new() -> Self {
        ServerEntry {
            ip: None,
            port: None,
            name: None,
            description: None,
            users: None,
            files: None,
            ping: None,
            fails: None,
            priority: None,
            preferences: None,
            dns: None,
            max_users: None,
            soft_files: None,
            hard_files: None,
            last_ping: None,
            version: None,
            udp_flag: None,
            auxiliary_port_list: None,
            low_id_clients: None,
            udp_key: None,
            udp_key_ip: None,
            tcp_port_obfuscation: None,
            upd_port_obfuscation: None,
            others: HashMap::new(),
        }
    }

    pub fn to_string(&mut self) -> String {
        let unwrap_ip = self.ip.unwrap_or([0u8; 4]);
        let unwrap_port = self.port.unwrap_or(0);
        let mut string = format!(
            "ip:port: {:#03}.{:#03}.{:#03}.{:#03}:{}",
            unwrap_ip[0], unwrap_ip[1], unwrap_ip[2], unwrap_ip[3], unwrap_port
        );
        if let Some(name) = &self.name {
            string += &format!(", name: {name}");
        };
        if let Some(description) = &self.description {
            string += &format!(", description: {description}")
        };
        if let Some(users) = &self.users {
            string += &format!(", users: {users}")
        };
        if let Some(files) = &self.files {
            string += &format!(", files: {files}")
        };
        if let Some(ping) = &self.ping {
            string += &format!(", ping: {ping}")
        };
        if let Some(fails) = &self.fails {
            string += &format!(", fails: {fails}")
        };
        if let Some(priority) = &self.priority {
            string += &format!(", priority: {priority}")
        };
        if let Some(preferences) = &self.preferences {
            string += &format!(", preferences: {preferences}")
        };
        if let Some(dns) = &self.dns {
            string += &format!(", dns: {dns}")
        };
        if let Some(max_users) = &self.max_users {
            string += &format!(", max_users: {max_users}")
        };
        if let Some(soft_files) = &self.soft_files {
            string += &format!(", soft_files: {soft_files}")
        };
        if let Some(hard_files) = &self.hard_files {
            string += &format!(", hard_files: {hard_files}")
        };
        if let Some(last_ping) = &self.last_ping {
            string += &format!(", last_ping: {last_ping}")
        };
        if let Some(version) = &self.version {
            string += &format!(", version: {version}")
        };
        if let Some(udp_flag) = &self.udp_flag {
            string += &format!(", udp_flags: {udp_flag}")
        };
        if let Some(auxiliary_port_list) = &self.auxiliary_port_list {
            string += &format!(", auxiliary_port_list: {auxiliary_port_list}")
        };
        if let Some(low_id_clients) = &self.low_id_clients {
            string += &format!(", low_id_clients: {low_id_clients}")
        };

        if let Some(udp_key) = &self.udp_key {
            string += &format!(", udp_key: {udp_key}")
        };
        if let Some(udp_key_ip) = &self.udp_key_ip {
            string += &format!(", udp_key_ip: {udp_key_ip}")
        };
        if let Some(tcp_port_obfuscation) = &self.tcp_port_obfuscation {
            string += &format!(", tcp_port_obfuscation: {tcp_port_obfuscation}")
        };
        if let Some(upd_port_obfuscation) = &self.upd_port_obfuscation {
            string += &format!(", upd_port_obfuscation: {upd_port_obfuscation}")
        };

        if !self.others.is_empty() {
            string += ", ";
            string += &self
                .others
                .iter()
                .map(|(key, value)| format!("{key}: {value}"))
                .collect::<Vec<_>>()
                .join(", ");
        }

        return string;
    }

    pub fn load_entry(&mut self, cursor: &mut Cursor<Bytes>) {
        self.ip = Some(cursor.read_ip().unwrap());
        self.port = Some(cursor.read_u16_le().unwrap());
        let tag_count = cursor.read_u32_le().unwrap();

        for _ in 0..tag_count {
            self.load_tlv(cursor);
        }

        println!("New server found: {}", self.to_string());
    }

    fn load_tlv(&mut self, cursor: &mut Cursor<Bytes>) {
        let tag_type = cursor.read_u8().unwrap();
        let tag_name_length = cursor.read_u16_le().unwrap();
        let tag_name = cursor.read_array(tag_name_length.into()).unwrap();

        match tag_name[0] {
            0x01u8 => self.name = ServerEntry::read_tlf_string(cursor),
            0x0Bu8 => self.description = ServerEntry::read_tlf_string(cursor),
            0x0Cu8 => self.ping = ServerEntry::read_u32(cursor),
            0x0Du8 => self.fails = ServerEntry::read_u32(cursor),
            0x0Eu8 => self.preferences = ServerEntry::read_u32(cursor),
            0x0Fu8 => self.port = ServerEntry::read_u32(cursor).map(|p| p as u16),
            0x10u8 => self.ip = ServerEntry::read_u32(cursor).map(|n| n.to_le_bytes()),
            0x85u8 => self.dns = ServerEntry::read_tlf_string(cursor),
            0x87u8 => self.max_users = ServerEntry::read_u32(cursor),
            0x88u8 => self.soft_files = ServerEntry::read_u32(cursor),
            0x89u8 => self.hard_files = ServerEntry::read_u32(cursor),
            0x90u8 => self.last_ping = ServerEntry::read_u32(cursor),
            0x91u8 => {
                self.version = match tag_type {
                    0x02 => ServerEntry::read_tlf_string(cursor),
                    0x03 => ServerEntry::read_u32(cursor).map(|v| v.to_string()),
                    _ => None,
                }
            }
            0x92u8 => self.udp_flag = ServerEntry::read_u32(cursor),
            0x93u8 => self.auxiliary_port_list = ServerEntry::read_tlf_string(cursor),
            0x94u8 => self.low_id_clients = ServerEntry::read_u32(cursor),
            0x95u8 => self.udp_key = ServerEntry::read_u32(cursor),
            0x96u8 => self.udp_key_ip = ServerEntry::read_u32(cursor),
            0x97u8 => self.tcp_port_obfuscation = ServerEntry::read_u32(cursor),
            0x98u8 => self.upd_port_obfuscation = ServerEntry::read_u32(cursor),
            _ => {
                let key = String::from_utf8(tag_name).unwrap();

                let value = match tag_type {
                    0x02u8 => ServerEntry::read_tlf_string(cursor).unwrap(),
                    0x03u8 => cursor.read_u32_le().unwrap().to_string(),
                    _ => format!("Unknown tag type: {:#04X}", tag_type),
                };

                match key.as_str() {
                    "files" => self.files = value.parse::<u32>().ok(),
                    "users" => self.users = value.parse::<u32>().ok(),
                    "maxusers" => self.max_users = value.parse::<u32>().ok(),
                    "lowusers" => self.low_id_clients = value.parse::<u32>().ok(),
                    _ => {
                        self.others.insert(key, value);
                    }
                }
            }
        }
    }

    fn read_tlf_string(cursor: &mut Cursor<Bytes>) -> Option<String> {
        let length = cursor.read_u16_le().ok()?;
        let vect = cursor.read_array(length.into()).ok()?;
        return String::from_utf8(vect).ok();
    }

    fn read_u32(cursor: &mut Cursor<Bytes>) -> Option<u32> {
        return cursor.read_u32_le().ok();
    }
}