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
    pub config: AppSettings,
}

impl Default for AppDorothy {
    fn default() -> Self {
        Self {
            name: "Dorothy".to_string(),
            droplog: DropLog::default(),
            pbhl_honors: PBHLHonors::Ignore,
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
        } = self;

        let mut local_settings_copy = self.config.clone();

        if self.config.app_settings.dark_mode {
            ctx.set_visuals(Visuals::dark());
            self.config.app_settings.dark_mode = true;
        } else {
            ctx.set_visuals(Visuals::light());
            self.config.app_settings.dark_mode = false;
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
                    if ui
                        .checkbox(
                            &mut self.config.app_settings.left_panel_visible,
                            "Show Left Panel",
                        )
                        .clicked()
                    {
                        local_settings_copy.app_settings.left_panel_visible =
                            self.config.app_settings.left_panel_visible;
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
                    }
                    if ui
                        .checkbox(
                            &mut self.config.app_settings.toggle_active_items,
                            "Adjust Grid Features",
                        )
                        .clicked()
                    {
                        local_settings_copy.app_settings.toggle_active_items =
                            self.config.app_settings.toggle_active_items;
                    }
                    if ui
                        .checkbox(
                            &mut self.config.app_settings.show_all_drops,
                            "Show All Drop Totals",
                        )
                        .clicked()
                    {
                        local_settings_copy.app_settings.show_all_drops =
                            self.config.app_settings.show_all_drops;
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
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    if ui
                        .checkbox(&mut self.config.app_settings.always_on_top, "Always On Top")
                        .clicked()
                    {
                        local_settings_copy.app_settings.always_on_top =
                            self.config.app_settings.always_on_top;
                        frame.set_always_on_top(local_settings_copy.app_settings.always_on_top);
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
                    }
                });
                #[cfg(not(target_arch = "wasm32"))]
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
                                    &local_settings_copy,
                                    ui,
                                );
                                place_percentage_label(
                                    Raid::Akasha,
                                    Item::NoDrop,
                                    ChestType::None,
                                    &local_settings_copy,
                                    ui,
                                );
                                place_percentage_label(
                                    Raid::Akasha,
                                    Item::HollowKey,
                                    ChestType::Blue,
                                    &local_settings_copy,
                                    ui,
                                );
                                place_percentage_label(
                                    Raid::Akasha,
                                    Item::SilverCentrum,
                                    ChestType::Blue,
                                    &local_settings_copy,
                                    ui,
                                );
                                place_percentage_label(
                                    Raid::Akasha,
                                    Item::GoldBrick,
                                    ChestType::Blue,
                                    &local_settings_copy,
                                    ui,
                                );
                                place_percentage_label(
                                    Raid::Akasha,
                                    Item::CoronationRing,
                                    ChestType::Blue,
                                    &local_settings_copy,
                                    ui,
                                );
                                place_percentage_label(
                                    Raid::Akasha,
                                    Item::LineageRing,
                                    ChestType::Blue,
                                    &local_settings_copy,
                                    ui,
                                );
                                place_percentage_label(
                                    Raid::Akasha,
                                    Item::IntricacyRing,
                                    ChestType::Blue,
                                    &local_settings_copy,
                                    ui,
                                );
                                place_percentage_label(
                                    Raid::Akasha,
                                    Item::ChampionMerit,
                                    ChestType::Blue,
                                    &local_settings_copy,
                                    ui,
                                );
                                place_percentage_label(
                                    Raid::Akasha,
                                    Item::SupremeMerit,
                                    ChestType::Blue,
                                    &local_settings_copy,
                                    ui,
                                );
                                place_percentage_label(
                                    Raid::Akasha,
                                    Item::LegendaryMerit,
                                    ChestType::Blue,
                                    &local_settings_copy,
                                    ui,
                                );
                                place_percentage_label(
                                    Raid::Akasha,
                                    Item::WeaponPlusMark1,
                                    ChestType::Blue,
                                    &local_settings_copy,
                                    ui,
                                );
                                place_percentage_label(
                                    Raid::Akasha,
                                    Item::WeaponPlusMark2,
                                    ChestType::Blue,
                                    &local_settings_copy,
                                    ui,
                                );
                                place_percentage_label(
                                    Raid::Akasha,
                                    Item::WeaponPlusMark3,
                                    ChestType::Blue,
                                    &local_settings_copy,
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
                                    &local_settings_copy,
                                    ui,
                                );
                                place_percentage_label(
                                    Raid::PBHL,
                                    Item::NoDrop,
                                    ChestType::None,
                                    &local_settings_copy,
                                    ui,
                                );
                                place_percentage_label(
                                    Raid::PBHL,
                                    Item::GoldBrick,
                                    ChestType::Blue,
                                    &local_settings_copy,
                                    ui,
                                );
                                place_percentage_label(
                                    Raid::PBHL,
                                    Item::CoronationRing,
                                    ChestType::Blue,
                                    &local_settings_copy,
                                    ui,
                                );
                                place_percentage_label(
                                    Raid::PBHL,
                                    Item::LineageRing,
                                    ChestType::Blue,
                                    &local_settings_copy,
                                    ui,
                                );
                                place_percentage_label(
                                    Raid::PBHL,
                                    Item::IntricacyRing,
                                    ChestType::Blue,
                                    &local_settings_copy,
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
                                    &local_settings_copy,
                                    ui,
                                );
                                place_percentage_label(
                                    Raid::GOHL,
                                    Item::NoDrop,
                                    ChestType::None,
                                    &local_settings_copy,
                                    ui,
                                );
                                place_percentage_label(
                                    Raid::GOHL,
                                    Item::VerdantAzurite,
                                    ChestType::Blue,
                                    &local_settings_copy,
                                    ui,
                                );
                                place_percentage_label(
                                    Raid::GOHL,
                                    Item::SilverCentrum,
                                    ChestType::Blue,
                                    &local_settings_copy,
                                    ui,
                                );
                                place_percentage_label(
                                    Raid::GOHL,
                                    Item::GoldBrick,
                                    ChestType::Blue,
                                    &local_settings_copy,
                                    ui,
                                );
                                place_percentage_label(
                                    Raid::GOHL,
                                    Item::CoronationRing,
                                    ChestType::Blue,
                                    &local_settings_copy,
                                    ui,
                                );
                                place_percentage_label(
                                    Raid::GOHL,
                                    Item::LineageRing,
                                    ChestType::Blue,
                                    &local_settings_copy,
                                    ui,
                                );
                                place_percentage_label(
                                    Raid::GOHL,
                                    Item::IntricacyRing,
                                    ChestType::Blue,
                                    &local_settings_copy,
                                    ui,
                                );
                                place_percentage_label(
                                    Raid::GOHL,
                                    Item::ChampionMerit,
                                    ChestType::Blue,
                                    &local_settings_copy,
                                    ui,
                                );
                                place_percentage_label(
                                    Raid::GOHL,
                                    Item::SupremeMerit,
                                    ChestType::Blue,
                                    &local_settings_copy,
                                    ui,
                                );
                                place_percentage_label(
                                    Raid::GOHL,
                                    Item::LegendaryMerit,
                                    ChestType::Blue,
                                    &local_settings_copy,
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
                                    &local_settings_copy,
                                    ui,
                                );
                                place_percentage_label(
                                    Raid::UBHL,
                                    Item::GoldBrick,
                                    ChestType::Host,
                                    &local_settings_copy,
                                    ui,
                                );
                                place_percentage_label(
                                    Raid::UBHL,
                                    Item::GoldBrick,
                                    ChestType::Flip,
                                    &local_settings_copy,
                                    ui,
                                );
                                place_percentage_label(
                                    Raid::PBHL,
                                    Item::GoldBrick,
                                    ChestType::Host,
                                    &local_settings_copy,
                                    ui,
                                );
                                place_percentage_label(
                                    Raid::Xeno,
                                    Item::GoldBrick,
                                    ChestType::Flip,
                                    &local_settings_copy,
                                    ui,
                                );
                            }
                        });
                });
        }

        if self.config.app_settings.right_panel_visible {
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
                                    if drop.chest == ChestType::Host
                                        || drop.chest == ChestType::Flip
                                            && drop.item == Item::GoldBrick
                                            && drop.raid != Raid::Xeno
                                    {
                                        let mut gold_brick_text_color =
                                            Color32::from_rgb(255, 221, 26);
                                        if self.config.app_settings.dark_mode == false {
                                            gold_brick_text_color = Color32::from_rgb(187, 152, 10);
                                        }
                                        if ui
                                            .add(
                                                egui::Label::new(
                                                    RichText::new(format!("{}", drop.item))
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
                                            let _ = local_settings_copy
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
                                        if self.config.app_settings.dark_mode == false {
                                            gold_brick_text_color = Color32::from_rgb(183, 138, 15);
                                        }
                                        if ui
                                            .add(
                                                egui::Label::new(
                                                    RichText::new(format!("{}", drop.item))
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
                                            let _ = local_settings_copy
                                                .droplog
                                                .drop
                                                .retain(|x| x.drop_id != drop.drop_id);
                                        }
                                        ui.add_space(3.)
                                    } else if drop.item == Item::GoldBrick {
                                        let mut gold_brick_text_color =
                                            Color32::from_rgb(255, 221, 26);
                                        if self.config.app_settings.dark_mode == false {
                                            gold_brick_text_color = Color32::from_rgb(187, 152, 10);
                                        }
                                        if ui
                                            .add(
                                                egui::Label::new(
                                                    RichText::new(format!("{}", drop.item))
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
                                            let _ = local_settings_copy
                                                .droplog
                                                .drop
                                                .retain(|x| x.drop_id != drop.drop_id);
                                        }
                                        ui.add_space(3.)
                                    } else {
                                        if ui
                                            .add(
                                                egui::Label::new(format!("{}", drop.item))
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
                                            let _ = local_settings_copy
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
                if ui
                    .selectable_value(
                        &mut self.config.app_settings.current_ui_tab,
                        UiTab::Pulls,
                        "Pull Calculator",
                    )
                    .changed()
                {
                    frame.set_window_title("Dorothy - Pull Calculator");
                };
                if ui
                    .selectable_value(
                        &mut self.config.app_settings.current_ui_tab,
                        UiTab::Akasha,
                        "Akasha",
                    )
                    .changed()
                {
                    frame.set_window_title("Dorothy - Akasha");
                };
                if ui
                    .selectable_value(
                        &mut self.config.app_settings.current_ui_tab,
                        UiTab::PBHL,
                        "PBHL",
                    )
                    .changed()
                {
                    frame.set_window_title("Dorothy - PBHL");
                };
                if ui
                    .selectable_value(
                        &mut self.config.app_settings.current_ui_tab,
                        UiTab::GOHL,
                        "GOHL",
                    )
                    .changed()
                {
                    frame.set_window_title("Dorothy - GOHL");
                };
                if ui
                    .selectable_value(
                        &mut self.config.app_settings.current_ui_tab,
                        UiTab::Hosts,
                        "Hosts",
                    )
                    .changed()
                {
                    frame.set_window_title("Dorothy - Hosts");
                };
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
                            .spacing((
                                self.config.app_settings.grid_spacing_x,
                                self.config.app_settings.grid_spacing_y,
                            ))
                            .show(ui, |ui| {
                                if !self.config.app_settings.vertical_grid {
                                    if self.config.app_settings.active_items[0] {
                                        ui.horizontal(|ui| {
                                            place_image_button_combo(
                                                Item::NoDrop,
                                                Raid::Akasha,
                                                ChestType::None,
                                                &mut self.config,
                                                ui,
                                            );
                                        });
                                    }
                                    if self.config.app_settings.active_items[1] {
                                        ui.horizontal(|ui| {
                                            place_image_button_combo(
                                                Item::HollowKey,
                                                Raid::Akasha,
                                                ChestType::Blue,
                                                &mut self.config,
                                                ui,
                                            );
                                        });
                                    }
                                    if self.config.app_settings.active_items[2] {
                                        ui.horizontal(|ui| {
                                            place_image_button_combo(
                                                Item::SilverCentrum,
                                                Raid::Akasha,
                                                ChestType::Blue,
                                                &mut self.config,
                                                ui,
                                            );
                                        });
                                    }
                                    if self.config.app_settings.active_items[3] {
                                        ui.horizontal(|ui| {
                                            place_image_button_combo(
                                                Item::GoldBrick,
                                                Raid::Akasha,
                                                ChestType::Blue,
                                                &mut self.config,
                                                ui,
                                            );
                                        });
                                    }
                                    if self.config.app_settings.active_items[0]
                                        || self.config.app_settings.active_items[1]
                                        || self.config.app_settings.active_items[2]
                                        || self.config.app_settings.active_items[3]
                                    {
                                        ui.end_row();
                                    }
                                    if self.config.app_settings.active_items[4] {
                                        ui.horizontal(|ui| {
                                            place_image_button_combo(
                                                Item::CoronationRing,
                                                Raid::Akasha,
                                                ChestType::Blue,
                                                &mut self.config,
                                                ui,
                                            );
                                        });
                                    }
                                    if self.config.app_settings.active_items[5] {
                                        ui.horizontal(|ui| {
                                            place_image_button_combo(
                                                Item::ChampionMerit,
                                                Raid::Akasha,
                                                ChestType::Blue,
                                                &mut self.config,
                                                ui,
                                            );
                                        });
                                    }
                                    if self.config.app_settings.active_items[6] {
                                        ui.horizontal(|ui| {
                                            place_image_button_combo(
                                                Item::WeaponPlusMark1,
                                                Raid::Akasha,
                                                ChestType::Blue,
                                                &mut self.config,
                                                ui,
                                            );
                                        });
                                    }
                                    if self.config.app_settings.active_items[4]
                                        || self.config.app_settings.active_items[5]
                                        || self.config.app_settings.active_items[6]
                                    {
                                        ui.end_row();
                                    }
                                    if self.config.app_settings.active_items[7] {
                                        ui.horizontal(|ui| {
                                            place_image_button_combo(
                                                Item::LineageRing,
                                                Raid::Akasha,
                                                ChestType::Blue,
                                                &mut self.config,
                                                ui,
                                            );
                                        });
                                    }
                                    if self.config.app_settings.active_items[8] {
                                        ui.horizontal(|ui| {
                                            place_image_button_combo(
                                                Item::SupremeMerit,
                                                Raid::Akasha,
                                                ChestType::Blue,
                                                &mut self.config,
                                                ui,
                                            );
                                        });
                                    }
                                    if self.config.app_settings.active_items[9] {
                                        ui.horizontal(|ui| {
                                            place_image_button_combo(
                                                Item::WeaponPlusMark2,
                                                Raid::Akasha,
                                                ChestType::Blue,
                                                &mut self.config,
                                                ui,
                                            );
                                        });
                                    }
                                    if self.config.app_settings.active_items[7]
                                        || self.config.app_settings.active_items[8]
                                        || self.config.app_settings.active_items[9]
                                    {
                                        ui.end_row();
                                    }
                                    if self.config.app_settings.active_items[10] {
                                        ui.horizontal(|ui| {
                                            place_image_button_combo(
                                                Item::IntricacyRing,
                                                Raid::Akasha,
                                                ChestType::Blue,
                                                &mut self.config,
                                                ui,
                                            );
                                        });
                                    }
                                    if self.config.app_settings.active_items[11] {
                                        ui.horizontal(|ui| {
                                            place_image_button_combo(
                                                Item::LegendaryMerit,
                                                Raid::Akasha,
                                                ChestType::Blue,
                                                &mut self.config,
                                                ui,
                                            );
                                        });
                                    }
                                    if self.config.app_settings.active_items[12] {
                                        ui.horizontal(|ui| {
                                            place_image_button_combo(
                                                Item::WeaponPlusMark3,
                                                Raid::Akasha,
                                                ChestType::Blue,
                                                &mut self.config,
                                                ui,
                                            );
                                        });
                                    }
                                } else {
                                    if self.config.app_settings.active_items[0] {
                                        ui.vertical(|ui| {
                                            place_image_button_combo(
                                                Item::NoDrop,
                                                Raid::Akasha,
                                                ChestType::None,
                                                &mut self.config,
                                                ui,
                                            );
                                        });
                                    }
                                    if self.config.app_settings.active_items[1] {
                                        ui.vertical(|ui| {
                                            place_image_button_combo(
                                                Item::HollowKey,
                                                Raid::Akasha,
                                                ChestType::Blue,
                                                &mut self.config,
                                                ui,
                                            );
                                        });
                                    }
                                    if self.config.app_settings.active_items[2] {
                                        ui.vertical(|ui| {
                                            place_image_button_combo(
                                                Item::SilverCentrum,
                                                Raid::Akasha,
                                                ChestType::Blue,
                                                &mut self.config,
                                                ui,
                                            );
                                        });
                                    }
                                    if self.config.app_settings.active_items[3] {
                                        ui.vertical(|ui| {
                                            place_image_button_combo(
                                                Item::GoldBrick,
                                                Raid::Akasha,
                                                ChestType::Blue,
                                                &mut self.config,
                                                ui,
                                            );
                                        });
                                    }
                                    if self.config.app_settings.active_items[0]
                                        || self.config.app_settings.active_items[1]
                                        || self.config.app_settings.active_items[2]
                                        || self.config.app_settings.active_items[3]
                                    {
                                        ui.end_row();
                                    }
                                    if self.config.app_settings.active_items[4] {
                                        ui.vertical(|ui| {
                                            place_image_button_combo(
                                                Item::CoronationRing,
                                                Raid::Akasha,
                                                ChestType::Blue,
                                                &mut self.config,
                                                ui,
                                            );
                                        });
                                    }
                                    if self.config.app_settings.active_items[5] {
                                        ui.vertical(|ui| {
                                            place_image_button_combo(
                                                Item::ChampionMerit,
                                                Raid::Akasha,
                                                ChestType::Blue,
                                                &mut self.config,
                                                ui,
                                            );
                                        });
                                    }
                                    if self.config.app_settings.active_items[6] {
                                        ui.vertical(|ui| {
                                            place_image_button_combo(
                                                Item::WeaponPlusMark1,
                                                Raid::Akasha,
                                                ChestType::Blue,
                                                &mut self.config,
                                                ui,
                                            );
                                        });
                                    }
                                    if self.config.app_settings.active_items[4]
                                        || self.config.app_settings.active_items[5]
                                        || self.config.app_settings.active_items[6]
                                    {
                                        ui.end_row();
                                    }
                                    if self.config.app_settings.active_items[7] {
                                        ui.vertical(|ui| {
                                            place_image_button_combo(
                                                Item::LineageRing,
                                                Raid::Akasha,
                                                ChestType::Blue,
                                                &mut self.config,
                                                ui,
                                            );
                                        });
                                    }
                                    if self.config.app_settings.active_items[8] {
                                        ui.vertical(|ui| {
                                            place_image_button_combo(
                                                Item::SupremeMerit,
                                                Raid::Akasha,
                                                ChestType::Blue,
                                                &mut self.config,
                                                ui,
                                            );
                                        });
                                    }
                                    if self.config.app_settings.active_items[9] {
                                        ui.vertical(|ui| {
                                            place_image_button_combo(
                                                Item::WeaponPlusMark2,
                                                Raid::Akasha,
                                                ChestType::Blue,
                                                &mut self.config,
                                                ui,
                                            );
                                        });
                                    }
                                    if self.config.app_settings.active_items[7]
                                        || self.config.app_settings.active_items[8]
                                        || self.config.app_settings.active_items[9]
                                    {
                                        ui.end_row();
                                    }
                                    if self.config.app_settings.active_items[10] {
                                        ui.vertical(|ui| {
                                            place_image_button_combo(
                                                Item::IntricacyRing,
                                                Raid::Akasha,
                                                ChestType::Blue,
                                                &mut self.config,
                                                ui,
                                            );
                                        });
                                    }
                                    if self.config.app_settings.active_items[11] {
                                        ui.vertical(|ui| {
                                            place_image_button_combo(
                                                Item::LegendaryMerit,
                                                Raid::Akasha,
                                                ChestType::Blue,
                                                &mut self.config,
                                                ui,
                                            );
                                        });
                                    }
                                    if self.config.app_settings.active_items[12] {
                                        ui.vertical(|ui| {
                                            place_image_button_combo(
                                                Item::WeaponPlusMark3,
                                                Raid::Akasha,
                                                ChestType::Blue,
                                                &mut self.config,
                                                ui,
                                            );
                                        });
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
                                if self.config.app_settings.active_items[13] {
                                    ui.horizontal(|ui| {
                                        ui.spacing_mut().item_spacing.x = 3.;
                                        let local_last_added_drop = &self
                                            .config
                                            .droplog
                                            .drop
                                            .iter()
                                            .rposition(|x| {
                                                x.item == Item::NoDrop && x.raid == Raid::PBHL
                                            })
                                            .unwrap_or_default();
                                        if self.config.app_settings.button_label_combo[1] {
                                            if ui
                                                .add(CustomImageButton::new(
                                                    &ui.ctx().load_texture(
                                                        "PBHL NO DROP",
                                                        load_image_from_memory(NO_BLUE_CHEST)
                                                            .unwrap(),
                                                    ),
                                                    (32., 32.),
                                                ))
                                                .clicked()
                                            {
                                                let shift = ui.input().modifiers.shift_only();
                                                if shift {
                                                    if &self
                                                        .config
                                                        .droplog
                                                        .drop
                                                        .iter()
                                                        .filter(|&x| {
                                                            x.item == Item::NoDrop
                                                                && x.raid == Raid::PBHL
                                                        })
                                                        .count()
                                                        > &0
                                                    {
                                                        let _ = self
                                                            .config
                                                            .droplog
                                                            .drop
                                                            .remove(*local_last_added_drop);
                                                    }
                                                } else if !shift {
                                                    let _ = &self.droplog.drop.push(ItemDrop::new(
                                                        self.config
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
                                                        ChestType::None,
                                                        Option::Some(format!(
                                                            "{}",
                                                            self.pbhl_honors
                                                        )),
                                                    ));
                                                    let _ = self.config.droplog.drop.push(
                                                        ItemDrop::new(
                                                            self.config
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
                                                            ChestType::None,
                                                            Option::Some(format!(
                                                                "{}",
                                                                self.pbhl_honors
                                                            )),
                                                        ),
                                                    );
                                                }
                                            }
                                        }
                                        if self.config.app_settings.button_label_combo[0] {
                                            if ui.button("No Drop").clicked() {
                                                let shift = ui.input().modifiers.shift_only();
                                                if shift {
                                                    if &self
                                                        .config
                                                        .droplog
                                                        .drop
                                                        .iter()
                                                        .filter(|&x| {
                                                            x.item == Item::NoDrop
                                                                && x.raid == Raid::PBHL
                                                        })
                                                        .count()
                                                        > &0
                                                    {
                                                        let _ = self
                                                            .config
                                                            .droplog
                                                            .drop
                                                            .remove(*local_last_added_drop);
                                                    }
                                                } else if !shift {
                                                    let _ = &self.droplog.drop.push(ItemDrop::new(
                                                        self.config
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
                                                        ChestType::None,
                                                        Option::Some(format!(
                                                            "{}",
                                                            self.pbhl_honors
                                                        )),
                                                    ));
                                                    let _ = self.config.droplog.drop.push(
                                                        ItemDrop::new(
                                                            self.config
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
                                                            ChestType::None,
                                                            Option::Some(format!(
                                                                "{}",
                                                                self.pbhl_honors
                                                            )),
                                                        ),
                                                    );
                                                }
                                            }
                                        }
                                        let drop_count = self
                                            .config
                                            .droplog
                                            .drop
                                            .iter()
                                            .filter(|x| {
                                                x.item == Item::NoDrop && x.raid == Raid::PBHL
                                            })
                                            .count();
                                        ui.label("x".to_string() + &drop_count.to_string());
                                    });
                                }
                                if self.config.app_settings.active_items[14] {
                                    ui.horizontal(|ui| {
                                        ui.spacing_mut().item_spacing.x = 3.;
                                        let local_last_added_drop = &self
                                            .config
                                            .droplog
                                            .drop
                                            .iter()
                                            .rposition(|x| {
                                                x.item == Item::GoldBrick && x.raid == Raid::PBHL
                                            })
                                            .unwrap_or_default();
                                        if self.config.app_settings.button_label_combo[1] {
                                            if ui
                                                .add(CustomImageButton::new(
                                                    &ui.ctx().load_texture(
                                                        "PBHL GOLD BRICK",
                                                        load_image_from_memory(GOLD_BAR).unwrap(),
                                                    ),
                                                    (32., 32.),
                                                ))
                                                .clicked()
                                            {
                                                let shift = ui.input().modifiers.shift_only();
                                                if shift {
                                                    if &self
                                                        .config
                                                        .droplog
                                                        .drop
                                                        .iter()
                                                        .filter(|&x| {
                                                            x.item == Item::GoldBrick
                                                                && x.raid == Raid::PBHL
                                                                && x.chest == ChestType::Blue
                                                        })
                                                        .count()
                                                        > &0
                                                    {
                                                        let _ = self
                                                            .config
                                                            .droplog
                                                            .drop
                                                            .remove(*local_last_added_drop);
                                                    }
                                                } else if !shift {
                                                    let _ = &self.droplog.drop.push(ItemDrop::new(
                                                        self.config
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
                                                        ChestType::Blue,
                                                        Option::Some(format!(
                                                            "{}",
                                                            self.pbhl_honors
                                                        )),
                                                    ));
                                                    let _ = self.config.droplog.drop.push(
                                                        ItemDrop::new(
                                                            self.config
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
                                                            ChestType::Blue,
                                                            Option::Some(format!(
                                                                "{}",
                                                                self.pbhl_honors
                                                            )),
                                                        ),
                                                    );
                                                }
                                            }
                                        }
                                        if self.config.app_settings.button_label_combo[0] {
                                            if ui.button("Gold Brick").clicked() {
                                                let shift = ui.input().modifiers.shift_only();
                                                if shift {
                                                    if &self
                                                        .config
                                                        .droplog
                                                        .drop
                                                        .iter()
                                                        .filter(|&x| {
                                                            x.item == Item::GoldBrick
                                                                && x.raid == Raid::PBHL
                                                                && x.chest == ChestType::Blue
                                                        })
                                                        .count()
                                                        > &0
                                                    {
                                                        let _ = self
                                                            .config
                                                            .droplog
                                                            .drop
                                                            .remove(*local_last_added_drop);
                                                    }
                                                } else if !shift {
                                                    let _ = &self.droplog.drop.push(ItemDrop::new(
                                                        self.config
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
                                                        ChestType::Blue,
                                                        Option::Some(format!(
                                                            "{}",
                                                            self.pbhl_honors
                                                        )),
                                                    ));
                                                    let _ = self.config.droplog.drop.push(
                                                        ItemDrop::new(
                                                            self.config
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
                                                            ChestType::Blue,
                                                            Option::Some(format!(
                                                                "{}",
                                                                self.pbhl_honors
                                                            )),
                                                        ),
                                                    );
                                                }
                                            }
                                        }
                                        let drop_count = self
                                            .config
                                            .droplog
                                            .drop
                                            .iter()
                                            .filter(|x| {
                                                x.item == Item::GoldBrick
                                                    && x.raid == Raid::PBHL
                                                    && x.chest == ChestType::Blue
                                            })
                                            .count();
                                        ui.label("x".to_string() + &drop_count.to_string());
                                    });
                                }
                                if self.config.app_settings.active_items[13]
                                    || self.config.app_settings.active_items[14]
                                {
                                    ui.end_row();
                                }
                                if self.config.app_settings.active_items[15] {
                                    ui.horizontal(|ui| {
                                        ui.spacing_mut().item_spacing.x = 3.;
                                        let local_last_added_drop = &self
                                            .config
                                            .droplog
                                            .drop
                                            .iter()
                                            .rposition(|x| {
                                                x.item == Item::CoronationRing
                                                    && x.raid == Raid::PBHL
                                            })
                                            .unwrap_or_default();
                                        if self.config.app_settings.button_label_combo[1] {
                                            if ui
                                                .add(CustomImageButton::new(
                                                    &ui.ctx().load_texture(
                                                        "PBHL CORONATION RING",
                                                        load_image_from_memory(C_RING).unwrap(),
                                                    ),
                                                    (32., 32.),
                                                ))
                                                .clicked()
                                            {
                                                let shift = ui.input().modifiers.shift_only();
                                                if shift {
                                                    if &self
                                                        .config
                                                        .droplog
                                                        .drop
                                                        .iter()
                                                        .filter(|&x| {
                                                            x.item == Item::CoronationRing
                                                                && x.raid == Raid::PBHL
                                                        })
                                                        .count()
                                                        > &0
                                                    {
                                                        let _ = self
                                                            .config
                                                            .droplog
                                                            .drop
                                                            .remove(*local_last_added_drop);
                                                    }
                                                } else if !shift {
                                                    let _ = &self.droplog.drop.push(ItemDrop::new(
                                                        self.config
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
                                                        ChestType::Blue,
                                                        Option::Some(format!(
                                                            "{}",
                                                            self.pbhl_honors
                                                        )),
                                                    ));
                                                    let _ = self.config.droplog.drop.push(
                                                        ItemDrop::new(
                                                            self.config
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
                                                            ChestType::Blue,
                                                            Option::Some(format!(
                                                                "{}",
                                                                self.pbhl_honors
                                                            )),
                                                        ),
                                                    );
                                                }
                                            }
                                        }
                                        if self.config.app_settings.button_label_combo[0] {
                                            if ui.button("Coronation Ring").clicked() {
                                                let shift = ui.input().modifiers.shift_only();
                                                if shift {
                                                    if &self
                                                        .config
                                                        .droplog
                                                        .drop
                                                        .iter()
                                                        .filter(|&x| {
                                                            x.item == Item::CoronationRing
                                                                && x.raid == Raid::PBHL
                                                        })
                                                        .count()
                                                        > &0
                                                    {
                                                        let _ = self
                                                            .config
                                                            .droplog
                                                            .drop
                                                            .remove(*local_last_added_drop);
                                                    }
                                                } else if !shift {
                                                    let _ = &self.droplog.drop.push(ItemDrop::new(
                                                        self.config
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
                                                        ChestType::Blue,
                                                        Option::Some(format!(
                                                            "{}",
                                                            self.pbhl_honors
                                                        )),
                                                    ));
                                                    let _ = self.config.droplog.drop.push(
                                                        ItemDrop::new(
                                                            self.config
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
                                                            ChestType::Blue,
                                                            Option::Some(format!(
                                                                "{}",
                                                                self.pbhl_honors
                                                            )),
                                                        ),
                                                    );
                                                }
                                            }
                                        }
                                        let drop_count = self
                                            .config
                                            .droplog
                                            .drop
                                            .iter()
                                            .filter(|x| {
                                                x.item == Item::CoronationRing
                                                    && x.raid == Raid::PBHL
                                            })
                                            .count();
                                        ui.label("x".to_string() + &drop_count.to_string());
                                    });
                                }
                                if self.config.app_settings.active_items[16] {
                                    ui.horizontal(|ui| {
                                        ui.spacing_mut().item_spacing.x = 3.;
                                        let local_last_added_drop = &self
                                            .config
                                            .droplog
                                            .drop
                                            .iter()
                                            .rposition(|x| {
                                                x.item == Item::LineageRing && x.raid == Raid::PBHL
                                            })
                                            .unwrap_or_default();
                                        if self.config.app_settings.button_label_combo[1] {
                                            if ui
                                                .add(CustomImageButton::new(
                                                    &ui.ctx().load_texture(
                                                        "PBHL LINEAGE RING",
                                                        load_image_from_memory(L_RING).unwrap(),
                                                    ),
                                                    (32., 32.),
                                                ))
                                                .clicked()
                                            {
                                                let shift = ui.input().modifiers.shift_only();
                                                if shift {
                                                    if &self
                                                        .config
                                                        .droplog
                                                        .drop
                                                        .iter()
                                                        .filter(|&x| {
                                                            x.item == Item::LineageRing
                                                                && x.raid == Raid::PBHL
                                                        })
                                                        .count()
                                                        > &0
                                                    {
                                                        let _ = self
                                                            .config
                                                            .droplog
                                                            .drop
                                                            .remove(*local_last_added_drop);
                                                    }
                                                } else if !shift {
                                                    let _ = &self.droplog.drop.push(ItemDrop::new(
                                                        self.config
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
                                                        ChestType::Blue,
                                                        Option::Some(format!(
                                                            "{}",
                                                            self.pbhl_honors
                                                        )),
                                                    ));
                                                    let _ = self.config.droplog.drop.push(
                                                        ItemDrop::new(
                                                            self.config
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
                                                            ChestType::Blue,
                                                            Option::Some(format!(
                                                                "{}",
                                                                self.pbhl_honors
                                                            )),
                                                        ),
                                                    );
                                                }
                                            }
                                        }
                                        if self.config.app_settings.button_label_combo[0] {
                                            if ui.button("Lineage Ring").clicked() {
                                                let shift = ui.input().modifiers.shift_only();
                                                if shift {
                                                    if &self
                                                        .config
                                                        .droplog
                                                        .drop
                                                        .iter()
                                                        .filter(|&x| {
                                                            x.item == Item::LineageRing
                                                                && x.raid == Raid::PBHL
                                                        })
                                                        .count()
                                                        > &0
                                                    {
                                                        let _ = self
                                                            .config
                                                            .droplog
                                                            .drop
                                                            .remove(*local_last_added_drop);
                                                    }
                                                } else if !shift {
                                                    let _ = &self.droplog.drop.push(ItemDrop::new(
                                                        self.config
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
                                                        ChestType::Blue,
                                                        Option::Some(format!(
                                                            "{}",
                                                            self.pbhl_honors
                                                        )),
                                                    ));
                                                    let _ = self.config.droplog.drop.push(
                                                        ItemDrop::new(
                                                            self.config
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
                                                            ChestType::Blue,
                                                            Option::Some(format!(
                                                                "{}",
                                                                self.pbhl_honors
                                                            )),
                                                        ),
                                                    );
                                                }
                                            }
                                        }
                                        let drop_count = self
                                            .config
                                            .droplog
                                            .drop
                                            .iter()
                                            .filter(|x| {
                                                x.item == Item::LineageRing && x.raid == Raid::PBHL
                                            })
                                            .count();
                                        ui.label("x".to_string() + &drop_count.to_string());
                                    });
                                }
                                if self.config.app_settings.active_items[17] {
                                    ui.horizontal(|ui| {
                                        ui.spacing_mut().item_spacing.x = 3.;
                                        let local_last_added_drop = &self
                                            .config
                                            .droplog
                                            .drop
                                            .iter()
                                            .rposition(|x| {
                                                x.item == Item::IntricacyRing
                                                    && x.raid == Raid::PBHL
                                            })
                                            .unwrap_or_default();
                                        if self.config.app_settings.button_label_combo[1] {
                                            if ui
                                                .add(CustomImageButton::new(
                                                    &ui.ctx().load_texture(
                                                        "PBHL INTRICACY RING",
                                                        load_image_from_memory(I_RING).unwrap(),
                                                    ),
                                                    (32., 32.),
                                                ))
                                                .clicked()
                                            {
                                                let shift = ui.input().modifiers.shift_only();
                                                if shift {
                                                    if &self
                                                        .config
                                                        .droplog
                                                        .drop
                                                        .iter()
                                                        .filter(|&x| {
                                                            x.item == Item::IntricacyRing
                                                                && x.raid == Raid::PBHL
                                                        })
                                                        .count()
                                                        > &0
                                                    {
                                                        let _ = self
                                                            .config
                                                            .droplog
                                                            .drop
                                                            .remove(*local_last_added_drop);
                                                    }
                                                } else if !shift {
                                                    let _ = &self.droplog.drop.push(ItemDrop::new(
                                                        self.config
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
                                                        ChestType::Blue,
                                                        Option::Some(format!(
                                                            "{}",
                                                            self.pbhl_honors
                                                        )),
                                                    ));
                                                    let _ = self.config.droplog.drop.push(
                                                        ItemDrop::new(
                                                            self.config
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
                                                            ChestType::Blue,
                                                            Option::Some(format!(
                                                                "{}",
                                                                self.pbhl_honors
                                                            )),
                                                        ),
                                                    );
                                                }
                                            }
                                        }
                                        if self.config.app_settings.button_label_combo[0] {
                                            if ui.button("Intricacy Ring").clicked() {
                                                let shift = ui.input().modifiers.shift_only();
                                                if shift {
                                                    if &self
                                                        .config
                                                        .droplog
                                                        .drop
                                                        .iter()
                                                        .filter(|&x| {
                                                            x.item == Item::IntricacyRing
                                                                && x.raid == Raid::PBHL
                                                        })
                                                        .count()
                                                        > &0
                                                    {
                                                        let _ = self
                                                            .config
                                                            .droplog
                                                            .drop
                                                            .remove(*local_last_added_drop);
                                                    }
                                                } else if !shift {
                                                    let _ = &self.droplog.drop.push(ItemDrop::new(
                                                        self.config
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
                                                        ChestType::Blue,
                                                        Option::Some(format!(
                                                            "{}",
                                                            self.pbhl_honors
                                                        )),
                                                    ));
                                                    let _ = self.config.droplog.drop.push(
                                                        ItemDrop::new(
                                                            self.config
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
                                                            ChestType::Blue,
                                                            Option::Some(format!(
                                                                "{}",
                                                                self.pbhl_honors
                                                            )),
                                                        ),
                                                    );
                                                }
                                            }
                                        }
                                        let drop_count = self
                                            .config
                                            .droplog
                                            .drop
                                            .iter()
                                            .filter(|x| {
                                                x.item == Item::IntricacyRing
                                                    && x.raid == Raid::PBHL
                                            })
                                            .count();
                                        ui.label("x".to_string() + &drop_count.to_string());
                                    });
                                }
                                if self.config.app_settings.active_items[15]
                                    || self.config.app_settings.active_items[16]
                                    || self.config.app_settings.active_items[17]
                                {
                                    ui.end_row();
                                }
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
                            .spacing((
                                self.config.app_settings.grid_spacing_x,
                                self.config.app_settings.grid_spacing_y,
                            ))
                            .show(ui, |ui| {
                                if self.config.app_settings.active_items[18] {
                                    ui.horizontal(|ui| {
                                        ui.spacing_mut().item_spacing.x = 3.;
                                        let local_last_added_drop = &self
                                            .config
                                            .droplog
                                            .drop
                                            .iter()
                                            .rposition(|x| {
                                                x.item == Item::NoDrop && x.raid == Raid::GOHL
                                            })
                                            .unwrap_or_default();
                                        if self.config.app_settings.button_label_combo[1] {
                                            if ui
                                                .add(CustomImageButton::new(
                                                    &ui.ctx().load_texture(
                                                        "GOHL NO DROP",
                                                        load_image_from_memory(NO_BLUE_CHEST)
                                                            .unwrap(),
                                                    ),
                                                    (32., 32.),
                                                ))
                                                .clicked()
                                            {
                                                let shift = ui.input().modifiers.shift_only();
                                                if shift {
                                                    if &self
                                                        .config
                                                        .droplog
                                                        .drop
                                                        .iter()
                                                        .filter(|&x| {
                                                            x.item == Item::NoDrop
                                                                && x.raid == Raid::GOHL
                                                        })
                                                        .count()
                                                        > &0
                                                    {
                                                        let _ = self
                                                            .config
                                                            .droplog
                                                            .drop
                                                            .remove(*local_last_added_drop);
                                                    }
                                                } else if !shift {
                                                    let _ = &self.droplog.drop.push(ItemDrop::new(
                                                        self.config
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
                                                        ChestType::None,
                                                        Option::Some("None".to_string()),
                                                    ));
                                                    let _ = self.config.droplog.drop.push(
                                                        ItemDrop::new(
                                                            self.config
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
                                                            ChestType::None,
                                                            Option::Some("None".to_string()),
                                                        ),
                                                    );
                                                }
                                            }
                                        }
                                        if self.config.app_settings.button_label_combo[0] {
                                            if ui.button("No Drop").clicked() {
                                                let shift = ui.input().modifiers.shift_only();
                                                if shift {
                                                    if &self
                                                        .config
                                                        .droplog
                                                        .drop
                                                        .iter()
                                                        .filter(|&x| {
                                                            x.item == Item::NoDrop
                                                                && x.raid == Raid::GOHL
                                                        })
                                                        .count()
                                                        > &0
                                                    {
                                                        let _ = self
                                                            .config
                                                            .droplog
                                                            .drop
                                                            .remove(*local_last_added_drop);
                                                    }
                                                } else if !shift {
                                                    let _ = &self.droplog.drop.push(ItemDrop::new(
                                                        self.config
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
                                                        ChestType::None,
                                                        Option::Some("None".to_string()),
                                                    ));
                                                    let _ = self.config.droplog.drop.push(
                                                        ItemDrop::new(
                                                            self.config
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
                                                            ChestType::None,
                                                            Option::Some("None".to_string()),
                                                        ),
                                                    );
                                                }
                                            }
                                        }
                                        let drop_count = self
                                            .config
                                            .droplog
                                            .drop
                                            .iter()
                                            .filter(|x| {
                                                x.item == Item::NoDrop && x.raid == Raid::GOHL
                                            })
                                            .count();
                                        ui.label("x".to_string() + &drop_count.to_string());
                                    });
                                }
                                if self.config.app_settings.active_items[19] {
                                    ui.horizontal(|ui| {
                                        ui.spacing_mut().item_spacing.x = 3.;
                                        let local_last_added_drop = &self
                                            .config
                                            .droplog
                                            .drop
                                            .iter()
                                            .rposition(|x| {
                                                x.item == Item::VerdantAzurite
                                                    && x.raid == Raid::GOHL
                                            })
                                            .unwrap_or_default();
                                        if self.config.app_settings.button_label_combo[1] {
                                            if ui
                                                .add(CustomImageButton::new(
                                                    &ui.ctx().load_texture(
                                                        "VERDANT AZURITE",
                                                        load_image_from_memory(VERDANT_AZURITE)
                                                            .unwrap(),
                                                    ),
                                                    (32., 32.),
                                                ))
                                                .clicked()
                                            {
                                                let shift = ui.input().modifiers.shift_only();
                                                if shift {
                                                    if &self
                                                        .config
                                                        .droplog
                                                        .drop
                                                        .iter()
                                                        .filter(|&x| {
                                                            x.item == Item::VerdantAzurite
                                                                && x.raid == Raid::GOHL
                                                        })
                                                        .count()
                                                        > &0
                                                    {
                                                        let _ = self
                                                            .config
                                                            .droplog
                                                            .drop
                                                            .remove(*local_last_added_drop);
                                                    }
                                                } else if !shift {
                                                    let _ = &self.droplog.drop.push(ItemDrop::new(
                                                        self.config
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
                                                        ChestType::Blue,
                                                        Option::Some("None".to_string()),
                                                    ));
                                                    let _ = self.config.droplog.drop.push(
                                                        ItemDrop::new(
                                                            self.config
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
                                                            ChestType::Blue,
                                                            Option::Some("None".to_string()),
                                                        ),
                                                    );
                                                }
                                            }
                                        }
                                        if self.config.app_settings.button_label_combo[0] {
                                            if ui.button("Verdant Azurite").clicked() {
                                                let shift = ui.input().modifiers.shift_only();
                                                if shift {
                                                    if &self
                                                        .config
                                                        .droplog
                                                        .drop
                                                        .iter()
                                                        .filter(|&x| {
                                                            x.item == Item::VerdantAzurite
                                                                && x.raid == Raid::GOHL
                                                        })
                                                        .count()
                                                        > &0
                                                    {
                                                        let _ = self
                                                            .config
                                                            .droplog
                                                            .drop
                                                            .remove(*local_last_added_drop);
                                                    }
                                                } else if !shift {
                                                    let _ = &self.droplog.drop.push(ItemDrop::new(
                                                        self.config
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
                                                        ChestType::Blue,
                                                        Option::Some("None".to_string()),
                                                    ));
                                                    let _ = self.config.droplog.drop.push(
                                                        ItemDrop::new(
                                                            self.config
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
                                                            ChestType::Blue,
                                                            Option::Some("None".to_string()),
                                                        ),
                                                    );
                                                }
                                            }
                                        }
                                        let drop_count = self
                                            .config
                                            .droplog
                                            .drop
                                            .iter()
                                            .filter(|x| {
                                                x.item == Item::VerdantAzurite
                                                    && x.raid == Raid::GOHL
                                            })
                                            .count();
                                        ui.label("x".to_string() + &drop_count.to_string());
                                    });
                                }
                                if self.config.app_settings.active_items[20] {
                                    ui.horizontal(|ui| {
                                        ui.spacing_mut().item_spacing.x = 3.;
                                        let local_last_added_drop = &self
                                            .config
                                            .droplog
                                            .drop
                                            .iter()
                                            .rposition(|x| {
                                                x.item == Item::SilverCentrum
                                                    && x.raid == Raid::GOHL
                                            })
                                            .unwrap_or_default();
                                        if self.config.app_settings.button_label_combo[1] {
                                            if ui
                                                .add(CustomImageButton::new(
                                                    &ui.ctx().load_texture(
                                                        "GOHL SILVER CENTRUM",
                                                        load_image_from_memory(SILVER_CENTRUM)
                                                            .unwrap(),
                                                    ),
                                                    (32., 32.),
                                                ))
                                                .clicked()
                                            {
                                                let shift = ui.input().modifiers.shift_only();
                                                if shift {
                                                    if &self
                                                        .config
                                                        .droplog
                                                        .drop
                                                        .iter()
                                                        .filter(|&x| {
                                                            x.item == Item::SilverCentrum
                                                                && x.raid == Raid::GOHL
                                                        })
                                                        .count()
                                                        > &0
                                                    {
                                                        let _ = self
                                                            .config
                                                            .droplog
                                                            .drop
                                                            .remove(*local_last_added_drop);
                                                    }
                                                } else if !shift {
                                                    let _ = &self.droplog.drop.push(ItemDrop::new(
                                                        self.config
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
                                                        ChestType::Blue,
                                                        Option::Some("None".to_string()),
                                                    ));
                                                    let _ = self.config.droplog.drop.push(
                                                        ItemDrop::new(
                                                            self.config
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
                                                            ChestType::Blue,
                                                            Option::Some("None".to_string()),
                                                        ),
                                                    );
                                                }
                                            }
                                        }
                                        if self.config.app_settings.button_label_combo[0] {
                                            if ui.button("Silver Centrum").clicked() {
                                                let shift = ui.input().modifiers.shift_only();
                                                if shift {
                                                    if &self
                                                        .config
                                                        .droplog
                                                        .drop
                                                        .iter()
                                                        .filter(|&x| {
                                                            x.item == Item::SilverCentrum
                                                                && x.raid == Raid::GOHL
                                                        })
                                                        .count()
                                                        > &0
                                                    {
                                                        let _ = self
                                                            .config
                                                            .droplog
                                                            .drop
                                                            .remove(*local_last_added_drop);
                                                    }
                                                } else if !shift {
                                                    let _ = &self.droplog.drop.push(ItemDrop::new(
                                                        self.config
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
                                                        ChestType::Blue,
                                                        Option::Some("None".to_string()),
                                                    ));
                                                    let _ = self.config.droplog.drop.push(
                                                        ItemDrop::new(
                                                            self.config
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
                                                            ChestType::Blue,
                                                            Option::Some("None".to_string()),
                                                        ),
                                                    );
                                                }
                                            }
                                        }
                                        let drop_count = self
                                            .config
                                            .droplog
                                            .drop
                                            .iter()
                                            .filter(|x| {
                                                x.item == Item::SilverCentrum
                                                    && x.raid == Raid::GOHL
                                            })
                                            .count();
                                        ui.label("x".to_string() + &drop_count.to_string());
                                    });
                                }
                                if self.config.app_settings.active_items[21] {
                                    ui.horizontal(|ui| {
                                        ui.spacing_mut().item_spacing.x = 3.;
                                        let local_last_added_drop = &self
                                            .config
                                            .droplog
                                            .drop
                                            .iter()
                                            .rposition(|x| {
                                                x.item == Item::GoldBrick && x.raid == Raid::GOHL
                                            })
                                            .unwrap_or_default();
                                        if self.config.app_settings.button_label_combo[1] {
                                            if ui
                                                .add(CustomImageButton::new(
                                                    &ui.ctx().load_texture(
                                                        "GOHL GOLD BRICK",
                                                        load_image_from_memory(GOLD_BAR).unwrap(),
                                                    ),
                                                    (32., 32.),
                                                ))
                                                .clicked()
                                            {
                                                let shift = ui.input().modifiers.shift_only();
                                                if shift {
                                                    if &self
                                                        .config
                                                        .droplog
                                                        .drop
                                                        .iter()
                                                        .filter(|&x| {
                                                            x.item == Item::GoldBrick
                                                                && x.raid == Raid::GOHL
                                                        })
                                                        .count()
                                                        > &0
                                                    {
                                                        let _ = self
                                                            .config
                                                            .droplog
                                                            .drop
                                                            .remove(*local_last_added_drop);
                                                    }
                                                } else if !shift {
                                                    let _ = &self.droplog.drop.push(ItemDrop::new(
                                                        self.config
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
                                                        ChestType::Blue,
                                                        Option::Some("None".to_string()),
                                                    ));
                                                    let _ = self.config.droplog.drop.push(
                                                        ItemDrop::new(
                                                            self.config
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
                                                            ChestType::Blue,
                                                            Option::Some("None".to_string()),
                                                        ),
                                                    );
                                                }
                                            }
                                        }
                                        if self.config.app_settings.button_label_combo[0] {
                                            if ui.button("Gold Brick").clicked() {
                                                let shift = ui.input().modifiers.shift_only();
                                                if shift {
                                                    if &self
                                                        .config
                                                        .droplog
                                                        .drop
                                                        .iter()
                                                        .filter(|&x| {
                                                            x.item == Item::GoldBrick
                                                                && x.raid == Raid::GOHL
                                                        })
                                                        .count()
                                                        > &0
                                                    {
                                                        let _ = self
                                                            .config
                                                            .droplog
                                                            .drop
                                                            .remove(*local_last_added_drop);
                                                    }
                                                } else if !shift {
                                                    let _ = &self.droplog.drop.push(ItemDrop::new(
                                                        self.config
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
                                                        ChestType::Blue,
                                                        Option::Some("None".to_string()),
                                                    ));
                                                    let _ = self.config.droplog.drop.push(
                                                        ItemDrop::new(
                                                            self.config
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
                                                            ChestType::Blue,
                                                            Option::Some("None".to_string()),
                                                        ),
                                                    );
                                                }
                                            }
                                        }
                                        let drop_count = self
                                            .config
                                            .droplog
                                            .drop
                                            .iter()
                                            .filter(|x| {
                                                x.item == Item::GoldBrick && x.raid == Raid::GOHL
                                            })
                                            .count();
                                        ui.label("x".to_string() + &drop_count.to_string());
                                    });
                                }
                                if self.config.app_settings.active_items[18]
                                    || self.config.app_settings.active_items[19]
                                    || self.config.app_settings.active_items[20]
                                    || self.config.app_settings.active_items[21]
                                {
                                    ui.end_row();
                                }
                                if self.config.app_settings.active_items[22] {
                                    ui.horizontal(|ui| {
                                        ui.spacing_mut().item_spacing.x = 3.;
                                        let local_last_added_drop = &self
                                            .config
                                            .droplog
                                            .drop
                                            .iter()
                                            .rposition(|x| {
                                                x.item == Item::CoronationRing
                                                    && x.raid == Raid::GOHL
                                            })
                                            .unwrap_or_default();
                                        if self.config.app_settings.button_label_combo[1] {
                                            if ui
                                                .add(CustomImageButton::new(
                                                    &ui.ctx().load_texture(
                                                        "GOHL CORONATION RING",
                                                        load_image_from_memory(C_RING).unwrap(),
                                                    ),
                                                    (32., 32.),
                                                ))
                                                .clicked()
                                            {
                                                let shift = ui.input().modifiers.shift_only();
                                                if shift {
                                                    if &self
                                                        .config
                                                        .droplog
                                                        .drop
                                                        .iter()
                                                        .filter(|&x| {
                                                            x.item == Item::CoronationRing
                                                                && x.raid == Raid::GOHL
                                                        })
                                                        .count()
                                                        > &0
                                                    {
                                                        let _ = self
                                                            .config
                                                            .droplog
                                                            .drop
                                                            .remove(*local_last_added_drop);
                                                    }
                                                } else if !shift {
                                                    let _ = &self.droplog.drop.push(ItemDrop::new(
                                                        self.config
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
                                                        ChestType::Blue,
                                                        Option::Some("None".to_string()),
                                                    ));
                                                    let _ = self.config.droplog.drop.push(
                                                        ItemDrop::new(
                                                            self.config
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
                                                            ChestType::Blue,
                                                            Option::Some("None".to_string()),
                                                        ),
                                                    );
                                                }
                                            }
                                        }
                                        if self.config.app_settings.button_label_combo[0] {
                                            if ui.button("Coronation Ring").clicked() {
                                                let shift = ui.input().modifiers.shift_only();
                                                if shift {
                                                    if &self
                                                        .config
                                                        .droplog
                                                        .drop
                                                        .iter()
                                                        .filter(|&x| {
                                                            x.item == Item::CoronationRing
                                                                && x.raid == Raid::GOHL
                                                        })
                                                        .count()
                                                        > &0
                                                    {
                                                        let _ = self
                                                            .config
                                                            .droplog
                                                            .drop
                                                            .remove(*local_last_added_drop);
                                                    }
                                                } else if !shift {
                                                    let _ = &self.droplog.drop.push(ItemDrop::new(
                                                        self.config
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
                                                        ChestType::Blue,
                                                        Option::Some("None".to_string()),
                                                    ));
                                                    let _ = self.config.droplog.drop.push(
                                                        ItemDrop::new(
                                                            self.config
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
                                                            ChestType::Blue,
                                                            Option::Some("None".to_string()),
                                                        ),
                                                    );
                                                }
                                            }
                                        }
                                        let drop_count = self
                                            .config
                                            .droplog
                                            .drop
                                            .iter()
                                            .filter(|x| {
                                                x.item == Item::CoronationRing
                                                    && x.raid == Raid::GOHL
                                            })
                                            .count();
                                        ui.label("x".to_string() + &drop_count.to_string());
                                    });
                                }
                                if self.config.app_settings.active_items[23] {
                                    ui.horizontal(|ui| {
                                        ui.spacing_mut().item_spacing.x = 3.;
                                        let local_last_added_drop = &self
                                            .config
                                            .droplog
                                            .drop
                                            .iter()
                                            .rposition(|x| {
                                                x.item == Item::ChampionMerit
                                                    && x.raid == Raid::GOHL
                                            })
                                            .unwrap_or_default();
                                        if self.config.app_settings.button_label_combo[1] {
                                            if ui
                                                .add(CustomImageButton::new(
                                                    &ui.ctx().load_texture(
                                                        "GOHL CHAMPION MERIT",
                                                        load_image_from_memory(C_MERIT).unwrap(),
                                                    ),
                                                    (32., 32.),
                                                ))
                                                .clicked()
                                            {
                                                let shift = ui.input().modifiers.shift_only();
                                                if shift {
                                                    if &self
                                                        .config
                                                        .droplog
                                                        .drop
                                                        .iter()
                                                        .filter(|&x| {
                                                            x.item == Item::ChampionMerit
                                                                && x.raid == Raid::GOHL
                                                        })
                                                        .count()
                                                        > &0
                                                    {
                                                        let _ = self
                                                            .config
                                                            .droplog
                                                            .drop
                                                            .remove(*local_last_added_drop);
                                                    }
                                                } else if !shift {
                                                    let _ = &self.droplog.drop.push(ItemDrop::new(
                                                        self.config
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
                                                        ChestType::Blue,
                                                        Option::Some("None".to_string()),
                                                    ));
                                                    let _ = self.config.droplog.drop.push(
                                                        ItemDrop::new(
                                                            self.config
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
                                                            ChestType::Blue,
                                                            Option::Some("None".to_string()),
                                                        ),
                                                    );
                                                }
                                            }
                                        }
                                        if self.config.app_settings.button_label_combo[0] {
                                            if ui.button("Champion Merit").clicked() {
                                                let shift = ui.input().modifiers.shift_only();
                                                if shift {
                                                    if &self
                                                        .config
                                                        .droplog
                                                        .drop
                                                        .iter()
                                                        .filter(|&x| {
                                                            x.item == Item::ChampionMerit
                                                                && x.raid == Raid::GOHL
                                                        })
                                                        .count()
                                                        > &0
                                                    {
                                                        let _ = self
                                                            .config
                                                            .droplog
                                                            .drop
                                                            .remove(*local_last_added_drop);
                                                    }
                                                } else if !shift {
                                                    let _ = &self.droplog.drop.push(ItemDrop::new(
                                                        self.config
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
                                                        ChestType::Blue,
                                                        Option::Some("None".to_string()),
                                                    ));
                                                    let _ = self.config.droplog.drop.push(
                                                        ItemDrop::new(
                                                            self.config
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
                                                            ChestType::Blue,
                                                            Option::Some("None".to_string()),
                                                        ),
                                                    );
                                                }
                                            }
                                        }
                                        let drop_count = self
                                            .config
                                            .droplog
                                            .drop
                                            .iter()
                                            .filter(|x| {
                                                x.item == Item::ChampionMerit
                                                    && x.raid == Raid::GOHL
                                            })
                                            .count();
                                        ui.label("x".to_string() + &drop_count.to_string());
                                    });
                                }
                                if self.config.app_settings.active_items[22]
                                    || self.config.app_settings.active_items[23]
                                {
                                    ui.end_row();
                                }
                                if self.config.app_settings.active_items[24] {
                                    ui.horizontal(|ui| {
                                        ui.spacing_mut().item_spacing.x = 3.;
                                        let local_last_added_drop = &self
                                            .config
                                            .droplog
                                            .drop
                                            .iter()
                                            .rposition(|x| {
                                                x.item == Item::LineageRing && x.raid == Raid::GOHL
                                            })
                                            .unwrap_or_default();
                                        if self.config.app_settings.button_label_combo[1] {
                                            if ui
                                                .add(CustomImageButton::new(
                                                    &ui.ctx().load_texture(
                                                        "GOHL LINEAGE RING",
                                                        load_image_from_memory(L_RING).unwrap(),
                                                    ),
                                                    (32., 32.),
                                                ))
                                                .clicked()
                                            {
                                                let shift = ui.input().modifiers.shift_only();
                                                if shift {
                                                    if &self
                                                        .config
                                                        .droplog
                                                        .drop
                                                        .iter()
                                                        .filter(|&x| {
                                                            x.item == Item::LineageRing
                                                                && x.raid == Raid::GOHL
                                                        })
                                                        .count()
                                                        > &0
                                                    {
                                                        let _ = self
                                                            .config
                                                            .droplog
                                                            .drop
                                                            .remove(*local_last_added_drop);
                                                    }
                                                } else if !shift {
                                                    let _ = &self.droplog.drop.push(ItemDrop::new(
                                                        self.config
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
                                                        ChestType::Blue,
                                                        Option::Some("None".to_string()),
                                                    ));
                                                    let _ = self.config.droplog.drop.push(
                                                        ItemDrop::new(
                                                            self.config
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
                                                            ChestType::Blue,
                                                            Option::Some("None".to_string()),
                                                        ),
                                                    );
                                                }
                                            }
                                        }
                                        if self.config.app_settings.button_label_combo[0] {
                                            if ui.button("Lineage Ring").clicked() {
                                                let shift = ui.input().modifiers.shift_only();
                                                if shift {
                                                    if &self
                                                        .config
                                                        .droplog
                                                        .drop
                                                        .iter()
                                                        .filter(|&x| {
                                                            x.item == Item::LineageRing
                                                                && x.raid == Raid::GOHL
                                                        })
                                                        .count()
                                                        > &0
                                                    {
                                                        let _ = self
                                                            .config
                                                            .droplog
                                                            .drop
                                                            .remove(*local_last_added_drop);
                                                    }
                                                } else if !shift {
                                                    let _ = &self.droplog.drop.push(ItemDrop::new(
                                                        self.config
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
                                                        ChestType::Blue,
                                                        Option::Some("None".to_string()),
                                                    ));
                                                    let _ = self.config.droplog.drop.push(
                                                        ItemDrop::new(
                                                            self.config
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
                                                            ChestType::Blue,
                                                            Option::Some("None".to_string()),
                                                        ),
                                                    );
                                                }
                                            }
                                        }
                                        let drop_count = self
                                            .config
                                            .droplog
                                            .drop
                                            .iter()
                                            .filter(|x| {
                                                x.item == Item::LineageRing && x.raid == Raid::GOHL
                                            })
                                            .count();
                                        ui.label("x".to_string() + &drop_count.to_string());
                                    });
                                }
                                if self.config.app_settings.active_items[25] {
                                    ui.horizontal(|ui| {
                                        ui.spacing_mut().item_spacing.x = 3.;
                                        let local_last_added_drop = &self
                                            .config
                                            .droplog
                                            .drop
                                            .iter()
                                            .rposition(|x| {
                                                x.item == Item::SupremeMerit && x.raid == Raid::GOHL
                                            })
                                            .unwrap_or_default();
                                        if self.config.app_settings.button_label_combo[1] {
                                            if ui
                                                .add(CustomImageButton::new(
                                                    &ui.ctx().load_texture(
                                                        "GOHL SUPREME MERIT",
                                                        load_image_from_memory(S_MERIT).unwrap(),
                                                    ),
                                                    (32., 32.),
                                                ))
                                                .clicked()
                                            {
                                                let shift = ui.input().modifiers.shift_only();
                                                if shift {
                                                    if &self
                                                        .config
                                                        .droplog
                                                        .drop
                                                        .iter()
                                                        .filter(|&x| {
                                                            x.item == Item::SupremeMerit
                                                                && x.raid == Raid::GOHL
                                                        })
                                                        .count()
                                                        > &0
                                                    {
                                                        let _ = self
                                                            .config
                                                            .droplog
                                                            .drop
                                                            .remove(*local_last_added_drop);
                                                    }
                                                } else if !shift {
                                                    let _ = &self.droplog.drop.push(ItemDrop::new(
                                                        self.config
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
                                                        ChestType::Blue,
                                                        Option::Some("None".to_string()),
                                                    ));
                                                    let _ = self.config.droplog.drop.push(
                                                        ItemDrop::new(
                                                            self.config
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
                                                            ChestType::Blue,
                                                            Option::Some("None".to_string()),
                                                        ),
                                                    );
                                                }
                                            }
                                        }
                                        if self.config.app_settings.button_label_combo[0] {
                                            if ui.button("Supreme Merit").clicked() {
                                                let shift = ui.input().modifiers.shift_only();
                                                if shift {
                                                    if &self
                                                        .config
                                                        .droplog
                                                        .drop
                                                        .iter()
                                                        .filter(|&x| {
                                                            x.item == Item::SupremeMerit
                                                                && x.raid == Raid::GOHL
                                                        })
                                                        .count()
                                                        > &0
                                                    {
                                                        let _ = self
                                                            .config
                                                            .droplog
                                                            .drop
                                                            .remove(*local_last_added_drop);
                                                    }
                                                } else if !shift {
                                                    let _ = &self.droplog.drop.push(ItemDrop::new(
                                                        self.config
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
                                                        ChestType::Blue,
                                                        Option::Some("None".to_string()),
                                                    ));
                                                    let _ = self.config.droplog.drop.push(
                                                        ItemDrop::new(
                                                            self.config
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
                                                            ChestType::Blue,
                                                            Option::Some("None".to_string()),
                                                        ),
                                                    );
                                                }
                                            }
                                        }
                                        let drop_count = self
                                            .config
                                            .droplog
                                            .drop
                                            .iter()
                                            .filter(|x| {
                                                x.item == Item::SupremeMerit && x.raid == Raid::GOHL
                                            })
                                            .count();
                                        ui.label("x".to_string() + &drop_count.to_string());
                                    });
                                }
                                if self.config.app_settings.active_items[24]
                                    || self.config.app_settings.active_items[25]
                                {
                                    ui.end_row();
                                }
                                if self.config.app_settings.active_items[26] {
                                    ui.horizontal(|ui| {
                                        ui.spacing_mut().item_spacing.x = 3.;
                                        let local_last_added_drop = &self
                                            .config
                                            .droplog
                                            .drop
                                            .iter()
                                            .rposition(|x| {
                                                x.item == Item::IntricacyRing
                                                    && x.raid == Raid::GOHL
                                            })
                                            .unwrap_or_default();
                                        if self.config.app_settings.button_label_combo[1] {
                                            if ui
                                                .add(CustomImageButton::new(
                                                    &ui.ctx().load_texture(
                                                        "GOHL INTRICACY RING",
                                                        load_image_from_memory(I_RING).unwrap(),
                                                    ),
                                                    (32., 32.),
                                                ))
                                                .clicked()
                                            {
                                                let shift = ui.input().modifiers.shift_only();
                                                if shift {
                                                    if &self
                                                        .config
                                                        .droplog
                                                        .drop
                                                        .iter()
                                                        .filter(|&x| {
                                                            x.item == Item::IntricacyRing
                                                                && x.raid == Raid::GOHL
                                                        })
                                                        .count()
                                                        > &0
                                                    {
                                                        let _ = self
                                                            .config
                                                            .droplog
                                                            .drop
                                                            .remove(*local_last_added_drop);
                                                    }
                                                } else if !shift {
                                                    let _ = &self.droplog.drop.push(ItemDrop::new(
                                                        self.config
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
                                                        ChestType::Blue,
                                                        Option::Some("None".to_string()),
                                                    ));
                                                    let _ = self.config.droplog.drop.push(
                                                        ItemDrop::new(
                                                            self.config
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
                                                            ChestType::Blue,
                                                            Option::Some("None".to_string()),
                                                        ),
                                                    );
                                                }
                                            }
                                        }
                                        if self.config.app_settings.button_label_combo[0] {
                                            if ui.button("Intricacy Ring").clicked() {
                                                let shift = ui.input().modifiers.shift_only();
                                                if shift {
                                                    if &self
                                                        .config
                                                        .droplog
                                                        .drop
                                                        .iter()
                                                        .filter(|&x| {
                                                            x.item == Item::IntricacyRing
                                                                && x.raid == Raid::GOHL
                                                        })
                                                        .count()
                                                        > &0
                                                    {
                                                        let _ = self
                                                            .config
                                                            .droplog
                                                            .drop
                                                            .remove(*local_last_added_drop);
                                                    }
                                                } else if !shift {
                                                    let _ = &self.droplog.drop.push(ItemDrop::new(
                                                        self.config
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
                                                        ChestType::Blue,
                                                        Option::Some("None".to_string()),
                                                    ));
                                                    let _ = self.config.droplog.drop.push(
                                                        ItemDrop::new(
                                                            self.config
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
                                                            ChestType::Blue,
                                                            Option::Some("None".to_string()),
                                                        ),
                                                    );
                                                }
                                            }
                                        }
                                        let drop_count = self
                                            .config
                                            .droplog
                                            .drop
                                            .iter()
                                            .filter(|x| {
                                                x.item == Item::IntricacyRing
                                                    && x.raid == Raid::GOHL
                                            })
                                            .count();
                                        ui.label("x".to_string() + &drop_count.to_string());
                                    });
                                }
                                if self.config.app_settings.active_items[27] {
                                    ui.horizontal(|ui| {
                                        ui.spacing_mut().item_spacing.x = 3.;
                                        let local_last_added_drop = &self
                                            .config
                                            .droplog
                                            .drop
                                            .iter()
                                            .rposition(|x| {
                                                x.item == Item::LegendaryMerit
                                                    && x.raid == Raid::GOHL
                                            })
                                            .unwrap_or_default();
                                        if self.config.app_settings.button_label_combo[1] {
                                            if ui
                                                .add(CustomImageButton::new(
                                                    &ui.ctx().load_texture(
                                                        "GOHL LEGENDARY MERIT",
                                                        load_image_from_memory(L_MERIT).unwrap(),
                                                    ),
                                                    (32., 32.),
                                                ))
                                                .clicked()
                                            {
                                                let shift = ui.input().modifiers.shift_only();
                                                if shift {
                                                    if &self
                                                        .config
                                                        .droplog
                                                        .drop
                                                        .iter()
                                                        .filter(|&x| {
                                                            x.item == Item::LegendaryMerit
                                                                && x.raid == Raid::GOHL
                                                        })
                                                        .count()
                                                        > &0
                                                    {
                                                        let _ = self
                                                            .config
                                                            .droplog
                                                            .drop
                                                            .remove(*local_last_added_drop);
                                                    }
                                                } else if !shift {
                                                    let _ = &self.droplog.drop.push(ItemDrop::new(
                                                        self.config
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
                                                        ChestType::Blue,
                                                        Option::Some("None".to_string()),
                                                    ));
                                                    let _ = self.config.droplog.drop.push(
                                                        ItemDrop::new(
                                                            self.config
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
                                                            ChestType::Blue,
                                                            Option::Some("None".to_string()),
                                                        ),
                                                    );
                                                }
                                            }
                                        }
                                        if self.config.app_settings.button_label_combo[0] {
                                            if ui.button("Legendary Merit").clicked() {
                                                let shift = ui.input().modifiers.shift_only();
                                                if shift {
                                                    if &self
                                                        .config
                                                        .droplog
                                                        .drop
                                                        .iter()
                                                        .filter(|&x| {
                                                            x.item == Item::LegendaryMerit
                                                                && x.raid == Raid::GOHL
                                                        })
                                                        .count()
                                                        > &0
                                                    {
                                                        let _ = self
                                                            .config
                                                            .droplog
                                                            .drop
                                                            .remove(*local_last_added_drop);
                                                    }
                                                } else if !shift {
                                                    let _ = &self.droplog.drop.push(ItemDrop::new(
                                                        self.config
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
                                                        ChestType::Blue,
                                                        Option::Some("None".to_string()),
                                                    ));
                                                    let _ = self.config.droplog.drop.push(
                                                        ItemDrop::new(
                                                            self.config
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
                                                            ChestType::Blue,
                                                            Option::Some("None".to_string()),
                                                        ),
                                                    );
                                                }
                                            }
                                        }
                                        let drop_count = self
                                            .config
                                            .droplog
                                            .drop
                                            .iter()
                                            .filter(|x| {
                                                x.item == Item::LegendaryMerit
                                                    && x.raid == Raid::GOHL
                                            })
                                            .count();
                                        ui.label("x".to_string() + &drop_count.to_string());
                                    });
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
                                if self.config.app_settings.active_items[28] {
                                    ui.horizontal(|ui| {
                                        ui.spacing_mut().item_spacing.x = 3.;
                                        let local_last_added_drop = &self
                                            .config
                                            .droplog
                                            .drop
                                            .iter()
                                            .rposition(|x| {
                                                x.item == Item::GoldBrick
                                                    && x.raid == Raid::UBHL
                                                    && x.chest == ChestType::Host
                                            })
                                            .unwrap_or_default();
                                        if ui
                                            .add(CustomImageButton::new(
                                                &ui.ctx().load_texture(
                                                    "UBHL HOST BAR",
                                                    load_image_from_memory(GOLD_BAR).unwrap(),
                                                ),
                                                (32., 32.),
                                            ))
                                            .clicked()
                                        {
                                            let shift = ui.input().modifiers.shift_only();
                                            if shift {
                                                if &self
                                                    .config
                                                    .droplog
                                                    .drop
                                                    .iter()
                                                    .filter(|&x| {
                                                        x.item == Item::GoldBrick
                                                            && x.raid == Raid::UBHL
                                                            && x.chest == ChestType::Host
                                                    })
                                                    .count()
                                                    > &0
                                                {
                                                    let _ = self
                                                        .config
                                                        .droplog
                                                        .drop
                                                        .remove(*local_last_added_drop);
                                                }
                                            } else if !shift {
                                                let _ = &self.droplog.drop.push(ItemDrop::new(
                                                    self.config
                                                        .droplog
                                                        .drop
                                                        .clone()
                                                        .iter()
                                                        .count()
                                                        .try_into()
                                                        .unwrap(),
                                                    get_time(),
                                                    Raid::UBHL,
                                                    Item::GoldBrick,
                                                    ChestType::Host,
                                                    Option::Some("None".to_string()),
                                                ));
                                                let _ =
                                                    self.config.droplog.drop.push(ItemDrop::new(
                                                        self.config
                                                            .droplog
                                                            .drop
                                                            .clone()
                                                            .iter()
                                                            .count()
                                                            .try_into()
                                                            .unwrap(),
                                                        get_time(),
                                                        Raid::UBHL,
                                                        Item::GoldBrick,
                                                        ChestType::Host,
                                                        Option::Some("None".to_string()),
                                                    ));
                                            }
                                        }
                                        if ui.button("UBHL Host").clicked() {
                                            let shift = ui.input().modifiers.shift_only();
                                            if shift {
                                                if &self
                                                    .config
                                                    .droplog
                                                    .drop
                                                    .iter()
                                                    .filter(|&x| {
                                                        x.item == Item::GoldBrick
                                                            && x.raid == Raid::UBHL
                                                            && x.chest == ChestType::Host
                                                    })
                                                    .count()
                                                    > &0
                                                {
                                                    let _ = self
                                                        .config
                                                        .droplog
                                                        .drop
                                                        .remove(*local_last_added_drop);
                                                }
                                            } else if !shift {
                                                let _ = &self.droplog.drop.push(ItemDrop::new(
                                                    self.config
                                                        .droplog
                                                        .drop
                                                        .clone()
                                                        .iter()
                                                        .count()
                                                        .try_into()
                                                        .unwrap(),
                                                    get_time(),
                                                    Raid::UBHL,
                                                    Item::GoldBrick,
                                                    ChestType::Host,
                                                    Option::Some("None".to_string()),
                                                ));
                                                let _ =
                                                    self.config.droplog.drop.push(ItemDrop::new(
                                                        self.config
                                                            .droplog
                                                            .drop
                                                            .clone()
                                                            .iter()
                                                            .count()
                                                            .try_into()
                                                            .unwrap(),
                                                        get_time(),
                                                        Raid::UBHL,
                                                        Item::GoldBrick,
                                                        ChestType::Host,
                                                        Option::Some("None".to_string()),
                                                    ));
                                            }
                                        }
                                        let drop_count = self
                                            .config
                                            .droplog
                                            .drop
                                            .iter()
                                            .filter(|x| {
                                                x.item == Item::GoldBrick
                                                    && x.raid == Raid::UBHL
                                                    && x.chest == ChestType::Host
                                            })
                                            .count();
                                        ui.label("x".to_string() + &drop_count.to_string());
                                    });
                                }
                                if self.config.app_settings.active_items[29] {
                                    ui.horizontal(|ui| {
                                        ui.spacing_mut().item_spacing.x = 3.;
                                        let local_last_added_drop = &self
                                            .config
                                            .droplog
                                            .drop
                                            .iter()
                                            .rposition(|x| {
                                                x.item == Item::GoldBrick
                                                    && x.raid == Raid::UBHL
                                                    && x.chest == ChestType::Flip
                                            })
                                            .unwrap_or_default();
                                        if ui
                                            .add(CustomImageButton::new(
                                                &ui.ctx().load_texture(
                                                    "UBHL FLIP BAR",
                                                    load_image_from_memory(GOLD_BAR).unwrap(),
                                                ),
                                                (32., 32.),
                                            ))
                                            .clicked()
                                        {
                                            let shift = ui.input().modifiers.shift_only();
                                            if shift {
                                                if &self
                                                    .config
                                                    .droplog
                                                    .drop
                                                    .iter()
                                                    .filter(|&x| {
                                                        x.item == Item::GoldBrick
                                                            && x.raid == Raid::UBHL
                                                            && x.chest == ChestType::Flip
                                                    })
                                                    .count()
                                                    > &0
                                                {
                                                    let _ = self
                                                        .config
                                                        .droplog
                                                        .drop
                                                        .remove(*local_last_added_drop);
                                                }
                                            } else if !shift {
                                                let _ = &self.droplog.drop.push(ItemDrop::new(
                                                    self.config
                                                        .droplog
                                                        .drop
                                                        .clone()
                                                        .iter()
                                                        .count()
                                                        .try_into()
                                                        .unwrap(),
                                                    get_time(),
                                                    Raid::UBHL,
                                                    Item::GoldBrick,
                                                    ChestType::Flip,
                                                    Option::Some("None".to_string()),
                                                ));
                                                let _ =
                                                    self.config.droplog.drop.push(ItemDrop::new(
                                                        self.config
                                                            .droplog
                                                            .drop
                                                            .clone()
                                                            .iter()
                                                            .count()
                                                            .try_into()
                                                            .unwrap(),
                                                        get_time(),
                                                        Raid::UBHL,
                                                        Item::GoldBrick,
                                                        ChestType::Flip,
                                                        Option::Some("None".to_string()),
                                                    ));
                                            }
                                        }
                                        if ui.button("UBHL Flip").clicked() {
                                            let shift = ui.input().modifiers.shift_only();
                                            if shift {
                                                if &self
                                                    .config
                                                    .droplog
                                                    .drop
                                                    .iter()
                                                    .filter(|&x| {
                                                        x.item == Item::GoldBrick
                                                            && x.raid == Raid::UBHL
                                                            && x.chest == ChestType::Flip
                                                    })
                                                    .count()
                                                    > &0
                                                {
                                                    let _ = self
                                                        .config
                                                        .droplog
                                                        .drop
                                                        .remove(*local_last_added_drop);
                                                }
                                            } else if !shift {
                                                let _ = &self.droplog.drop.push(ItemDrop::new(
                                                    self.config
                                                        .droplog
                                                        .drop
                                                        .clone()
                                                        .iter()
                                                        .count()
                                                        .try_into()
                                                        .unwrap(),
                                                    get_time(),
                                                    Raid::UBHL,
                                                    Item::GoldBrick,
                                                    ChestType::Flip,
                                                    Option::Some("None".to_string()),
                                                ));
                                                let _ =
                                                    self.config.droplog.drop.push(ItemDrop::new(
                                                        self.config
                                                            .droplog
                                                            .drop
                                                            .clone()
                                                            .iter()
                                                            .count()
                                                            .try_into()
                                                            .unwrap(),
                                                        get_time(),
                                                        Raid::UBHL,
                                                        Item::GoldBrick,
                                                        ChestType::Flip,
                                                        Option::Some("None".to_string()),
                                                    ));
                                            }
                                        }
                                        let drop_count = self
                                            .config
                                            .droplog
                                            .drop
                                            .iter()
                                            .filter(|x| {
                                                x.item == Item::GoldBrick
                                                    && x.raid == Raid::UBHL
                                                    && x.chest == ChestType::Flip
                                            })
                                            .count();
                                        ui.label("x".to_string() + &drop_count.to_string());
                                    });
                                }
                                if self.config.app_settings.active_items[30] {
                                    ui.horizontal(|ui| {
                                        ui.spacing_mut().item_spacing.x = 3.;
                                        let local_last_added_drop = &self
                                            .config
                                            .droplog
                                            .drop
                                            .iter()
                                            .rposition(|x| {
                                                x.item == Item::GoldBrick
                                                    && x.raid == Raid::PBHL
                                                    && x.chest == ChestType::Host
                                            })
                                            .unwrap_or_default();
                                        if ui
                                            .add(CustomImageButton::new(
                                                &ui.ctx().load_texture(
                                                    "PBHL HOST BAR",
                                                    load_image_from_memory(GOLD_BAR).unwrap(),
                                                ),
                                                (32., 32.),
                                            ))
                                            .clicked()
                                        {
                                            let shift = ui.input().modifiers.shift_only();
                                            if shift {
                                                if &self
                                                    .config
                                                    .droplog
                                                    .drop
                                                    .iter()
                                                    .filter(|&x| {
                                                        x.item == Item::GoldBrick
                                                            && x.raid == Raid::PBHL
                                                            && x.chest == ChestType::Host
                                                    })
                                                    .count()
                                                    > &0
                                                {
                                                    let _ = self
                                                        .config
                                                        .droplog
                                                        .drop
                                                        .remove(*local_last_added_drop);
                                                }
                                            } else if !shift {
                                                let _ = &self.droplog.drop.push(ItemDrop::new(
                                                    self.config
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
                                                    ChestType::Host,
                                                    Option::Some("None".to_string()),
                                                ));
                                                let _ =
                                                    self.config.droplog.drop.push(ItemDrop::new(
                                                        self.config
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
                                                        ChestType::Host,
                                                        Option::Some("None".to_string()),
                                                    ));
                                            }
                                        }
                                        if ui.button("PBHL Host").clicked() {
                                            let shift = ui.input().modifiers.shift_only();
                                            if shift {
                                                if &self
                                                    .config
                                                    .droplog
                                                    .drop
                                                    .iter()
                                                    .filter(|&x| {
                                                        x.item == Item::GoldBrick
                                                            && x.raid == Raid::PBHL
                                                            && x.chest == ChestType::Host
                                                    })
                                                    .count()
                                                    > &0
                                                {
                                                    let _ = self
                                                        .config
                                                        .droplog
                                                        .drop
                                                        .remove(*local_last_added_drop);
                                                }
                                            } else if !shift {
                                                let _ = &self.droplog.drop.push(ItemDrop::new(
                                                    self.config
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
                                                    ChestType::Host,
                                                    Option::Some("None".to_string()),
                                                ));
                                                let _ =
                                                    self.config.droplog.drop.push(ItemDrop::new(
                                                        self.config
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
                                                        ChestType::Host,
                                                        Option::Some("None".to_string()),
                                                    ));
                                            }
                                        }
                                        let drop_count = self
                                            .config
                                            .droplog
                                            .drop
                                            .iter()
                                            .filter(|x| {
                                                x.item == Item::GoldBrick
                                                    && x.raid == Raid::PBHL
                                                    && x.chest == ChestType::Host
                                            })
                                            .count();
                                        ui.label("x".to_string() + &drop_count.to_string());
                                    });
                                }
                                if self.config.app_settings.active_items[31] {
                                    ui.horizontal(|ui| {
                                        ui.spacing_mut().item_spacing.x = 3.;
                                        let local_last_added_drop = &self
                                            .config
                                            .droplog
                                            .drop
                                            .iter()
                                            .rposition(|x| {
                                                x.item == Item::GoldBrick && x.raid == Raid::Xeno
                                            })
                                            .unwrap_or_default();
                                        if ui
                                            .add(CustomImageButton::new(
                                                &ui.ctx().load_texture(
                                                    "XENO FLIP BAR",
                                                    load_image_from_memory(GOLD_BAR).unwrap(),
                                                ),
                                                (32., 32.),
                                            ))
                                            .clicked()
                                        {
                                            let shift = ui.input().modifiers.shift_only();
                                            if shift {
                                                if &self
                                                    .config
                                                    .droplog
                                                    .drop
                                                    .iter()
                                                    .filter(|&x| {
                                                        x.item == Item::GoldBrick
                                                            && x.raid == Raid::Xeno
                                                            && x.chest == ChestType::Flip
                                                    })
                                                    .count()
                                                    > &0
                                                {
                                                    let _ = self
                                                        .config
                                                        .droplog
                                                        .drop
                                                        .remove(*local_last_added_drop);
                                                }
                                            } else if !shift {
                                                let _ = &self.droplog.drop.push(ItemDrop::new(
                                                    self.config
                                                        .droplog
                                                        .drop
                                                        .clone()
                                                        .iter()
                                                        .count()
                                                        .try_into()
                                                        .unwrap(),
                                                    get_time(),
                                                    Raid::Xeno,
                                                    Item::GoldBrick,
                                                    ChestType::Flip,
                                                    Option::Some("None".to_string()),
                                                ));
                                                let _ =
                                                    self.config.droplog.drop.push(ItemDrop::new(
                                                        self.config
                                                            .droplog
                                                            .drop
                                                            .clone()
                                                            .iter()
                                                            .count()
                                                            .try_into()
                                                            .unwrap(),
                                                        get_time(),
                                                        Raid::Xeno,
                                                        Item::GoldBrick,
                                                        ChestType::Flip,
                                                        Option::Some("None".to_string()),
                                                    ));
                                            }
                                        }
                                        if ui.button("Xeno").clicked() {
                                            let shift = ui.input().modifiers.shift_only();
                                            if shift {
                                                if &self
                                                    .config
                                                    .droplog
                                                    .drop
                                                    .iter()
                                                    .filter(|&x| {
                                                        x.item == Item::GoldBrick
                                                            && x.raid == Raid::Xeno
                                                    })
                                                    .count()
                                                    > &0
                                                {
                                                    let _ = self
                                                        .config
                                                        .droplog
                                                        .drop
                                                        .remove(*local_last_added_drop);
                                                }
                                            } else if !shift {
                                                let _ = &self.droplog.drop.push(ItemDrop::new(
                                                    self.config
                                                        .droplog
                                                        .drop
                                                        .clone()
                                                        .iter()
                                                        .count()
                                                        .try_into()
                                                        .unwrap(),
                                                    get_time(),
                                                    Raid::Xeno,
                                                    Item::GoldBrick,
                                                    ChestType::Flip,
                                                    Option::Some("None".to_string()),
                                                ));
                                                let _ =
                                                    self.config.droplog.drop.push(ItemDrop::new(
                                                        self.config
                                                            .droplog
                                                            .drop
                                                            .clone()
                                                            .iter()
                                                            .count()
                                                            .try_into()
                                                            .unwrap(),
                                                        get_time(),
                                                        Raid::Xeno,
                                                        Item::GoldBrick,
                                                        ChestType::Flip,
                                                        Option::Some("None".to_string()),
                                                    ));
                                            }
                                        }
                                        let drop_count = self
                                            .config
                                            .droplog
                                            .drop
                                            .iter()
                                            .filter(|x| {
                                                x.item == Item::GoldBrick && x.raid == Raid::Xeno
                                            })
                                            .count();
                                        ui.label("x".to_string() + &drop_count.to_string());
                                    });
                                }
                            });
                    }
                });
            ui.add_space(50.);
            egui::warn_if_debug_build(ui);
        });
        if !self.config.app_settings.button_label_combo[0]
            && !self.config.app_settings.button_label_combo[1]
        {
            egui::Window::new("Warning: Can't log items!").show(ctx, |ui| {
                ui.label(
                    "Oh you've really screwed up now haven't you? Turn one of those back on."
                        .to_string(),
                );
            });
        }
        if self.config.app_settings.toggle_active_items {
            egui::Window::new("Grid Features").vscroll(true).show(ctx, |ui| {

                    if ui
                        .checkbox(
                            &mut self.config.app_settings.button_label_combo[0],
                            "Show buttons",
                        )
                        .clicked(){}
                    if ui
                        .checkbox(
                            &mut self.config.app_settings.button_label_combo[1],
                            "Show icons",
                        )
                        .clicked()
                    {
                        local_settings_copy.app_settings.button_label_combo[1] =
                            self.config.app_settings.button_label_combo[1];
                    }
                if ui
                    .checkbox(
                        &mut self.config.app_settings.vertical_grid,
                        "Change item count placement to below icons/buttons",
                    )
                    .clicked()
                {
                    local_settings_copy.app_settings.vertical_grid =
                        self.config.app_settings.vertical_grid;
                }
                ui.add_space(5.);
                ui.label("Grid X Spacing.".to_string());
                if ui.add(egui::Slider::new(&mut self.config.app_settings.grid_spacing_x, 0.0..=50.0)).changed() {
                    local_settings_copy.app_settings.grid_spacing_x =
                        self.config.app_settings.grid_spacing_x;

                }
                ui.add_space(5.);
                ui.label("Grid Y Spacing.".to_string());
                if ui.add(egui::Slider::new(&mut self.config.app_settings.grid_spacing_y, 0.0..=50.0)).changed() {
                    local_settings_copy.app_settings.grid_spacing_y =
                            self.config.app_settings.grid_spacing_y;

                }
                ui.add_space(5.);
                ui.label("The grid isn't smart enough to adjust but you can toggle specific items off here.".to_string());
                ui.heading("Akasha");
                    if ui
                        .checkbox(
                            &mut self.config.app_settings.active_items[0],
                            "Show No Blue Box",
                        )
                        .clicked()
                    {
                        local_settings_copy.app_settings.active_items[0] =
                            self.config.app_settings.active_items[0];

                    }
                    if ui
                        .checkbox(
                            &mut self.config.app_settings.active_items[1],
                            "Show Hollow Key",
                        )
                        .clicked()
                    {
                        local_settings_copy.app_settings.active_items[1] =
                            self.config.app_settings.active_items[1];

                    }
                    if ui
                        .checkbox(
                            &mut self.config.app_settings.active_items[2],
                            "Show Silver Centrum",
                        )
                        .clicked()
                    {
                        local_settings_copy.app_settings.active_items[2] =
                            self.config.app_settings.active_items[2];

                    }
                    if ui
                        .checkbox(
                            &mut self.config.app_settings.active_items[3],
                            "Show Gold Bar",
                        )
                        .clicked()
                    {
                        local_settings_copy.app_settings.active_items[3] =
                            self.config.app_settings.active_items[3];

                    }
                    if ui
                        .checkbox(
                            &mut self.config.app_settings.active_items[4],
                            "Show Coronation Ring",
                        )
                        .clicked()
                    {
                        local_settings_copy.app_settings.active_items[4] =
                            self.config.app_settings.active_items[4];

                    }
                    if ui
                        .checkbox(
                            &mut self.config.app_settings.active_items[7],
                            "Show Lineage Ring",
                        )
                        .clicked()
                    {
                        local_settings_copy.app_settings.active_items[7] =
                            self.config.app_settings.active_items[7];

                    }
                    if ui
                        .checkbox(
                            &mut self.config.app_settings.active_items[10],
                            "Show Intricacy Ring",
                        )
                        .clicked()
                    {
                        local_settings_copy.app_settings.active_items[10] =
                            self.config.app_settings.active_items[10];

                    }
                    if ui
                        .checkbox(
                            &mut self.config.app_settings.active_items[5],
                            "Show Champion Merit",
                        )
                        .clicked()
                    {
                        local_settings_copy.app_settings.active_items[5] =
                            self.config.app_settings.active_items[5];

                    }
                    if ui
                        .checkbox(
                            &mut self.config.app_settings.active_items[8],
                            "Show Supreme Merit",
                        )
                        .clicked()
                    {
                        local_settings_copy.app_settings.active_items[8] =
                            self.config.app_settings.active_items[8];

                    }
                    if ui
                        .checkbox(
                            &mut self.config.app_settings.active_items[11],
                            "Show Legendary Merit",
                        )
                        .clicked()
                    {
                        local_settings_copy.app_settings.active_items[11] =
                            self.config.app_settings.active_items[11];

                    }
                    if ui
                        .checkbox(
                            &mut self.config.app_settings.active_items[6],
                            "Show +1 Mark",
                        )
                        .clicked()
                    {
                        local_settings_copy.app_settings.active_items[6] =
                            self.config.app_settings.active_items[6];

                    }
                    if ui
                        .checkbox(
                            &mut self.config.app_settings.active_items[9],
                            "Show +2 Mark",
                        )
                        .clicked()
                    {
                        local_settings_copy.app_settings.active_items[9] =
                            self.config.app_settings.active_items[9];

                    }
                    if ui
                        .checkbox(
                            &mut self.config.app_settings.active_items[12],
                            "Show +3 Mark",
                        )
                        .clicked()
                    {
                        local_settings_copy.app_settings.active_items[12] =
                            self.config.app_settings.active_items[12];

                    }
                ui.heading("PBHL");
                    if ui
                        .checkbox(
                            &mut self.config.app_settings.active_items[13],
                            "Show No Blue Box",
                        )
                        .clicked()
                    {
                        local_settings_copy.app_settings.active_items[13] =
                            self.config.app_settings.active_items[13];

                    }
                    if ui
                        .checkbox(
                            &mut self.config.app_settings.active_items[14],
                            "Show Gold Bar",
                        )
                        .clicked()
                    {
                        local_settings_copy.app_settings.active_items[14] =
                            self.config.app_settings.active_items[14];

                    }
                    if ui
                        .checkbox(
                            &mut self.config.app_settings.active_items[15],
                            "Show Coronation Ring",
                        )
                        .clicked()
                    {
                        local_settings_copy.app_settings.active_items[15] =
                            self.config.app_settings.active_items[15];

                    }
                    if ui
                        .checkbox(
                            &mut self.config.app_settings.active_items[16],
                            "Show Lineage Ring",
                        )
                        .clicked()
                    {
                        local_settings_copy.app_settings.active_items[16] =
                            self.config.app_settings.active_items[16];

                    }
                    if ui
                        .checkbox(
                            &mut self.config.app_settings.active_items[17],
                            "Show Lineage Ring",
                        )
                        .clicked()
                    {
                        local_settings_copy.app_settings.active_items[17] =
                            self.config.app_settings.active_items[17];

                    }
                ui.heading("GOHL");
                    if ui
                        .checkbox(
                            &mut self.config.app_settings.active_items[18],
                            "Show No Blue Box",
                        )
                        .clicked()
                    {
                        local_settings_copy.app_settings.active_items[18] =
                            self.config.app_settings.active_items[18];

                    }
                    if ui
                        .checkbox(
                            &mut self.config.app_settings.active_items[19],
                            "Show Verdant Azurite",
                        )
                        .clicked()
                    {
                        local_settings_copy.app_settings.active_items[19] =
                            self.config.app_settings.active_items[19];

                    }
                    if ui
                        .checkbox(
                            &mut self.config.app_settings.active_items[20],
                            "Show Silver Centrum",
                        )
                        .clicked()
                    {
                        local_settings_copy.app_settings.active_items[20] =
                            self.config.app_settings.active_items[20];

                    }
                    if ui
                        .checkbox(
                            &mut self.config.app_settings.active_items[21],
                            "Show Gold Bar",
                        )
                        .clicked()
                    {
                        local_settings_copy.app_settings.active_items[21] =
                            self.config.app_settings.active_items[21];

                    }
                    if ui
                        .checkbox(
                            &mut self.config.app_settings.active_items[22],
                            "Show Coronation Ring",
                        )
                        .clicked()
                    {
                        local_settings_copy.app_settings.active_items[22] =
                            self.config.app_settings.active_items[22];

                    }
                    if ui
                        .checkbox(
                            &mut self.config.app_settings.active_items[24],
                            "Show Lineage Ring",
                        )
                        .clicked()
                    {
                        local_settings_copy.app_settings.active_items[24] =
                            self.config.app_settings.active_items[24];

                    }
                    if ui
                        .checkbox(
                            &mut self.config.app_settings.active_items[26],
                            "Show Intricacy Ring",
                        )
                        .clicked()
                    {
                        local_settings_copy.app_settings.active_items[26] =
                            self.config.app_settings.active_items[26];

                    }
                    if ui
                        .checkbox(
                            &mut self.config.app_settings.active_items[23],
                            "Show Champion Merit",
                        )
                        .clicked()
                    {
                        local_settings_copy.app_settings.active_items[23] =
                            self.config.app_settings.active_items[23];

                    }
                    if ui
                        .checkbox(
                            &mut self.config.app_settings.active_items[25],
                            "Show Supreme Merit",
                        )
                        .clicked()
                    {
                        local_settings_copy.app_settings.active_items[25] =
                            self.config.app_settings.active_items[25];

                    }
                    if ui
                        .checkbox(
                            &mut self.config.app_settings.active_items[27],
                            "Show Legendary Merit",
                        )
                        .clicked()
                    {
                        local_settings_copy.app_settings.active_items[27] =
                            self.config.app_settings.active_items[27];

                    }
                ui.heading("Hosts");
                    if ui
                        .checkbox(
                            &mut self.config.app_settings.active_items[28],
                            "Show UBHL Host Gold Bar",
                        )
                        .clicked()
                    {
                        local_settings_copy.app_settings.active_items[28] =
                            self.config.app_settings.active_items[28];

                    }
                    if ui
                        .checkbox(
                            &mut self.config.app_settings.active_items[29],
                            "Show UBHL Flip Gold Bar",
                        )
                        .clicked()
                    {
                        local_settings_copy.app_settings.active_items[29] =
                            self.config.app_settings.active_items[29];

                    }
                    if ui
                        .checkbox(
                            &mut self.config.app_settings.active_items[30],
                            "Show PBHL Host Gold Bar",
                        )
                        .clicked()
                    {
                        local_settings_copy.app_settings.active_items[30] =
                            self.config.app_settings.active_items[30];

                    }
                    if ui
                        .checkbox(
                            &mut self.config.app_settings.active_items[31],
                            "Show Xeno Gold Bar",
                        )
                        .clicked()
                    {
                        local_settings_copy.app_settings.active_items[31] =
                            self.config.app_settings.active_items[31];

                    }

            });
        }
    }
}
