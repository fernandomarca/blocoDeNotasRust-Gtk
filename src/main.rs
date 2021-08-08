#![allow(unused_variables, non_snake_case, unused_imports, unused_must_use)]
extern crate gio;
extern crate gtk;

use gdk::prelude::*;
use gio::prelude::*;
use glib::clone;
use gtk::{
    glib, prelude::*, Builder, FileChooserAction, FileChooserDialog, MenuItem, TextBuffer,
    TextTagTable, TextView, ToolButton, Window,
};
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};

fn builder_window(glade_src: &str, name: &str) -> Window {
    let builder = Builder::from_string(glade_src);
    let window: Window = builder.object(name).unwrap();
    window
}

fn builder_text_view(glade_src: &str, name: &str) -> TextView {
    let builder = Builder::from_string(glade_src);
    let text_view = builder.object(name).unwrap();
    text_view
}

fn builder_tool_button(glade_src: &str, name: &str) -> ToolButton {
    let builder = Builder::from_string(glade_src);
    let tool_button = builder.object(name).unwrap();
    tool_button
}

fn builder_menu_item(glade_src: &str, name: &str) -> Option<MenuItem> {
    let builder = Builder::from_string(glade_src);
    let menu_item = builder.object(name);
    menu_item
}

fn main() {
    if gtk::init().is_err() {
        println!("Failed initialize GTK.");
        return;
    }
    //Css provider custom
    let screen = gdk::Screen::default().unwrap();
    let provider = gtk::CssProvider::new();
    gtk::CssProvider::load_from_path(&provider, "./styles.css");
    gtk::StyleContext::add_provider_for_screen(
        &screen,
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
    //
    let file: Option<File> = None;
    //
    let glade_src = include_str!("mainWindow.glade");
    //
    let window = builder_window(glade_src, "mainWindow");
    window.show();
    window.connect_destroy(|_| {
        close();
    });
    //
    let text_view: TextView = builder_text_view(glade_src, "text_area");
    // text_view.show();
    let button_new: ToolButton = builder_tool_button(glade_src, "button_new");
    button_new.connect_clicked(clone!(@weak text_view => move |_| {
        handler_new_document(text_view);
    }));
    //salvar o texto escrito no TextView em disco no texto_teste.txt
    let button_save: ToolButton = builder_tool_button(glade_src, "button_save");
    button_save.connect_clicked(clone!(@weak text_view => move |_| {
        handler_save_file(text_view);
    }));
    //
    let button_save_as: ToolButton = builder_tool_button(glade_src, "button_save_as");
    button_save_as.connect_clicked(clone!(@weak text_view => move |_| handler_save_as(text_view)));
    //abre um arquivo de texto qualquer e apresenta no TextView
    let button_open: ToolButton = builder_tool_button(glade_src, "button_open");
    button_open.connect_clicked(clone!(@weak text_view => move |_elem| {
        handler_open_file(text_view);
    }));
    //
    let menu_quit: MenuItem = builder_menu_item(glade_src, "menu_quit").unwrap();
    menu_quit.connect_activate(|_| {
        close();
    });
    //
    let menu_open: MenuItem = builder_menu_item(glade_src, "menu_open").unwrap();
    menu_open.connect_activate(clone!(@weak text_view => move |_ele| {
        handler_open_file(text_view);
    }));
    //
    let menu_save: MenuItem = builder_menu_item(glade_src, "menu_save").unwrap();
    menu_open.connect_activate(clone!(@weak text_view => move |_elem| {
        handler_save_file(text_view);
    }));
    //
    fn handler_open_file(text_view: TextView) {
        let action_open = FileChooserAction::Open;
        let window = Window::new(gtk::WindowType::Popup);
        let dialog_file_chooser = FileChooserDialog::with_buttons(
            Some("Abrir arquivo"),
            Some(&window),
            action_open,
            &[
                (&"Cancelar", gtk::ResponseType::Cancel),
                (&"Abrir", gtk::ResponseType::Accept),
            ],
        );
        dialog_file_chooser.show();
        let res = dialog_file_chooser.run();
        if res == gtk::ResponseType::Accept {
            //
            let file = dialog_file_chooser.file().unwrap();
            //
            let file_path = file.path().unwrap();
            //
            let mut f = File::open(file_path).unwrap();
            //
            let mut context = String::new();
            let _ = f.read_to_string(&mut context);
            //
            let buffer = TextBuffer::new(Some(&TextTagTable::new()));
            buffer.set_text(&context);
            //
            text_view.set_buffer(Some(&buffer));
            //
            dialog_file_chooser.close();
        }
        dialog_file_chooser.close();
    }
    //
    fn handler_save_file(text_view_clone: TextView) {
        //pega o texto do text_view
        let context = text_view_clone.buffer().unwrap();
        let start = context.start_iter();
        let end = context.end_iter();
        //
        let text = context.text(&start, &end, true).unwrap();
        let buf = text.as_bytes();
        //
        let mut open = OpenOptions::new();
        let mut file = open
            .write(true)
            .create(true)
            .append(true)
            .open("texto_teste.txt")
            .unwrap();
        let _re = file.write(buf);
        file.flush();
    }
    //
    fn handler_save_as(text_view_clone: TextView) {
        let action_open = FileChooserAction::Save;
        let window = Window::new(gtk::WindowType::Popup);
        let dialog_file_chooser = FileChooserDialog::with_buttons(
            Some("Salvar"),
            Some(&window),
            action_open,
            &[
                (&"Cancelar", gtk::ResponseType::Cancel),
                (&"Salvar", gtk::ResponseType::Accept),
            ],
        );
        dialog_file_chooser.show();
        let res = dialog_file_chooser.run();
        if res == gtk::ResponseType::Accept {
            //
            let file_name = dialog_file_chooser.filename().unwrap();
            //
            //pega o texto do text_view
            let context = text_view_clone.buffer().unwrap();
            let start = context.start_iter();
            let end = context.end_iter();
            //
            let text = context.text(&start, &end, true).unwrap();
            let buf = text.as_bytes();
            //
            let mut open = OpenOptions::new();
            let mut file = open
                .write(true)
                .create(true)
                .truncate(true)
                .open(file_name)
                .unwrap();
            let _re = file.write(buf);
            file.flush();
            dialog_file_chooser.close();
        }
        dialog_file_chooser.close();
    }
    //
    fn handler_new_document(text_view: TextView) {
        let buffer = TextBuffer::new(Some(&TextTagTable::new()));
        buffer.set_text("");
        text_view.set_buffer(Some(&buffer));
    }
    //
    fn handler_close_document() {
        // handler_new_document();
        todo!()
    }
    //
    fn handler_about() {
        todo!()
    }
    //
    fn close() {
        gtk::main_quit();
    }
    //

    gtk::main();
}
