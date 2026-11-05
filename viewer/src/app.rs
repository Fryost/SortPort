use crate::data::{self, Table};
use crate::style::{self, BEIGE, BLUE, GREY, WHITE};
use eframe::egui::{self, RichText};
use egui_extras::{Column, TableBuilder};
use std::path::PathBuf;
use std::time::{Duration, SystemTime};

pub struct Viewer {
    folder: PathBuf,
    tables: Vec<Table>,
    selected: usize,
    loaded_at: Option<SystemTime>,
}

impl Viewer {
    pub fn new(folder: &str) -> Self {
        let mut viewer = Viewer { folder: PathBuf::from(folder), tables: Vec::new(), selected: 0, loaded_at: None };
        viewer.reload();
        viewer
    }

    fn reload(&mut self) {
        self.tables = data::load_tables(&self.folder);
        self.loaded_at = data::last_change(&self.folder);
        if self.selected >= self.tables.len() {
            self.selected = 0;
        }
    }

    fn tabs(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            for (i, table) in self.tables.iter().enumerate() {
                let fill = if i == self.selected { WHITE } else { BEIGE };
                if ui.add(egui::Button::new(&table.name).fill(fill)).clicked() {
                    self.selected = i;
                }
            }
            if ui.button("Reload").clicked() {
                self.reload();
            }
        });
    }

    fn table(&self, ui: &mut egui::Ui) {
        let Some(table) = self.tables.get(self.selected) else {
            ui.label(RichText::new("No CSV files in the output folder yet.").color(GREY));
            return;
        };
        let mut builder = TableBuilder::new(ui).striped(true).resizable(true).column(Column::auto().at_least(40.0));
        for _ in &table.headers {
            builder = builder.column(Column::remainder().at_least(60.0).clip(true));
        }
        builder
            .header(22.0, |mut header| {
                header.col(|ui| {
                    ui.label(RichText::new("#").color(GREY));
                });
                for name in &table.headers {
                    header.col(|ui| {
                        ui.label(RichText::new(name).font(style::bold(13.0)));
                    });
                }
            })
            .body(|body| {
                body.rows(20.0, table.rows.len(), |mut row| {
                    let index = row.index();
                    row.col(|ui| {
                        ui.label(RichText::new((index + 1).to_string()).color(GREY));
                    });
                    for cell in &table.rows[index] {
                        row.col(|ui| {
                            ui.label(cell.replace('\n', " "));
                        });
                    }
                });
            });
    }

    fn status(&self, ui: &mut egui::Ui) {
        if let Some(table) = self.tables.get(self.selected) {
            ui.label(format!("{} rows, {} columns", table.rows.len(), table.headers.len()));
        }
    }
}

impl eframe::App for Viewer {
    fn logic(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if data::last_change(&self.folder) != self.loaded_at {
            self.reload();
        }
        ctx.request_repaint_after(Duration::from_secs(1));
    }

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let bar = |fill| egui::Frame::new().fill(fill).inner_margin(6.0);
        egui::Panel::top("title").show_separator_line(false).frame(bar(BLUE)).show(ui, |ui| {
            ui.vertical_centered(|ui| {
                ui.label(RichText::new("SortPort Custom Viewer!").font(style::bold(15.0)).color(WHITE));
            });
        });
        egui::Panel::top("tabs").show_separator_line(false).frame(bar(BEIGE)).show(ui, |ui| self.tabs(ui));
        egui::Panel::bottom("status").show_separator_line(false).frame(bar(BEIGE)).show(ui, |ui| self.status(ui));
        egui::CentralPanel::default().frame(bar(WHITE)).show(ui, |ui| self.table(ui));
    }
}