#![forbid(unsafe_code)]
#![cfg_attr(not(debug_assertions), deny(warnings))]
#![warn(clippy::all, rust_2018_idioms)]
#![feature(derive_default_enum)]
#![feature(drain_filter)]

use std::error::Error;
use std::fs::{self, OpenOptions};
use std::path::Path;
use std::{f32::INFINITY, fmt};

use chrono::{DateTime, Local};
use csv;
use serde::{Deserialize, Serialize};

pub mod app;

pub fn get_time() -> String {
    let logged_time: DateTime<Local> = Local::now();
    logged_time.format("%Y-%m-%d %H:%M:%S").to_string()
}

pub fn calculate_pulls(crystals: f32, tenners: f32, singles: f32) -> String {
    let crystal_ten_pulls = ((crystals / 3000.0).floor()) * 10.0;
    let remaining_crystals_for_single_pulls = crystals % 3000.0;
    let crystal_single_pulls = (remaining_crystals_for_single_pulls / 300.0).floor();
    let total_pulls =
        (crystal_ten_pulls + crystal_single_pulls + (tenners * 10.0) + singles).to_string();
    let total = String::from(total_pulls);
    let spark_percentage = get_percentage(total.parse::<f32>().unwrap(), 300.0);
    "Total: ".to_owned() + &total + " pulls (" + &spark_percentage + ")"
}

pub fn get_percentage(x: f32, y: f32) -> String {
    let result = (x * 100.0) / y;
    return format!("{:.2}%", result);
}

pub fn create_path(path: &str) -> std::io::Result<()> {
    fs::create_dir(path)?;
    Ok(())
}

pub fn export(droplog: DropLog) -> Result<(), Box<dyn Error>> {
    let logged_drops = droplog.drop.iter().count().to_string();
    let export_time: DateTime<Local> = Local::now();
    let export_four_digit_year = export_time.format("%Y").to_string();
    let export_month = export_time.format("%m").to_string();
    let export_day = export_time.format("%d").to_string();
    if !Path::new("./exports/").exists() {
        create_path("./exports/")?;
    }
    let str_path = format!(
        "./exports/dorothy-{}-drops-{}-{}-{}.csv",
        &logged_drops, &export_four_digit_year, &export_month, &export_day
    );

    if Path::new("./exports/").exists() {
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .append(true)
            .open(&str_path)
            .unwrap();
        let mut wtr = csv::Writer::from_writer(&file);

        for drop in droplog.drop {
            wtr.serialize(drop)?;
        }
        wtr.flush()?;
    }
    Ok(())
}

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    pub version: u8,
    pub app_settings: DorothyConfig,
    pub droplog: DropLog,
}

#[derive(PartialEq, Default, Debug, Serialize, Deserialize)]
pub enum PBHLHonors {
    Honors800k,
    Honors1000k,
    Honors1200k,
    Honors1400k,
    Honors1600k,
    Honors1800k,
    Honors2000k,
    #[default]
    Ignore,
}

impl fmt::Display for PBHLHonors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            PBHLHonors::Ignore => write!(f, ""),
            PBHLHonors::Honors800k => write!(f, "0~800k"),
            PBHLHonors::Honors1000k => write!(f, "801~1000k"),
            PBHLHonors::Honors1200k => write!(f, "1001~1200k"),
            PBHLHonors::Honors1400k => write!(f, "1201~1400k"),
            PBHLHonors::Honors1600k => write!(f, "1401~1600k"),
            PBHLHonors::Honors1800k => write!(f, "1601~1800k"),
            PBHLHonors::Honors2000k => write!(f, "1801~2000k or more"),
        }
    }
}

#[derive(PartialEq, Clone, Default, Debug, Serialize, Deserialize)]
pub enum UiTab {
    Pulls,
    Akasha,
    PBHL,
    GOHL,
    Hosts,
    #[default]
    None,
}
#[derive(PartialEq, Clone, Default, Debug, Serialize, Deserialize)]
pub enum Raid {
    Akasha,
    PBHL,
    GOHL,
    UBHL,
    Xeno,
    #[default]
    None,
}

impl fmt::Display for Raid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Raid::Akasha => write!(f, "Akasha"),
            Raid::PBHL => write!(f, "PBHL"),
            Raid::GOHL => write!(f, "GOHL"),
            Raid::UBHL => write!(f, "UBHL"),
            Raid::Xeno => write!(f, "Xeno Showdown"),
            Raid::None => write!(f, ""),
        }
    }
}

