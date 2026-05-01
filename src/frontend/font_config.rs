use eframe::egui::{self, FontData, FontDefinitions, FontFamily, TextStyle, FontId};
use std::collections::BTreeMap;

// examples from
// https://github.com/emilk/egui/blob/main/examples/custom_font_style/src/main.rs
// https://github.com/emilk/egui/blob/main/examples/custom_font/src/main.rs

#[inline]
fn heading2() -> TextStyle {
    TextStyle::Name("Heading2".into())
}

#[inline]
fn heading3() -> TextStyle {
    TextStyle::Name("ContextHeading".into())
}

pub fn configure_text_styles(ctx: &egui::Context) {
    use FontFamily::{Monospace, Proportional};

    let text_styles: BTreeMap<TextStyle, FontId> = [
        (TextStyle::Heading, FontId::new(25.0, Proportional)),
        (heading2(), FontId::new(22.0, Proportional)),
        (heading3(), FontId::new(19.0, Proportional)),
        (TextStyle::Body, FontId::new(16.0, Proportional)),
        (TextStyle::Monospace, FontId::new(12.0, Monospace)),
        (TextStyle::Button, FontId::new(12.0, Proportional)),
        (TextStyle::Small, FontId::new(8.0, Proportional)),
    ]
    .into();
    ctx.all_styles_mut(move |style| style.text_styles = text_styles.clone());
}


pub fn replace_fonts(ctx: &egui::Context) {
    let mut fonts = FontDefinitions::default();

    fonts.font_data.insert(
        "my_font".to_owned(),
        std::sync::Arc::new(egui::FontData::from_static(include_bytes!(
        "assets\\fonts\\JetBrainsMonoNLNerdFont-Medium.ttf"
        ))),
    );

    fonts.families
        .get_mut(&FontFamily::Monospace)
        .unwrap()
        .insert(0, "my_font".to_owned());

    // Put my font first (highest priority) for Monospace text:
    fonts
        .families
        .entry(egui::FontFamily::Monospace)
        .or_default()
        .insert(0, "my_font".to_owned());

    // Put my font as last fallback for monospace:
    // fonts
    //     .families
    //     .entry(egui::FontFamily::Monospace)
    //     .or_default()
    //     .push("my_font".to_owned());

    ctx.set_fonts(fonts);
}