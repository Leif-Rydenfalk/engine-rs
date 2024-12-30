mod common;

use common::*;
use simple_logger::SimpleLogger;
use std::error::Error;

const APP_NAME: &str = "test window";

fn main() -> Result<(), Box<dyn Error>> {
    SimpleLogger::new().init()?;

    let system = System::new(APP_NAME)?;
    // system.imgui.io_mut().config_flags |= imgui::ConfigFlags::DOCKING_ENABLE;
    system.run((), |run, ui, _| ui.show_demo_window(run))?;

    Ok(())
}
