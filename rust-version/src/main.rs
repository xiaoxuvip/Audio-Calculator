#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod audio;
mod screenshot;
mod settings;
mod shell;

use app::AudioCalculatorApp;
use eframe::egui;

fn main() -> eframe::Result<()> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([420.0, 620.0])
            .with_min_inner_size([420.0, 620.0])
            .with_max_inner_size([420.0, 620.0])
            .with_resizable(false)
            .with_icon(load_icon()),
        centered: true,
        ..Default::default()
    };

    eframe::run_native(
        "音频时长统计 v2.0 BY:晓旭",
        options,
        Box::new(move |cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            setup_fonts(&cc.egui_ctx);
            Ok(Box::new(AudioCalculatorApp::new(cc, args.clone())))
        }),
    )
}

fn load_icon() -> egui::IconData {
    let icon_bytes = include_bytes!("../../logo.png");
    let image = image::load_from_memory(icon_bytes)
        .expect("Failed to load icon")
        .into_rgba8();
    let (width, height) = image.dimensions();
    egui::IconData {
        rgba: image.into_raw(),
        width,
        height,
    }
}

fn setup_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();
    
    // 加载微软雅黑字体
    if let Ok(font_data) = std::fs::read("C:\\Windows\\Fonts\\msyh.ttc") {
        fonts.font_data.insert(
            "microsoft_yahei".to_owned(),
            egui::FontData::from_owned(font_data).into(),
        );
        
        fonts.families
            .entry(egui::FontFamily::Proportional)
            .or_default()
            .insert(0, "microsoft_yahei".to_owned());
        
        fonts.families
            .entry(egui::FontFamily::Monospace)
            .or_default()
            .push("microsoft_yahei".to_owned());
    }
    
    ctx.set_fonts(fonts);
}
