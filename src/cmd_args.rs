use std::env;

pub(crate) struct Settings {
    pub(crate) gui: bool,
    pub(crate) input: String,
    pub(crate) simple: bool,
}

static HELP_TEXT: &str = "
Convert text to alternating case.

This is frequently used to \"MoCk-TyPe\" online. Runs as both command-line tool, and with GUI.

USAGE:
    altcase [options] [input text]

OPTIONS:
    --help          Prints this help page and exits
    --version       Prints the program version and exits
    --gui           Start the program in GUI mode
    --simple        Set the conversion mode to 'simple', which does not pay extra attention to 'i' and
                    'L', possibly making the output more difficult to read.

EXAMPLE USES:
    Converting directly from the command line
        $ altcase convert this text
        >> CoNvErT tHiS tExT
    Converting the word 'million' in 'normal' mode
        $ altcase million
        >> MiLLiOn
    Converting the word 'million' in 'simple' mode
        $ altcase --simple million
        >> MiLlIoN
";

fn print_help_and_exit() {
    println!("{}", HELP_TEXT);
    std::process::exit(0);
}

fn print_version_and_exit() {
    println!("AltCase V{}", env!("CARGO_PKG_VERSION"));
    println!("JTC, April 2023");
    std::process::exit(0);
}

pub(crate) fn parse_args() -> Settings {
    let mut gui = false;
    let mut simple = false;

    let args = env::args();
    let mut parse_as_input = false;
    let mut input = String::new();
    let mut is_first = true;
    for arg in args {
        // Skip parsing the name of the program
        if is_first {
            is_first = false;
            continue;
        }
        if !parse_as_input {
            match arg.as_str() {
                "--help" => print_help_and_exit(),
                "--version" => print_version_and_exit(),
                "--gui" => gui = true,
                "--simple" => simple = true,
                _ => {
                    if let Some(x) = arg.chars().nth(0) {
                        if x == '-' {
                            println!("Unrecognised option '{arg}'");
                            std::process::exit(1);
                        } else {
                            input.push_str(&arg);
                            parse_as_input = true;
                        }
                    }
                },
            }
        } else {
            input.push(' ');
            input.push_str(&arg);
        }
    }

    Settings {
        gui,
        input,
        simple,
    }
}