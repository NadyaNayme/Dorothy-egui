use std::{
    f32::INFINITY,
    fmt::{self, Display},
};

use chrono::{DateTime, Local};
use eframe::{
    egui::{self, RichText, Visuals},
    epaint::Color32,
    epi, NativeOptions,
};
use image;

#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "persistence", serde(default))]

pub fn load_icon(icon_bytes: &Vec<u8>) -> Option<epi::IconData> {
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

fn get_percentage(x: f32, y: f32) -> String {
    let result = (x * 100.0) / y;
    return format!("{:.2}%", result);
}

#[derive(PartialEq)]
enum PBHLHonors {
    Ignore,
    Honors800k,
    Honors1000k,
    Honors1200k,
    Honors1400k,
    Honors1600k,
    Honors1800k,
    Honors2000k,
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

#[derive(PartialEq)]
enum UiTab {
    Pulls,
    Akasha,
    PBHL,
    GOHL,
    Hosts,
}
#[derive(PartialEq)]
enum Raid {
    Akasha,
    PBHL,
    GOHL,
    UBHL,
    Xeno,
}

impl fmt::Display for Raid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Raid::Akasha => write!(f, "Akasha"),
            Raid::PBHL => write!(f, "PBHL"),
            Raid::GOHL => write!(f, "GOHL"),
            Raid::UBHL => write!(f, "UBHL"),
            Raid::Xeno => write!(f, "Xeno Showdown"),
        }
    }
}

#[derive(PartialEq)]
enum Item {
    NoDrop,
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

#[derive(Default)]
struct DropLog {
    drop: Vec<ItemDrop>,
}

struct ItemDrop {
    date_obtained: String,
    raid: Raid,
    item: Item,
    honors: Option<String>,
}

impl ItemDrop {
    fn new(date_obtained: String, raid: Raid, item: Item, honors: Option<String>) -> Self {
        Self {
            date_obtained,
            raid,
            item,
            honors,
        }
    }
}

pub struct Dorothy {
    name: String,
    config: DorothyConfig,
    droplog: DropLog,
    pbhl_honors: PBHLHonors,
}

struct DorothyConfig {
    dark_mode: bool,
    left_panel_visible: bool,
    right_panel_visible: bool,
    always_on_top: bool,
    reset_on_export: bool,
    droprate_by_kills: bool,
    show_all_drops: bool,
    current_ui_tab: UiTab,
    crystals_amount: String,
    ten_pulls_amount: String,
    single_pulls_amount: String,
    total_pulls: String,
}

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

impl Dorothy {
    pub fn new() -> Self {
        Self {
            name: "Dorothy".to_string(),
            config: DorothyConfig::new(),
            droplog: DropLog::default(),
            pbhl_honors: PBHLHonors::Ignore,
        }
    }
    fn icon_data(&self) -> Option<epi::IconData> {
        let icon_bytes = include_bytes!("./images/dorothy.ico");
        load_icon(&icon_bytes.to_vec())
    }
}

impl epi::App for Dorothy {
    fn name(&self) -> &str {
        "Dorothy"
    }

    fn setup(
        &mut self,
        _ctx: &egui::Context,
        _frame: &epi::Frame,
        _storage: Option<&dyn epi::Storage>,
    ) {
        // Load previous app state (if any).
        #[cfg(feature = "persistence")]
        if let Some(storage) = _storage {
            *self = epi::get_value(storage, epi::APP_KEY).unwrap_or_default()
        }
    }

