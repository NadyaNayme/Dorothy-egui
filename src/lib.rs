#![forbid(unsafe_code)]
#![warn(clippy::all, rust_2018_idioms)]
#![feature(derive_default_enum)]
#![feature(drain_filter)]

use eframe::egui::{widgets, Response, Sense, Ui, Widget, WidgetInfo, WidgetType};
use eframe::epaint::{ColorImage, Rounding, TextureId, Vec2};
use serde::{Deserialize, Serialize};

#[cfg(any(target_os = "windows", target_os = "macos", target_os = "linux"))]
use chrono::{DateTime, Local};
#[cfg(target_arch = "wasm32")]
use chrono::{DateTime, NaiveDateTime, Utc};
#[cfg(any(target_os = "windows", target_os = "macos", target_os = "linux"))]
use self_update::cargo_crate_version;
use std::error::Error;
use std::fs::{self, OpenOptions};
use std::path::Path;
use std::{f32::INFINITY, fmt};

pub mod app;

pub static BLUE_CHEST: &[u8] = include_bytes!("./images/blue_chest.png");
pub static NO_BLUE_CHEST: &[u8] = include_bytes!("./images/no_blue_chest.png");
pub static SILVER_CENTRUM: &[u8] = include_bytes!("./images/silver_centrum.png");
pub static HOLLOW_KEY: &[u8] = include_bytes!("./images/hollow_key.png");
pub static VERDANT_AZURITE: &[u8] = include_bytes!("./images/verdant_azurite.png");
pub static C_RING: &[u8] = include_bytes!("./images/coronation_ring.png");
pub static L_RING: &[u8] = include_bytes!("./images/lineage_ring.png");
pub static I_RING: &[u8] = include_bytes!("./images/intricacy_ring.png");
pub static C_MERIT: &[u8] = include_bytes!("./images/champion_merit.png");
pub static S_MERIT: &[u8] = include_bytes!("./images/supreme_merit.png");
pub static L_MERIT: &[u8] = include_bytes!("./images/legendary_merit.png");
pub static P_MARK_1: &[u8] = include_bytes!("./images/weapon_plus_mark_1.png");
pub static P_MARK_2: &[u8] = include_bytes!("./images/weapon_plus_mark_2.png");
pub static P_MARK_3: &[u8] = include_bytes!("./images/weapon_plus_mark_3.png");
pub static GOLD_BAR: &[u8] = include_bytes!("./images/hihi.png");
pub static UBHL_HOST_BAR: &[u8] = include_bytes!("./images/ubhl_host.png");
pub static UBHL_FLIP_BAR: &[u8] = include_bytes!("./images/ubhl_flip.png");
pub static PBHL_HOST_BAR: &[u8] = include_bytes!("./images/pbhl_host.png");
pub static XENO_FLIP_BAR: &[u8] = include_bytes!("./images/xeno_flip.png");
pub static HL_HOST_BAR: &[u8] = include_bytes!("./images/hl_host.png");
pub static QL_HOST_BAR: &[u8] = include_bytes!("./images/ql_host.png");
pub static HLQL_HOST_BAR: &[u8] = include_bytes!("./images/hlql_host.png");
pub static DOROTHY: &[u8] = include_bytes!("./images/dorothy.ico");

#[cfg(not(target_arch = "wasm32"))]
pub fn get_time() -> String {
    let logged_time: DateTime<Local> = Local::now();
    logged_time.format("%Y-%m-%d %H:%M:%S").to_string()
}
// chrono crate doesn't support wasm32 arch yet, workaround
#[cfg(target_arch = "wasm32")]
pub fn get_time() -> String {
    let now = js_sys::Date::now().to_string();
    let now_as_epoch = now.parse::<i64>().unwrap();
    let datetime = DateTime::<Utc>::from_utc(
        NaiveDateTime::from_timestamp_opt(
            now_as_epoch / 1000,
            (now_as_epoch % 1000) as u32 * 1_000_000,
        )
        .unwrap(),
        Utc,
    );
    datetime.to_string()
}

#[cfg(not(target_arch = "wasm32"))]
pub enum ReleaseStatus {
    NewVersion,
    UpToDate,
}

