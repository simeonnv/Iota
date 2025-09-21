use eframe::egui;
use egui::Ui;
use egui_transition_animation::prelude::*;
use hooks::use_state;
use hooks::use_state::use_state;

struct MyApp {
    name: String,
    page: Page,
    age: u32,
}

#[derive(PartialEq, PartialOrd, Clone, Eq)]
enum Page {
    Page1,
    Page2,
    Page3,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            name: "World".to_owned(),
            age: 42,
            page: Page::Page1,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui: &mut Ui| {
            ctx.style_mut(|style| style.animation_time = 0.4);

            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.page, Page::Page1, "Page 1");
                ui.selectable_value(&mut self.page, Page::Page2, "Page 2");
                ui.selectable_value(&mut self.page, Page::Page3, "Page 3");
            });

            let num = use_state!(0_i32);

            if ui.button("+").clicked() {
                num.set(num.get().saturating_add(1));
            }
            if ui.button("-").clicked() {
                num.set(num.get().saturating_add(-1));
            }

            ui.label(num.get().to_string());

            animated_pager(
                ui,
                self.page.clone(),
                &TransitionStyle::fade(ui),
                egui::Id::new("pager"),
                |ui, page| match page {
                    Page::Page1 => ui.label("Hello from page 1"),
                    Page::Page2 => ui.heading("Hello from page 2"),
                    Page::Page3 => ui.monospace("Hello from page 3"),
                },
            )
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 300.0])
            .with_min_inner_size([300.0, 220.0]),
        ..Default::default()
    };
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|_cc| Ok(Box::new(MyApp::default()))),
    )
}
