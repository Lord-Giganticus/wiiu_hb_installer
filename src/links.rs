use std::collections::HashMap;
use serde::Deserialize;

const LINKS: &str = include_str!("..\\links.json");

#[derive(Debug, Clone, Deserialize)]
pub struct Links {
    pub options: HashMap<String, String>,
    pub programs: Vec<String>
}

pub fn getlinks() -> Links {
    serde_json::from_str(LINKS).unwrap()
}