use console::Color;
use phf::{Map, phf_map};
use chrono::{NaiveDate, Local};
use crate::links::Links;

pub const COLORS: Map<&str, Color> = phf_map! {
    "Red" => Color::Color256(9),
    "Green" => Color::Color256(10),
    "Blue" => Color::Color256(33)
};

pub const NAMES: Map<u8, &str> = phf_map! {
    0u8 => "tiramisu",
    1u8 => "indexiine",
    2u8 => "haxchi",
    3u8 => "cbhc"
};

pub fn get_link_from_names(num: u8, links: &Links) -> String {
    let name = NAMES[&num];
    links.options[name].clone()
}

fn get_eshop_shutdown() -> NaiveDate {
    NaiveDate::from_ymd(2023, 3, 27)
}

fn get_today() -> NaiveDate {
    Local::today().naive_local()
}

pub fn eshop_down() -> bool {
    get_today() > get_eshop_shutdown()
}