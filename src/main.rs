#![allow(unused_variables, non_snake_case, unused_imports, unused_must_use)]
extern crate gio;
extern crate gtk;
mod main_window;
use gdk::prelude::*;
use gio::prelude::*;
use gtk::prelude::*;
use main_window::*;

fn main() {
    if gtk::init().is_err() {
        println!("Failed initialize GTK.");
        return;
    }
    let window = builder_window("mainWindow");
    window.show();
    window.connect_destroy(|_| {
        close();
    });
    gtk::main();
}
