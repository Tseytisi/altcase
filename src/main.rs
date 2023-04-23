mod cmd_args;
mod case_convert;
mod gui;

use gtk::prelude::*;
use gtk::Application;

fn main() {
    let settings = cmd_args::parse_args();

    if !settings.gui {
        // Command-line mode
        if settings.input.is_empty() {
            println!("Error: no input given");
            std::process::exit(1);
        }

        println!("{}", case_convert::convert(&settings.input, settings.simple).as_str());
    } else {
        // GUI mode
        let app = Application::builder()
            .application_id("com.example.altcase")
            .build();

        gui::start(&app, &settings);

        let no_args: [String; 0] = [];
        app.run_with_args(&no_args);
    }
}
