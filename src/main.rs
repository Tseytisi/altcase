mod cmd_args;
mod case_convert;

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
        println!("GUI is not yet implemented");
    }
}
