use std::{collections::HashMap, net::IpAddr};
pub enum NodeType {
    Undefined = 0,
    ESP8266 = 82,
    ESP32 = 32,
    ESP32S2 = 33,
    ESP32S3 = 34,
    ESP32C3 = 35,
}
pub struct NodeStruct {
    pub node_name: &'static str,
    pub ip: IpAddr,
    pub age: u8,
    pub node_type: NodeType,
    pub build: u32,
}
impl NodeStruct {
    pub fn new(name: &'static str, addr: IpAddr) -> Self {
        NodeStruct {
            node_name: name,
            ip: addr,
            age: 0,
            node_type: NodeType::Undefined,
            build: 0,
        }
    }
}

pub type NodeMap = HashMap<u8, NodeStruct>;
