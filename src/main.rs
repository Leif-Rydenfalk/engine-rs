mod common;

use common::*;
use imgui::*;
use simple_logger::SimpleLogger;
use std::error::Error;

const APP_NAME: &str = "creating windows";

fn main() -> Result<(), Box<dyn Error>> {
    SimpleLogger::new().init()?;

    let mut demo_window_open = true;
    System::new(APP_NAME)?.run((), move |_, ui, _| {
        if demo_window_open {
            ui.show_demo_window(&mut demo_window_open);
        }

        ui.window("Settings").always_auto_resize(true).build(|| {
            ui.checkbox("Show Imgui Demo Window", &mut demo_window_open);
        });

        ui.window("Pipeline creator")
            .size([300.0, 400.0], Condition::FirstUseEver)
            .build(|| {
                ui.text("Hello World!");
                ui.text_wrapped("こんにちは世界！");
            });
    })?;

    Ok(())
}