#[cfg(not(target_arch = "wasm32"))]
pub fn check_for_update() -> Result<ReleaseStatus, Box<dyn (::std::error::Error)>> {
    let releases = self_update::backends::github::ReleaseList::configure()
        .repo_owner("NadyaNayme")
        .repo_name("Dorothy-egui")
        .build()?
        .fetch()?;

    if releases.is_empty() {
        return Ok(ReleaseStatus::UpToDate);
    }

    let latest_release = &releases[0];
    let is_new =
        self_update::version::bump_is_greater(cargo_crate_version!(), &latest_release.version)?;

    if is_new {
        Ok(ReleaseStatus::NewVersion)
    } else {
        Ok(ReleaseStatus::UpToDate)
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub fn self_update() -> Result<(), Box<dyn ::std::error::Error>> {
    let _releases = self_update::backends::github::ReleaseList::configure()
        .repo_owner("NadyaNayme")
        .repo_name("Dorothy-egui")
        .build()?
        .fetch()?;

    let _status = self_update::backends::github::Update::configure()
        .repo_owner("NadyaNayme")
        .repo_name("Dorothy-egui")
        .bin_name("dorothy-egui_bin")
        .show_download_progress(true)
        .current_version(cargo_crate_version!())
        .no_confirm(true)
        .build()?
        .update()?;
    Ok(())
}

pub fn calculate_pulls(crystals: f32, tenners: f32, singles: f32) -> String {
    let crystal_ten_pulls = ((crystals / 3000.0).floor()) * 10.0;
    let remaining_crystals_for_single_pulls = crystals % 3000.0;
    let crystal_single_pulls = (remaining_crystals_for_single_pulls / 300.0).floor();
    let total_pulls =
        (crystal_ten_pulls + crystal_single_pulls + (tenners * 10.0) + singles).to_string();
    let total = total_pulls;
    let spark_percentage = get_percentage(total.parse::<f32>().unwrap(), 300.0);
    "Total: ".to_owned() + &total + " pulls (" + &spark_percentage + ")"
}

pub fn get_percentage(x: f32, y: f32) -> String {
    let result = (x * 100.0) / y;
    return format!("{:.2}%", result);
}

#[cfg(not(target_arch = "wasm32"))]
pub fn load_image_from_path(
    path: &std::path::Path,
) -> Result<eframe::egui::ColorImage, image::ImageError> {
    let image = image::io::Reader::open(path)?.decode()?;
    let size = [image.width() as _, image.height() as _];
    let image_buffer = image.to_rgba8();
    let pixels = image_buffer.as_flat_samples();
    Ok(eframe::egui::ColorImage::from_rgba_unmultiplied(
        size,
        pixels.as_slice(),
    ))
}

pub fn load_image_from_memory(image_data: &[u8]) -> Result<ColorImage, image::ImageError> {
    let image = image::load_from_memory(image_data)?;
    let size = [image.width() as _, image.height() as _];
    let image_buffer = image.to_rgba8();
    let pixels: image::FlatSamples<&[u8]> = image_buffer.as_flat_samples();
    Ok(ColorImage::from_rgba_unmultiplied(size, pixels.as_slice()))
}

pub fn load_icon(icon_bytes: &[u8]) -> Option<epi::IconData> {
    if let Ok(image) = image::load_from_memory(icon_bytes) {
        let image = image.to_rgba8();
        let (width, height) = image.dimensions();
        Some(epi::IconData {
            width,
            height,
            rgba: image.as_raw().to_vec(),
        })
    } else {
        None
    }
}

pub fn place_total_header(
    raid: Raid,
    _item: Item,
    _chest: ChestType,
    settings: &AppSettings,
    ui: &mut Ui,
) {
    let mut total_drops_of_item = settings
        .droplog
        .drop
        .iter()
        .filter(|x| x.raid == raid && x.chest != ChestType::Host && x.chest != ChestType::Flip)
        .count();
    if raid == Raid::UBHL || raid == Raid::Xeno {
        total_drops_of_item = settings
            .droplog
            .drop
            .iter()
            .filter(|x| {
                x.raid == Raid::UBHL
                    || x.raid == Raid::Xeno
                    || (x.raid == Raid::PBHL && x.chest == ChestType::Host)
                    || x.raid == Raid::HLQL
                    || x.raid == Raid::Huanglong
                    || x.raid == Raid::Qilin
            })
            .count();
    }
    let raid_heading: String = match raid {
        Raid::Akasha => "Akasha - ".to_string(),
        Raid::PBHL => "PBHL - ".to_string(),
        Raid::GOHL => "GOHL - ".to_string(),
        Raid::UBHL => "Hosts - ".to_string(),
        Raid::Xeno => "Hosts - ".to_string(),
        Raid::Huanglong => "Hosts - ".to_string(),
        Raid::Qilin => "Hosts - ".to_string(),
        Raid::HLQL => "Hosts - ".to_string(),
        Raid::None => "".to_string(),
    };
    ui.add_space(20.);
    ui.heading(raid_heading + &total_drops_of_item.to_string());
    ui.add_space(5.);
}

pub fn place_percentage_label(
    raid: Raid,
    item: Item,
    chest: ChestType,
    settings: &AppSettings,
    ui: &mut Ui,
) {
    let mut total_drops_of_item = settings
        .droplog
        .drop
        .iter()
        .filter(|x| x.raid == raid && x.chest != ChestType::Host && x.chest != ChestType::Flip)
        .count();
    if raid != Raid::Akasha && raid != Raid::GOHL
        || (raid == Raid::PBHL && chest == ChestType::Host)
    {
        total_drops_of_item = settings
            .droplog
            .drop
            .iter()
            .filter(|x| {
                x.raid != Raid::Akasha && x.raid != Raid::GOHL
                    || (x.raid == Raid::PBHL
                        && x.chest != ChestType::Blue
                        && x.chest != ChestType::None)
            })
            .count();
    }
    let no_drop_count = settings
        .droplog
        .drop
        .iter()
        .filter(|x| x.item == Item::NoDrop && x.raid == raid)
        .count();
    let mut label_text: String = match item {
        Item::NoDrop => "No Drop: ".to_string(),
        Item::HollowKey => "Hollow Key: ".to_string(),
        Item::VerdantAzurite => "Verdant Azurite: ".to_string(),
        Item::SilverCentrum => "Silver Centrum: ".to_string(),
        Item::GoldBrick => "Gold Brick: ".to_string(),
        Item::ChampionMerit => "Champion Merit: ".to_string(),
        Item::SupremeMerit => "Supreme Merit: ".to_string(),
        Item::LegendaryMerit => "Legendary Merit: ".to_string(),
        Item::CoronationRing => "Coronation Ring: ".to_string(),
        Item::LineageRing => "Lineage Ring: ".to_string(),
        Item::IntricacyRing => "Intricacy Ring: ".to_string(),
        Item::WeaponPlusMark1 => "+1 Weapon Mark: ".to_string(),
        Item::WeaponPlusMark2 => "+2 Weapon Mark: ".to_string(),
        Item::WeaponPlusMark3 => "+3 Weapon Mark: ".to_string(),
    };
    if (raid == Raid::PBHL && chest == ChestType::Host)
        || raid == Raid::Huanglong
        || raid == Raid::Qilin
        || raid == Raid::HLQL
        || (raid == Raid::UBHL && chest == ChestType::Host || chest == ChestType::Flip)
            && raid != Raid::Akasha
            && raid != Raid::GOHL
    {
        label_text = format!("{} {} {}", raid, chest, ": ");
    }
    let items_dropped = settings
        .droplog
        .drop
        .iter()
        .filter(|x| x.item == item && x.raid == raid && x.chest == chest)
        .count();

    if settings.app_settings.droprate_by_kills {
        use format_num::NumberFormat;
        let mut drop_percent_rate = format!(
            " ({})",
            NumberFormat::new().format(".2%", items_dropped as f32 / total_drops_of_item as f32)
        );
        if items_dropped == 0 {
            drop_percent_rate = "".to_string();
        }
        ui.label(label_text + &items_dropped.to_string() + &drop_percent_rate);
    } else if !settings.app_settings.droprate_by_kills && chest != ChestType::None {
        use format_num::NumberFormat;
        let mut drop_percent_rate = format!(
            " ({})",
            NumberFormat::new().format(".2%", items_dropped as f32 / total_drops_of_item as f32)
        );
        if items_dropped == 0 {
            drop_percent_rate = "".to_string();
        }
        ui.label(label_text + &items_dropped.to_string() + &drop_percent_rate);
    } else {
        ui.label("No Drop: ".to_string() + &no_drop_count.to_string());
    }
}

pub fn place_image_button_combo(
    item: Item,
    raid: Raid,
    chest: ChestType,
    honors: &PBHLHonors,
    settings: &mut AppSettings,
    ui: &mut Ui,
) {
    let image_item_name = match item {
        Item::NoDrop => "no_drop.png",
        Item::HollowKey => "hollow_key.png",
        Item::VerdantAzurite => "verdant_azurite.png",
        Item::SilverCentrum => "silver_centrum.png",
        Item::GoldBrick => "gold_brick.png",
        Item::ChampionMerit => "champion_merit.png",
        Item::SupremeMerit => "supreme_merit.png",
        Item::LegendaryMerit => "legendary_merit.png",
        Item::CoronationRing => "coronation_ring.png",
        Item::LineageRing => "lineage_ring.png",
        Item::IntricacyRing => "intricacy_ring.png",
        Item::WeaponPlusMark1 => "weapon_plus_mark_1.png",
        Item::WeaponPlusMark2 => "weapon_plus_mark_2.png",
        Item::WeaponPlusMark3 => "weapon_plus_mark_3.png",
    };
    let mut label_text = match item {
        Item::NoDrop => "No Drop",
        Item::HollowKey => "Hollow Key",
        Item::VerdantAzurite => "Verdant Azurite",
        Item::SilverCentrum => "Silver Centrum",
        Item::GoldBrick => "Gold Brick",
        Item::ChampionMerit => "Champion Merit",
        Item::SupremeMerit => "Supreme Merit",
        Item::LegendaryMerit => "Legendary Merit",
        Item::CoronationRing => "Coronation Ring",
        Item::LineageRing => "Lineage Ring",
        Item::IntricacyRing => "Intricacy Ring",
        Item::WeaponPlusMark1 => "+1 Weapon Mark",
        Item::WeaponPlusMark2 => "+2 Weapon Mark",
        Item::WeaponPlusMark3 => "+3 Weapon Mark",
    };
    if chest == ChestType::Host {
        label_text = match raid {
            Raid::UBHL => "UBHL - Host Bar",
            Raid::PBHL => "PBHL - Host Bar",
            Raid::Huanglong => "Huanglong Bar",
            Raid::Qilin => "Qilin Bar",
            Raid::HLQL => "HLQL Bar",
            _ => "",
        };
    }
    if chest == ChestType::Flip {
        label_text = match raid {
            Raid::UBHL => "UBHL - Flip Bar",
            Raid::Xeno => "Xeno Bar",
            _ => "",
        };
    }
    let mut item_image = match item {
        Item::NoDrop => NO_BLUE_CHEST,
        Item::HollowKey => HOLLOW_KEY,
        Item::VerdantAzurite => VERDANT_AZURITE,
        Item::SilverCentrum => SILVER_CENTRUM,
        Item::GoldBrick => GOLD_BAR,
        Item::ChampionMerit => C_MERIT,
        Item::SupremeMerit => S_MERIT,
        Item::LegendaryMerit => L_MERIT,
        Item::CoronationRing => C_RING,
        Item::LineageRing => L_RING,
        Item::IntricacyRing => I_RING,
        Item::WeaponPlusMark1 => P_MARK_1,
        Item::WeaponPlusMark2 => P_MARK_2,
        Item::WeaponPlusMark3 => P_MARK_3,
    };
    if chest == ChestType::Host {
        if raid == Raid::Huanglong {
            item_image = HL_HOST_BAR;
        }
        if raid == Raid::Qilin {
            item_image = QL_HOST_BAR;
        }
        if raid == Raid::HLQL {
            item_image = HLQL_HOST_BAR;
        }
        if raid == Raid::UBHL {
            item_image = UBHL_HOST_BAR;
        }
        if raid == Raid::PBHL {
            item_image = PBHL_HOST_BAR;
        }
    }
    if chest == ChestType::Flip {
        if raid == Raid::UBHL {
            item_image = UBHL_FLIP_BAR;
        }
        if raid == Raid::Xeno {
            item_image = XENO_FLIP_BAR;
        }
    }
    ui.spacing_mut().item_spacing.x = 3.;
    let local_last_added_drop = &settings
        .droplog
        .drop
        .iter()
        .rposition(|x| x.item == item && x.raid == raid && x.chest == chest)
        .unwrap_or_default();
    if settings.app_settings.button_label_combo[1]
        && ui
            .add(CustomImageButton::new(
                &ui.ctx()
                    .load_texture(image_item_name, load_image_from_memory(item_image).unwrap()),
                (32., 32.),
            ))
            .clicked()
    {
        let shift = ui.input().modifiers.shift_only();
        if shift {
            if settings
                .droplog
                .drop
                .iter()
                .filter(|x| x.item == item && x.raid == raid && x.chest == chest)
                .count()
                > 0
            {
                let _ = settings.droplog.drop.remove(*local_last_added_drop);
            }
        } else if !shift {
            let _ = settings.droplog.drop.push(ItemDrop::new(
                settings.droplog.drop.clone().len().try_into().unwrap(),
                get_time(),
                raid,
                item,
                chest,
                Some(format!("{}", honors)),
            ));
        }
    }
    if settings.app_settings.button_label_combo[0] && ui.button(label_text).clicked() {
        let shift = ui.input().modifiers.shift_only();
        if shift {
            if settings
                .droplog
                .drop
                .iter()
                .filter(|x| x.item == item && x.raid == raid && x.chest == chest)
                .count()
                > 0
            {
                let _ = settings.droplog.drop.remove(*local_last_added_drop);
            }
        } else if !shift {
            let _ = settings.droplog.drop.push(ItemDrop::new(
                settings.droplog.drop.clone().len().try_into().unwrap(),
                get_time(),
                raid,
                item,
                chest,
                Some(format!("{}", honors)),
            ));
        }
    }
    if settings.app_settings.active_items_2[26] {
        let drop_count = settings
            .droplog
            .drop
            .iter()
            .filter(|x| x.item == item && x.raid == raid && x.chest == chest)
            .count();
        ui.label("x".to_string() + &drop_count.to_string());
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub fn create_path(path: &str) -> std::io::Result<()> {
    fs::create_dir(path)?;
    Ok(())
}

#[cfg(not(target_arch = "wasm32"))]
pub fn export(droplog: DropLog) -> Result<(), Box<dyn Error>> {
    let logged_drops = droplog.drop.len().to_string();
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
            PBHLHonors::Honors800k => write!(f, "with around 0~800k honors"),
            PBHLHonors::Honors1000k => write!(f, "with around 801~1000k honors"),
            PBHLHonors::Honors1200k => write!(f, "with around 1001~1200k honors"),
            PBHLHonors::Honors1400k => write!(f, "with around 1201~1400k honors"),
            PBHLHonors::Honors1600k => write!(f, "with around 1401~1600k honors"),
            PBHLHonors::Honors1800k => write!(f, "with around 1601~1800k honors"),
            PBHLHonors::Honors2000k => write!(f, "with around 1801~2000k or more honors"),
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
#[derive(PartialEq, Copy, Clone, Default, Debug, Serialize, Deserialize)]
pub enum Raid {
    Akasha,
    PBHL,
    GOHL,
    UBHL,
    Xeno,
    Huanglong,
    Qilin,
    HLQL,
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
            Raid::Xeno => write!(f, "Xeno"),
            Raid::Huanglong => write!(f, "Huanglong"),
            Raid::Qilin => write!(f, "Qilin"),
            Raid::HLQL => write!(f, "HLQL"),
            Raid::None => write!(f, ""),
        }
    }
}

#[derive(PartialEq, Copy, Clone, Default, Debug, Serialize, Deserialize)]
pub enum ChestType {
    Host,
    Mvp,
    Flip,
    None,
    #[default]
    Blue,
}

impl fmt::Display for ChestType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            ChestType::Host => write!(f, "Host Chest"),
            ChestType::Mvp => write!(f, "MVP Chest"),
            ChestType::Flip => write!(f, "Flip Chest"),
            ChestType::None => write!(f, ""),
            ChestType::Blue => write!(f, "Blue Chest"),
        }
    }
}

