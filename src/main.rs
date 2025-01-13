mod database;

use database::{load_tasks, save_task, delete_task, Task};
use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow, Button, Box, Entry, ListBox, ListBoxRow, Label};

const APP_ID: &str = "org.gtk_rs.MeowList";

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run()
}

fn add_task_to_list(list_box: &ListBox, description: String, task_id: i32) {
    let hbox = Box::new(gtk::Orientation::Horizontal, 10);
    let note_label = Label::new(Some(&description));
    let remove_button = Button::with_label("X");
    let hbox_row = ListBoxRow::new();
    hbox_row.set_child(Some(&hbox));

    remove_button.connect_clicked(glib::clone! {
        #[weak] hbox_row,
        #[weak] list_box,
        move |_| {
            list_box.remove(&hbox_row);
            delete_task(task_id);
        }
    });

    hbox.append(&note_label);
    hbox.append(&remove_button);
    list_box.append(&hbox_row);
}

fn build_ui(app: &Application) {
    let vbox = Box::new(gtk::Orientation::Vertical, 10);
    let note_box = Entry::new();
    note_box.set_placeholder_text(Some("Enter task here:"));
    let list_box = ListBox::new();
    list_box.set_vexpand(true);
    let receiver = Button::with_label("Save Task");

    let list_box_clone = list_box.clone();
    let note_box_clone = note_box.clone();
    receiver.connect_clicked(move |_| {
        let text = note_box_clone.text().to_string();
        if !text.is_empty() {
            let task = Task {
                id: 0,
                description: text.clone(),
                completed: false,
            };
            let task_id = save_task(&task);
            add_task_to_list(&list_box_clone, text, task_id);
            note_box_clone.set_text("");
        }
    });

    let tasks = load_tasks();
    for task in tasks {
        add_task_to_list(&list_box, task.description, task.id);
    }

    vbox.append(&note_box);
    vbox.append(&receiver);
    vbox.append(&list_box);

    let window = ApplicationWindow::builder()
        .application(app)
        .title("MeowList GTK Beta")
        .child(&vbox)
        .build();

    window.set_default_size(1000, 1000);
    window.present();
}