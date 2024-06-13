use eframe::egui;
use crate::api::{get_option_margin, get_option_symbols, OptionSymbol, OptionMarginResponse};
use tokio::runtime::Runtime;
use std::time::{Duration, Instant};

pub struct App {
    api_key: String,
    access_token: String,
    symbols: Vec<OptionSymbol>,
    selected_symbol: Option<String>,
    margin_info: Option<OptionMarginResponse>,
    last_refresh: Instant,
    error_message: Option<String>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            api_key: "".to_owned(),
            access_token: "".to_owned(),
            symbols: vec![],
            selected_symbol: None,
            margin_info: None,
            last_refresh: Instant::now(),
            error_message: None,
        }
    }
}

impl App {
    pub fn refresh_symbols(&mut self) {
        let api_key = self.api_key.clone();
        let access_token = self.access_token.clone();
        let rt = Runtime::new().unwrap();
        rt.block_on(async {
            match get_option_symbols(&api_key, &access_token).await {
                Ok(symbols) => self.symbols = symbols,
                Err(err) => self.error_message = Some(format!("Error fetching symbols: {:?}", err)),
            }
        });
        self.last_refresh = Instant::now();
    }

    pub fn fetch_margin(&mut self) {
        if let Some(symbol) = &self.selected_symbol {
            let api_key = self.api_key.clone();
            let access_token = self.access_token.clone();
            let rt = Runtime::new().unwrap();
            rt.block_on(async {
                match get_option_margin(&api_key, &access_token, symbol, "NSE", "FO").await {
                    Ok(margin) => self.margin_info = Some(margin),
                    Err(err) => self.error_message = Some(format!("Error fetching margin: {:?}", err)),
                }
            });
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Zerodha Option Margin");

            ui.horizontal(|ui| {
                ui.label("API Key:");
                ui.text_edit_singleline(&mut self.api_key);
            });

            ui.horizontal(|ui| {
                ui.label("Access Token:");
                ui.text_edit_singleline(&mut self.access_token);
            });

            if ui.button("Refresh Symbols").clicked() {
                self.refresh_symbols();
            }

            if Instant::now().duration_since(self.last_refresh) > Duration::from_secs(60) {
                self.refresh_symbols();
            }

            egui::ComboBox::from_label("Select Symbol")
                .selected_text(self.selected_symbol.clone().unwrap_or_default())
                .show_ui(ui, |ui| {
                    for symbol in &self.symbols {
                        ui.selectable_value(&mut self.selected_symbol, Some(symbol.tradingsymbol.clone()), &symbol.tradingsymbol);
                    }
                });

            if ui.button("Get Margin").clicked() {
                self.fetch_margin();
            }

            if let Some(margin_info) = &self.margin_info {
                ui.label("Margin Info:");
                for (symbol, margin) in &margin_info.data {
                    ui.group(|ui| {
                        ui.label(format!("Symbol: {}", symbol));
                        ui.label(format!("Initial Margin: {}", margin.initial));
                        ui.label(format!("Maintenance Margin: {}", margin.maintenance));
                        ui.label(format!("Total Margin: {}", margin.total));
                    });
                }
            }

            if let Some(error_message) = &self.error_message {
                ui.label(format!("Error: {}", error_message));
            }
        });
    }
}