#[derive(PartialEq, Copy, Clone, Default, Debug, Serialize, Deserialize)]
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
            Item::WeaponPlusMark1 => write!(f, "+1 Weapon Mark"),
            Item::WeaponPlusMark2 => write!(f, "+2 Weapon Mark"),
            Item::WeaponPlusMark3 => write!(f, "+3 Weapon Mark"),
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
    chest: ChestType,
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
        chest: ChestType,
        honors: Option<String>,
    ) -> Self {
        Self {
            drop_id,
            date_obtained,
            raid,
            item,
            chest,
            honors,
        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DorothyConfig {
    pub dark_mode: bool,
    pub left_panel_visible: bool,
    pub right_panel_visible: bool,
    pub move_right_to_bottom: bool,
    pub always_on_top: bool,
    pub reset_on_export: bool,
    pub droprate_by_kills: bool,
    pub show_all_drops: bool,
    pub toggle_active_items: bool,
    pub vertical_grid: bool,
    pub auto_update_enabled: bool,
    pub auto_update_status: u8,
    pub grid_spacing_x: f32,
    pub grid_spacing_y: f32,
    pub font_size: f32,
    pub active_items: [bool; 32],
    pub active_items_2: [bool; 32],
    pub button_label_combo: [bool; 2],
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
            move_right_to_bottom: false,
            always_on_top: false,
            reset_on_export: true,
            droprate_by_kills: false,
            show_all_drops: false,
            toggle_active_items: false,
            vertical_grid: false,
            auto_update_enabled: true,
            auto_update_status: 0,
            grid_spacing_x: 10.,
            grid_spacing_y: 20.,
            font_size: 14.,
            active_items: [true; 32],
            active_items_2: [true; 32],
            button_label_combo: [true; 2],
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
            move_right_to_bottom: false,
            always_on_top: false,
            reset_on_export: true,
            droprate_by_kills: false,
            show_all_drops: false,
            toggle_active_items: false,
            vertical_grid: false,
            auto_update_enabled: true,
            auto_update_status: 0,
            grid_spacing_x: 10.,
            grid_spacing_y: 20.,
            font_size: 14.,
            active_items: [true; 32],
            active_items_2: [true; 32],
            button_label_combo: [true; 2],
            current_ui_tab: UiTab::Akasha,
            crystals_amount: "0".to_string(),
            ten_pulls_amount: "0".to_string(),
            single_pulls_amount: "0".to_string(),
            total_pulls: "".to_string(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct CustomImageButton {
    image: widgets::Image,
    sense: Sense,
    frame: bool,
    selected: bool,
}

impl CustomImageButton {
    pub fn new(texture_id: impl Into<TextureId>, size: impl Into<Vec2>) -> Self {
        Self {
            image: widgets::Image::new(texture_id, size),
            sense: Sense::click(),
            frame: false,
            selected: false,
        }
    }

    /// If `true`, mark this button as "selected".
    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    /// Turn off the frame
    pub fn frame(mut self, frame: bool) -> Self {
        self.frame = frame;
        self
    }

    /// By default, buttons senses clicks.
    /// Change this to a drag-button with `Sense::drag()`.
    pub fn sense(mut self, sense: Sense) -> Self {
        self.sense = sense;
        self
    }
}

impl Widget for CustomImageButton {
    fn ui(self, ui: &mut Ui) -> Response {
        let Self {
            image,
            sense,
            frame,
            selected,
        } = self;

        let padding = if frame {
            // so we can see that it is a button:
            Vec2::splat(ui.spacing().button_padding.x)
        } else {
            Vec2::ZERO
        };
        let padded_size = image.size() + 2.0 * padding;
        let (rect, response) = ui.allocate_exact_size(padded_size, sense);
        response.widget_info(|| WidgetInfo::new(WidgetType::ImageButton));

        if ui.is_rect_visible(rect) {
            let (expansion, rounding, fill, stroke) = if selected {
                let selection = ui.visuals().selection;
                (
                    -padding,
                    Rounding::none(),
                    selection.bg_fill,
                    selection.stroke,
                )
            } else if frame {
                let visuals = ui.style().interact(&response);
                let expansion = if response.hovered() {
                    Vec2::splat(visuals.expansion) - padding
                } else {
                    Vec2::splat(visuals.expansion)
                };
                (
                    expansion,
                    visuals.rounding,
                    visuals.bg_fill,
                    visuals.bg_stroke,
                )
            } else {
                Default::default()
            };

            // Draw frame background (for transparent images):
            ui.painter()
                .rect_filled(rect.expand2(expansion), rounding, fill);

            let image_rect = ui
                .layout()
                .align_size_within_rect(image.size(), rect.shrink2(padding));
            // let image_rect = image_rect.expand2(expansion); // can make it blurry, so let's not
            image.paint_at(ui, image_rect);

            // Draw frame outline:
            ui.painter()
                .rect_stroke(rect.expand2(expansion), rounding, stroke);
        }

        response
    }
}

#[cfg(target_arch = "wasm32")]
use crate::app::AppDorothy;
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
