#![allow(unused_variables, non_snake_case, unused_imports, unused_must_use)]
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};

extern crate gio;
extern crate gtk;
use gio::prelude::*;
use gtk::{
    prelude::*, Builder, Dialog, FileChooserAction, FileChooserDialog, TextBuffer, TextTagTable,
    TextView, ToolButton, Window,
};
fn main() {
    if gtk::init().is_err() {
        println!("Failed initialize GTK.");
        return;
    }
    let glade_src = include_str!("mainWindow.glade");
    let builder = Builder::from_string(glade_src);

    let window: Window = builder.get_object("mainWindow").unwrap();
    window.show();
    // window.close();
    let text_view: TextView = builder.get_object("text_area").unwrap();
    let ref_to_text_view: TextView = text_view.clone(); //Encontrar solução para não clonar um Widget//
    text_view.show();
    //salvar o texto escrito no TextView em disco no texto_teste.txt
    let button_save: ToolButton = builder.get_object("button_save").unwrap();
    button_save.connect_clicked(move |_| {
        handler_save_file(&text_view);
    });
    //abre um arquivo de texto qualquer e apresenta no TextView
    let button_open: ToolButton = builder.get_object("button_open").unwrap();
    button_open.connect_clicked(move |_elem| {
        let dialog_file_chooser = handler_open_file(&window, &ref_to_text_view);
    });
    //
    fn handler_open_file(parent: &Window, textView: &TextView) {
        let action_open = FileChooserAction::Open;
        let dialog_file_chooser = FileChooserDialog::with_buttons(
            Some("Abrir arquivo"),
            Some(parent),
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
            let file = dialog_file_chooser.get_file().unwrap();
            //
            let file_path = file.get_path().unwrap();
            //
            let mut f = File::open(file_path).unwrap();
            //
            let mut context = String::new();
            let _ = f.read_to_string(&mut context);
            //
            let buffer = TextBuffer::new(Some(&TextTagTable::new()));
            buffer.set_text(&context);
            //
            textView.set_buffer(Some(&buffer));
            //
            dialog_file_chooser.close();
        }
        dialog_file_chooser.close();
    }
    //
    fn handler_save_file(text_view: &TextView) {
        //pega o texto do text_view
        let context = text_view.get_buffer().unwrap();
        let start = context.get_start_iter();
        let end = context.get_end_iter();
        //
        let text = context.get_text(&start, &end, true).unwrap();
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
    gtk::main();
}