    /// Called by the framework to save state before shutdown.
    #[cfg(feature = "persistence")]
    fn save(&mut self, storage: &mut dyn epi::Storage) {
        epi::set_value(storage, epi::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, frame: &epi::Frame) {
        let Self {
            name,
            config,
            droplog,
            pbhl_honors: PBHLHonors,
        } = self;

        if self.config.dark_mode {
            ctx.set_visuals(Visuals::dark());
        } else {
            ctx.set_visuals(Visuals::light());
        }

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Export").clicked() {
                        ();
                    }
                    if ui.button("Quit").clicked() {
                        frame.quit();
                    }
                });
                ui.menu_button("View", |ui| {
                    if ui
                        .checkbox(&mut self.config.left_panel_visible, "Show Left Panel")
                        .clicked()
                    {}
                    if ui
                        .checkbox(&mut self.config.right_panel_visible, "Show Right Panel")
                        .clicked()
                    {}
                    if ui
                        .checkbox(&mut self.config.show_all_drops, "Show All Raid Drops")
                        .clicked()
                    {}
                });
                ui.menu_button("Settings", |ui| {
                    ui.style_mut().wrap = Some(false);
                    if ui
                        .checkbox(&mut self.config.dark_mode, "Dark Mode")
                        .clicked()
                    {}
                    if ui
                        .checkbox(&mut self.config.always_on_top, "Always On Top")
                        .clicked()
                    {}
                    if ui
                        .checkbox(&mut self.config.reset_on_export, "Reset Counts on Export")
                        .clicked()
                    {}
                    if ui
                        .checkbox(
                            &mut self.config.droprate_by_kills,
                            "Calculate droprates by total kills",
                        )
                        .clicked()
                    {}
                });
                ui.menu_button("Helpful Links", |ui| {
                    ui.style_mut().wrap = Some(false);
                    if ui.button("Latest Dorothy Release (github.com)").clicked() {
                        let url = "https://github.com/NadyaNayme/Dorothy/releases/latest";
                        let _ = open::that(&url).unwrap();
                    }
                    if ui.button("GBF Wiki (gbf.wiki)").clicked() {
                        let url = "https://gbf.wiki/Main_Page";
                        let _ = open::that(&url).unwrap();
                    }
                    if ui.button("Online Tools (granblue.party)").clicked() {
                        let url = "https://www.granblue.party/";
                        let _ = open::that(&url).unwrap();
                    }
                    if ui.button("Raidfinder (gbf.life)").clicked() {
                        let url = "https://gbf.life/";
                        let _ = open::that(&url).unwrap();
                    }
                    if ui.button("/r/Granblue_en (reddit.com)").clicked() {
                        let url = "https://www.reddit.com/r/Granblue_en/";
                        let _ = open::that(&url).unwrap();
                    }
                });
            });
        });
        if self.config.left_panel_visible {
            egui::SidePanel::left("left_side_panel")
                .min_width(250.)
                .max_width(300.)
                .show(ctx, |ui| {
                    ui.add_space(20.);
                    if ui.button("Export to .csv").clicked() {
                        ();
                    }

                    ui.heading("Drop Totals");
                    ui.add_space(3.);
                    ui.separator();
                    ui.add_space(5.);
                    egui::ScrollArea::vertical()
                        .auto_shrink([false, false])
                        .max_height(INFINITY)
                        .max_width(INFINITY)
                        .show(ui, |ui| {
                            if self.config.current_ui_tab == UiTab::Akasha
                                || self.config.show_all_drops
                            {
                                ui.add_space(20.);
                                let total_akasha_drops = self
                                    .droplog
                                    .drop
                                    .iter()
                                    .filter(|x| x.raid == Raid::Akasha)
                                    .count()
                                    .to_string();
                                ui.heading("Akasha - ".to_owned() + &total_akasha_drops);
                                ui.add_space(5.);

                                let no_drop = self
                                    .droplog
                                    .drop
                                    .iter()
                                    .filter(|x| x.item == Item::NoDrop && x.raid == Raid::Akasha)
                                    .count()
                                    .to_string();
                                ui.label("No Drop: ".to_string() + &no_drop);
                                let hollow_key_drops = self
                                    .droplog
                                    .drop
                                    .iter()
                                    .filter(|x| x.item == Item::HollowKey && x.raid == Raid::Akasha)
                                    .count()
                                    .to_string();
                                ui.label("Hollow Key: ".to_string() + &hollow_key_drops);
                                let champion_merit_drops = self
                                    .droplog
                                    .drop
                                    .iter()
                                    .filter(|x| {
                                        x.item == Item::ChampionMerit && x.raid == Raid::Akasha
                                    })
                                    .count()
                                    .to_string();
                                ui.label("Champion Merit: ".to_string() + &champion_merit_drops);
                                let supreme_merit_drops = self
                                    .droplog
                                    .drop
                                    .iter()
                                    .filter(|x| {
                                        x.item == Item::SupremeMerit && x.raid == Raid::Akasha
                                    })
                                    .count()
                                    .to_string();
                                ui.label("Supreme Merit: ".to_string() + &supreme_merit_drops);
                                let legendary_merit_drops = self
                                    .droplog
                                    .drop
                                    .iter()
                                    .filter(|x| {
                                        x.item == Item::LegendaryMerit && x.raid == Raid::Akasha
                                    })
                                    .count()
                                    .to_string();
                                ui.label("Legendary Merit: ".to_string() + &legendary_merit_drops);
                                let silver_centrum_drops = self
                                    .droplog
                                    .drop
                                    .iter()
                                    .filter(|x| {
                                        x.item == Item::SilverCentrum && x.raid == Raid::Akasha
                                    })
                                    .count()
                                    .to_string();
                                ui.label("Silver Centrum: ".to_string() + &silver_centrum_drops);
                                let weapon_plus_mark_1_drops = self
                                    .droplog
                                    .drop
                                    .iter()
                                    .filter(|x| {
                                        x.item == Item::WeaponPlusMark1 && x.raid == Raid::Akasha
                                    })
                                    .count()
                                    .to_string();
                                ui.label(
                                    "Weapon Plus Mark +1: ".to_string() + &weapon_plus_mark_1_drops,
                                );
                                let weapon_plus_mark_2_drops = self
                                    .droplog
                                    .drop
                                    .iter()
                                    .filter(|x| {
                                        x.item == Item::WeaponPlusMark2 && x.raid == Raid::Akasha
                                    })
                                    .count()
                                    .to_string();
                                ui.label(
                                    "Weapon Plus Mark +2: ".to_string() + &weapon_plus_mark_2_drops,
                                );
                                let weapon_plus_mark_3_drops = self
                                    .droplog
                                    .drop
                                    .iter()
                                    .filter(|x| {
                                        x.item == Item::WeaponPlusMark3 && x.raid == Raid::Akasha
                                    })
                                    .count()
                                    .to_string();
                                ui.label(
                                    "Weapon Plus Mark +3: ".to_string() + &weapon_plus_mark_3_drops,
                                );
                                let coronation_ring_drops = self
                                    .droplog
                                    .drop
                                    .iter()
                                    .filter(|x| {
                                        x.item == Item::CoronationRing && x.raid == Raid::Akasha
                                    })
                                    .count()
                                    .to_string();
                                ui.label("Coronation Ring: ".to_string() + &coronation_ring_drops);
                                let lineage_ring_drops = self
                                    .droplog
                                    .drop
                                    .iter()
                                    .filter(|x| {
                                        x.item == Item::LineageRing && x.raid == Raid::Akasha
                                    })
                                    .count()
                                    .to_string();
                                ui.label("Lineage Ring: ".to_string() + &lineage_ring_drops);
                                let intricacy_ring_drops = self
                                    .droplog
                                    .drop
                                    .iter()
                                    .filter(|x| {
                                        x.item == Item::IntricacyRing && x.raid == Raid::Akasha
                                    })
                                    .count()
                                    .to_string();
                                ui.label("Intricacy Ring: ".to_string() + &intricacy_ring_drops);
                                let gold_brick_drops = self
                                    .droplog
                                    .drop
                                    .iter()
                                    .filter(|x| x.item == Item::GoldBrick && x.raid == Raid::Akasha)
                                    .count()
                                    .to_string();
                                ui.label("Gold Brick: ".to_string() + &gold_brick_drops);
                            }
                            if self.config.current_ui_tab == UiTab::PBHL
                                || self.config.show_all_drops
                            {
                                ui.add_space(20.);
                                let total_pbhl_drops = self
                                    .droplog
                                    .drop
                                    .iter()
                                    .filter(|x| x.raid == Raid::PBHL)
                                    .count()
                                    .to_string();
                                ui.heading("PBHL - ".to_owned() + &total_pbhl_drops);
                                ui.add_space(5.);

                                let no_drop = self
                                    .droplog
                                    .drop
                                    .iter()
                                    .filter(|x| x.item == Item::NoDrop && x.raid == Raid::PBHL)
                                    .count()
                                    .to_string();
                                ui.label("No Drop: ".to_string() + &no_drop);
                                let coronation_ring_drops = self
                                    .droplog
                                    .drop
                                    .iter()
                                    .filter(|x| {
                                        x.item == Item::CoronationRing && x.raid == Raid::PBHL
                                    })
                                    .count()
                                    .to_string();
                                ui.label("Coronation Ring: ".to_string() + &coronation_ring_drops);
                                let lineage_ring_drops = self
                                    .droplog
                                    .drop
                                    .iter()
                                    .filter(|x| x.item == Item::LineageRing && x.raid == Raid::PBHL)
                                    .count()
                                    .to_string();
                                ui.label("Lineage Ring: ".to_string() + &lineage_ring_drops);
                                let intricacy_ring_drops = self
                                    .droplog
                                    .drop
                                    .iter()
                                    .filter(|x| {
                                        x.item == Item::IntricacyRing && x.raid == Raid::PBHL
                                    })
                                    .count()
                                    .to_string();
                                ui.label("Intricacy Ring: ".to_string() + &intricacy_ring_drops);
                                let gold_brick_drops = self
                                    .droplog
                                    .drop
                                    .iter()
                                    .filter(|x| x.item == Item::GoldBrick && x.raid == Raid::PBHL)
                                    .count()
                                    .to_string();
                                ui.label("Gold Brick: ".to_string() + &gold_brick_drops);
                            }
                            if self.config.current_ui_tab == UiTab::GOHL
                                || self.config.show_all_drops
                            {
                                ui.add_space(20.);
                                let total_gohl_drops = self
                                    .droplog
                                    .drop
                                    .iter()
                                    .filter(|x| x.raid == Raid::GOHL)
                                    .count()
                                    .to_string();
                                ui.heading("GOHL - ".to_owned() + &total_gohl_drops);
                                ui.add_space(5.);
                                let no_drop = self
                                    .droplog
                                    .drop
                                    .iter()
                                    .filter(|x| x.item == Item::NoDrop && x.raid == Raid::GOHL)
                                    .count()
                                    .to_string();
                                ui.label("No Drop: ".to_string() + &no_drop);
                                let verdant_azurite_drops = self
                                    .droplog
                                    .drop
                                    .iter()
                                    .filter(|x| {
                                        x.item == Item::VerdantAzurite && x.raid == Raid::GOHL
                                    })
                                    .count()
                                    .to_string();
                                ui.label("Verdant Azurite: ".to_string() + &verdant_azurite_drops);
                                let champion_merit_drops = self
                                    .droplog
                                    .drop
                                    .iter()
                                    .filter(|x| {
                                        x.item == Item::ChampionMerit && x.raid == Raid::GOHL
                                    })
                                    .count()
                                    .to_string();
                                ui.label("Champion Merit: ".to_string() + &champion_merit_drops);
                                let supreme_merit_drops = self
                                    .droplog
                                    .drop
                                    .iter()
                                    .filter(|x| {
                                        x.item == Item::SupremeMerit && x.raid == Raid::GOHL
                                    })
                                    .count()
                                    .to_string();
                                ui.label("Supreme Merit: ".to_string() + &supreme_merit_drops);
                                let legendary_merit_drops = self
                                    .droplog
                                    .drop
                                    .iter()
                                    .filter(|x| {
                                        x.item == Item::LegendaryMerit && x.raid == Raid::GOHL
                                    })
                                    .count()
                                    .to_string();
                                ui.label("Legendary Merit: ".to_string() + &legendary_merit_drops);
                                let silver_centrum_drops = self
                                    .droplog
                                    .drop
                                    .iter()
                                    .filter(|x| {
                                        x.item == Item::SilverCentrum && x.raid == Raid::GOHL
                                    })
                                    .count()
                                    .to_string();
                                ui.label("Silver Centrum: ".to_string() + &silver_centrum_drops);
                                let coronation_ring_drops = self
                                    .droplog
                                    .drop
                                    .iter()
                                    .filter(|x| {
                                        x.item == Item::CoronationRing && x.raid == Raid::GOHL
                                    })
                                    .count()
                                    .to_string();
                                ui.label("Coronation Ring: ".to_string() + &coronation_ring_drops);
                                let lineage_ring_drops = self
                                    .droplog
                                    .drop
                                    .iter()
                                    .filter(|x| x.item == Item::LineageRing && x.raid == Raid::GOHL)
                                    .count()
                                    .to_string();
                                ui.label("Lineage Ring: ".to_string() + &lineage_ring_drops);
                                let intricacy_ring_drops = self
                                    .droplog
                                    .drop
                                    .iter()
                                    .filter(|x| {
                                        x.item == Item::IntricacyRing && x.raid == Raid::GOHL
                                    })
                                    .count()
                                    .to_string();
                                ui.label("Intricacy Ring: ".to_string() + &intricacy_ring_drops);
                                let gold_brick_drops = self
                                    .droplog
                                    .drop
                                    .iter()
                                    .filter(|x| x.item == Item::GoldBrick && x.raid == Raid::GOHL)
                                    .count()
                                    .to_string();
                                ui.label("Gold Brick: ".to_string() + &gold_brick_drops);
                            }
                            if self.config.current_ui_tab == UiTab::Hosts
                                || self.config.show_all_drops
                            {
                                ui.add_space(20.);
                                let total_hosts_drops = self
                                    .droplog
                                    .drop
                                    .iter()
                                    .filter(|x| x.raid == Raid::UBHL || x.raid == Raid::Xeno)
                                    .count()
                                    .to_string();
                                ui.heading("Hosts - ".to_owned() + &total_hosts_drops);
                                ui.add_space(5.);
                            }
                        });
                });
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.selectable_value(
                    &mut self.config.current_ui_tab,
                    UiTab::Pulls,
                    "Pull Calculator",
                );
                ui.selectable_value(&mut self.config.current_ui_tab, UiTab::Akasha, "Akasha");
                ui.selectable_value(&mut self.config.current_ui_tab, UiTab::PBHL, "PBHL");
                ui.selectable_value(&mut self.config.current_ui_tab, UiTab::GOHL, "GOHL");
                ui.selectable_value(&mut self.config.current_ui_tab, UiTab::Hosts, "Hosts");
            });

            ui.add_space(30.);
            if self.config.current_ui_tab == UiTab::Pulls {
                ui.horizontal(|ui| {
                    ui.label("Crystals: ");
                    let response =
                        ui.add(egui::TextEdit::singleline(&mut self.config.crystals_amount));
                    if response.changed() {
                        self.config.total_pulls = calculate_pulls(
                            self.config
                                .crystals_amount
                                .parse::<f32>()
                                .unwrap_or_default(),
                            self.config
                                .ten_pulls_amount
                                .parse::<f32>()
                                .unwrap_or_default(),
                            self.config
                                .single_pulls_amount
                                .parse::<f32>()
                                .unwrap_or_default(),
                        )
                    }
                });
                ui.add_space(5.);
                ui.horizontal(|ui| {
                    ui.label("10-Pull Tickets: ");
                    let response = ui.add(egui::TextEdit::singleline(
                        &mut self.config.ten_pulls_amount,
                    ));
                    if response.changed() {
                        self.config.total_pulls = calculate_pulls(
                            self.config
                                .crystals_amount
                                .parse::<f32>()
                                .unwrap_or_default(),
                            self.config
                                .ten_pulls_amount
                                .parse::<f32>()
                                .unwrap_or_default(),
                            self.config
                                .single_pulls_amount
                                .parse::<f32>()
                                .unwrap_or_default(),
                        )
                    }
                });
                ui.add_space(5.);
                ui.horizontal(|ui| {
                    ui.label("1-Pull Tickets: ");
                    let response = ui.add(egui::TextEdit::singleline(
                        &mut self.config.single_pulls_amount,
                    ));
                    if response.changed() {
                        self.config.total_pulls = calculate_pulls(
                            self.config
                                .crystals_amount
                                .parse::<f32>()
                                .unwrap_or_default(),
                            self.config
                                .ten_pulls_amount
                                .parse::<f32>()
                                .unwrap_or_default(),
                            self.config
                                .single_pulls_amount
                                .parse::<f32>()
                                .unwrap_or_default(),
                        )
                    }
                });
                ui.add_space(5.);
                ui.label(&self.config.total_pulls);
            }

            if self.config.current_ui_tab == UiTab::Akasha {
                egui::Grid::new("akasha_item_grid")
                    .spacing((15., 20.))
                    .show(ui, |ui| {
                        if ui.button("No Drop").clicked() {
                            let _ = &self.droplog.drop.push(ItemDrop::new(
                                get_time(),
                                Raid::Akasha,
                                Item::NoDrop,
                                Option::Some("None".to_string()),
                            ));
                        }
                        if ui.button("Hollow Key").clicked() {
                            let _ = &self.droplog.drop.push(ItemDrop::new(
                                get_time(),
                                Raid::Akasha,
                                Item::HollowKey,
                                Option::Some("None".to_string()),
                            ));
                        }
                        if ui.button("Champion Merit").clicked() {
                            let _ = &self.droplog.drop.push(ItemDrop::new(
                                get_time(),
                                Raid::Akasha,
                                Item::ChampionMerit,
                                Option::Some("None".to_string()),
                            ));
                        }
                        if ui.button("Supreme Merit").clicked() {
                            let _ = &self.droplog.drop.push(ItemDrop::new(
                                get_time(),
                                Raid::Akasha,
                                Item::SupremeMerit,
                                Option::Some("None".to_string()),
                            ));
                        }
                        if ui.button("Legendary Merit").clicked() {
                            let _ = &self.droplog.drop.push(ItemDrop::new(
                                get_time(),
                                Raid::Akasha,
                                Item::LegendaryMerit,
                                Option::Some("None".to_string()),
                            ));
                        }
                        ui.end_row();
                        if ui.button("Silver Centrum").clicked() {
                            let _ = &self.droplog.drop.push(ItemDrop::new(
                                get_time(),
                                Raid::Akasha,
                                Item::SilverCentrum,
                                Option::Some("None".to_string()),
                            ));
                        }
                        if ui.button("Weapon Plus Mark +1").clicked() {
                            let _ = &self.droplog.drop.push(ItemDrop::new(
                                get_time(),
                                Raid::Akasha,
                                Item::WeaponPlusMark1,
                                Option::Some("None".to_string()),
                            ));
                        }
                        if ui.button("Weapon Plus Mark +2").clicked() {
                            let _ = &self.droplog.drop.push(ItemDrop::new(
                                get_time(),
                                Raid::Akasha,
                                Item::WeaponPlusMark2,
                                Option::Some("None".to_string()),
                            ));
                        }
                        if ui.button("Weapon Plus Mark +3").clicked() {
                            let _ = &self.droplog.drop.push(ItemDrop::new(
                                get_time(),
                                Raid::Akasha,
                                Item::WeaponPlusMark3,
                                Option::Some("None".to_string()),
                            ));
                        }
                        ui.end_row();
                        if ui.button("Coronation Ring").clicked() {
                            let _ = &self.droplog.drop.push(ItemDrop::new(
                                get_time(),
                                Raid::Akasha,
                                Item::CoronationRing,
                                Option::Some("None".to_string()),
                            ));
                        }
                        if ui.button("Lineage Ring").clicked() {
                            let _ = &self.droplog.drop.push(ItemDrop::new(
                                get_time(),
                                Raid::Akasha,
                                Item::LineageRing,
                                Option::Some("None".to_string()),
                            ));
                        }
                        if ui.button("Intricacy Ring").clicked() {
                            let _ = &self.droplog.drop.push(ItemDrop::new(
                                get_time(),
                                Raid::Akasha,
                                Item::IntricacyRing,
                                Option::Some("None".to_string()),
                            ));
                        }
                        if ui.button("Gold Brick").clicked() {
                            let _ = &self.droplog.drop.push(ItemDrop::new(
                                get_time(),
                                Raid::Akasha,
                                Item::GoldBrick,
                                Option::Some("None".to_string()),
                            ));
                        }
                        ui.end_row();
                    });
            }
            if self.config.current_ui_tab == UiTab::PBHL {
                egui::Grid::new("pbhl_item_grid")
                    .spacing((15., 20.))
                    .show(ui, |ui| {
                        if ui.button("No Drop").clicked() {
                            let _ = &self.droplog.drop.push(ItemDrop::new(
                                get_time(),
                                Raid::PBHL,
                                Item::NoDrop,
                                Option::Some("None".to_string()),
                            ));
                        }
                        if ui.button("Coronation Ring").clicked() {
                            let _ = &self.droplog.drop.push(ItemDrop::new(
                                get_time(),
                                Raid::PBHL,
                                Item::CoronationRing,
                                Option::Some(format!("{}", self.pbhl_honors)),
                            ));
                        }
                        if ui.button("Lineage Ring").clicked() {
                            let _ = &self.droplog.drop.push(ItemDrop::new(
                                get_time(),
                                Raid::PBHL,
                                Item::LineageRing,
                                Option::Some(format!("{}", self.pbhl_honors)),
                            ));
                        }
                        if ui.button("Intricacy Ring").clicked() {
                            let _ = &self.droplog.drop.push(ItemDrop::new(
                                get_time(),
                                Raid::PBHL,
                                Item::IntricacyRing,
                                Option::Some(format!("{}", self.pbhl_honors)),
                            ));
                        }
                        if ui.button("Gold Brick").clicked() {
                            let _ = &self.droplog.drop.push(ItemDrop::new(
                                get_time(),
                                Raid::PBHL,
                                Item::GoldBrick,
                                Option::Some(format!("{}", self.pbhl_honors)),
                            ));
                        }
                        ui.end_row();
                    });

                ui.add_space(20.);
                ui.heading("Honors");
                ui.label("Select the closest match rounding down.");
                ui.add_space(5.);

                egui::Grid::new("pbhl_honors_grid")
                    .spacing((15., 10.))
                    .show(ui, |ui| {
                        ui.selectable_value(
                            &mut self.pbhl_honors,
                            PBHLHonors::Ignore,
                            "Don't Care",
                        );
                        ui.selectable_value(
                            &mut self.pbhl_honors,
                            PBHLHonors::Honors800k,
                            "800k or less",
                        );
                        ui.selectable_value(
                            &mut self.pbhl_honors,
                            PBHLHonors::Honors1000k,
                            "1000k",
                        );
                        ui.selectable_value(
                            &mut self.pbhl_honors,
                            PBHLHonors::Honors1200k,
                            "1200k",
                        );
                        ui.end_row();
                        ui.selectable_value(
                            &mut self.pbhl_honors,
                            PBHLHonors::Honors1400k,
                            "1400k",
                        );
                        ui.selectable_value(
                            &mut self.pbhl_honors,
                            PBHLHonors::Honors1600k,
                            "1600k",
                        );
                        ui.selectable_value(
                            &mut self.pbhl_honors,
                            PBHLHonors::Honors1800k,
                            "1800k",
                        );
                        ui.selectable_value(
                            &mut self.pbhl_honors,
                            PBHLHonors::Honors2000k,
                            "2000k or more",
                        );
                    });
            }
            if self.config.current_ui_tab == UiTab::GOHL {
                egui::Grid::new("gohl_item_grid")
                    .spacing((15., 20.))
                    .show(ui, |ui| {
                        if ui.button("No Drop").clicked() {
                            let _ = &self.droplog.drop.push(ItemDrop::new(
                                get_time(),
                                Raid::GOHL,
                                Item::NoDrop,
                                Option::Some("None".to_string()),
                            ));
                        }
                        if ui.button("Verdant Azurite").clicked() {
                            let _ = &self.droplog.drop.push(ItemDrop::new(
                                get_time(),
                                Raid::GOHL,
                                Item::VerdantAzurite,
                                Option::Some("None".to_string()),
                            ));
                        }
                        if ui.button("Champion Merit").clicked() {
                            let _ = &self.droplog.drop.push(ItemDrop::new(
                                get_time(),
                                Raid::GOHL,
                                Item::ChampionMerit,
                                Option::Some("None".to_string()),
                            ));
                        }
                        if ui.button("Supreme Merit").clicked() {
                            let _ = &self.droplog.drop.push(ItemDrop::new(
                                get_time(),
                                Raid::GOHL,
                                Item::SupremeMerit,
                                Option::Some("None".to_string()),
                            ));
                        }
                        if ui.button("Legendary Merit").clicked() {
                            let _ = &self.droplog.drop.push(ItemDrop::new(
                                get_time(),
                                Raid::GOHL,
                                Item::LegendaryMerit,
                                Option::Some("None".to_string()),
                            ));
                        }
                        ui.end_row();
                        if ui.button("Silver Centrum").clicked() {
                            let _ = &self.droplog.drop.push(ItemDrop::new(
                                get_time(),
                                Raid::GOHL,
                                Item::SilverCentrum,
                                Option::Some("None".to_string()),
                            ));
                        }
                        if ui.button("Coronation Ring").clicked() {
                            let _ = &self.droplog.drop.push(ItemDrop::new(
                                get_time(),
                                Raid::GOHL,
                                Item::CoronationRing,
                                Option::Some("None".to_string()),
                            ));
                        }
                        if ui.button("Lineage Ring").clicked() {
                            let _ = &self.droplog.drop.push(ItemDrop::new(
                                get_time(),
                                Raid::GOHL,
                                Item::LineageRing,
                                Option::Some("None".to_string()),
                            ));
                        }
                        if ui.button("Intricacy Ring").clicked() {
                            let _ = &self.droplog.drop.push(ItemDrop::new(
                                get_time(),
                                Raid::GOHL,
                                Item::IntricacyRing,
                                Option::Some("None".to_string()),
                            ));
                        }
                        if ui.button("Gold Brick").clicked() {
                            let _ = &self.droplog.drop.push(ItemDrop::new(
                                get_time(),
                                Raid::GOHL,
                                Item::GoldBrick,
                                Option::Some("None".to_string()),
                            ));
                        }
                        ui.end_row();
                    });
            }
            ui.add_space(50.);
            egui::warn_if_debug_build(ui);
        });

        if self.config.right_panel_visible {
            egui::SidePanel::right("right_side_panel")
                .min_width(250.)
                .max_width(300.)
                .show(ctx, |ui| {
                    ui.heading("Recent Drops");
                    ui.add_space(5.);

                    egui::ScrollArea::vertical()
                        .auto_shrink([false, false])
                        .max_height(INFINITY)
                        .max_width(INFINITY)
                        .show(ui, |ui| {
                            for drop in self
                                .droplog
                                .drop
                                .iter()
                                .filter(|x| x.item != Item::NoDrop)
                                .rev()
                            {
                                ui.horizontal(|ui| {
                                    ui.spacing_mut().item_spacing.x = 0.0;
                                    if drop.item == Item::GoldBrick && drop.raid != Raid::PBHL {
                                        let mut gold_brick_text_color = Color32::from_rgb(255, 221, 26);
                                        if self.config.dark_mode == false {
                                            gold_brick_text_color = Color32::from_rgb(187, 152, 10);
                                        }
                                        ui.label(
                                            RichText::new(format!("{}", drop.item))
                                                .color(gold_brick_text_color),
                                        )
                                        .on_hover_text(
                                            format!("On {} from {}", drop.date_obtained, drop.raid),
                                        );
                                        ui.add_space(3.)
                                    } else if drop.item == Item::GoldBrick
                                        && drop.raid == Raid::PBHL
                                    {
                                        let mut gold_brick_text_color = Color32::from_rgb(255, 221, 26);
                                        if self.config.dark_mode == false {
                                            gold_brick_text_color = Color32::from_rgb(183, 138, 15);
                                        }
                                        ui.label(
                                            RichText::new(format!("{}", drop.item))
                                                .color(gold_brick_text_color),
                                        )
                                        .on_hover_text(
                                            format!(
                                                "On {} from {} with around {} honors",
                                                drop.date_obtained,
                                                drop.raid,
                                                drop.honors.as_ref().unwrap()
                                            ),
                                        );
                                        ui.add_space(3.)
                                    } else if drop.raid == Raid::PBHL {
                                        ui.label(format!("{}", drop.item)).on_hover_text(format!(
                                            "On {} from {} with around {} honors",
                                            drop.date_obtained,
                                            drop.raid,
                                            drop.honors.as_ref().unwrap()
                                        ));
                                        ui.add_space(3.)
                                    } else {
                                        ui.label(format!("{}", drop.item)).on_hover_text(format!(
                                            "On {} from {}",
                                            drop.date_obtained, drop.raid
                                        ));
                                        ui.add_space(3.)
                                    }
                                });
                            }
                        });
                });
        }

        if false {
            egui::Window::new("Window").show(ctx, |ui| {
                ui.label("Windows can be moved by dragging them.");
                ui.label("They are automatically sized based on contents.");
                ui.label("You can turn on resizing and scrolling if you like.");
                ui.label("You would normally chose either panels OR windows.");
            });
        }
    }
}
