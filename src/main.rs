use eframe::{
    egui::{self, Vec2, ViewportBuilder, WindowLevel},
    NativeOptions,
};
use state::Projects;

mod state;
mod util;
mod view;

// #[derive(Default)]
// struct MyEguiApp {}

// impl MyEguiApp {
//     fn new(cc: &eframe::CreationContext<'_>) -> Self {
//         // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
//         // Restore app state using cc.storage (requires the "persistence" feature).
//         // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
//         // for e.g. egui::PaintCallback.
//         Self::default()
//     }
// }

struct Beamline {
    projects: Projects,
}

impl Beamline {
    fn new(cc: &eframe::CreationContext<'_>, projects: Projects) -> Self {
        //setup_style(&cc.egui_ctx);
        Self { projects }
    }
}

impl eframe::App for Beamline {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        view::project_manager(ctx, frame, &self.projects.projects);
    }
}

// fn setup_style(ctx: &egui::Context) {
//     ctx.style_mut(|style| {
//         style.visuals.widgets.noninteractive.rounding = egui::Rounding::ZERO;
//         style.visuals.menu_rounding = egui::Rounding::ZERO;
//         style.visuals.widgets.open.rounding = egui::Rounding::ZERO;
//         style.visuals.widgets.active.rounding = egui::Rounding::ZERO;
//     });
// }

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
    let settings = match state::Settings::new() {
        Ok(settings) => settings,
        Err(e) => {
            eprintln!("There was an error opening the settings.toml file:\n{}", e);
            std::process::exit(1);
        }
    };

    let projects = match state::Projects::new(&settings) {
        Some(projects) => projects,
        None => {
            eprintln!("There was an error creating the Projects data struct");
            std::process::exit(1);
        }
    };

    dbg!(&projects);

    eframe::run_native(
        "Beamline",
        build_native_options(),
        Box::new(|cc| Ok(Box::new(Beamline::new(cc, projects)))),
    )
}
