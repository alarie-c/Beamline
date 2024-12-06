use crate::Beamline;
use eframe::egui::{self, Vec2};

pub fn render_project_manager(app: &mut Beamline, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    // Configure the viewport for this screen
    // Set the size to (600, 400)
    // Remove the titlebar and outline
    // Set the window level to be always on top
    egui::ViewportCommand::center_on_screen(ctx);
    ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(Vec2::new(600.0, 400.0)));
    ctx.send_viewport_cmd(egui::ViewportCommand::Resizable(false));
    ctx.send_viewport_cmd(egui::ViewportCommand::WindowLevel(
        egui::WindowLevel::AlwaysOnTop,
    ));

    // Will be used later to create the menu buttons on the right panel
    #[inline(always)]
    fn menu_button(ui: &mut eframe::egui::Ui, text: &str, width: f32) -> egui::Response {
        let response = ui.add_sized(Vec2::new(width, 40.0), egui::Button::new(text));
        return response;
    }

    // Widths for each side of the panels
    let width_l: f32 = ctx.available_rect().width() * (2.0 / 3.0);
    let width_r: f32 = ctx.available_rect().width() * (1.0 / 3.0);

    egui::SidePanel::left("panel_projects")
        .resizable(false)
        .default_width(width_l)
        .show(ctx, |ui| {
            ui.heading("Recent Projects");
            ui.separator();
            ui.add_space(10.0);

            // Create cards for recent projects
            egui::ScrollArea::vertical().show(ui, |_| {
            })
        });

    egui::SidePanel::right("panel_buttons")
        .default_width(width_r)
        .resizable(false)
        .show(ctx, |ui| {
            ui.heading("Get Started");
            ui.separator();

            ui.vertical(|ui| {
                ui.add_space(10.0);
                ui.horizontal(|ui| {
                    if menu_button(ui, "New Project", width_r - 20.0).clicked() {
                        println!("New project");
                    }
                });
                ui.add_space(10.0);
                ui.horizontal(|ui| {
                    if menu_button(ui, "Open Project", width_r - 20.0).clicked() {
                        println!("Open");
                    }
                });
                ui.add_space(10.0);
                ui.horizontal(|ui| {
                    if menu_button(ui, "Quit to Desktop", width_r - 20.0).clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
            });
        });
}
