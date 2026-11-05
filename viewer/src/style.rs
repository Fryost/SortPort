use eframe::egui::{self, Color32, CornerRadius, FontData, FontDefinitions, FontFamily, FontId, Stroke, TextStyle};
use std::sync::Arc;

pub const BEIGE: Color32 = Color32::from_rgb(236, 233, 216);
pub const BLUE: Color32 = Color32::from_rgb(0, 84, 227);
pub const WHITE: Color32 = Color32::WHITE;
pub const GREY: Color32 = Color32::from_rgb(90, 90, 90);

pub fn bold(size: f32) -> FontId {
    FontId::new(size, FontFamily::Name("bold".into()))
}

pub fn apply(ctx: &egui::Context) {
    ctx.set_fonts(fonts());
    ctx.set_theme(egui::Theme::Light);
    ctx.all_styles_mut(|style| {
        style.text_styles.insert(TextStyle::Body, FontId::proportional(13.0));
        style.text_styles.insert(TextStyle::Button, FontId::proportional(13.0));
        let v = &mut style.visuals;
        v.panel_fill = BEIGE;
        v.extreme_bg_color = WHITE;
        v.faint_bg_color = Color32::from_rgb(241, 245, 251);
        v.override_text_color = Some(Color32::BLACK);
        v.selection.bg_fill = Color32::from_rgb(49, 106, 197);
        v.widgets.noninteractive.bg_stroke = Stroke::NONE;
        v.widgets.inactive.weak_bg_fill = Color32::from_rgb(245, 244, 237);
        v.widgets.inactive.bg_stroke = Stroke::new(1.0, Color32::from_rgb(0, 60, 116));
        v.widgets.hovered.weak_bg_fill = WHITE;
        v.widgets.hovered.bg_stroke = Stroke::new(1.0, Color32::from_rgb(0, 30, 90));
        v.widgets.inactive.corner_radius = CornerRadius::same(3);
        v.widgets.hovered.corner_radius = CornerRadius::same(3);
        v.widgets.active.corner_radius = CornerRadius::same(3);
    });
}

fn fonts() -> FontDefinitions {
    let mut fonts = FontDefinitions::default();
    let regular = std::fs::read(r"C:\Windows\Fonts\arial.ttf");
    let bold = std::fs::read(r"C:\Windows\Fonts\arialbd.ttf");
    if let (Ok(regular), Ok(bold)) = (regular, bold) {
        fonts.font_data.insert("arial".into(), Arc::new(FontData::from_owned(regular)));
        fonts.font_data.insert("arial_bold".into(), Arc::new(FontData::from_owned(bold)));
        fonts.families.get_mut(&FontFamily::Proportional).unwrap().insert(0, "arial".into());
    }
    let mut bold_family = fonts.families[&FontFamily::Proportional].clone();
    if fonts.font_data.contains_key("arial_bold") {
        bold_family.insert(0, "arial_bold".into());
    }
    fonts.families.insert(FontFamily::Name("bold".into()), bold_family);
    fonts
}