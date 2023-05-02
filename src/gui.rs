use gtk::prelude::*;
use gtk::{ApplicationWindow, Application};
use gtk::gdk_pixbuf::Pixbuf;
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
        .default_width(600)
        .default_height(400)
        .title("AltCase")
        .build();

    if let Ok(app_icon) = Pixbuf::from_file("img/altcase.svg") {
        window.set_icon(Some(&app_icon));
    }

    // Simple mode toggle
    let simple_mode_label = gtk::Label::builder()
        .label("Simple Mode")
        .build();
    let simple_mode_switch = gtk::Switch::builder()
        .build();
    let simple_mode_info_icon = gtk::Image::from_icon_name(Some("help-about"), gtk::IconSize::LargeToolbar);

    // Simple mode container
    let simple_mode_box = gtk::Box::new(gtk::Orientation::Horizontal, 10);
    simple_mode_box.add(&simple_mode_switch);
    simple_mode_box.add(&simple_mode_label);
    simple_mode_box.add(&simple_mode_info_icon);
    simple_mode_box.set_tooltip_text(
        Some("When simple mode is disabled, certain letters (such as 'i' and 'L') \
        will always be kept upper or lower case to avoid confusion."));

    // Set state of simple mode switch
    {
        // Set initial state of the simple-mode switch
        let initial_mode = SIMPLE_MODE.lock()
            .expect("Error (gui.create_window.set_simple_mode_switch_state): Could not lock mutex");
        simple_mode_switch.set_state(*initial_mode);
    }

    // Input and output text areas
    let tag_table = gtk::TextTagTable::new();
    let input_buffer = gtk::TextBuffer::new(Some(&tag_table));
    let input_text_field = gtk::TextView::builder()
        .expand(true)
        .buffer(&input_buffer)
        .border_width(5)
        .wrap_mode(gtk::WrapMode::WordChar)
        .build();
    // Scroll box so the window won't resize if you add more text than fits the box
    let input_scroll_box = gtk::ScrolledWindow::builder()
        //.expand(true)
        .build();
    input_scroll_box.add(&input_text_field);

    let output_buffer = gtk::TextBuffer::new(Some(&tag_table));
    let output_text_field = gtk::TextView::builder()
        .expand(true)
        .editable(false)
        .buffer(&output_buffer)
        .border_width(5)
        .wrap_mode(gtk::WrapMode::WordChar)
        .build();
    let output_scroll_box = gtk::ScrolledWindow::builder()
        //.expand(true)
        .build();
    output_scroll_box.add(&output_text_field);

    // Convert button
    let convert_button_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .valign(gtk::Align::Center)
        .build();
    let convert_button = gtk::Button::builder()
        .build();
    convert_button_box.add(&convert_button);
    if let Ok(image) = Pixbuf::from_file("img/arrow.png") {
        let convert_arrow = gtk::Image::from_pixbuf(Some(&image));
        convert_button.set_image(Some(&convert_arrow));
        convert_button.set_always_show_image(true);
    } else {
        convert_button.set_label("Convert");
    }

    // Copy-to-clipboard button
    let copy_to_clipboard_button = gtk::Button::builder()
        .label("Copy to clipboard")
        .build();
    let copy_icon = gtk::Image::from_icon_name(Some("edit-copy"), gtk::IconSize::Button);
    copy_icon.set_margin_end(5);
    copy_to_clipboard_button.set_image(Some(&copy_icon));
    copy_to_clipboard_button.set_always_show_image(true);

    // Layout
    let grid = gtk::Grid::builder()
        .border_width(10)
        .column_spacing(10)
        .row_spacing(10)
        .build();

    // To ensure the two columns are the same width
    let text_size_group = gtk::SizeGroup::new(gtk::SizeGroupMode::Horizontal);
    text_size_group.add_widget(&input_text_field);
    text_size_group.add_widget(&output_text_field);
    text_size_group.add_widget(&simple_mode_box); // these were messing up the width even
    text_size_group.add_widget(&copy_to_clipboard_button); // when there was enough space

    // Layout left
    grid.attach(&simple_mode_box, 0, 0, 1, 1);
    grid.attach(&input_scroll_box, 0, 1, 1, 1);

    // Centre
    grid.attach(&convert_button_box, 1, 1, 1, 1);

    // Right side
    grid.attach(&output_scroll_box, 2, 1, 1, 1);
    grid.attach(&copy_to_clipboard_button, 2, 2, 1, 1);

    // Button functionality
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

    copy_to_clipboard_button.connect_clicked(|_button| {
        // Retrieve the last output from the mutex
        let last_output = LAST_OUTPUT.lock().expect("Error (ctc_button.click): Could not lock mutex");
        let current_text = String::from(&*last_output);

        // Create an object that represents the default clipboard
        let clipboard = gtk::Clipboard::get(&gtk::gdk::Atom::intern("CLIPBOARD"));
        clipboard.set_text(&current_text);
    });

    // Finalising
    window.add(&grid);
    window.show_all();
}