use eframe::egui;

struct MyApp {
    button1_clicks: u32,
    button2_clicks: u32,
    button3_clicks: u32,
    button4_clicks: u32,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            button1_clicks: 0,
            button2_clicks: 0,
            button3_clicks: 0,
            button4_clicks: 0,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Four Button Example");

            ui.add_space(20.0);

            if ui.button("Button 1").clicked() {
                self.button1_clicks += 1;
                println!("Button 1 was clicked! Total clicks: {}", self.button1_clicks);
            }

            if ui.button("Button 2").clicked() {
                self.button2_clicks += 1;
                println!("Button 2 was pressed! Total presses: {}", self.button2_clicks);
            }

            if ui.button("Button 3").clicked() {
                self.button3_clicks += 1;
                println!("You activated Button 3! Activation count: {}", self.button3_clicks);
            }

            if ui.button("Button 4").clicked() {
                self.button4_clicks += 1;
                println!("Button 4 says hello! Greeting count: {}", self.button4_clicks);
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
