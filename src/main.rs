use eframe::{
    egui::{CentralPanel, Direction, Layout, SidePanel},
    run_native, App,
};
use egui_extras::{Column, TableBuilder};

struct Test {
    test_data: Vec<[String; 3]>,
}

impl Default for Test {
    fn default() -> Self {
        let mut test_data = Vec::with_capacity(1000);

        for i in 0..1000 {
            test_data.push([
                "First one".to_owned(),
                "Second one".to_owned(),
                format!(
                    "{i}: The last one that should be looooooooooooooooooooooooooooooooooooong"
                ),
            ]);
        }

        Self { test_data }
    }
}

impl App for Test {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        SidePanel::right("side_panel")
            .default_width(225.0)
            .max_width(225.0)
            .resizable(false)
            .show(ctx, |ui| {
                ui.label("just a simple side panel");
            });

        CentralPanel::default().show(ctx, |ui| {
            TableBuilder::new(ui)
                .striped(true)
                .column(Column::initial(50.0).at_least(50.0).at_most(50.0))
                .column(Column::initial(150.0).at_least(50.0))
                .column(Column::remainder().at_least(60.0))
                .cell_layout(Layout::centered_and_justified(Direction::LeftToRight))
                .resizable(true)
                .header(20.0, |mut header| {
                    for text in ["One", "Two", "Long Text"] {
                        header.col(|ui| {
                            ui.heading(text);
                        });
                    }
                })
                .body(|body| {
                    body.rows(100.0, 1000, |row_idx, mut row| {
                        row.col(|ui| {
                            ui.label(&self.test_data[row_idx][0]);
                        });
                        row.col(|ui| {

                            ui.centered_and_justified(|ui| {
                                ui.label("The last one that should be looooooooooooooooooooooooooooooooooooong");
                            });
                        });
                        row.col(|ui| {
                            ui.label(&self.test_data[row_idx][1]);
                        });
                    })
                })
        });
    }
}

fn main() {
    let native_options = eframe::NativeOptions {
        drag_and_drop_support: true,
        initial_window_size: Some([1280.0, 1024.0].into()),
        #[cfg(feature = "wgpu")]
        renderer: eframe::Renderer::Wgpu,
        ..Default::default()
    };

    run_native(
        "Test",
        native_options,
        Box::new(|_| Box::new(Test::default())),
    );
}