#[derive(PartialEq, Clone, Default, Debug, Serialize, Deserialize)]
pub enum Item {
    VerdantAzurite,
    HollowKey,
    ChampionMerit,
    SupremeMerit,
    LegendaryMerit,
    SilverCentrum,
    WeaponPlusMark1,
    WeaponPlusMark2,
    WeaponPlusMark3,
    CoronationRing,
    LineageRing,
    IntricacyRing,
    GoldBrick,
    #[default]
    NoDrop,
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Item::NoDrop => write!(f, "No Blue Chest"),
            Item::VerdantAzurite => write!(f, "Verdant Azurite"),
            Item::HollowKey => write!(f, "Hollow Key"),
            Item::ChampionMerit => write!(f, "Champion Merit"),
            Item::SupremeMerit => write!(f, "Supreme Merit"),
            Item::LegendaryMerit => write!(f, "Legendary Merit"),
            Item::SilverCentrum => write!(f, "Silver Centrum"),
            Item::WeaponPlusMark1 => write!(f, "Weapon Plus Mark 1"),
            Item::WeaponPlusMark2 => write!(f, "Weapon Plus Mark 2"),
            Item::WeaponPlusMark3 => write!(f, "Weapon Plus Mark 3"),
            Item::CoronationRing => write!(f, "Coronation Ring"),
            Item::LineageRing => write!(f, "Lineage Ring"),
            Item::IntricacyRing => write!(f, "Intricacy Ring"),
            Item::GoldBrick => write!(f, "Gold Brick"),
        }
    }
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DropLog {
    #[serde(default)]
    pub drop: Vec<ItemDrop>,
}

impl DropLog {
    pub fn reset() -> Vec<ItemDrop> {
        Vec::new()
    }

    pub fn remove(drop_log: DropLog, removing_item: u32) -> DropLog {
        let mut new_droplog: DropLog = drop_log;
        new_droplog.drop.retain(|x| x.drop_id == removing_item);
        new_droplog
    }
}

#[derive(PartialEq, Default, Clone, Debug, Serialize, Deserialize)]
pub struct ItemDrop {
    drop_id: u32,
    date_obtained: String,
    raid: Raid,
    item: Item,
    #[serde(default)]
    honors: Option<String>,
}

#[allow(dead_code)]
impl ItemDrop {
    fn new(
        drop_id: u32,
        date_obtained: String,
        raid: Raid,
        item: Item,
        honors: Option<String>,
    ) -> Self {
        Self {
            drop_id,
            date_obtained,
            raid,
            item,
            honors,
        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DorothyConfig {
    pub dark_mode: bool,
    pub left_panel_visible: bool,
    pub right_panel_visible: bool,
    pub always_on_top: bool,
    pub reset_on_export: bool,
    pub droprate_by_kills: bool,
    pub show_all_drops: bool,
    pub crystals_amount: String,
    pub ten_pulls_amount: String,
    pub single_pulls_amount: String,
    pub total_pulls: String,
    #[serde(default)]
    pub current_ui_tab: UiTab,
}

#[allow(dead_code)]
impl Default for DorothyConfig {
    fn default() -> Self {
        Self {
            dark_mode: true,
            left_panel_visible: true,
            right_panel_visible: true,
            always_on_top: false,
            reset_on_export: true,
            droprate_by_kills: false,
            show_all_drops: false,
            current_ui_tab: UiTab::Akasha,
            crystals_amount: "0".to_string(),
            ten_pulls_amount: "0".to_string(),
            single_pulls_amount: "0".to_string(),
            total_pulls: "".to_string(),
        }
    }
}

#[allow(dead_code)]
impl DorothyConfig {
    fn new() -> Self {
        Self {
            dark_mode: true,
            left_panel_visible: true,
            right_panel_visible: false,
            always_on_top: false,
            reset_on_export: true,
            droprate_by_kills: false,
            show_all_drops: false,
            current_ui_tab: UiTab::Akasha,
            crystals_amount: "0".to_string(),
            ten_pulls_amount: "0".to_string(),
            single_pulls_amount: "0".to_string(),
            total_pulls: "".to_string(),
        }
    }
}

#[cfg(target_arch = "wasm32")]
use eframe::wasm_bindgen::{self, prelude::*};

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn start(canvas_id: &str) -> Result<(), eframe::wasm_bindgen::JsValue> {
    console_error_panic_hook::set_once();
    tracing_wasm::set_as_global_default();

    let app = AppDorothy::default();
    eframe::start_web(canvas_id, Box::new(app))
}
