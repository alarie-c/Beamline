use eframe::{
    egui::{self, Vec2, ViewportBuilder, WindowLevel},
    NativeOptions,
};
use state::{Project, ProjectPrimitive, Projects};

mod state;
mod util;
mod view;

pub enum Screen {
    ProjectManager,
    NewProject(ProjectPrimitive),
}

pub struct Beamline {
    pub screen: Screen,
    pub projects: Projects,
}

impl Beamline {
    fn new(cc: &eframe::CreationContext<'_>, projects: Projects) -> Self {
        //setup_style(&cc.egui_ctx);
        Self {
            screen: Screen::ProjectManager,
            projects,
        }
    }
    pub fn new_project(&mut self) {
        self.screen = Screen::NewProject(ProjectPrimitive::new_blank_for_project_creator());
    }
}

impl eframe::App for Beamline {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        match &mut self.screen {
            Screen::ProjectManager => view::project_manager(self, ctx, frame),
            Screen::NewProject(project_primitive) => {
                view::popup_new_project(ctx, project_primitive)
            }
        }
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
