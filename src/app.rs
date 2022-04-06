use crate::*;
use confy;
use eframe::{
    egui::{self, RichText, Visuals},
    epaint::Color32,
    epi,
};
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "persistence", serde(default))]
#[derive(Default, Serialize, Deserialize)]
pub struct AppDorothy {
    pub name: String,
    #[serde(default)]
    pub droplog: DropLog,
    #[serde(default)]
    pub pbhl_honors: PBHLHonors,
    #[serde(default)]
    pub config: AppSettings,
}

impl AppDorothy {
    pub fn new() -> Self {
        let saved_config: AppSettings = confy::load("dorothy-egui").unwrap_or_default();
        let saved_droplog: DropLog = saved_config.droplog.clone();
        Self {
            name: "Dorothy".to_string(),
            config: saved_config,
            droplog: saved_droplog,
            pbhl_honors: PBHLHonors::Ignore,
        }
    }

    pub fn get_config() -> AppSettings {
        let saved_config: AppSettings = confy::load("dorothy-egui").unwrap_or_default();
        saved_config
    }

    pub fn save_config(config: AppSettings) -> bool {
        confy::store("dorothy-egui", config).is_ok()
    }
}

impl epi::App for AppDorothy {
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

    #[allow(unused_variables)]
    fn update(&mut self, ctx: &egui::Context, frame: &epi::Frame) {
        let Self {
            name,
            config,
            droplog,
            pbhl_honors: pbhlhonors,
        } = self;

        let mut local_settings_copy = AppDorothy::get_config();

        if self.config.app_settings.dark_mode {
            ctx.set_visuals(Visuals::dark());
            local_settings_copy.app_settings.dark_mode = true;
        } else {
            ctx.set_visuals(Visuals::light());
            local_settings_copy.app_settings.dark_mode = false;
        }

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            if ctx.is_pointer_over_area() {
                local_settings_copy.app_settings.current_ui_tab =
                    self.config.app_settings.current_ui_tab.clone();
                let _ = confy::store("dorothy-egui", &local_settings_copy);
            }

            #[cfg(target_family = "windows")]
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Export").clicked() {
                        let _ = export(local_settings_copy.droplog.clone());
                        if local_settings_copy.app_settings.reset_on_export {
                            self.config.droplog.drop = DropLog::reset();
                            local_settings_copy.droplog.drop = DropLog::reset();
                            let _ = confy::store("dorothy-egui", &local_settings_copy);
                        }
                    }
                    ui.separator();
                    if ui.button("Reset Droplog").clicked() {
                        self.config.droplog.drop = DropLog::reset();
                        local_settings_copy.droplog.drop = DropLog::reset();
                        let _ = confy::store("dorothy-egui", &local_settings_copy);
                    }
                    if ui.button("Quit & Save").clicked() {
                        let _ = confy::store("dorothy-egui", &local_settings_copy);
                        frame.quit();
                    }
                });
                ui.menu_button("View", |ui| {
                    if ui
                        .checkbox(
                            &mut self.config.app_settings.left_panel_visible,
                            "Show Left Panel",
                        )
                        .clicked()
                    {
                        local_settings_copy.app_settings.left_panel_visible =
                            self.config.app_settings.left_panel_visible;
                        let _ = confy::store("dorothy-egui", &local_settings_copy);
                    }
                    if ui
                        .checkbox(
                            &mut self.config.app_settings.right_panel_visible,
                            "Show Right Panel",
                        )
                        .clicked()
                    {
                        local_settings_copy.app_settings.right_panel_visible =
                            self.config.app_settings.right_panel_visible;
                        let _ = confy::store("dorothy-egui", &local_settings_copy);
                    }
                    if ui
                        .checkbox(
                            &mut self.config.app_settings.show_all_drops,
                            "Show All Raid Drops",
                        )
                        .clicked()
                    {
                        local_settings_copy.app_settings.show_all_drops =
                            self.config.app_settings.show_all_drops;
                        let _ = confy::store("dorothy-egui", &local_settings_copy);
                    }
                });
                ui.menu_button("Settings", |ui| {
                    ui.style_mut().wrap = Some(false);
                    if ui
                        .checkbox(&mut self.config.app_settings.dark_mode, "Dark Mode")
                        .clicked()
                    {
                        local_settings_copy.app_settings.dark_mode =
                            self.config.app_settings.dark_mode;
                        let _ = confy::store("dorothy-egui", &local_settings_copy);
                    }
                    if ui
                        .checkbox(&mut self.config.app_settings.always_on_top, "Always On Top")
                        .clicked()
                    {
                        local_settings_copy.app_settings.always_on_top =
                            self.config.app_settings.always_on_top;
                        let _ = confy::store("dorothy-egui", &local_settings_copy);
                    }
                    if ui
                        .checkbox(
                            &mut self.config.app_settings.reset_on_export,
                            "Reset Counts on Export",
                        )
                        .clicked()
                    {
                        local_settings_copy.app_settings.reset_on_export =
                            self.config.app_settings.reset_on_export;
                        let _ = confy::store("dorothy-egui", &local_settings_copy);
                    }
                    if ui
                        .checkbox(
                            &mut self.config.app_settings.droprate_by_kills,
                            "Calculate droprates by total kills",
                        )
                        .clicked()
                    {
                        local_settings_copy.app_settings.droprate_by_kills =
                            self.config.app_settings.droprate_by_kills;
                        let _ = confy::store("dorothy-egui", &local_settings_copy);
                    }
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
        if self.config.app_settings.left_panel_visible {
            egui::SidePanel::left("left_side_panel")
                .min_width(200.)
                .max_width(300.)
                .show(ctx, |ui| {
                    if ctx.is_pointer_over_area() {
                        local_settings_copy.app_settings.current_ui_tab =
                            self.config.app_settings.current_ui_tab.clone();
                        let _ = confy::store("dorothy-egui", &local_settings_copy);
                    }

                    ui.add_space(5.);
                    ui.heading("Drop Totals");
                    ui.add_space(3.);
                    ui.separator();
                    ui.add_space(5.);
                    egui::ScrollArea::vertical()
                        .auto_shrink([false, false])
                        .max_height(INFINITY)
                        .max_width(INFINITY)
                        .show(ui, |ui| {
                            if self.config.app_settings.current_ui_tab == UiTab::Akasha
                                || self.config.app_settings.show_all_drops
                            {
                                ui.add_space(20.);
                                let total_akasha_drops = local_settings_copy
                                    .droplog
                                    .drop
                                    .iter()
                                    .filter(|x| x.raid == Raid::Akasha)
                                    .count();
                                ui.heading(
                                    "Akasha - ".to_owned() + &total_akasha_drops.to_string(),
                                );
                                ui.add_space(5.);

                                let no_drop = local_settings_copy
                                    .droplog
                                    .drop
                                    .iter()
                                    .filter(|x| x.item == Item::NoDrop && x.raid == Raid::Akasha)
                                    .count();
                                if local_settings_copy.app_settings.droprate_by_kills {
                                    let no_drop_percent = format!(
                                        "{:.1$}%",
                                        ((no_drop as f32 / total_akasha_drops as f32) * 100.)
                                            .to_string(),
                                        3
                                    );
                                    ui.label(
                                        "No Drop: ".to_string()
                                            + &no_drop.to_string()
                                            + " ("
                                            + &no_drop_percent.to_string()
                                            + ")",
                                    );
                                } else if !local_settings_copy.app_settings.droprate_by_kills {
                                    ui.label("No Drop: ".to_string() + &no_drop.to_string());
                                }
                                let hollow_key_drops = local_settings_copy
                                    .droplog
                                    .drop
                                    .iter()
                                    .filter(|x| x.item == Item::HollowKey && x.raid == Raid::Akasha)
                                    .count();
                                if local_settings_copy.app_settings.droprate_by_kills {
                                    let hollow_key_percent = format!(
                                        "{:.1$}%",
                                        ((hollow_key_drops as f32 / total_akasha_drops as f32)
                                            * 100.)
                                            .to_string(),
                                        3
                                    );
                                    ui.label(
                                        "Hollow Key: ".to_string()
                                            + &hollow_key_drops.to_string()
                                            + " ("
                                            + &hollow_key_percent.to_string()
                                            + ")",
                                    );
                                } else if !local_settings_copy.app_settings.droprate_by_kills {
                                    let hollow_key_percent = format!(
                                        "{:.1$}%",
                                        ((hollow_key_drops as f32
                                            / (total_akasha_drops as f32 - no_drop as f32))
                                            * 100.)
                                            .to_string(),
                                        3
                                    );
                                    ui.label(
                                        "Hollow Key: ".to_string()
                                            + &hollow_key_drops.to_string()
                                            + " ("
                                            + &hollow_key_percent.to_string()
                                            + ")",
                                    );
                                }
                                let champion_merit_drops = local_settings_copy
                                    .droplog
                                    .drop
                                    .iter()
                                    .filter(|x| {
                                        x.item == Item::ChampionMerit && x.raid == Raid::Akasha
                                    })
                                    .count();
                                if local_settings_copy.app_settings.droprate_by_kills {
                                    let champion_merit_percent = format!(
                                        "{:.1$}%",
                                        ((champion_merit_drops as f32 / total_akasha_drops as f32)
                                            * 100.)
                                            .to_string(),
                                        3
                                    );
                                    ui.label(
                                        "Champion Merit: ".to_string()
                                            + &champion_merit_drops.to_string()
                                            + " ("
                                            + &champion_merit_percent.to_string()
                                            + ")",
                                    );
                                } else if !local_settings_copy.app_settings.droprate_by_kills {
                                    let champion_merit_percent = format!(
                                        "{:.1$}%",
                                        ((champion_merit_drops as f32
                                            / (total_akasha_drops as f32 - no_drop as f32))
                                            * 100.)
                                            .to_string(),
                                        3
                                    );
                                    ui.label(
                                        "Champion Merit: ".to_string()
                                            + &champion_merit_drops.to_string()
                                            + " ("
                                            + &champion_merit_percent.to_string()
                                            + ")",
                                    );
                                }
                                let supreme_merit_drops = local_settings_copy
                                    .droplog
                                    .drop
                                    .iter()
                                    .filter(|x| {
                                        x.item == Item::SupremeMerit && x.raid == Raid::Akasha
                                    })
                                    .count();
                                if local_settings_copy.app_settings.droprate_by_kills {
                                    let supreme_merit_percent = format!(
                                        "{:.1$}%",
                                        ((supreme_merit_drops as f32 / total_akasha_drops as f32)
                                            * 100.)
                                            .to_string(),
                                        3
                                    );
                                    ui.label(
                                        "Supreme Merit: ".to_string()
                                            + &supreme_merit_drops.to_string()
                                            + " ("
                                            + &supreme_merit_percent.to_string()
                                            + ")",
                                    );
                                } else if !local_settings_copy.app_settings.droprate_by_kills {
                                    let supreme_merit_percent = format!(
                                        "{:.1$}%",
                                        ((supreme_merit_drops as f32
                                            / (total_akasha_drops as f32 - no_drop as f32))
                                            * 100.)
                                            .to_string(),
                                        3
                                    );
                                    ui.label(
                                        "Supreme Merit: ".to_string()
                                            + &supreme_merit_drops.to_string()
                                            + " ("
                                            + &supreme_merit_percent.to_string()
                                            + ")",
                                    );
                                }
                                let legendary_merit_drops = local_settings_copy
                                    .droplog
                                    .drop
                                    .iter()
                                    .filter(|x| {
                                        x.item == Item::LegendaryMerit && x.raid == Raid::Akasha
                                    })
                                    .count();
                                if local_settings_copy.app_settings.droprate_by_kills {
                                    let legendary_merit_percent = format!(
                                        "{:.1$}%",
                                        ((legendary_merit_drops as f32
                                            / total_akasha_drops as f32)
                                            * 100.)
                                            .to_string(),
                                        3
                                    );
                                    ui.label(
                                        "Legendary Merit: ".to_string()
                                            + &legendary_merit_drops.to_string()
                                            + " ("
                                            + &legendary_merit_percent.to_string()
                                            + ")",
                                    );
                                } else if !local_settings_copy.app_settings.droprate_by_kills {
                                    let legendary_merit_percent = format!(
                                        "{:.1$}%",
                                        ((legendary_merit_drops as f32
                                            / (total_akasha_drops as f32 - no_drop as f32))
                                            * 100.)
                                            .to_string(),
                                        3
                                    );
                                    ui.label(
                                        "Legendary Merit: ".to_string()
                                            + &legendary_merit_drops.to_string()
                                            + " ("
                                            + &legendary_merit_percent.to_string()
                                            + ")",
                                    );
                                }
                                let silver_centrum_drops = local_settings_copy
                                    .droplog
                                    .drop
                                    .iter()
                                    .filter(|x| {
                                        x.item == Item::SilverCentrum && x.raid == Raid::Akasha
                                    })
                                    .count();
                                if local_settings_copy.app_settings.droprate_by_kills {
                                    let silver_centrum_percent = format!(
                                        "{:.1$}%",
                                        ((silver_centrum_drops as f32 / total_akasha_drops as f32)
                                            * 100.)
                                            .to_string(),
                                        3
                                    );
                                    ui.label(
                                        "Silver Centrum: ".to_string()
                                            + &silver_centrum_drops.to_string()
                                            + " ("
                                            + &silver_centrum_percent.to_string()
                                            + ")",
                                    );
                                } else if !local_settings_copy.app_settings.droprate_by_kills {
                                    let silver_centrum_percent = format!(
                                        "{:.1$}%",
                                        ((silver_centrum_drops as f32
                                            / (total_akasha_drops as f32 - no_drop as f32))
                                            * 100.)
                                            .to_string(),
                                        3
                                    );
                                    ui.label(
                                        "Silver Centrum: ".to_string()
                                            + &silver_centrum_drops.to_string()
                                            + " ("
                                            + &silver_centrum_percent.to_string()
                                            + ")",
                                    );
                                }
                                let weapon_plus_mark_1_drops = local_settings_copy
                                    .droplog
                                    .drop
                                    .iter()
                                    .filter(|x| {
                                        x.item == Item::WeaponPlusMark1 && x.raid == Raid::Akasha
                                    })
                                    .count();
                                if local_settings_copy.app_settings.droprate_by_kills {
                                    let weapon_plus_mark_1_percent = format!(
                                        "{:.1$}%",
                                        ((weapon_plus_mark_1_drops as f32
                                            / total_akasha_drops as f32)
                                            * 100.)
                                            .to_string(),
                                        3
                                    );
                                    ui.label(
                                        "Weapon Plus Mark +1: ".to_string()
                                            + &weapon_plus_mark_1_drops.to_string()
                                            + " ("
                                            + &weapon_plus_mark_1_percent.to_string()
                                            + ")",
                                    );
                                } else if !local_settings_copy.app_settings.droprate_by_kills {
                                    let weapon_plus_mark_1_percent = format!(
                                        "{:.1$}%",
                                        ((weapon_plus_mark_1_drops as f32
                                            / (total_akasha_drops as f32 - no_drop as f32))
                                            * 100.)
                                            .to_string(),
                                        3
                                    );
                                    ui.label(
                                        "Weapon Plus Mark +1: ".to_string()
                                            + &weapon_plus_mark_1_drops.to_string()
                                            + " ("
                                            + &weapon_plus_mark_1_percent.to_string()
                                            + ")",
                                    );
                                }
                                let weapon_plus_mark_2_drops = local_settings_copy
                                    .droplog
                                    .drop
                                    .iter()
                                    .filter(|x| {
                                        x.item == Item::WeaponPlusMark2 && x.raid == Raid::Akasha
                                    })
                                    .count();
                                if local_settings_copy.app_settings.droprate_by_kills {
                                    let weapon_plus_mark_2_percent = format!(
                                        "{:.1$}%",
                                        ((weapon_plus_mark_2_drops as f32
                                            / total_akasha_drops as f32)
                                            * 100.)
                                            .to_string(),
                                        3
                                    );
                                    ui.label(
                                        "Weapon Plus Mark +2: ".to_string()
                                            + &weapon_plus_mark_2_drops.to_string()
                                            + " ("
                                            + &weapon_plus_mark_2_percent.to_string()
                                            + ")",
                                    );
                                } else if !local_settings_copy.app_settings.droprate_by_kills {
                                    let weapon_plus_mark_2_percent = format!(
                                        "{:.1$}%",
                                        ((weapon_plus_mark_2_drops as f32
                                            / (total_akasha_drops as f32 - no_drop as f32))
                                            * 100.)
                                            .to_string(),
                                        3
                                    );
                                    ui.label(
                                        "Weapon Plus Mark +2: ".to_string()
                                            + &weapon_plus_mark_2_drops.to_string()
                                            + " ("
                                            + &weapon_plus_mark_2_percent.to_string()
                                            + ")",
                                    );
                                }
                                let weapon_plus_mark_3_drops = local_settings_copy
                                    .droplog
                                    .drop
                                    .iter()
                                    .filter(|x| {
                                        x.item == Item::WeaponPlusMark3 && x.raid == Raid::Akasha
                                    })
                                    .count();
                                if local_settings_copy.app_settings.droprate_by_kills {
                                    let weapon_plus_mark_3_percent = format!(
                                        "{:.1$}%",
                                        ((weapon_plus_mark_3_drops as f32
                                            / total_akasha_drops as f32)
                                            * 100.)
                                            .to_string(),
                                        3
                                    );
                                    ui.label(
                                        "Weapon Plus Mark +3: ".to_string()
                                            + &weapon_plus_mark_3_drops.to_string()
                                            + " ("
                                            + &weapon_plus_mark_3_percent.to_string()
                                            + ")",
                                    );
                                } else if !local_settings_copy.app_settings.droprate_by_kills {
                                    let weapon_plus_mark_3_percent = format!(
                                        "{:.1$}%",
                                        ((weapon_plus_mark_3_drops as f32
                                            / (total_akasha_drops as f32 - no_drop as f32))
                                            * 100.)
                                            .to_string(),
                                        3
                                    );
                                    ui.label(
                                        "Weapon Plus Mark +3: ".to_string()
                                            + &weapon_plus_mark_3_drops.to_string()
                                            + " ("
                                            + &weapon_plus_mark_3_percent.to_string()
                                            + ")",
                                    );
                                }
                                let coronation_ring_drops = local_settings_copy
                                    .droplog
                                    .drop
                                    .iter()
                                    .filter(|x| {
                                        x.item == Item::CoronationRing && x.raid == Raid::Akasha
                                    })
                                    .count();
                                if local_settings_copy.app_settings.droprate_by_kills {
                                    let coronation_ring_percent = format!(
                                        "{:.1$}%",
                                        ((coronation_ring_drops as f32
                                            / total_akasha_drops as f32)
                                            * 100.)
                                            .to_string(),
                                        3
                                    );
                                    ui.label(
                                        "Coronation Ring: ".to_string()
                                            + &coronation_ring_drops.to_string()
                                            + " ("
                                            + &coronation_ring_percent.to_string()
                                            + ")",
                                    );
                                } else if !local_settings_copy.app_settings.droprate_by_kills {
                                    let coronation_ring_percent = format!(
                                        "{:.1$}%",
                                        ((coronation_ring_drops as f32
                                            / (total_akasha_drops as f32 - no_drop as f32))
                                            * 100.)
                                            .to_string(),
                                        3
                                    );
                                    ui.label(
                                        "Coronation Ring: ".to_string()
                                            + &coronation_ring_drops.to_string()
                                            + " ("
                                            + &coronation_ring_percent.to_string()
                                            + ")",
                                    );
                                }
                                let lineage_ring_drops = local_settings_copy
                                    .droplog
                                    .drop
                                    .iter()
                                    .filter(|x| {
                                        x.item == Item::LineageRing && x.raid == Raid::Akasha
                                    })
                                    .count();
                                if local_settings_copy.app_settings.droprate_by_kills {
                                    let lineage_ring_percent = format!(
                                        "{:.1$}%",
                                        ((lineage_ring_drops as f32 / total_akasha_drops as f32)
                                            * 100.)
                                            .to_string(),
                                        3
                                    );
                                    ui.label(
                                        "Lineage Ring: ".to_string()
                                            + &lineage_ring_drops.to_string()
                                            + " ("
                                            + &lineage_ring_percent.to_string()
                                            + ")",
                                    );
                                } else if !local_settings_copy.app_settings.droprate_by_kills {
                                    let lineage_ring_percent = format!(
                                        "{:.1$}%",
                                        ((lineage_ring_drops as f32
                                            / (total_akasha_drops as f32 - no_drop as f32))
                                            * 100.)
                                            .to_string(),
                                        3
                                    );
                                    ui.label(
                                        "Lineage Ring: ".to_string()
                                            + &lineage_ring_drops.to_string()
                                            + " ("
                                            + &lineage_ring_percent.to_string()
                                            + ")",
                                    );
                                }
                                let intricacy_ring_drops = local_settings_copy
                                    .droplog
                                    .drop
                                    .iter()
                                    .filter(|x| {
                                        x.item == Item::IntricacyRing && x.raid == Raid::Akasha
                                    })
                                    .count();
                                if local_settings_copy.app_settings.droprate_by_kills {
                                    let intricacy_ring_percent = format!(
                                        "{:.1$}%",
                                        ((intricacy_ring_drops as f32 / total_akasha_drops as f32)
                                            * 100.)
                                            .to_string(),
                                        3
                                    );
                                    ui.label(
                                        "Intricacy Ring: ".to_string()
                                            + &intricacy_ring_drops.to_string()
                                            + " ("
                                            + &intricacy_ring_percent.to_string()
                                            + ")",
                                    );
                                } else if !local_settings_copy.app_settings.droprate_by_kills {
                                    let intricacy_ring_percent = format!(
                                        "{:.1$}%",
                                        ((intricacy_ring_drops as f32
                                            / (total_akasha_drops as f32 - no_drop as f32))
                                            * 100.)
                                            .to_string(),
                                        3
                                    );
                                    ui.label(
                                        "Intricacy Ring: ".to_string()
                                            + &intricacy_ring_drops.to_string()
                                            + " ("
                                            + &intricacy_ring_percent.to_string()
                                            + ")",
                                    );
                                }
                                let gold_brick_drops = local_settings_copy
                                    .droplog
                                    .drop
                                    .iter()
                                    .filter(|x| x.item == Item::GoldBrick && x.raid == Raid::Akasha)
                                    .count();
                                if local_settings_copy.app_settings.droprate_by_kills {
                                    let gold_brick_percent = format!(
                                        "{:.1$}%",
                                        ((gold_brick_drops as f32 / total_akasha_drops as f32)
                                            * 100.)
                                            .to_string(),
                                        3
                                    );
                                    ui.label(
                                        "Gold Brick: ".to_string()
                                            + &gold_brick_drops.to_string()
                                            + " ("
                                            + &gold_brick_percent.to_string()
                                            + ")",
                                    );
                                } else if !local_settings_copy.app_settings.droprate_by_kills {
                                    let gold_brick_percent = format!(
                                        "{:.1$}%",
                                        ((gold_brick_drops as f32
                                            / (total_akasha_drops as f32 - no_drop as f32))
                                            * 100.)
                                            .to_string(),
                                        3
                                    );
                                    ui.label(
                                        "Gold Brick: ".to_string()
                                            + &gold_brick_drops.to_string()
                                            + " ("
                                            + &gold_brick_percent.to_string()
                                            + ")",
                                    );
                                }
                            }
                            if self.config.app_settings.current_ui_tab == UiTab::PBHL
                                || self.config.app_settings.show_all_drops
                            {
                                ui.add_space(20.);
                                let total_pbhl_drops = local_settings_copy
                                    .droplog
                                    .drop
                                    .iter()
                                    .filter(|x| x.raid == Raid::PBHL)
                                    .count();
                                ui.heading("PBHL - ".to_owned() + &total_pbhl_drops.to_string());
                                ui.add_space(5.);

                                let no_drop = local_settings_copy
                                    .droplog
                                    .drop
                                    .iter()
                                    .filter(|x| x.item == Item::NoDrop && x.raid == Raid::PBHL)
                                    .count();
                                if local_settings_copy.app_settings.droprate_by_kills {
                                    let no_drop_percent = format!(
                                        "{:.1$}%",
                                        ((no_drop as f32 / total_pbhl_drops as f32) * 100.)
                                            .to_string(),
                                        3
                                    );
                                    ui.label(
                                        "No Drop: ".to_string()
                                            + &no_drop.to_string()
                                            + " ("
                                            + &no_drop_percent.to_string()
                                            + ")",
                                    );
                                } else if !local_settings_copy.app_settings.droprate_by_kills {
                                    ui.label("No Drop: ".to_string() + &no_drop.to_string());
                                }
                                let coronation_ring_drops = local_settings_copy
                                    .droplog
                                    .drop
                                    .iter()
                                    .filter(|x| {
                                        x.item == Item::CoronationRing && x.raid == Raid::PBHL
                                    })
                                    .count();
                                if local_settings_copy.app_settings.droprate_by_kills {
                                    let coronation_ring_percent = format!(
                                        "{:.1$}%",
                                        ((coronation_ring_drops as f32 / total_pbhl_drops as f32)
                                            * 100.)
                                            .to_string(),
                                        3
                                    );
                                    ui.label(
                                        "Coronation Ring: ".to_string()
                                            + &coronation_ring_drops.to_string()
                                            + " ("
                                            + &coronation_ring_percent.to_string()
                                            + ")",
                                    );
                                } else if !local_settings_copy.app_settings.droprate_by_kills {
                                    let coronation_ring_percent = format!(
                                        "{:.1$}%",
                                        ((coronation_ring_drops as f32
                                            / (total_pbhl_drops as f32 - no_drop as f32))
                                            * 100.)
                                            .to_string(),
                                        3
                                    );
                                    ui.label(
                                        "Coronation Ring: ".to_string()
                                            + &coronation_ring_drops.to_string()
                                            + " ("
                                            + &coronation_ring_percent.to_string()
                                            + ")",
                                    );
                                }
                                let lineage_ring_drops = local_settings_copy
                                    .droplog
                                    .drop
                                    .iter()
                                    .filter(|x| x.item == Item::LineageRing && x.raid == Raid::PBHL)
                                    .count();
                                if local_settings_copy.app_settings.droprate_by_kills {
                                    let lineage_ring_percent = format!(
                                        "{:.1$}%",
                                        ((lineage_ring_drops as f32 / total_pbhl_drops as f32)
                                            * 100.)
                                            .to_string(),
                                        3
                                    );
                                    ui.label(
                                        "Lineage Ring: ".to_string()
                                            + &lineage_ring_drops.to_string()
                                            + " ("
                                            + &lineage_ring_percent.to_string()
                                            + ")",
                                    );
                                } else if !local_settings_copy.app_settings.droprate_by_kills {
                                    let lineage_ring_percent = format!(
                                        "{:.1$}%",
                                        ((lineage_ring_drops as f32
                                            / (total_pbhl_drops as f32 - no_drop as f32))
                                            * 100.)
                                            .to_string(),
                                        3
                                    );
                                    ui.label(
                                        "Lineage Ring: ".to_string()
                                            + &lineage_ring_drops.to_string()
                                            + " ("
                                            + &lineage_ring_percent.to_string()
                                            + ")",
                                    );
                                }
                                let intricacy_ring_drops = local_settings_copy
                                    .droplog
                                    .drop
                                    .iter()
                                    .filter(|x| {
                                        x.item == Item::IntricacyRing && x.raid == Raid::PBHL
                                    })
                                    .count();
                                if local_settings_copy.app_settings.droprate_by_kills {
                                    let intricacy_ring_percent = format!(
                                        "{:.1$}%",
                                        ((intricacy_ring_drops as f32 / total_pbhl_drops as f32)
                                            * 100.)
                                            .to_string(),
                                        3
                                    );
                                    ui.label(
                                        "Intricacy Ring: ".to_string()
                                            + &intricacy_ring_drops.to_string()
                                            + " ("
                                            + &intricacy_ring_percent.to_string()
                                            + ")",
                                    );
                                } else if !local_settings_copy.app_settings.droprate_by_kills {
                                    let intricacy_ring_percent = format!(
                                        "{:.1$}%",
                                        ((intricacy_ring_drops as f32
                                            / (total_pbhl_drops as f32 - no_drop as f32))
                                            * 100.)
                                            .to_string(),
                                        3
                                    );
                                    ui.label(
                                        "Intricacy Ring: ".to_string()
                                            + &intricacy_ring_drops.to_string()
                                            + " ("
                                            + &intricacy_ring_percent.to_string()
                                            + ")",
                                    );
                                }
                                let gold_brick_drops = local_settings_copy
                                    .droplog
                                    .drop
                                    .iter()
                                    .filter(|x| x.item == Item::GoldBrick && x.raid == Raid::PBHL)
                                    .count();
                                if local_settings_copy.app_settings.droprate_by_kills {
                                    let gold_brick_percent = format!(
                                        "{:.1$}%",
                                        ((gold_brick_drops as f32 / total_pbhl_drops as f32)
                                            * 100.)
                                            .to_string(),
                                        3
                                    );
                                    ui.label(
                                        "Gold Brick: ".to_string()
                                            + &gold_brick_drops.to_string()
                                            + " ("
                                            + &gold_brick_percent.to_string()
                                            + ")",
                                    );
                                } else if !local_settings_copy.app_settings.droprate_by_kills {
                                    let gold_brick_percent = format!(
                                        "{:.1$}%",
                                        ((gold_brick_drops as f32
                                            / (total_pbhl_drops as f32 - no_drop as f32))
                                            * 100.)
                                            .to_string(),
                                        3
                                    );
                                    ui.label(
                                        "Gold Brick: ".to_string()
                                            + &gold_brick_drops.to_string()
                                            + " ("
                                            + &gold_brick_percent.to_string()
                                            + ")",
                                    );
                                }
                            }
                            if self.config.app_settings.current_ui_tab == UiTab::GOHL
                                || self.config.app_settings.show_all_drops
                            {
                                ui.add_space(20.);
                                let total_gohl_drops = local_settings_copy
                                    .droplog
                                    .drop
                                    .iter()
                                    .filter(|x| x.raid == Raid::GOHL)
                                    .count();
                                ui.heading("GOHL - ".to_owned() + &total_gohl_drops.to_string());
                                ui.add_space(5.);
                                let no_drop = local_settings_copy
                                    .droplog
                                    .drop
                                    .iter()
                                    .filter(|x| x.item == Item::NoDrop && x.raid == Raid::GOHL)
                                    .count()
                                    .to_string();
                                let no_drop = local_settings_copy
                                    .droplog
                                    .drop
                                    .iter()
                                    .filter(|x| x.item == Item::NoDrop && x.raid == Raid::Akasha)
                                    .count();
                                if local_settings_copy.app_settings.droprate_by_kills {
                                    let no_drop_percent = format!(
                                        "{:.1$}%",
                                        ((no_drop as f32 / total_gohl_drops as f32) * 100.)
                                            .to_string(),
                                        3
                                    );
                                    ui.label(
                                        "No Drop: ".to_string()
                                            + &no_drop.to_string()
                                            + " ("
                                            + &no_drop_percent.to_string()
                                            + ")",
                                    );
                                } else if !local_settings_copy.app_settings.droprate_by_kills {
                                    ui.label("No Drop: ".to_string() + &no_drop.to_string());
                                }
                                let verdant_azurite_drops = local_settings_copy
                                    .droplog
                                    .drop
                                    .iter()
                                    .filter(|x| x.item == Item::HollowKey && x.raid == Raid::GOHL)
                                    .count();
                                if local_settings_copy.app_settings.droprate_by_kills {
                                    let verdant_azurite_percent = format!(
                                        "{:.1$}%",
                                        ((verdant_azurite_drops as f32 / total_gohl_drops as f32)
                                            * 100.)
                                            .to_string(),
                                        3
                                    );
                                    ui.label(
                                        "Verdant Azurite: ".to_string()
                                            + &verdant_azurite_drops.to_string()
                                            + " ("
                                            + &verdant_azurite_percent.to_string()
                                            + ")",
                                    );
                                } else if !local_settings_copy.app_settings.droprate_by_kills {
                                    let verdant_azurite_percent = format!(
                                        "{:.1$}%",
                                        ((verdant_azurite_drops as f32
                                            / (total_gohl_drops as f32 - no_drop as f32))
                                            * 100.)
                                            .to_string(),
                                        3
                                    );
                                    ui.label(
                                        "Verdant Azurite: ".to_string()
                                            + &verdant_azurite_drops.to_string()
                                            + " ("
                                            + &verdant_azurite_percent.to_string()
                                            + ")",
                                    );
                                }
                                let champion_merit_drops = local_settings_copy
                                    .droplog
                                    .drop
                                    .iter()
                                    .filter(|x| {
                                        x.item == Item::ChampionMerit && x.raid == Raid::GOHL
                                    })
                                    .count();
                                if local_settings_copy.app_settings.droprate_by_kills {
                                    let champion_merit_percent = format!(
                                        "{:.1$}%",
                                        ((champion_merit_drops as f32 / total_gohl_drops as f32)
                                            * 100.)
                                            .to_string(),
                                        3
                                    );
                                    ui.label(
                                        "Champion Merit: ".to_string()
                                            + &champion_merit_drops.to_string()
                                            + " ("
                                            + &champion_merit_percent.to_string()
                                            + ")",
                                    );
                                } else if !local_settings_copy.app_settings.droprate_by_kills {
                                    let champion_merit_percent = format!(
                                        "{:.1$}%",
                                        ((champion_merit_drops as f32
                                            / (total_gohl_drops as f32 - no_drop as f32))
                                            * 100.)
                                            .to_string(),
                                        3
                                    );
                                    ui.label(
                                        "Champion Merit: ".to_string()
                                            + &champion_merit_drops.to_string()
                                            + " ("
                                            + &champion_merit_percent.to_string()
                                            + ")",
                                    );
                                }
                                let supreme_merit_drops = local_settings_copy
                                    .droplog
                                    .drop
                                    .iter()
                                    .filter(|x| {
                                        x.item == Item::SupremeMerit && x.raid == Raid::GOHL
                                    })
                                    .count();
                                if local_settings_copy.app_settings.droprate_by_kills {
                                    let supreme_merit_percent = format!(
                                        "{:.1$}%",
                                        ((supreme_merit_drops as f32 / total_gohl_drops as f32)
                                            * 100.)
                                            .to_string(),
                                        3
                                    );
                                    ui.label(
                                        "Supreme Merit: ".to_string()
                                            + &supreme_merit_drops.to_string()
                                            + " ("
                                            + &supreme_merit_percent.to_string()
                                            + ")",
                                    );
                                } else if !local_settings_copy.app_settings.droprate_by_kills {
                                    let supreme_merit_percent = format!(
                                        "{:.1$}%",
                                        ((supreme_merit_drops as f32
                                            / (total_gohl_drops as f32 - no_drop as f32))
                                            * 100.)
                                            .to_string(),
                                        3
                                    );
                                    ui.label(
                                        "Supreme Merit: ".to_string()
                                            + &supreme_merit_drops.to_string()
                                            + " ("
                                            + &supreme_merit_percent.to_string()
                                            + ")",
                                    );
                                }
                                let legendary_merit_drops = local_settings_copy
                                    .droplog
                                    .drop
                                    .iter()
                                    .filter(|x| {
                                        x.item == Item::LegendaryMerit && x.raid == Raid::GOHL
                                    })
                                    .count();
                                if local_settings_copy.app_settings.droprate_by_kills {
                                    let legendary_merit_percent = format!(
                                        "{:.1$}%",
                                        ((legendary_merit_drops as f32 / total_gohl_drops as f32)
                                            * 100.)
                                            .to_string(),
                                        3
                                    );
                                    ui.label(
                                        "Legendary Merit: ".to_string()
                                            + &legendary_merit_drops.to_string()
                                            + " ("
                                            + &legendary_merit_percent.to_string()
                                            + ")",
                                    );
                                } else if !local_settings_copy.app_settings.droprate_by_kills {
                                    let legendary_merit_percent = format!(
                                        "{:.1$}%",
                                        ((legendary_merit_drops as f32
                                            / (total_gohl_drops as f32 - no_drop as f32))
                                            * 100.)
                                            .to_string(),
                                        3
                                    );
                                    ui.label(
                                        "Legendary Merit: ".to_string()
                                            + &legendary_merit_drops.to_string()
                                            + " ("
                                            + &legendary_merit_percent.to_string()
                                            + ")",
                                    );
                                }
                                let silver_centrum_drops = local_settings_copy
                                    .droplog
                                    .drop
                                    .iter()
                                    .filter(|x| {
                                        x.item == Item::SilverCentrum && x.raid == Raid::GOHL
                                    })
                                    .count();
                                if local_settings_copy.app_settings.droprate_by_kills {
                                    let silver_centrum_percent = format!(
                                        "{:.1$}%",
                                        ((silver_centrum_drops as f32 / total_gohl_drops as f32)
                                            * 100.)
                                            .to_string(),
                                        3
                                    );
                                    ui.label(
                                        "Silver Centrum: ".to_string()
                                            + &silver_centrum_drops.to_string()
                                            + " ("
                                            + &silver_centrum_percent.to_string()
                                            + ")",
                                    );
                                } else if !local_settings_copy.app_settings.droprate_by_kills {
                                    let silver_centrum_percent = format!(
                                        "{:.1$}%",
                                        ((silver_centrum_drops as f32
                                            / (total_gohl_drops as f32 - no_drop as f32))
                                            * 100.)
                                            .to_string(),
                                        3
                                    );
                                    ui.label(
                                        "Silver Centrum: ".to_string()
                                            + &silver_centrum_drops.to_string()
                                            + " ("
                                            + &silver_centrum_percent.to_string()
                                            + ")",
                                    );
                                }
                                let coronation_ring_drops = local_settings_copy
                                    .droplog
                                    .drop
                                    .iter()
                                    .filter(|x| {
                                        x.item == Item::CoronationRing && x.raid == Raid::GOHL
                                    })
                                    .count();
                                if local_settings_copy.app_settings.droprate_by_kills {
                                    let coronation_ring_percent = format!(
                                        "{:.1$}%",
                                        ((coronation_ring_drops as f32 / total_gohl_drops as f32)
                                            * 100.)
                                            .to_string(),
                                        3
                                    );
                                    ui.label(
                                        "Coronation Ring: ".to_string()
                                            + &coronation_ring_drops.to_string()
                                            + " ("
                                            + &coronation_ring_percent.to_string()
                                            + ")",
                                    );
                                } else if !local_settings_copy.app_settings.droprate_by_kills {
                                    let coronation_ring_percent = format!(
                                        "{:.1$}%",
                                        ((coronation_ring_drops as f32
                                            / (total_gohl_drops as f32 - no_drop as f32))
                                            * 100.)
                                            .to_string(),
                                        3
                                    );
                                    ui.label(
                                        "Coronation Ring: ".to_string()
                                            + &coronation_ring_drops.to_string()
                                            + " ("
                                            + &coronation_ring_percent.to_string()
                                            + ")",
                                    );
                                }
                                let lineage_ring_drops = local_settings_copy
                                    .droplog
                                    .drop
                                    .iter()
                                    .filter(|x| x.item == Item::LineageRing && x.raid == Raid::GOHL)
                                    .count();
                                if local_settings_copy.app_settings.droprate_by_kills {
                                    let lineage_ring_percent = format!(
                                        "{:.1$}%",
                                        ((lineage_ring_drops as f32 / total_gohl_drops as f32)
                                            * 100.)
                                            .to_string(),
                                        3
                                    );
                                    ui.label(
                                        "Lineage Ring: ".to_string()
                                            + &lineage_ring_drops.to_string()
                                            + " ("
                                            + &lineage_ring_percent.to_string()
                                            + ")",
                                    );
                                } else if !local_settings_copy.app_settings.droprate_by_kills {
                                    let lineage_ring_percent = format!(
                                        "{:.1$}%",
                                        ((lineage_ring_drops as f32
                                            / (total_gohl_drops as f32 - no_drop as f32))
                                            * 100.)
                                            .to_string(),
                                        3
                                    );
                                    ui.label(
                                        "Lineage Ring: ".to_string()
                                            + &lineage_ring_drops.to_string()
                                            + " ("
                                            + &lineage_ring_percent.to_string()
                                            + ")",
                                    );
                                }
                                let intricacy_ring_drops = local_settings_copy
                                    .droplog
                                    .drop
                                    .iter()
                                    .filter(|x| {
                                        x.item == Item::IntricacyRing && x.raid == Raid::GOHL
                                    })
                                    .count();
                                if local_settings_copy.app_settings.droprate_by_kills {
                                    let intricacy_ring_percent = format!(
                                        "{:.1$}%",
                                        ((intricacy_ring_drops as f32 / total_gohl_drops as f32)
                                            * 100.)
                                            .to_string(),
                                        3
                                    );
                                    ui.label(
                                        "Intricacy Ring: ".to_string()
                                            + &intricacy_ring_drops.to_string()
                                            + " ("
                                            + &intricacy_ring_percent.to_string()
                                            + ")",
                                    );
                                } else if !local_settings_copy.app_settings.droprate_by_kills {
                                    let intricacy_ring_percent = format!(
                                        "{:.1$}%",
                                        ((intricacy_ring_drops as f32
                                            / (total_gohl_drops as f32 - no_drop as f32))
                                            * 100.)
                                            .to_string(),
                                        3
                                    );
                                    ui.label(
                                        "Intricacy Ring: ".to_string()
                                            + &intricacy_ring_drops.to_string()
                                            + " ("
                                            + &intricacy_ring_percent.to_string()
                                            + ")",
                                    );
                                }
                                let gold_brick_drops = local_settings_copy
                                    .droplog
                                    .drop
                                    .iter()
                                    .filter(|x| x.item == Item::GoldBrick && x.raid == Raid::GOHL)
                                    .count();
                                if local_settings_copy.app_settings.droprate_by_kills {
                                    let gold_brick_percent = format!(
                                        "{:.1$}%",
                                        ((gold_brick_drops as f32 / total_gohl_drops as f32)
                                            * 100.)
                                            .to_string(),
                                        3
                                    );
                                    ui.label(
                                        "Gold Brick: ".to_string()
                                            + &gold_brick_drops.to_string()
                                            + " ("
                                            + &gold_brick_percent.to_string()
                                            + ")",
                                    );
                                } else if !local_settings_copy.app_settings.droprate_by_kills {
                                    let gold_brick_percent = format!(
                                        "{:.1$}%",
                                        ((gold_brick_drops as f32
                                            / (total_gohl_drops as f32 - no_drop as f32))
                                            * 100.)
                                            .to_string(),
                                        3
                                    );
                                    ui.label(
                                        "Gold Brick: ".to_string()
                                            + &gold_brick_drops.to_string()
                                            + " ("
                                            + &gold_brick_percent.to_string()
                                            + ")",
                                    );
                                }
                            }
                            if self.config.app_settings.current_ui_tab == UiTab::Hosts
                                || self.config.app_settings.show_all_drops
                            {
                                ui.add_space(20.);
                                let total_hosts_drops = local_settings_copy
                                    .droplog
                                    .drop
                                    .iter()
                                    .filter(|x| x.raid == Raid::UBHL || x.raid == Raid::Xeno)
                                    .count()
                                    .to_string();
                                ui.heading("Hosts - ".to_owned() + &total_hosts_drops);
                                ui.add_space(5.);
                                ui.label("Coming Soon...");
                            }
                        });
                });
        }

        if self.config.app_settings.right_panel_visible {
            egui::SidePanel::right("right_side_panel")
                .min_width(150.)
                .max_width(300.)
                .show(ctx, |ui| {
                    ui.heading("Recent Drops");
                    ui.add_space(5.);

                    egui::ScrollArea::vertical()
                        .auto_shrink([false, false])
                        .max_height(INFINITY)
                        .max_width(INFINITY)
                        .show(ui, |ui| {
                            for drop in local_settings_copy
                                .droplog
                                .drop
                                .clone()
                                .into_iter()
                                .filter(|x| x.item != Item::NoDrop)
                                .rev()
                            {
                                ui.horizontal(|ui| {
                                    ui.spacing_mut().item_spacing.x = 0.0;
                                    if drop.item == Item::GoldBrick && drop.raid != Raid::PBHL {
                                        let mut gold_brick_text_color =
                                            Color32::from_rgb(255, 221, 26);
                                        if self.config.app_settings.dark_mode == false {
                                            gold_brick_text_color = Color32::from_rgb(187, 152, 10);
                                        }
                                        if ui
                                            .button(
                                                RichText::new(format!("{}", drop.item))
                                                    .color(gold_brick_text_color),
                                            )
                                            .on_hover_text(format!(
                                                "On {} from {}",
                                                drop.date_obtained, drop.raid
                                            ))
                                            .clicked()
                                        {
                                            let _ = self
                                                .config
                                                .droplog
                                                .drop
                                                .retain(|x| x.drop_id != drop.drop_id);
                                            let _ = local_settings_copy
                                                .droplog
                                                .drop
                                                .retain(|x| x.drop_id != drop.drop_id);
                                            let _ =
                                                confy::store("dorothy-egui", &local_settings_copy);
                                        }
                                        ui.add_space(3.)
                                    } else if drop.item == Item::GoldBrick
                                        && drop.raid == Raid::PBHL
                                    {
                                        let mut gold_brick_text_color =
                                            Color32::from_rgb(255, 221, 26);
                                        if self.config.app_settings.dark_mode == false {
                                            gold_brick_text_color = Color32::from_rgb(183, 138, 15);
                                        }
                                        if ui
                                            .button(
                                                RichText::new(format!("{}", drop.item))
                                                    .color(gold_brick_text_color),
                                            )
                                            .on_hover_text(format!(
                                                "On {} from {} with around {} honors",
                                                drop.date_obtained,
                                                drop.raid,
                                                drop.honors.as_ref().unwrap()
                                            ))
                                            .clicked()
                                        {
                                            let _ = self
                                                .config
                                                .droplog
                                                .drop
                                                .retain(|x| x.drop_id != drop.drop_id);
                                            let _ = local_settings_copy
                                                .droplog
                                                .drop
                                                .retain(|x| x.drop_id != drop.drop_id);
                                            let _ =
                                                confy::store("dorothy-egui", &local_settings_copy);
                                        }
                                        ui.add_space(3.)
                                    } else if drop.raid == Raid::PBHL {
                                        if ui
                                            .button(format!("{}", drop.item))
                                            .on_hover_text(format!(
                                                "On {} from {} with around {} honors",
                                                drop.date_obtained,
                                                drop.raid,
                                                drop.honors.as_ref().unwrap()
                                            ))
                                            .clicked()
                                        {
                                            let _ = self
                                                .config
                                                .droplog
                                                .drop
                                                .retain(|x| x.drop_id != drop.drop_id);
                                            let _ = local_settings_copy
                                                .droplog
                                                .drop
                                                .retain(|x| x.drop_id != drop.drop_id);
                                            let _ =
                                                confy::store("dorothy-egui", &local_settings_copy);
                                        }
                                        ui.add_space(3.)
                                    } else {
                                        if ui
                                            .button(format!("{}", drop.item))
                                            .on_hover_text(format!(
                                                "On {} from {}",
                                                drop.date_obtained, drop.raid
                                            ))
                                            .clicked()
                                        {
                                            let _ = self
                                                .config
                                                .droplog
                                                .drop
                                                .retain(|x| x.drop_id != drop.drop_id);
                                            let _ = local_settings_copy
                                                .droplog
                                                .drop
                                                .retain(|x| x.drop_id != drop.drop_id);
                                            let _ =
                                                confy::store("dorothy-egui", &local_settings_copy);
                                        }
                                        ui.add_space(3.)
                                    }
                                });
                            }
                        });
                });
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.selectable_value(
                    &mut self.config.app_settings.current_ui_tab,
                    UiTab::Pulls,
                    "Pull Calculator",
                );
                ui.selectable_value(
                    &mut self.config.app_settings.current_ui_tab,
                    UiTab::Akasha,
                    "Akasha",
                );
                ui.selectable_value(
                    &mut self.config.app_settings.current_ui_tab,
                    UiTab::PBHL,
                    "PBHL",
                );
                ui.selectable_value(
                    &mut self.config.app_settings.current_ui_tab,
                    UiTab::GOHL,
                    "GOHL",
                );
                ui.selectable_value(
                    &mut self.config.app_settings.current_ui_tab,
                    UiTab::Hosts,
                    "Hosts",
                );
            });

            ui.add_space(30.);
            egui::ScrollArea::both()
                .auto_shrink([false, false])
                .max_height(INFINITY)
                .max_width(INFINITY)
                .show(ui, |ui| {
                    if self.config.app_settings.current_ui_tab == UiTab::Pulls {
                        ui.horizontal(|ui| {
                            ui.label("Crystals: ");
                            let response = ui.add(egui::TextEdit::singleline(
                                &mut self.config.app_settings.crystals_amount,
                            ));
                            if response.changed() {
                                self.config.app_settings.total_pulls = calculate_pulls(
                                    self.config
                                        .app_settings
                                        .crystals_amount
                                        .parse::<f32>()
                                        .unwrap_or_default(),
                                    self.config
                                        .app_settings
                                        .ten_pulls_amount
                                        .parse::<f32>()
                                        .unwrap_or_default(),
                                    self.config
                                        .app_settings
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
                                &mut self.config.app_settings.ten_pulls_amount,
                            ));
                            if response.changed() {
                                self.config.app_settings.total_pulls = calculate_pulls(
                                    self.config
                                        .app_settings
                                        .crystals_amount
                                        .parse::<f32>()
                                        .unwrap_or_default(),
                                    self.config
                                        .app_settings
                                        .ten_pulls_amount
                                        .parse::<f32>()
                                        .unwrap_or_default(),
                                    self.config
                                        .app_settings
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
                                &mut self.config.app_settings.single_pulls_amount,
                            ));
                            if response.changed() {
                                self.config.app_settings.total_pulls = calculate_pulls(
                                    self.config
                                        .app_settings
                                        .crystals_amount
                                        .parse::<f32>()
                                        .unwrap_or_default(),
                                    self.config
                                        .app_settings
                                        .ten_pulls_amount
                                        .parse::<f32>()
                                        .unwrap_or_default(),
                                    self.config
                                        .app_settings
                                        .single_pulls_amount
                                        .parse::<f32>()
                                        .unwrap_or_default(),
                                )
                            }
                        });
                        ui.add_space(5.);
                        ui.label(&self.config.app_settings.total_pulls);
                    }
                    if self.config.app_settings.current_ui_tab == UiTab::Akasha {
                        egui::Grid::new("akasha_item_grid")
                            .spacing((15., 20.))
                            .show(ui, |ui| {
                                if ui.button("No Drop").clicked() {
                                    let _ = &self.droplog.drop.push(ItemDrop::new(
                                        local_settings_copy
                                            .droplog
                                            .drop
                                            .clone()
                                            .iter()
                                            .count()
                                            .try_into()
                                            .unwrap(),
                                        get_time(),
                                        Raid::Akasha,
                                        Item::NoDrop,
                                        Option::Some("None".to_string()),
                                    ));
                                    let _ = local_settings_copy.droplog.drop.push(ItemDrop::new(
                                        local_settings_copy
                                            .droplog
                                            .drop
                                            .clone()
                                            .iter()
                                            .count()
                                            .try_into()
                                            .unwrap(),
                                        get_time(),
                                        Raid::Akasha,
                                        Item::NoDrop,
                                        Option::Some("None".to_string()),
                                    ));
                                    let _ = confy::store("dorothy-egui", &local_settings_copy);
                                }
                                if ui.button("Hollow Key").clicked() {
                                    let _ = &self.droplog.drop.push(ItemDrop::new(
                                        local_settings_copy
                                            .droplog
                                            .drop
                                            .clone()
                                            .iter()
                                            .count()
                                            .try_into()
                                            .unwrap(),
                                        get_time(),
                                        Raid::Akasha,
                                        Item::HollowKey,
                                        Option::Some("None".to_string()),
                                    ));
                                    let _ = local_settings_copy.droplog.drop.push(ItemDrop::new(
                                        local_settings_copy
                                            .droplog
                                            .drop
                                            .clone()
                                            .iter()
                                            .count()
                                            .try_into()
                                            .unwrap(),
                                        get_time(),
                                        Raid::Akasha,
                                        Item::HollowKey,
                                        Option::Some("None".to_string()),
                                    ));
                                    let _ = confy::store("dorothy-egui", &local_settings_copy);
                                }
                                if ui.button("Champion Merit").clicked() {
                                    let _ = &self.droplog.drop.push(ItemDrop::new(
                                        local_settings_copy
                                            .droplog
                                            .drop
                                            .clone()
                                            .iter()
                                            .count()
                                            .try_into()
                                            .unwrap(),
                                        get_time(),
                                        Raid::Akasha,
                                        Item::ChampionMerit,
                                        Option::Some("None".to_string()),
                                    ));
                                    let _ = local_settings_copy.droplog.drop.push(ItemDrop::new(
                                        local_settings_copy
                                            .droplog
                                            .drop
                                            .clone()
                                            .iter()
                                            .count()
                                            .try_into()
                                            .unwrap(),
                                        get_time(),
                                        Raid::Akasha,
                                        Item::ChampionMerit,
                                        Option::Some("None".to_string()),
                                    ));
                                    let _ = confy::store("dorothy-egui", &local_settings_copy);
                                }
                                if ui.button("Supreme Merit").clicked() {
                                    let _ = &self.droplog.drop.push(ItemDrop::new(
                                        local_settings_copy
                                            .droplog
                                            .drop
                                            .clone()
                                            .iter()
                                            .count()
                                            .try_into()
                                            .unwrap(),
                                        get_time(),
                                        Raid::Akasha,
                                        Item::SupremeMerit,
                                        Option::Some("None".to_string()),
                                    ));
                                    let _ = local_settings_copy.droplog.drop.push(ItemDrop::new(
                                        local_settings_copy
                                            .droplog
                                            .drop
                                            .clone()
                                            .iter()
                                            .count()
                                            .try_into()
                                            .unwrap(),
                                        get_time(),
                                        Raid::Akasha,
                                        Item::SupremeMerit,
                                        Option::Some("None".to_string()),
                                    ));
                                    let _ = confy::store("dorothy-egui", &local_settings_copy);
                                }
                                if ui.button("Legendary Merit").clicked() {
                                    let _ = &self.droplog.drop.push(ItemDrop::new(
                                        local_settings_copy
                                            .droplog
                                            .drop
                                            .clone()
                                            .iter()
                                            .count()
                                            .try_into()
                                            .unwrap(),
                                        get_time(),
                                        Raid::Akasha,
                                        Item::LegendaryMerit,
                                        Option::Some("None".to_string()),
                                    ));
                                    let _ = local_settings_copy.droplog.drop.push(ItemDrop::new(
                                        local_settings_copy
                                            .droplog
                                            .drop
                                            .clone()
                                            .iter()
                                            .count()
                                            .try_into()
                                            .unwrap(),
                                        get_time(),
                                        Raid::Akasha,
                                        Item::LegendaryMerit,
                                        Option::Some("None".to_string()),
                                    ));
                                    let _ = confy::store("dorothy-egui", &local_settings_copy);
                                }
                                ui.end_row();
                                if ui.button("Silver Centrum").clicked() {
                                    let _ = &self.droplog.drop.push(ItemDrop::new(
                                        local_settings_copy
                                            .droplog
                                            .drop
                                            .clone()
                                            .iter()
                                            .count()
                                            .try_into()
                                            .unwrap(),
                                        get_time(),
                                        Raid::Akasha,
                                        Item::SilverCentrum,
                                        Option::Some("None".to_string()),
                                    ));
                                    let _ = local_settings_copy.droplog.drop.push(ItemDrop::new(
                                        local_settings_copy
                                            .droplog
                                            .drop
                                            .clone()
                                            .iter()
                                            .count()
                                            .try_into()
                                            .unwrap(),
                                        get_time(),
                                        Raid::Akasha,
                                        Item::SilverCentrum,
                                        Option::Some("None".to_string()),
                                    ));
                                    let _ = confy::store("dorothy-egui", &local_settings_copy);
                                }
                                if ui.button("Weapon Plus Mark +1").clicked() {
                                    let _ = &self.droplog.drop.push(ItemDrop::new(
                                        local_settings_copy
                                            .droplog
                                            .drop
                                            .clone()
                                            .iter()
                                            .count()
                                            .try_into()
                                            .unwrap(),
                                        get_time(),
                                        Raid::Akasha,
                                        Item::WeaponPlusMark1,
                                        Option::Some("None".to_string()),
                                    ));
                                    let _ = local_settings_copy.droplog.drop.push(ItemDrop::new(
                                        local_settings_copy
                                            .droplog
                                            .drop
                                            .clone()
                                            .iter()
                                            .count()
                                            .try_into()
                                            .unwrap(),
                                        get_time(),
                                        Raid::Akasha,
                                        Item::WeaponPlusMark1,
                                        Option::Some("None".to_string()),
                                    ));
                                    let _ = confy::store("dorothy-egui", &local_settings_copy);
                                }
                                if ui.button("Weapon Plus Mark +2").clicked() {
                                    let _ = &self.droplog.drop.push(ItemDrop::new(
                                        local_settings_copy
                                            .droplog
                                            .drop
                                            .clone()
                                            .iter()
                                            .count()
                                            .try_into()
                                            .unwrap(),
                                        get_time(),
                                        Raid::Akasha,
                                        Item::WeaponPlusMark2,
                                        Option::Some("None".to_string()),
                                    ));
                                    let _ = local_settings_copy.droplog.drop.push(ItemDrop::new(
                                        local_settings_copy
                                            .droplog
                                            .drop
                                            .clone()
                                            .iter()
                                            .count()
                                            .try_into()
                                            .unwrap(),
                                        get_time(),
                                        Raid::Akasha,
                                        Item::WeaponPlusMark2,
                                        Option::Some("None".to_string()),
                                    ));
                                    let _ = confy::store("dorothy-egui", &local_settings_copy);
                                }
                                if ui.button("Weapon Plus Mark +3").clicked() {
                                    let _ = &self.droplog.drop.push(ItemDrop::new(
                                        local_settings_copy
                                            .droplog
                                            .drop
                                            .clone()
                                            .iter()
                                            .count()
                                            .try_into()
                                            .unwrap(),
                                        get_time(),
                                        Raid::Akasha,
                                        Item::WeaponPlusMark3,
                                        Option::Some("None".to_string()),
                                    ));
                                    let _ = local_settings_copy.droplog.drop.push(ItemDrop::new(
                                        local_settings_copy
                                            .droplog
                                            .drop
                                            .clone()
                                            .iter()
                                            .count()
                                            .try_into()
                                            .unwrap(),
                                        get_time(),
                                        Raid::Akasha,
                                        Item::WeaponPlusMark3,
                                        Option::Some("None".to_string()),
                                    ));
                                    let _ = confy::store("dorothy-egui", &local_settings_copy);
                                }
                                ui.end_row();
                                if ui.button("Coronation Ring").clicked() {
                                    let _ = &self.droplog.drop.push(ItemDrop::new(
                                        local_settings_copy
                                            .droplog
                                            .drop
                                            .clone()
                                            .iter()
                                            .count()
                                            .try_into()
                                            .unwrap(),
                                        get_time(),
                                        Raid::Akasha,
                                        Item::CoronationRing,
                                        Option::Some("None".to_string()),
                                    ));
                                    let _ = local_settings_copy.droplog.drop.push(ItemDrop::new(
                                        local_settings_copy
                                            .droplog
                                            .drop
                                            .clone()
                                            .iter()
                                            .count()
                                            .try_into()
                                            .unwrap(),
                                        get_time(),
                                        Raid::Akasha,
                                        Item::CoronationRing,
                                        Option::Some("None".to_string()),
                                    ));
                                    let _ = confy::store("dorothy-egui", &local_settings_copy);
                                }
                                if ui.button("Lineage Ring").clicked() {
                                    let _ = &self.droplog.drop.push(ItemDrop::new(
                                        local_settings_copy
                                            .droplog
                                            .drop
                                            .clone()
                                            .iter()
                                            .count()
                                            .try_into()
                                            .unwrap(),
                                        get_time(),
                                        Raid::Akasha,
                                        Item::LineageRing,
                                        Option::Some("None".to_string()),
                                    ));
                                    let _ = local_settings_copy.droplog.drop.push(ItemDrop::new(
                                        local_settings_copy
                                            .droplog
                                            .drop
                                            .clone()
                                            .iter()
                                            .count()
                                            .try_into()
                                            .unwrap(),
                                        get_time(),
                                        Raid::Akasha,
                                        Item::LineageRing,
                                        Option::Some("None".to_string()),
                                    ));
                                    let _ = confy::store("dorothy-egui", &local_settings_copy);
                                }
                                if ui.button("Intricacy Ring").clicked() {
                                    let _ = &self.droplog.drop.push(ItemDrop::new(
                                        local_settings_copy
                                            .droplog
                                            .drop
                                            .clone()
                                            .iter()
                                            .count()
                                            .try_into()
                                            .unwrap(),
                                        get_time(),
                                        Raid::Akasha,
                                        Item::IntricacyRing,
                                        Option::Some("None".to_string()),
                                    ));
                                    let _ = local_settings_copy.droplog.drop.push(ItemDrop::new(
                                        local_settings_copy
                                            .droplog
                                            .drop
                                            .clone()
                                            .iter()
                                            .count()
                                            .try_into()
                                            .unwrap(),
                                        get_time(),
                                        Raid::Akasha,
                                        Item::IntricacyRing,
                                        Option::Some("None".to_string()),
                                    ));
                                    let _ = confy::store("dorothy-egui", &local_settings_copy);
                                }
                                if ui.button("Gold Brick").clicked() {
                                    let _ = &self.droplog.drop.push(ItemDrop::new(
                                        local_settings_copy
                                            .droplog
                                            .drop
                                            .clone()
                                            .iter()
                                            .count()
                                            .try_into()
                                            .unwrap(),
                                        get_time(),
                                        Raid::Akasha,
                                        Item::GoldBrick,
                                        Option::Some("None".to_string()),
                                    ));
                                    let _ = local_settings_copy.droplog.drop.push(ItemDrop::new(
                                        local_settings_copy
                                            .droplog
                                            .drop
                                            .clone()
                                            .iter()
                                            .count()
                                            .try_into()
                                            .unwrap(),
                                        get_time(),
                                        Raid::Akasha,
                                        Item::GoldBrick,
                                        Option::Some("None".to_string()),
                                    ));
                                    let _ = confy::store("dorothy-egui", &local_settings_copy);
                                }
                                ui.end_row();
                            });
                    }
                    if self.config.app_settings.current_ui_tab == UiTab::PBHL {
                        egui::Grid::new("pbhl_item_grid")
                            .spacing((15., 20.))
                            .show(ui, |ui| {
                                if ui.button("No Drop").clicked() {
                                    let _ = &self.droplog.drop.push(ItemDrop::new(
                                        local_settings_copy
                                            .droplog
                                            .drop
                                            .clone()
                                            .iter()
                                            .count()
                                            .try_into()
                                            .unwrap(),
                                        get_time(),
                                        Raid::PBHL,
                                        Item::NoDrop,
                                        Option::Some("None".to_string()),
                                    ));
                                    let _ = local_settings_copy.droplog.drop.push(ItemDrop::new(
                                        local_settings_copy
                                            .droplog
                                            .drop
                                            .clone()
                                            .iter()
                                            .count()
                                            .try_into()
                                            .unwrap(),
                                        get_time(),
                                        Raid::PBHL,
                                        Item::NoDrop,
                                        Option::Some(format!("{}", self.pbhl_honors)),
                                    ));
                                    let _ = confy::store("dorothy-egui", &local_settings_copy);
                                }
                                if ui.button("Coronation Ring").clicked() {
                                    let _ = &self.droplog.drop.push(ItemDrop::new(
                                        local_settings_copy
                                            .droplog
                                            .drop
                                            .clone()
                                            .iter()
                                            .count()
                                            .try_into()
                                            .unwrap(),
                                        get_time(),
                                        Raid::PBHL,
                                        Item::CoronationRing,
                                        Option::Some(format!("{}", self.pbhl_honors)),
                                    ));
                                    let _ = local_settings_copy.droplog.drop.push(ItemDrop::new(
                                        local_settings_copy
                                            .droplog
                                            .drop
                                            .clone()
                                            .iter()
                                            .count()
                                            .try_into()
                                            .unwrap(),
                                        get_time(),
                                        Raid::PBHL,
                                        Item::CoronationRing,
                                        Option::Some(format!("{}", self.pbhl_honors)),
                                    ));
                                    let _ = confy::store("dorothy-egui", &local_settings_copy);
                                }
                                if ui.button("Lineage Ring").clicked() {
                                    let _ = &self.droplog.drop.push(ItemDrop::new(
                                        local_settings_copy
                                            .droplog
                                            .drop
                                            .clone()
                                            .iter()
                                            .count()
                                            .try_into()
                                            .unwrap(),
                                        get_time(),
                                        Raid::PBHL,
                                        Item::LineageRing,
                                        Option::Some(format!("{}", self.pbhl_honors)),
                                    ));
                                    let _ = local_settings_copy.droplog.drop.push(ItemDrop::new(
                                        local_settings_copy
                                            .droplog
                                            .drop
                                            .clone()
                                            .iter()
                                            .count()
                                            .try_into()
                                            .unwrap(),
                                        get_time(),
                                        Raid::PBHL,
                                        Item::LineageRing,
                                        Option::Some(format!("{}", self.pbhl_honors)),
                                    ));
                                    let _ = confy::store("dorothy-egui", &local_settings_copy);
                                }
                                if ui.button("Intricacy Ring").clicked() {
                                    let _ = &self.droplog.drop.push(ItemDrop::new(
                                        local_settings_copy
                                            .droplog
                                            .drop
                                            .clone()
                                            .iter()
                                            .count()
                                            .try_into()
                                            .unwrap(),
                                        get_time(),
                                        Raid::PBHL,
                                        Item::IntricacyRing,
                                        Option::Some(format!("{}", self.pbhl_honors)),
                                    ));
                                    let _ = local_settings_copy.droplog.drop.push(ItemDrop::new(
                                        local_settings_copy
                                            .droplog
                                            .drop
                                            .clone()
                                            .iter()
                                            .count()
                                            .try_into()
                                            .unwrap(),
                                        get_time(),
                                        Raid::PBHL,
                                        Item::IntricacyRing,
                                        Option::Some(format!("{}", self.pbhl_honors)),
                                    ));
                                    let _ = confy::store("dorothy-egui", &local_settings_copy);
                                }
                                if ui.button("Gold Brick").clicked() {
                                    let _ = &self.droplog.drop.push(ItemDrop::new(
                                        local_settings_copy
                                            .droplog
                                            .drop
                                            .clone()
                                            .iter()
                                            .count()
                                            .try_into()
                                            .unwrap(),
                                        get_time(),
                                        Raid::PBHL,
                                        Item::GoldBrick,
                                        Option::Some(format!("{}", self.pbhl_honors)),
                                    ));
                                    let _ = local_settings_copy.droplog.drop.push(ItemDrop::new(
                                        local_settings_copy
                                            .droplog
                                            .drop
                                            .clone()
                                            .iter()
                                            .count()
                                            .try_into()
                                            .unwrap(),
                                        get_time(),
                                        Raid::PBHL,
                                        Item::GoldBrick,
                                        Option::Some(format!("{}", self.pbhl_honors)),
                                    ));
                                    let _ = confy::store("dorothy-egui", &local_settings_copy);
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
                    if self.config.app_settings.current_ui_tab == UiTab::GOHL {
                        egui::Grid::new("gohl_item_grid")
                            .spacing((15., 20.))
                            .show(ui, |ui| {
                                if ui.button("No Drop").clicked() {
                                    let _ = &self.droplog.drop.push(ItemDrop::new(
                                        local_settings_copy
                                            .droplog
                                            .drop
                                            .clone()
                                            .iter()
                                            .count()
                                            .try_into()
                                            .unwrap(),
                                        get_time(),
                                        Raid::GOHL,
                                        Item::NoDrop,
                                        Option::Some("None".to_string()),
                                    ));
                                    let _ = local_settings_copy.droplog.drop.push(ItemDrop::new(
                                        local_settings_copy
                                            .droplog
                                            .drop
                                            .clone()
                                            .iter()
                                            .count()
                                            .try_into()
                                            .unwrap(),
                                        get_time(),
                                        Raid::GOHL,
                                        Item::NoDrop,
                                        Option::Some("None".to_string()),
                                    ));
                                    let _ = confy::store("dorothy-egui", &local_settings_copy);
                                }
                                if ui.button("Verdant Azurite").clicked() {
                                    let _ = &self.droplog.drop.push(ItemDrop::new(
                                        local_settings_copy
                                            .droplog
                                            .drop
                                            .clone()
                                            .iter()
                                            .count()
                                            .try_into()
                                            .unwrap(),
                                        get_time(),
                                        Raid::GOHL,
                                        Item::VerdantAzurite,
                                        Option::Some("None".to_string()),
                                    ));
                                    let _ = local_settings_copy.droplog.drop.push(ItemDrop::new(
                                        local_settings_copy
                                            .droplog
                                            .drop
                                            .clone()
                                            .iter()
                                            .count()
                                            .try_into()
                                            .unwrap(),
                                        get_time(),
                                        Raid::GOHL,
                                        Item::VerdantAzurite,
                                        Option::Some("None".to_string()),
                                    ));
                                    let _ = confy::store("dorothy-egui", &local_settings_copy);
                                }
                                if ui.button("Champion Merit").clicked() {
                                    let _ = &self.droplog.drop.push(ItemDrop::new(
                                        local_settings_copy
                                            .droplog
                                            .drop
                                            .clone()
                                            .iter()
                                            .count()
                                            .try_into()
                                            .unwrap(),
                                        get_time(),
                                        Raid::GOHL,
                                        Item::ChampionMerit,
                                        Option::Some("None".to_string()),
                                    ));
                                    let _ = local_settings_copy.droplog.drop.push(ItemDrop::new(
                                        local_settings_copy
                                            .droplog
                                            .drop
                                            .clone()
                                            .iter()
                                            .count()
                                            .try_into()
                                            .unwrap(),
                                        get_time(),
                                        Raid::GOHL,
                                        Item::ChampionMerit,
                                        Option::Some("None".to_string()),
                                    ));
                                    let _ = confy::store("dorothy-egui", &local_settings_copy);
                                }
                                if ui.button("Supreme Merit").clicked() {
                                    let _ = &self.droplog.drop.push(ItemDrop::new(
                                        local_settings_copy
                                            .droplog
                                            .drop
                                            .clone()
                                            .iter()
                                            .count()
                                            .try_into()
                                            .unwrap(),
                                        get_time(),
                                        Raid::GOHL,
                                        Item::SupremeMerit,
                                        Option::Some("None".to_string()),
                                    ));
                                    let _ = local_settings_copy.droplog.drop.push(ItemDrop::new(
                                        local_settings_copy
                                            .droplog
                                            .drop
                                            .clone()
                                            .iter()
                                            .count()
                                            .try_into()
                                            .unwrap(),
                                        get_time(),
                                        Raid::GOHL,
                                        Item::SupremeMerit,
                                        Option::Some("None".to_string()),
                                    ));
                                    let _ = confy::store("dorothy-egui", &local_settings_copy);
                                }
                                if ui.button("Legendary Merit").clicked() {
                                    let _ = &self.droplog.drop.push(ItemDrop::new(
                                        local_settings_copy
                                            .droplog
                                            .drop
                                            .clone()
                                            .iter()
                                            .count()
                                            .try_into()
                                            .unwrap(),
                                        get_time(),
                                        Raid::GOHL,
                                        Item::LegendaryMerit,
                                        Option::Some("None".to_string()),
                                    ));
                                    let _ = local_settings_copy.droplog.drop.push(ItemDrop::new(
                                        local_settings_copy
                                            .droplog
                                            .drop
                                            .clone()
                                            .iter()
                                            .count()
                                            .try_into()
                                            .unwrap(),
                                        get_time(),
                                        Raid::GOHL,
                                        Item::LegendaryMerit,
                                        Option::Some("None".to_string()),
                                    ));
                                    let _ = confy::store("dorothy-egui", &local_settings_copy);
                                }
                                ui.end_row();
                                if ui.button("Silver Centrum").clicked() {
                                    let _ = &self.droplog.drop.push(ItemDrop::new(
                                        local_settings_copy
                                            .droplog
                                            .drop
                                            .clone()
                                            .iter()
                                            .count()
                                            .try_into()
                                            .unwrap(),
                                        get_time(),
                                        Raid::GOHL,
                                        Item::SilverCentrum,
                                        Option::Some("None".to_string()),
                                    ));
                                    let _ = local_settings_copy.droplog.drop.push(ItemDrop::new(
                                        local_settings_copy
                                            .droplog
                                            .drop
                                            .clone()
                                            .iter()
                                            .count()
                                            .try_into()
                                            .unwrap(),
                                        get_time(),
                                        Raid::GOHL,
                                        Item::SilverCentrum,
                                        Option::Some("None".to_string()),
                                    ));
                                    let _ = confy::store("dorothy-egui", &local_settings_copy);
                                }
                                if ui.button("Coronation Ring").clicked() {
                                    let _ = &self.droplog.drop.push(ItemDrop::new(
                                        local_settings_copy
                                            .droplog
                                            .drop
                                            .clone()
                                            .iter()
                                            .count()
                                            .try_into()
                                            .unwrap(),
                                        get_time(),
                                        Raid::GOHL,
                                        Item::CoronationRing,
                                        Option::Some("None".to_string()),
                                    ));
                                    let _ = local_settings_copy.droplog.drop.push(ItemDrop::new(
                                        local_settings_copy
                                            .droplog
                                            .drop
                                            .clone()
                                            .iter()
                                            .count()
                                            .try_into()
                                            .unwrap(),
                                        get_time(),
                                        Raid::GOHL,
                                        Item::CoronationRing,
                                        Option::Some("None".to_string()),
                                    ));
                                    let _ = confy::store("dorothy-egui", &local_settings_copy);
                                }
                                if ui.button("Lineage Ring").clicked() {
                                    let _ = &self.droplog.drop.push(ItemDrop::new(
                                        local_settings_copy
                                            .droplog
                                            .drop
                                            .clone()
                                            .iter()
                                            .count()
                                            .try_into()
                                            .unwrap(),
                                        get_time(),
                                        Raid::GOHL,
                                        Item::LineageRing,
                                        Option::Some("None".to_string()),
                                    ));
                                    let _ = local_settings_copy.droplog.drop.push(ItemDrop::new(
                                        local_settings_copy
                                            .droplog
                                            .drop
                                            .clone()
                                            .iter()
                                            .count()
                                            .try_into()
                                            .unwrap(),
                                        get_time(),
                                        Raid::GOHL,
                                        Item::LineageRing,
                                        Option::Some("None".to_string()),
                                    ));
                                    let _ = confy::store("dorothy-egui", &local_settings_copy);
                                }
                                if ui.button("Intricacy Ring").clicked() {
                                    let _ = &self.droplog.drop.push(ItemDrop::new(
                                        local_settings_copy
                                            .droplog
                                            .drop
                                            .clone()
                                            .iter()
                                            .count()
                                            .try_into()
                                            .unwrap(),
                                        get_time(),
                                        Raid::GOHL,
                                        Item::IntricacyRing,
                                        Option::Some("None".to_string()),
                                    ));
                                    let _ = local_settings_copy.droplog.drop.push(ItemDrop::new(
                                        local_settings_copy
                                            .droplog
                                            .drop
                                            .clone()
                                            .iter()
                                            .count()
                                            .try_into()
                                            .unwrap(),
                                        get_time(),
                                        Raid::GOHL,
                                        Item::IntricacyRing,
                                        Option::Some("None".to_string()),
                                    ));
                                    let _ = confy::store("dorothy-egui", &local_settings_copy);
                                }
                                if ui.button("Gold Brick").clicked() {
                                    let _ = &self.droplog.drop.push(ItemDrop::new(
                                        local_settings_copy
                                            .droplog
                                            .drop
                                            .clone()
                                            .iter()
                                            .count()
                                            .try_into()
                                            .unwrap(),
                                        get_time(),
                                        Raid::GOHL,
                                        Item::GoldBrick,
                                        Option::Some("None".to_string()),
                                    ));
                                    let _ = local_settings_copy.droplog.drop.push(ItemDrop::new(
                                        local_settings_copy
                                            .droplog
                                            .drop
                                            .clone()
                                            .iter()
                                            .count()
                                            .try_into()
                                            .unwrap(),
                                        get_time(),
                                        Raid::GOHL,
                                        Item::GoldBrick,
                                        Option::Some("None".to_string()),
                                    ));
                                    let _ = confy::store("dorothy-egui", &local_settings_copy);
                                }
                                ui.end_row();
                            });
                    }
                });
            ui.add_space(50.);
            egui::warn_if_debug_build(ui);
        });
        if false {
            egui::Window::new("EasterEgg").show(ctx, |ui| {
                // TODO IMPLEMENT
            });
        }
    }
}
