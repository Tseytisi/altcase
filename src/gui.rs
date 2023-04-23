use gtk::prelude::*;
use gtk::{ApplicationWindow, Application};
use std::sync::Mutex;

use crate::case_convert::convert;
use crate::cmd_args::Settings;

// Keeps track of the last text that was converted, which will also be the current text of the
// output box. Used for the copy-to-clipboard button
static LAST_OUTPUT: Mutex<String> = Mutex::new(String::new());
// Keeps track of the simple mode setting
static SIMPLE_MODE: Mutex<bool> = Mutex::new(false);

pub(crate) fn start(app: &Application, settings: &Settings) {
    // Set initial value of simple mode
    let mut simple_mode_setting = SIMPLE_MODE.lock()
        .expect("Error (gui.start): Could not lock simple-mode mutex");
    *simple_mode_setting = settings.simple;

    // Create window
    app.connect_activate(create_window);
}

fn create_window(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .default_width(400)
        .default_height(600)
        .title("AltCase")
        .build();

    let layout = gtk::Box::new(gtk::Orientation::Vertical, 0);
    layout.set_homogeneous(true);

    let upper_half = gtk::Box::new(gtk::Orientation::Vertical, 0);
    let bottom_half = gtk::Box::new(gtk::Orientation::Vertical, 0);

    let top_box = gtk::Box::new(gtk::Orientation::Horizontal, 0);
    let simple_mode_label = gtk::Label::builder()
        .label("Simple Mode")
        .build();
    let simple_mode_switch = gtk::Switch::builder()
        .margin(10)
        .build();
    {
        // Set initial state of the simple-mode switch
        let initial_mode = SIMPLE_MODE.lock()
            .expect("Error (gui.create_window.set_simple_mode_switch_state): Could not lock mutex");
        simple_mode_switch.set_state(*initial_mode);
    }
    top_box.add(&simple_mode_switch);
    top_box.add(&simple_mode_label);
    top_box.set_tooltip_text(
        Some("When simple mode is disabled, certain letters (such as 'i' and 'L') \
        will be kept upper or lower case to avoid confusion."));

    let tag_table = gtk::TextTagTable::new();
    let input_buffer = gtk::TextBuffer::new(Some(&tag_table));
    let input_text_field = gtk::TextView::builder()
        .expand(true)
        .margin(10)
        .buffer(&input_buffer)
        .border_width(5)
        .build();

    let output_buffer = gtk::TextBuffer::new(Some(&tag_table));
    let output_text_field = gtk::TextView::builder()
        .expand(true)
        .editable(false)
        .margin(10)
        .buffer(&output_buffer)
        .border_width(5)
        .build();
    let convert_button = gtk::Button::builder()
        .label("Convert")
        .margin(10)
        .build();

    convert_button.connect_clicked(move |_button| {
        // Get the current contents of the input text area
        let input_text = input_buffer.text(
            &input_buffer.start_iter(),
            &input_buffer.end_iter(),
            false);

        if let Some(text) = input_text {
            // Retrieve simple-mode state
            let simple_mode = simple_mode_switch.state();

            // Convert to alternating case
            let output_text = convert(text.as_str(), simple_mode);

            // Store the result for use with the copy-to-clipboard button
            let mut last_output = LAST_OUTPUT.lock()
                .expect("Error (convert_button.click): Could not lock output mutex");
            *last_output = output_text;

            // Set that same text in the output text area
            output_buffer.set_text(&last_output);
        } else {
            println!("Error (convert_button.click): Input text was None");
        }
    });

    let copy_to_clipboard_button = gtk::Button::builder()
        .margin(10)
        .label("Copy to clipboard")
        .build();

    copy_to_clipboard_button.connect_clicked(|_button| {
        // Retrieve the last output from the mutex
        let last_output = LAST_OUTPUT.lock().expect("Error (ctc_button.click): Could not lock mutex");
        let current_text = String::from(&*last_output);

        // Create an object that represents the default clipboard in X
        let clipboard = gtk::Clipboard::get(&gtk::gdk::Atom::intern("CLIPBOARD"));
        clipboard.set_text(&current_text);
    });

    upper_half.add(&top_box);
    upper_half.add(&input_text_field);
    upper_half.add(&convert_button);
    bottom_half.add(&output_text_field);
    bottom_half.add(&copy_to_clipboard_button);

    layout.add(&upper_half);
    layout.add(&bottom_half);
    window.add(&layout);
    window.show_all();
}