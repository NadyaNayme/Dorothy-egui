use crate::*;
use eframe::{
    egui::{self, RichText, Visuals},
    epaint::Color32,
    epi,
};
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "persistence", serde(default))]
pub struct AppDorothy {
    pub name: String,
    pub droplog: DropLog,
    pub pbhl_honors: PBHLHonors,
    pub selected_raid: Raid,
    pub config: AppSettings,
}

impl Default for AppDorothy {
    fn default() -> Self {
        Self {
            name: "Dorothy".to_string(),
            droplog: DropLog::default(),
            pbhl_honors: PBHLHonors::Ignore,
            selected_raid: Raid::None,
            config: AppSettings::default(),
        }
    }
}

impl epi::App for AppDorothy {
    fn name(&self) -> &str {
        "Dorothy"
    }

    #[cfg(feature = "persistence")]
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

        #[cfg(any(target_os = "windows", target_os = "macos", target_os = "linux"))]
        let need_to_update = check_for_update().unwrap();
        #[cfg(any(target_os = "windows", target_os = "macos", target_os = "linux"))]
        match need_to_update {
            ReleaseStatus::NewVersion => {
                let update_worked = self_update();
                match update_worked {
                    Ok(()) => {
                        self.config.app_settings.auto_update_status = 1;
                    }
                    Err(e) => {
                        println!("{}", e);
                        self.config.app_settings.auto_update_status = 2;
                    }
                }
            }
            ReleaseStatus::UpToDate => {
                self.config.app_settings.auto_update_status = 3;
            }
        }
    }

    /// Called by the framework to save state before shutdown.
    #[cfg(feature = "persistence")]
    fn save(&mut self, storage: &mut dyn epi::Storage) {
        epi::set_value(storage, epi::APP_KEY, self);
    }

    fn auto_save_interval(&self) -> std::time::Duration {
        std::time::Duration::from_secs(3)
    }

    #[allow(unused_variables)]
    fn update(&mut self, ctx: &egui::Context, frame: &epi::Frame) {
        let Self {
            name,
            config,
            droplog,
            pbhl_honors: pbhlhonors,
            selected_raid: selectedraid
        } = self;

        if !ctx.is_using_pointer() {
            ctx.set_pixels_per_point(self.config.app_settings.ui_scale);
            let mut style = (*ctx.style()).clone();
            let header_font_size = self.config.app_settings.body_font_size * 1.3;
            let small_font_size = self.config.app_settings.body_font_size * 0.8;
            style.text_styles = [
                (
                    crate::app::egui::TextStyle::Heading,
                    crate::app::egui::FontId::new(
                        header_font_size,
                        crate::app::egui::FontFamily::Proportional,
                    ),
                ),
                (
                    crate::app::egui::TextStyle::Body,
                    crate::app::egui::FontId::new(
                        self.config.app_settings.body_font_size,
                        crate::app::egui::FontFamily::Proportional,
                    ),
                ),
                (
                    crate::app::egui::TextStyle::Monospace,
                    crate::app::egui::FontId::new(
                        self.config.app_settings.body_font_size,
                        crate::app::egui::FontFamily::Proportional,
                    ),
                ),
                (
                    crate::app::egui::TextStyle::Button,
                    crate::app::egui::FontId::new(
                        self.config.app_settings.body_font_size,
                        crate::app::egui::FontFamily::Proportional,
                    ),
                ),
                (
                    crate::app::egui::TextStyle::Small,
                    crate::app::egui::FontId::new(
                        small_font_size,
                        crate::app::egui::FontFamily::Proportional,
                    ),
                ),
            ]
            .into();
            ctx.set_style(style);
        }

        #[cfg(not(target_arch = "wasm32"))]
        if self.config.app_settings.auto_update_status == 1 {
            egui::Window::new("Updated!").show(ctx, |ui| {
                    ui.heading("Please Restart!");
                    ui.add_space(5.);
                    ui.label("Dorothy has automatically updated to a new release. A restart is required. Please check the Github repo for what has changed.");
                });
        } else if self.config.app_settings.auto_update_status == 2 {
            egui::Window::new("Update Failed...").show(ctx, |ui| {
                    ui.heading("404: Dorothy's Brain Not Found");
                    ui.add_space(5.);
                    ui.label("Dorothy has failed to update. Please open an issue on Github for Nadya to investigate.");
                });
        }

        if self.config.app_settings.dark_mode {
            ctx.set_visuals(Visuals::dark());
            self.config.app_settings.dark_mode = true;
        } else {
            ctx.set_visuals(Visuals::light());
            self.config.app_settings.dark_mode = false;
        }
        
        if self.config.app_settings.always_on_top {
            frame.set_always_on_top(true)
        } else {
            frame.set_always_on_top(false)
        }

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    #[cfg(not(target_arch = "wasm32"))]
                    if ui.button("Export").clicked() {
                        let _ = export(self.config.droplog.clone());
                        if self.config.app_settings.reset_on_export {
                            self.config.droplog.drop = DropLog::reset();
                        }
                    }
                    ui.separator();
                    if ui.button("Reset Droplog").clicked() {
                        self.config.droplog.drop = DropLog::reset();
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    if ui.button("Quit & Save").clicked() {
                        frame.quit();
                    }
                });
                ui.menu_button("View", |ui| {
                    ui.style_mut().wrap = Some(false);
                    ui.checkbox(
                        &mut self.config.app_settings.left_panel_visible,
                        "Show Left Panel",
                    );
                    ui.checkbox(
                        &mut self.config.app_settings.right_panel_visible,
                        "Show Right/Bottom Panel",
                    );
                    ui.checkbox(
                        &mut self.config.app_settings.move_right_to_bottom,
                        "Move Right Panel to Bottom",
                    );
                    ui.checkbox(
                        &mut self.config.app_settings.toggle_active_items,
                        "Adjust Center Panel Features",
                    );
                    ui.checkbox(
                        &mut self.config.app_settings.show_all_drops,
                        "Show All Drop Totals",
                    );
                });
                ui.menu_button("Settings", |ui| {
                    ui.style_mut().wrap = Some(false);
                    #[cfg(not(target_arch = "wasm32"))]
                    ui.checkbox(
                        &mut self.config.app_settings.auto_update_enabled,
                        "Auto Update on Startup",
                    );
                    ui.checkbox(&mut self.config.app_settings.dark_mode, "Dark Mode");
                    #[cfg(not(target_arch = "wasm32"))]
                    ui.checkbox(&mut self.config.app_settings.always_on_top, "Always On Top");
                    #[cfg(not(target_arch = "wasm32"))]
                    ui.checkbox(
                        &mut self.config.app_settings.reset_on_export,
                        "Reset Counts on Export",
                    );
                    ui.checkbox(
                        &mut self.config.app_settings.droprate_by_kills,
                        "Calculate droprates by total kills",
                    );
                });
                ui.menu_button("Helpful Links", |ui| {
                    ui.style_mut().wrap = Some(false);
                    ui.hyperlink_to(
                        "Latest Dorothy Release",
                        "https://github.com/NadyaNayme/Dorothy-egui/releases/latest",
                    );
                    ui.add_space(3.);
                    ui.hyperlink_to("GBF Wiki", "https://gbf.wiki/Main_Page");
                    ui.add_space(3.);
                    ui.hyperlink_to("Online Tools", "https://www.granblue.party/");
                    ui.add_space(3.);
                    ui.hyperlink_to("Raidfinder", "https://gbf.life/");
                    ui.add_space(3.);
                    ui.hyperlink_to("/r/Granblue_en", "https://www.reddit.com/r/Granblue_en/");
                });
            });
        });
        if self.config.app_settings.left_panel_visible {
            egui::SidePanel::left("left_side_panel")
                .min_width(180.)
                .max_width(400.)
                .show(ctx, |ui| {
                    ui.add_space(5.);
                    ui.heading("Drop Totals");
                    ui.add_space(3.);
                    ui.separator();
                    ui.add_space(1.);
                    egui::ScrollArea::vertical()
                        .auto_shrink([false, false])
                        .max_height(INFINITY)
                        .max_width(INFINITY)
                        .show(ui, |ui| {
                            if self.config.app_settings.current_ui_tab == UiTab::Akasha
                                || self.config.app_settings.show_all_drops
                            {
                                place_total_header(
                                    Raid::Akasha,
                                    Item::NoDrop,
                                    ChestType::Blue,
                                    &self.config,
                                    ui,
                                );
                                place_percentage_label(
                                    Raid::Akasha,
                                    Item::NoDrop,
                                    ChestType::None,
                                    &self.config,
                                    ui,
                                );
                                place_percentage_label(
                                    Raid::Akasha,
                                    Item::HollowKey,
                                    ChestType::Blue,
                                    &self.config,
                                    ui,
                                );
                                place_percentage_label(
                                    Raid::Akasha,
                                    Item::SilverCentrum,
                                    ChestType::Blue,
                                    &self.config,
                                    ui,
                                );
                                place_percentage_label(
                                    Raid::Akasha,
                                    Item::GoldBrick,
                                    ChestType::Blue,
                                    &self.config,
                                    ui,
                                );
                                place_percentage_label(
                                    Raid::Akasha,
                                    Item::CoronationRing,
                                    ChestType::Blue,
                                    &self.config,
                                    ui,
                                );
                                place_percentage_label(
                                    Raid::Akasha,
                                    Item::LineageRing,
                                    ChestType::Blue,
                                    &self.config,
                                    ui,
                                );
                                place_percentage_label(
                                    Raid::Akasha,
                                    Item::IntricacyRing,
                                    ChestType::Blue,
                                    &self.config,
                                    ui,
                                );
                                place_percentage_label(
                                    Raid::Akasha,
                                    Item::ChampionMerit,
                                    ChestType::Blue,
                                    &self.config,
                                    ui,
                                );
                                place_percentage_label(
                                    Raid::Akasha,
                                    Item::SupremeMerit,
                                    ChestType::Blue,
                                    &self.config,
                                    ui,
                                );
                                place_percentage_label(
                                    Raid::Akasha,
                                    Item::LegendaryMerit,
                                    ChestType::Blue,
                                    &self.config,
                                    ui,
                                );
                                place_percentage_label(
                                    Raid::Akasha,
                                    Item::WeaponPlusMark1,
                                    ChestType::Blue,
                                    &self.config,
                                    ui,
                                );
                                place_percentage_label(
                                    Raid::Akasha,
                                    Item::WeaponPlusMark2,
                                    ChestType::Blue,
                                    &self.config,
                                    ui,
                                );
                                place_percentage_label(
                                    Raid::Akasha,
                                    Item::WeaponPlusMark3,
                                    ChestType::Blue,
                                    &self.config,
                                    ui,
                                );
                            }
                            if self.config.app_settings.current_ui_tab == UiTab::PBHL
                                || self.config.app_settings.show_all_drops
                            {
                                place_total_header(
                                    Raid::PBHL,
                                    Item::NoDrop,
                                    ChestType::Blue,
                                    &self.config,
                                    ui,
                                );
                                place_percentage_label(
                                    Raid::PBHL,
                                    Item::NoDrop,
                                    ChestType::None,
                                    &self.config,
                                    ui,
                                );
                                place_percentage_label(
                                    Raid::PBHL,
                                    Item::GoldBrick,
                                    ChestType::Blue,
                                    &self.config,
                                    ui,
                                );
                                place_percentage_label(
                                    Raid::PBHL,
                                    Item::CoronationRing,
                                    ChestType::Blue,
                                    &self.config,
                                    ui,
                                );
                                place_percentage_label(
                                    Raid::PBHL,
                                    Item::LineageRing,
                                    ChestType::Blue,
                                    &self.config,
                                    ui,
                                );
                                place_percentage_label(
                                    Raid::PBHL,
                                    Item::IntricacyRing,
                                    ChestType::Blue,
                                    &self.config,
                                    ui,
                                );
                            }
                            if self.config.app_settings.current_ui_tab == UiTab::GOHL
                                || self.config.app_settings.show_all_drops
                            {
                                place_total_header(
                                    Raid::GOHL,
                                    Item::NoDrop,
                                    ChestType::Blue,
                                    &self.config,
                                    ui,
                                );
                                place_percentage_label(
                                    Raid::GOHL,
                                    Item::NoDrop,
                                    ChestType::None,
                                    &self.config,
                                    ui,
                                );
                                place_percentage_label(
                                    Raid::GOHL,
                                    Item::VerdantAzurite,
                                    ChestType::Blue,
                                    &self.config,
                                    ui,
                                );
                                place_percentage_label(
                                    Raid::GOHL,
                                    Item::SilverCentrum,
                                    ChestType::Blue,
                                    &self.config,
                                    ui,
                                );
                                place_percentage_label(
                                    Raid::GOHL,
                                    Item::GoldBrick,
                                    ChestType::Blue,
                                    &self.config,
                                    ui,
                                );
                                place_percentage_label(
                                    Raid::GOHL,
                                    Item::CoronationRing,
                                    ChestType::Blue,
                                    &self.config,
                                    ui,
                                );
                                place_percentage_label(
                                    Raid::GOHL,
                                    Item::LineageRing,
                                    ChestType::Blue,
                                    &self.config,
                                    ui,
                                );
                                place_percentage_label(
                                    Raid::GOHL,
                                    Item::IntricacyRing,
                                    ChestType::Blue,
                                    &self.config,
                                    ui,
                                );
                                place_percentage_label(
                                    Raid::GOHL,
                                    Item::ChampionMerit,
                                    ChestType::Blue,
                                    &self.config,
                                    ui,
                                );
                                place_percentage_label(
                                    Raid::GOHL,
                                    Item::SupremeMerit,
                                    ChestType::Blue,
                                    &self.config,
                                    ui,
                                );
                                place_percentage_label(
                                    Raid::GOHL,
                                    Item::LegendaryMerit,
                                    ChestType::Blue,
                                    &self.config,
                                    ui,
                                );
                            }

                            if self.config.app_settings.current_ui_tab == UiTab::Hosts
                                || self.config.app_settings.show_all_drops
                            {
                                place_total_header(
                                    Raid::UBHL,
                                    Item::NoDrop,
                                    ChestType::Blue,
                                    &self.config,
                                    ui,
                                );
                                place_percentage_label(
                                    Raid::UBHL,
                                    Item::GoldBrick,
                                    ChestType::Host,
                                    &self.config,
                                    ui,
                                );
                                place_percentage_label(
                                    Raid::UBHL,
                                    Item::GoldBrick,
                                    ChestType::Flip,
                                    &self.config,
                                    ui,
                                );
                                place_percentage_label(
                                    Raid::PBHL,
                                    Item::GoldBrick,
                                    ChestType::Host,
                                    &self.config,
                                    ui,
                                );
                                place_percentage_label(
                                    Raid::Xeno,
                                    Item::GoldBrick,
                                    ChestType::Flip,
                                    &self.config,
                                    ui,
                                );
                                place_percentage_label(
                                    Raid::Huanglong,
                                    Item::GoldBrick,
                                    ChestType::Host,
                                    &self.config,
                                    ui,
                                );
                                place_percentage_label(
                                    Raid::Qilin,
                                    Item::GoldBrick,
                                    ChestType::Host,
                                    &self.config,
                                    ui,
                                );
                                place_percentage_label(
                                    Raid::HLQL,
                                    Item::GoldBrick,
                                    ChestType::Host,
                                    &self.config,
                                    ui,
                                );
                            }

                            if self.config.app_settings.current_ui_tab == UiTab::SixDragons
                                || self.config.app_settings.show_all_drops
                            {
                                place_total_header(
                                    Raid::Wilnas,
                                    Item::NoDrop,
                                    ChestType::Blue,
                                    &self.config,
                                    ui,
                                );
                                place_percentage_label(
                                    Raid::Wilnas,
                                    Item::NoDrop,
                                    ChestType::None,
                                    &self.config,
                                    ui,
                                );
                                place_percentage_label(
                                    Raid::Wilnas,
                                    Item::BrimstoneEarrings,
                                    ChestType::Blue,
                                    &self.config,
                                    ui,
                                );
                                place_percentage_label(
                                    Raid::Wilnas,
                                    Item::EternitySand,
                                    ChestType::Blue,
                                    &self.config,
                                    ui,
                                );
                                place_total_header(
                                    Raid::Wamdus,
                                    Item::NoDrop,
                                    ChestType::Blue,
                                    &self.config,
                                    ui,
                                );
                                place_percentage_label(
                                    Raid::Wamdus,
                                    Item::NoDrop,
                                    ChestType::None,
                                    &self.config,
                                    ui,
                                );
                                place_percentage_label(
                                    Raid::Wamdus,
                                    Item::PermafrostEarrings,
                                    ChestType::Blue,
                                    &self.config,
                                    ui,
                                );
                                place_percentage_label(
                                    Raid::Wamdus,
                                    Item::EternitySand,
                                    ChestType::Blue,
                                    &self.config,
                                    ui,
                                );
                                place_total_header(
                                    Raid::Galleon,
                                    Item::NoDrop,
                                    ChestType::Blue,
                                    &self.config,
                                    ui,
                                );
                                place_percentage_label(
                                    Raid::Galleon,
                                    Item::NoDrop,
                                    ChestType::None,
                                    &self.config,
                                    ui,
                                );
                                place_percentage_label(
                                    Raid::Galleon,
                                    Item::BrickearthEarrings,
                                    ChestType::Blue,
                                    &self.config,
                                    ui,
                                );
                                place_percentage_label(
                                    Raid::Galleon,
                                    Item::EternitySand,
                                    ChestType::Blue,
                                    &self.config,
                                    ui,
                                );
                                place_total_header(
                                    Raid::Ewiyar,
                                    Item::NoDrop,
                                    ChestType::Blue,
                                    &self.config,
                                    ui,
                                );
                                place_percentage_label(
                                    Raid::Ewiyar,
                                    Item::NoDrop,
                                    ChestType::None,
                                    &self.config,
                                    ui,
                                );
                                place_percentage_label(
                                    Raid::Ewiyar,
                                    Item::JetstreamEarrings,
                                    ChestType::Blue,
                                    &self.config,
                                    ui,
                                );
                                place_percentage_label(
                                    Raid::Ewiyar,
                                    Item::EternitySand,
                                    ChestType::Blue,
                                    &self.config,
                                    ui,
                                );
                                place_total_header(
                                    Raid::LuWoh,
                                    Item::NoDrop,
                                    ChestType::Blue,
                                    &self.config,
                                    ui,
                                );
                                place_percentage_label(
                                    Raid::LuWoh,
                                    Item::NoDrop,
                                    ChestType::None,
                                    &self.config,
                                    ui,
                                );
                                place_percentage_label(
                                    Raid::LuWoh,
                                    Item::SunbeamEarrings,
                                    ChestType::Blue,
                                    &self.config,
                                    ui,
                                );
                                place_percentage_label(
                                    Raid::LuWoh,
                                    Item::EternitySand,
                                    ChestType::Blue,
                                    &self.config,
                                    ui,
                                );
                                place_total_header(
                                    Raid::Fediel,
                                    Item::NoDrop,
                                    ChestType::Blue,
                                    &self.config,
                                    ui,
                                );
                                place_percentage_label(
                                    Raid::Fediel,
                                    Item::NoDrop,
                                    ChestType::None,
                                    &self.config,
                                    ui,
                                );
                                place_percentage_label(
                                    Raid::Fediel,
                                    Item::NightshadeEarrings,
                                    ChestType::Blue,
                                    &self.config,
                                    ui,
                                );
                                place_percentage_label(
                                    Raid::Fediel,
                                    Item::EternitySand,
                                    ChestType::Blue,
                                    &self.config,
                                    ui,
                                );
                                
                            }
                            
                        });
                    ui.add_space(50.);
                    egui::warn_if_debug_build(ui);
                });
        }

        if self.config.app_settings.right_panel_visible
            && !self.config.app_settings.move_right_to_bottom
        {
            egui::SidePanel::right("right_side_panel")
                .min_width(120.)
                .max_width(400.)
                .show(ctx, |ui| {
                    ui.heading("Recent Drops");
                    ui.add_space(5.);

                    egui::ScrollArea::vertical()
                        .auto_shrink([false, false])
                        .max_height(INFINITY)
                        .max_width(INFINITY)
                        .show(ui, |ui| {
                            for drop in self
                                .config
                                .droplog
                                .drop
                                .clone()
                                .into_iter()
                                .filter(|x| x.item != Item::NoDrop)
                                .rev()
                            {
                                ui.horizontal(|ui| {
                                    ui.spacing_mut().item_spacing.x = 0.0;
                                    if drop.chest == ChestType::Host
                                        || drop.chest == ChestType::Flip
                                            && drop.item == Item::GoldBrick
                                            && drop.raid != Raid::Xeno
                                    {
                                        let mut gold_brick_text_color =
                                            Color32::from_rgb(255, 221, 26);
                                        if !self.config.app_settings.dark_mode {
                                            gold_brick_text_color = Color32::from_rgb(187, 152, 10);
                                        }
                                        if ui
                                            .add(
                                                egui::Label::new(
                                                    RichText::new(format!(
                                                        "{} - {}",
                                                        drop.item, drop.raid
                                                    ))
                                                    .color(gold_brick_text_color),
                                                )
                                                .sense(egui::Sense::click()),
                                            )
                                            .on_hover_text(format!(
                                                "On {} from {} in a {}",
                                                drop.date_obtained, drop.raid, drop.chest
                                            ))
                                            .clicked()
                                        {
                                            let _ = self
                                                .config
                                                .droplog
                                                .drop
                                                .retain(|x| x.drop_id != drop.drop_id);
                                        }
                                        ui.add_space(3.)
                                    } else if drop.item == Item::GoldBrick
                                        && drop.raid == Raid::PBHL
                                        && drop.chest == ChestType::Blue
                                    {
                                        let mut gold_brick_text_color =
                                            Color32::from_rgb(255, 221, 26);
                                        if !self.config.app_settings.dark_mode {
                                            gold_brick_text_color = Color32::from_rgb(183, 138, 15);
                                        }
                                        if ui
                                            .add(
                                                egui::Label::new(
                                                    RichText::new(format!(
                                                        "{} - {}",
                                                        drop.item, drop.raid
                                                    ))
                                                    .color(gold_brick_text_color),
                                                )
                                                .sense(egui::Sense::click()),
                                            )
                                            .on_hover_text(format!(
                                                "On {} from {} {}",
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
                                        }
                                        ui.add_space(3.)
                                    } else if drop.item == Item::GoldBrick {
                                        let mut gold_brick_text_color =
                                            Color32::from_rgb(255, 221, 26);
                                        if !self.config.app_settings.dark_mode {
                                            gold_brick_text_color = Color32::from_rgb(187, 152, 10);
                                        }
                                        if ui
                                            .add(
                                                egui::Label::new(
                                                    RichText::new(format!(
                                                        "{} - {}",
                                                        drop.item, drop.raid
                                                    ))
                                                    .color(gold_brick_text_color),
                                                )
                                                .sense(egui::Sense::click()),
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
                                        }
                                        ui.add_space(3.)
                                    } else {
                                        let mut drop_honors = drop.honors.as_ref().unwrap();
                                        let empty_string = "".to_string();
                                        if drop.raid != Raid::PBHL {
                                            drop_honors = &empty_string;
                                        }
                                        if ui
                                            .add(
                                                egui::Label::new(format!("{}", drop.item))
                                                    .sense(egui::Sense::click()),
                                            )
                                            .on_hover_text(format!(
                                                "On {} from {} {}",
                                                drop.date_obtained, drop.raid, drop_honors
                                            ))
                                            .clicked()
                                        {
                                            let _ = self
                                                .config
                                                .droplog
                                                .drop
                                                .retain(|x| x.drop_id != drop.drop_id);
                                        }
                                        ui.add_space(3.)
                                    }
                                });
                            }
                        });
                });
        }

        if self.config.app_settings.right_panel_visible
            && self.config.app_settings.move_right_to_bottom
        {
            egui::TopBottomPanel::bottom("bottom_panel")
                .min_height(100.)
                .max_height(800.)
                .resizable(true)
                .height_range(std::ops::RangeInclusive::new(100., 800.))
                .show(ctx, |ui| {
                    ui.add_space(15.);
                    ui.heading("Recent Drops");
                    ui.add_space(5.);

                    egui::ScrollArea::vertical()
                        .auto_shrink([false, false])
                        .max_height(INFINITY)
                        .max_width(INFINITY)
                        .show(ui, |ui| {
                            for drop in self
                                .config
                                .droplog
                                .drop
                                .clone()
                                .into_iter()
                                .filter(|x| x.item != Item::NoDrop)
                                .rev()
                            {
                                ui.horizontal(|ui| {
                                    ui.spacing_mut().item_spacing.x = 0.0;
                                    if drop.chest == ChestType::Host
                                        || drop.chest == ChestType::Flip
                                            && drop.item == Item::GoldBrick
                                            && drop.raid != Raid::Xeno
                                    {
                                        let mut gold_brick_text_color =
                                            Color32::from_rgb(255, 221, 26);
                                        if !self.config.app_settings.dark_mode {
                                            gold_brick_text_color = Color32::from_rgb(187, 152, 10);
                                        }
                                        if ui
                                            .add(
                                                egui::Label::new(
                                                    RichText::new(format!(
                                                        "{} - {}",
                                                        drop.item, drop.raid
                                                    ))
                                                    .color(gold_brick_text_color),
                                                )
                                                .sense(egui::Sense::click()),
                                            )
                                            .on_hover_text(format!(
                                                "On {} from {} in a {}",
                                                drop.date_obtained, drop.raid, drop.chest
                                            ))
                                            .clicked()
                                        {
                                            let _ = self
                                                .config
                                                .droplog
                                                .drop
                                                .retain(|x| x.drop_id != drop.drop_id);
                                        }
                                        ui.add_space(3.)
                                    } else if drop.item == Item::GoldBrick
                                        && drop.raid == Raid::PBHL
                                        && drop.chest == ChestType::Blue
                                    {
                                        let mut gold_brick_text_color =
                                            Color32::from_rgb(255, 221, 26);
                                        if !self.config.app_settings.dark_mode {
                                            gold_brick_text_color = Color32::from_rgb(183, 138, 15);
                                        }
                                        if ui
                                            .add(
                                                egui::Label::new(
                                                    RichText::new(format!(
                                                        "{} - {}",
                                                        drop.item, drop.raid
                                                    ))
                                                    .color(gold_brick_text_color),
                                                )
                                                .sense(egui::Sense::click()),
                                            )
                                            .on_hover_text(format!(
                                                "On {} from {} {}",
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
                                        }
                                        ui.add_space(3.)
                                    } else if drop.item == Item::GoldBrick {
                                        let mut gold_brick_text_color =
                                            Color32::from_rgb(255, 221, 26);
                                        if !self.config.app_settings.dark_mode {
                                            gold_brick_text_color = Color32::from_rgb(187, 152, 10);
                                        }
                                        if ui
                                            .add(
                                                egui::Label::new(
                                                    RichText::new(format!(
                                                        "{} - {}",
                                                        drop.item, drop.raid
                                                    ))
                                                    .color(gold_brick_text_color),
                                                )
                                                .sense(egui::Sense::click()),
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
                                        }
                                        ui.add_space(3.)
                                    } else {
                                        let mut drop_honors = drop.honors.as_ref().unwrap();
                                        let empty_string = "".to_string();
                                        if drop.raid != Raid::PBHL {
                                            drop_honors = &empty_string;
                                        }
                                        if ui
                                            .add(
                                                egui::Label::new(format!("{}", drop.item))
                                                    .sense(egui::Sense::click()),
                                            )
                                            .on_hover_text(format!(
                                                "On {} from {} {}",
                                                drop.date_obtained, drop.raid, drop_honors
                                            ))
                                            .clicked()
                                        {
                                            let _ = self
                                                .config
                                                .droplog
                                                .drop
                                                .retain(|x| x.drop_id != drop.drop_id);
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
                if self.config.app_settings.active_items_2[20]
                    && ui
                        .selectable_value(
                            &mut self.config.app_settings.current_ui_tab,
                            UiTab::Pulls,
                            "Pull Calculator",
                        )
                        .changed()
                {
                    frame.set_window_title("Dorothy - Pull Calculator");
                }
                if self.config.app_settings.active_items_2[21]
                    && ui
                        .selectable_value(
                            &mut self.config.app_settings.current_ui_tab,
                            UiTab::Akasha,
                            "Akasha",
                        )
                        .changed()
                {
                    frame.set_window_title("Dorothy - Akasha");
                }
                if self.config.app_settings.active_items_2[22]
                    && ui
                        .selectable_value(
                            &mut self.config.app_settings.current_ui_tab,
                            UiTab::PBHL,
                            "PBHL",
                        )
                        .changed()
                {
                    frame.set_window_title("Dorothy - PBHL");
                }
                if self.config.app_settings.active_items_2[23]
                    && ui
                        .selectable_value(
                            &mut self.config.app_settings.current_ui_tab,
                            UiTab::GOHL,
                            "GOHL",
                        )
                        .changed()
                {
                    frame.set_window_title("Dorothy - GOHL");
                }
                if self.config.app_settings.active_items_2[24]
                    && ui
                        .selectable_value(
                            &mut self.config.app_settings.current_ui_tab,
                            UiTab::Hosts,
                            "Hosts",
                        )
                        .changed()
                {
                    frame.set_window_title("Dorothy - Hosts");
                }
                if self.config.app_settings.active_items_2[25]
                    && ui
                    .selectable_value(
                        &mut self.config.app_settings.current_ui_tab,
                        UiTab::SixDragons,
                        "6 Dragons",
                    )
                    .changed()
                {
                    frame.set_window_title("Dorothy - 6 Dragons");
                }
                if self.config.app_settings.active_items_2[26]
                    && ui
                    .selectable_value(
                        &mut self.config.app_settings.current_ui_tab,
                        UiTab::EternitySand,
                        "Eternity Sands",
                    )
                    .changed()
                {
                    frame.set_window_title("Dorothy - Eternity Sands");
                }
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
                                    self.config
                                        .app_settings
                                        .shrimp_amount
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
                                    self.config
                                        .app_settings
                                        .shrimp_amount
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
                                    self.config
                                        .app_settings
                                        .shrimp_amount
                                        .parse::<f32>()
                                        .unwrap_or_default(),
                                )
                            }
                        });
                        ui.add_space(5.);
                        ui.horizontal(|ui| {
                            ui.label("Ebi  Fry: ");
                            let response = ui.add(egui::TextEdit::singleline(
                                &mut self.config.app_settings.shrimp_amount,
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
                                    self.config
                                        .app_settings
                                        .shrimp_amount
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
                            .spacing((
                                self.config.app_settings.grid_spacing_x,
                                self.config.app_settings.grid_spacing_y,
                            ))
                            .show(ui, |ui| {
                                ui.style_mut().wrap = Some(false);
                                if !self.config.app_settings.vertical_grid {
                                    let akasha_drops = vec![
                                        Item::NoDrop,
                                        Item::HollowKey,
                                        Item::SilverCentrum,
                                        Item::GoldBrick,
                                        Item::CoronationRing,
                                        Item::ChampionMerit,
                                        Item::WeaponPlusMark1,
                                        Item::LineageRing,
                                        Item::SupremeMerit,
                                        Item::WeaponPlusMark2,
                                        Item::IntricacyRing,
                                        Item::LegendaryMerit,
                                        Item::WeaponPlusMark3,
                                    ];
                                    for (pos, drop) in akasha_drops.iter().enumerate() {
                                        let chest = match drop {
                                            Item::NoDrop => ChestType::None,
                                            _ => ChestType::Blue,
                                        };
                                        if self.config.app_settings.active_items[pos] {
                                            ui.horizontal(|ui| {
                                                place_image_button_combo(
                                                    *drop,
                                                    Raid::Akasha,
                                                    chest,
                                                    &self.pbhl_honors,
                                                    &mut self.config,
                                                    ui,
                                                )
                                            });
                                        }
                                        if pos == 3 || pos == 6 || pos == 9 {
                                            ui.end_row();
                                        }
                                    }
                                } else {
                                    let akasha_drops = vec![
                                        Item::NoDrop,
                                        Item::HollowKey,
                                        Item::SilverCentrum,
                                        Item::GoldBrick,
                                        Item::CoronationRing,
                                        Item::ChampionMerit,
                                        Item::WeaponPlusMark1,
                                        Item::LineageRing,
                                        Item::SupremeMerit,
                                        Item::WeaponPlusMark2,
                                        Item::IntricacyRing,
                                        Item::LegendaryMerit,
                                        Item::WeaponPlusMark3,
                                    ];
                                    for (pos, drop) in akasha_drops.iter().enumerate() {
                                        let chest = match drop {
                                            Item::NoDrop => ChestType::None,
                                            _ => ChestType::Blue,
                                        };
                                        if self.config.app_settings.active_items[pos] {
                                            ui.vertical(|ui| {
                                                place_image_button_combo(
                                                    *drop,
                                                    Raid::Akasha,
                                                    chest,
                                                    &self.pbhl_honors,
                                                    &mut self.config,
                                                    ui,
                                                )
                                            });
                                        }
                                        if pos == 3 || pos == 6 || pos == 9 {
                                            ui.end_row();
                                        }
                                    }
                                }
                            });
                    }
                    if self.config.app_settings.current_ui_tab == UiTab::PBHL {
                        egui::Grid::new("pbhl_item_grid")
                            .spacing((
                                self.config.app_settings.grid_spacing_x,
                                self.config.app_settings.grid_spacing_y,
                            ))
                            .show(ui, |ui| {
                                ui.style_mut().wrap = Some(false);
                                if !self.config.app_settings.vertical_grid {
                                    let pbhl_drops = vec![
                                        Item::NoDrop,
                                        Item::GoldBrick,
                                        Item::CoronationRing,
                                        Item::LineageRing,
                                        Item::IntricacyRing,
                                    ];
                                    for (pos, drop) in pbhl_drops.iter().enumerate() {
                                        let chest = match drop {
                                            Item::NoDrop => ChestType::None,
                                            _ => ChestType::Blue,
                                        };
                                        if self.config.app_settings.active_items[pos + 13] {
                                            ui.horizontal(|ui| {
                                                place_image_button_combo(
                                                    *drop,
                                                    Raid::PBHL,
                                                    chest,
                                                    &self.pbhl_honors,
                                                    &mut self.config,
                                                    ui,
                                                )
                                            });
                                        }
                                        if pos == 2 {
                                            ui.end_row();
                                        }
                                    }
                                } else {
                                    let pbhl_drops = vec![
                                        Item::NoDrop,
                                        Item::GoldBrick,
                                        Item::CoronationRing,
                                        Item::LineageRing,
                                        Item::IntricacyRing,
                                    ];
                                    for (pos, drop) in pbhl_drops.iter().enumerate() {
                                        let chest = match drop {
                                            Item::NoDrop => ChestType::None,
                                            _ => ChestType::Blue,
                                        };
                                        if self.config.app_settings.active_items[pos + 13] {
                                            ui.vertical(|ui| {
                                                place_image_button_combo(
                                                    *drop,
                                                    Raid::PBHL,
                                                    chest,
                                                    &self.pbhl_honors,
                                                    &mut self.config,
                                                    ui,
                                                )
                                            });
                                        }
                                        if pos == 2 {
                                            ui.end_row();
                                        }
                                    }
                                }
                            });

                        ui.add_space(20.);
                        ui.heading("Honors");
                        ui.label("Select the closest match rounding down.");
                        ui.add_space(5.);

                        egui::Grid::new("pbhl_honors_grid")
                            .spacing((15., 10.))
                            .show(ui, |ui| {
                                ui.style_mut().wrap = Some(false);
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
                            .spacing((
                                self.config.app_settings.grid_spacing_x,
                                self.config.app_settings.grid_spacing_y,
                            ))
                            .show(ui, |ui| {
                                ui.style_mut().wrap = Some(false);
                                if !self.config.app_settings.vertical_grid {
                                    let gohl_drops = vec![
                                        Item::NoDrop,
                                        Item::VerdantAzurite,
                                        Item::SilverCentrum,
                                        Item::GoldBrick,
                                        Item::CoronationRing,
                                        Item::ChampionMerit,
                                        Item::LineageRing,
                                        Item::SupremeMerit,
                                        Item::IntricacyRing,
                                        Item::LegendaryMerit,
                                    ];
                                    for (pos, drop) in gohl_drops.iter().enumerate() {
                                        let chest = match drop {
                                            Item::NoDrop => ChestType::None,
                                            _ => ChestType::Blue,
                                        };
                                        if self.config.app_settings.active_items[pos + 18] {
                                            ui.horizontal(|ui| {
                                                place_image_button_combo(
                                                    *drop,
                                                    Raid::GOHL,
                                                    chest,
                                                    &self.pbhl_honors,
                                                    &mut self.config,
                                                    ui,
                                                )
                                            });
                                        }
                                        if pos == 3 || pos == 5 || pos == 7 || pos == 9 {
                                            ui.end_row();
                                        }
                                    }
                                } else {
                                    let gohl_drops = vec![
                                        Item::NoDrop,
                                        Item::VerdantAzurite,
                                        Item::SilverCentrum,
                                        Item::GoldBrick,
                                        Item::CoronationRing,
                                        Item::ChampionMerit,
                                        Item::LineageRing,
                                        Item::SupremeMerit,
                                        Item::IntricacyRing,
                                        Item::LegendaryMerit,
                                    ];
                                    for (pos, drop) in gohl_drops.iter().enumerate() {
                                        let chest = match drop {
                                            Item::NoDrop => ChestType::None,
                                            _ => ChestType::Blue,
                                        };
                                        if self.config.app_settings.active_items[pos + 18] {
                                            ui.vertical(|ui| {
                                                place_image_button_combo(
                                                    *drop,
                                                    Raid::GOHL,
                                                    chest,
                                                    &self.pbhl_honors,
                                                    &mut self.config,
                                                    ui,
                                                )
                                            });
                                        }
                                        if pos == 3 || pos == 5 || pos == 7 || pos == 9 {
                                            ui.end_row();
                                        }
                                    }
                                }
                            });
                    }
                    if self.config.app_settings.current_ui_tab == UiTab::Hosts {
                        egui::Grid::new("hosts_item_grid")
                            .spacing((
                                self.config.app_settings.grid_spacing_x,
                                self.config.app_settings.grid_spacing_y,
                            ))
                            .show(ui, |ui| {
                                ui.style_mut().wrap = Some(false);
                                if !self.config.app_settings.vertical_grid {
                                    if self.config.app_settings.active_items[28] {
                                        ui.horizontal(|ui| {
                                            place_image_button_combo(
                                                Item::GoldBrick,
                                                Raid::UBHL,
                                                ChestType::Host,
                                                &self.pbhl_honors,
                                                &mut self.config,
                                                ui,
                                            );
                                        });
                                    }
                                    if self.config.app_settings.active_items[29] {
                                        ui.horizontal(|ui| {
                                            place_image_button_combo(
                                                Item::GoldBrick,
                                                Raid::UBHL,
                                                ChestType::Flip,
                                                &self.pbhl_honors,
                                                &mut self.config,
                                                ui,
                                            );
                                        });
                                    }
                                    if self.config.app_settings.active_items[30] {
                                        ui.horizontal(|ui| {
                                            place_image_button_combo(
                                                Item::GoldBrick,
                                                Raid::PBHL,
                                                ChestType::Host,
                                                &self.pbhl_honors,
                                                &mut self.config,
                                                ui,
                                            );
                                        });
                                    }
                                    if self.config.app_settings.active_items[31] {
                                        ui.horizontal(|ui| {
                                            place_image_button_combo(
                                                Item::GoldBrick,
                                                Raid::Xeno,
                                                ChestType::Flip,
                                                &self.pbhl_honors,
                                                &mut self.config,
                                                ui,
                                            );
                                        });
                                    }
                                    ui.end_row();
                                    if self.config.app_settings.active_items_2[0] {
                                        ui.horizontal(|ui| {
                                            place_image_button_combo(
                                                Item::GoldBrick,
                                                Raid::Huanglong,
                                                ChestType::Host,
                                                &self.pbhl_honors,
                                                &mut self.config,
                                                ui,
                                            );
                                        });
                                    }
                                    if self.config.app_settings.active_items_2[1] {
                                        ui.horizontal(|ui| {
                                            place_image_button_combo(
                                                Item::GoldBrick,
                                                Raid::Qilin,
                                                ChestType::Host,
                                                &self.pbhl_honors,
                                                &mut self.config,
                                                ui,
                                            );
                                        });
                                    }
                                    if self.config.app_settings.active_items_2[2] {
                                        ui.horizontal(|ui| {
                                            place_image_button_combo(
                                                Item::GoldBrick,
                                                Raid::HLQL,
                                                ChestType::Host,
                                                &self.pbhl_honors,
                                                &mut self.config,
                                                ui,
                                            );
                                        });
                                    }
                                } else {
                                    if self.config.app_settings.active_items[28] {
                                        ui.vertical(|ui| {
                                            place_image_button_combo(
                                                Item::GoldBrick,
                                                Raid::UBHL,
                                                ChestType::Host,
                                                &self.pbhl_honors,
                                                &mut self.config,
                                                ui,
                                            );
                                        });
                                    }
                                    if self.config.app_settings.active_items[29] {
                                        ui.vertical(|ui| {
                                            place_image_button_combo(
                                                Item::GoldBrick,
                                                Raid::UBHL,
                                                ChestType::Flip,
                                                &self.pbhl_honors,
                                                &mut self.config,
                                                ui,
                                            );
                                        });
                                    }
                                    if self.config.app_settings.active_items[30] {
                                        ui.vertical(|ui| {
                                            place_image_button_combo(
                                                Item::GoldBrick,
                                                Raid::PBHL,
                                                ChestType::Host,
                                                &self.pbhl_honors,
                                                &mut self.config,
                                                ui,
                                            );
                                        });
                                    }
                                    if self.config.app_settings.active_items[31] {
                                        ui.vertical(|ui| {
                                            place_image_button_combo(
                                                Item::GoldBrick,
                                                Raid::Xeno,
                                                ChestType::Flip,
                                                &self.pbhl_honors,
                                                &mut self.config,
                                                ui,
                                            );
                                        });
                                    }
                                    ui.end_row();
                                    if self.config.app_settings.active_items_2[0] {
                                        ui.vertical(|ui| {
                                            place_image_button_combo(
                                                Item::GoldBrick,
                                                Raid::Huanglong,
                                                ChestType::Host,
                                                &self.pbhl_honors,
                                                &mut self.config,
                                                ui,
                                            );
                                        });
                                    }
                                    if self.config.app_settings.active_items_2[1] {
                                        ui.vertical(|ui| {
                                            place_image_button_combo(
                                                Item::GoldBrick,
                                                Raid::Qilin,
                                                ChestType::Host,
                                                &self.pbhl_honors,
                                                &mut self.config,
                                                ui,
                                            );
                                        });
                                    }
                                    if self.config.app_settings.active_items_2[2] {
                                        ui.vertical(|ui| {
                                            place_image_button_combo(
                                                Item::GoldBrick,
                                                Raid::HLQL,
                                                ChestType::Host,
                                                &self.pbhl_honors,
                                                &mut self.config,
                                                ui,
                                            );
                                        });
                                    }
                                }
                            });
                    }
                    if self.config.app_settings.current_ui_tab == UiTab::SixDragons {
                        egui::Grid::new("six_dragons_item_grid")
                            .spacing((
                                self.config.app_settings.grid_spacing_x,
                                self.config.app_settings.grid_spacing_y,
                            ))
                            .show(ui, |ui| {
                                ui.style_mut().wrap = Some(false);
                                if !self.config.app_settings.vertical_grid {
                                    let six_dragon_drops = vec![
                                        Item::NoDrop,
                                        Item::BrimstoneEarrings,
                                        Item::PermafrostEarrings,
                                        Item::BrickearthEarrings,
                                        Item::JetstreamEarrings,
                                        Item::SunbeamEarrings,
                                        Item::NightshadeEarrings,
                                        Item::EternitySand,
                                    ];
                                    for (pos, drop) in six_dragon_drops.iter().enumerate() {
                                        let chest = match drop {
                                            Item::NoDrop => ChestType::None,
                                            _ => ChestType::Blue,
                                        };
                                        if pos == 0 {
                                            ui.horizontal(|ui| {
                                                place_image_button_combo(
                                                    *drop,
                                                    self.selected_raid,
                                                    chest,
                                                    &self.pbhl_honors,
                                                    &mut self.config,
                                                    ui,
                                                )
                                            });
                                        }
                                        if pos == 1 {
                                            ui.horizontal(|ui| {
                                                place_image_button_combo(
                                                    *drop,
                                                    Raid::Wilnas,
                                                    chest,
                                                    &self.pbhl_honors,
                                                    &mut self.config,
                                                    ui,
                                                )
                                            });
                                        }
                                        if pos == 2 {
                                            ui.horizontal(|ui| {
                                                place_image_button_combo(
                                                    *drop,
                                                    Raid::Wamdus,
                                                    chest,
                                                    &self.pbhl_honors,
                                                    &mut self.config,
                                                    ui,
                                                )
                                            });
                                        }
                                        if pos == 3 {
                                            ui.horizontal(|ui| {
                                                place_image_button_combo(
                                                    *drop,
                                                    Raid::Galleon,
                                                    chest,
                                                    &self.pbhl_honors,
                                                    &mut self.config,
                                                    ui,
                                                )
                                            });
                                        }
                                        if pos == 4 {
                                            ui.horizontal(|ui| {
                                                place_image_button_combo(
                                                    *drop,
                                                    Raid::Ewiyar,
                                                    chest,
                                                    &self.pbhl_honors,
                                                    &mut self.config,
                                                    ui,
                                                )
                                            });
                                        }
                                        if pos == 5 {
                                            ui.horizontal(|ui| {
                                                place_image_button_combo(
                                                    *drop,
                                                    Raid::LuWoh,
                                                    chest,
                                                    &self.pbhl_honors,
                                                    &mut self.config,
                                                    ui,
                                                )
                                            });
                                        }
                                        if pos == 6 {
                                            ui.horizontal(|ui| {
                                                place_image_button_combo(
                                                    *drop,
                                                    Raid::Fediel,
                                                    chest,
                                                    &self.pbhl_honors,
                                                    &mut self.config,
                                                    ui,
                                                )
                                            });
                                        }
                                        if pos == 7 {
                                            ui.horizontal(|ui| {
                                                place_image_button_combo(
                                                    *drop,
                                                    self.selected_raid,
                                                    chest,
                                                    &self.pbhl_honors,
                                                    &mut self.config,
                                                    ui,
                                                )
                                            });
                                        }
                                    if pos == 0 || pos == 3 || pos == 6 {
                                        ui.end_row();
                                    }
                                    }
                                } else {
                                    let six_dragon_drops = vec![
                                        Item::NoDrop,
                                        Item::BrimstoneEarrings,
                                        Item::PermafrostEarrings,
                                        Item::BrickearthEarrings,
                                        Item::JetstreamEarrings,
                                        Item::SunbeamEarrings,
                                        Item::NightshadeEarrings,
                                        Item::EternitySand,
                                    ];
                                    for (pos, drop) in six_dragon_drops.iter().enumerate() {
                                        let chest = match drop {
                                            Item::NoDrop => ChestType::None,
                                            _ => ChestType::Blue,
                                        };
                                        if pos == 0 {
                                            ui.vertical(|ui| {
                                                place_image_button_combo(
                                                    *drop,
                                                    self.selected_raid,
                                                    chest,
                                                    &self.pbhl_honors,
                                                    &mut self.config,
                                                    ui,
                                                )
                                            });
                                        }
                                        if pos == 1 {
                                            ui.vertical(|ui| {
                                                place_image_button_combo(
                                                    *drop,
                                                    Raid::Wilnas,
                                                    chest,
                                                    &self.pbhl_honors,
                                                    &mut self.config,
                                                    ui,
                                                )
                                            });
                                        }
                                        if pos == 2 {
                                            ui.vertical(|ui| {
                                                place_image_button_combo(
                                                    *drop,
                                                    Raid::Wamdus,
                                                    chest,
                                                    &self.pbhl_honors,
                                                    &mut self.config,
                                                    ui,
                                                )
                                            });
                                        }
                                        if pos == 3 {
                                            ui.vertical(|ui| {
                                                place_image_button_combo(
                                                    *drop,
                                                    Raid::Galleon,
                                                    chest,
                                                    &self.pbhl_honors,
                                                    &mut self.config,
                                                    ui,
                                                )
                                            });
                                        }
                                        if pos == 4 {
                                            ui.vertical(|ui| {
                                                place_image_button_combo(
                                                    *drop,
                                                    Raid::Ewiyar,
                                                    chest,
                                                    &self.pbhl_honors,
                                                    &mut self.config,
                                                    ui,
                                                )
                                            });
                                        }
                                        if pos == 5 {
                                            ui.vertical(|ui| {
                                                place_image_button_combo(
                                                    *drop,
                                                    Raid::LuWoh,
                                                    chest,
                                                    &self.pbhl_honors,
                                                    &mut self.config,
                                                    ui,
                                                )
                                            });
                                        }
                                        if pos == 6 {
                                            ui.vertical(|ui| {
                                                place_image_button_combo(
                                                    *drop,
                                                    Raid::Fediel,
                                                    chest,
                                                    &self.pbhl_honors,
                                                    &mut self.config,
                                                    ui,
                                                )
                                            });
                                        }
                                        if pos == 7 {
                                            ui.vertical(|ui| {
                                                place_image_button_combo(
                                                    *drop,
                                                    self.selected_raid,
                                                    chest,
                                                    &self.pbhl_honors,
                                                    &mut self.config,
                                                    ui,
                                                )
                                            });
                                        }
                                        if pos == 0 || pos == 3 || pos == 6 {
                                            ui.end_row();
                                        }
                                    }
                                }
                            });

                        ui.add_space(20.);
                        ui.heading("Current Raid");
                        ui.label("Select the current raid you are farming.");
                        ui.add_space(5.);

                        egui::Grid::new("current_raid_grid")
                            .spacing((15., 10.))
                            .show(ui, |ui| {
                                ui.style_mut().wrap = Some(false);
                                ui.selectable_value(
                                    &mut self.selected_raid,
                                    Raid::None,
                                    "Don't Care",
                                );
                                ui.end_row();
                                ui.selectable_value(
                                    &mut self.selected_raid,
                                    Raid::Wilnas,
                                    "Wilnas",
                                );
                                ui.selectable_value(
                                    &mut self.selected_raid,
                                    Raid::Wamdus,
                                    "Wamdus",
                                );
                                ui.selectable_value(
                                    &mut self.selected_raid,
                                    Raid::Galleon,
                                    "Galleon",
                                );
                                ui.selectable_value(
                                    &mut self.selected_raid,
                                    Raid::Ewiyar,
                                    "Ewiyar",
                                );
                                ui.selectable_value(
                                    &mut self.selected_raid,
                                    Raid::LuWoh,
                                    "Lu Woh",
                                );
                                ui.selectable_value(
                                    &mut self.selected_raid,
                                    Raid::Fediel,
                                    "Fediel",
                                );
                            });
                    }
                });
        });
        if !self.config.app_settings.button_label_combo[0]
            && !self.config.app_settings.button_label_combo[1]
        {
            egui::Window::new("Warning: Can't log items!").show(ctx, |ui| {
                ui.label(
                    "Oh you've really screwed up now haven't you? Turn one of these back on to log your drops."
                        .to_string(),
                );
                ui
                    .checkbox(
                        &mut self.config.app_settings.button_label_combo[0],
                        "Show buttons",
                    );
                ui
                    .checkbox(
                        &mut self.config.app_settings.button_label_combo[1],
                        "Show icons",
                    );
            });
        }
        if self.config.app_settings.toggle_active_items {
            egui::Window::new("Center Panel Features").open(&mut self.config.app_settings.toggle_active_items).vscroll(true).show(ctx, |ui| {
                    ui.label("UI Scale.".to_string());
                    ui.add(egui::Slider::new(&mut self.config.app_settings.ui_scale, 1.0..=1.75));
                    ui.label("Font Size.".to_string());
                    ui.add(egui::Slider::new(&mut self.config.app_settings.body_font_size, 12.0..=40.0));

                    ui
                        .checkbox(
                            &mut self.config.app_settings.active_items_2[20],
                            "Show Pull Calculator Tab",
                        );
                    ui
                        .checkbox(
                            &mut self.config.app_settings.active_items_2[21],
                            "Show Akasha Tab",
                        );
                    ui
                        .checkbox(
                            &mut self.config.app_settings.active_items_2[22],
                            "Show PBHL Tab",
                        );
                    ui
                        .checkbox(
                            &mut self.config.app_settings.active_items_2[23],
                            "Show GOHL Tab",
                        );
                    ui
                        .checkbox(
                            &mut self.config.app_settings.active_items_2[24],
                            "Show Hosts Tab",
                        );
                    ui
                        .checkbox(
                            &mut self.config.app_settings.button_label_combo[0],
                            "Show buttons",
                        );
                    ui
                        .checkbox(
                            &mut self.config.app_settings.button_label_combo[1],
                            "Show icons",
                        );
                    ui
                        .checkbox(
                            &mut self.config.app_settings.active_items_2[26],
                            "Show item counts",
                        );
                ui
                    .checkbox(
                        &mut self.config.app_settings.vertical_grid,
                        "Change item count placement to below icons/buttons",
                    );
                ui.add_space(5.);
                ui.label("Grid X Spacing.".to_string());
                ui.add(egui::Slider::new(&mut self.config.app_settings.grid_spacing_x, 0.0..=50.0));
                ui.add_space(5.);
                ui.label("Grid Y Spacing.".to_string());
                ui.add(egui::Slider::new(&mut self.config.app_settings.grid_spacing_y, 0.0..=50.0));
                ui.add_space(5.);
                ui.label("The grid isn't smart enough to adjust but you can toggle specific items off here.".to_string());
                ui.heading("Akasha");
                    ui
                        .checkbox(
                            &mut self.config.app_settings.active_items[0],
                            "Show No Blue Box",
                        );
                    ui
                        .checkbox(
                            &mut self.config.app_settings.active_items[1],
                            "Show Hollow Key",
                        );
                    ui
                        .checkbox(
                            &mut self.config.app_settings.active_items[2],
                            "Show Silver Centrum",
                        );
                    ui
                        .checkbox(
                            &mut self.config.app_settings.active_items[3],
                            "Show Gold Bar",
                        );
                    ui
                        .checkbox(
                            &mut self.config.app_settings.active_items[4],
                            "Show Coronation Ring",
                        );
                    ui
                        .checkbox(
                            &mut self.config.app_settings.active_items[7],
                            "Show Lineage Ring",
                        );
                    ui
                        .checkbox(
                            &mut self.config.app_settings.active_items[10],
                            "Show Intricacy Ring",
                        );
                    ui
                        .checkbox(
                            &mut self.config.app_settings.active_items[5],
                            "Show Champion Merit",
                        );
                    ui
                        .checkbox(
                            &mut self.config.app_settings.active_items[8],
                            "Show Supreme Merit",
                        );
                    ui
                        .checkbox(
                            &mut self.config.app_settings.active_items[11],
                            "Show Legendary Merit",
                        );
                    ui
                        .checkbox(
                            &mut self.config.app_settings.active_items[6],
                            "Show +1 Mark",
                        );
                    ui
                        .checkbox(
                            &mut self.config.app_settings.active_items[9],
                            "Show +2 Mark",
                        );
                    ui
                        .checkbox(
                            &mut self.config.app_settings.active_items[12],
                            "Show +3 Mark",
                        );
                ui.heading("PBHL");
                    ui
                        .checkbox(
                            &mut self.config.app_settings.active_items[13],
                            "Show No Blue Box",
                        );
                    ui
                        .checkbox(
                            &mut self.config.app_settings.active_items[14],
                            "Show Gold Bar",
                        );
                    ui
                        .checkbox(
                            &mut self.config.app_settings.active_items[15],
                            "Show Coronation Ring",
                        );
                    ui
                        .checkbox(
                            &mut self.config.app_settings.active_items[16],
                            "Show Lineage Ring",
                        );
                    ui
                        .checkbox(
                            &mut self.config.app_settings.active_items[17],
                            "Show Lineage Ring",
                        );
                ui.heading("GOHL");
                    ui
                        .checkbox(
                            &mut self.config.app_settings.active_items[18],
                            "Show No Blue Box",
                        );
                    ui
                        .checkbox(
                            &mut self.config.app_settings.active_items[19],
                            "Show Verdant Azurite",
                        );
                    ui
                        .checkbox(
                            &mut self.config.app_settings.active_items[20],
                            "Show Silver Centrum",
                        );
                    ui
                        .checkbox(
                            &mut self.config.app_settings.active_items[21],
                            "Show Gold Bar",
                        );
                    ui
                        .checkbox(
                            &mut self.config.app_settings.active_items[22],
                            "Show Coronation Ring",
                        );
                    ui
                        .checkbox(
                            &mut self.config.app_settings.active_items[24],
                            "Show Lineage Ring",
                        );
                    ui
                        .checkbox(
                            &mut self.config.app_settings.active_items[26],
                            "Show Intricacy Ring",
                        );
                    ui
                        .checkbox(
                            &mut self.config.app_settings.active_items[23],
                            "Show Champion Merit",
                        );
                    ui
                        .checkbox(
                            &mut self.config.app_settings.active_items[25],
                            "Show Supreme Merit",
                        );
                    ui
                        .checkbox(
                            &mut self.config.app_settings.active_items[27],
                            "Show Legendary Merit",
                        );
                ui.heading("Hosts");
                    ui
                        .checkbox(
                            &mut self.config.app_settings.active_items[28],
                            "Show UBHL Host Gold Bar",
                        );
                    ui
                        .checkbox(
                            &mut self.config.app_settings.active_items[29],
                            "Show UBHL Flip Gold Bar",
                        );
                    ui
                        .checkbox(
                            &mut self.config.app_settings.active_items[30],
                            "Show PBHL Host Gold Bar",
                        );
                    ui
                        .checkbox(
                            &mut self.config.app_settings.active_items[31],
                            "Show Xeno Gold Bar",
                        );
                    ui
                        .checkbox(
                            &mut self.config.app_settings.active_items_2[0],
                            "Show Huanglong Gold Bar",
                        );
                    ui
                        .checkbox(
                            &mut self.config.app_settings.active_items_2[1],
                            "Show Qilin Gold Bar",
                        );
                        ui
                        .checkbox(
                            &mut self.config.app_settings.active_items_2[2],
                            "Show HLQL Gold Bar",
                        );

            });
        }
    }
}
