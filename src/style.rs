use eframe::egui;

pub fn styles() -> egui::Style {
    let mut style = egui::Style::default();

    style.visuals.window_fill = egui::Color32::from_black_alpha(220);
    style.visuals.widgets.active.bg_fill = egui::Color32::from_rgb(0, 100, 255);
    style.visuals.widgets.hovered.bg_fill = egui::Color32::from_rgb(0, 150, 255);
    style.visuals.override_text_color = Some(egui::Color32::from_rgb(255, 255, 255));

    style


}