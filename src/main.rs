use eframe::egui;

struct MyApp {
    buttons: [String; 4],
    clicks: [u32; 4],
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            buttons: [
                String::from("Button 1"),
                String::from("Button 2"),
                String::from("Button 3"),
                String::from("Button 4"),
            ],
            clicks: [0, 0, 0, 0],
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Four Button Example");

            ui.add_space(20.0);

            for index in [0, 1, 2, 3] {
                if ui.button(self.buttons[index].clone()).clicked() {
                    self.clicks[index] += 1;
                    println!("{} was clicked! Total clicks: {}", self.buttons[index], self.clicks[index]);
                }
            }
        });
    }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_always_on_top()
            .with_inner_size([320.0, 320.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Four Button App",
        options,
        Box::new(|_context| {
            //egui_extras::install_image_loaders(&context.egui_ctx);
            Ok(Box::new(
                MyApp::default()
            ))
        }),
    )
}
