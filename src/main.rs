use eframe::{
    egui::{self, Vec2, ViewportBuilder, WindowLevel},
    NativeOptions,
};
use state::Screen;

mod elements;
mod state;
mod view;

pub struct Beamline {
    pub screen: Screen,
}

impl Beamline {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            screen: Screen::ProjectManager,
        }
    }
}

impl eframe::App for Beamline {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        match &self.screen {
            Screen::ProjectManager => view::render_project_manager(self, ctx, frame),
        }
    }
}

fn build_viewport() -> ViewportBuilder {
    ViewportBuilder {
        title: Some(String::from("Beamline - Project Manager")),
        window_level: Some(WindowLevel::AlwaysOnTop),
        inner_size: Some(Vec2::new(600.0, 400.0)),
        ..ViewportBuilder::default()
    }
}

fn build_native_options() -> NativeOptions {
    NativeOptions {
        viewport: build_viewport(),
        centered: true,
        ..NativeOptions::default()
    }
}

fn main() -> eframe::Result {
    eframe::run_native(
        "Beamline",
        build_native_options(),
        Box::new(|cc| Ok(Box::new(Beamline::new(cc)))),
    )
}
