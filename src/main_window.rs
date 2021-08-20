extern crate gtk;
use glib::clone;
use gtk::{
  glib, prelude::*, Builder, FileChooserAction, FileChooserDialog, MenuItem, TextBuffer,
  TextTagTable, TextView, ToolButton, Window,
};
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::PathBuf;
static mut PATH: Option<PathBuf> = None; //compartilha o caminho do arquivo
struct FilePath {
  path: Option<PathBuf>,
}

impl FilePath {
  fn new() -> FilePath {
    FilePath { path: None }
  }
  fn put_path(&mut self, path: PathBuf) {
    self.path = Some(path);
  }
  fn get_path(self) -> Option<PathBuf> {
    self.path
  }
}

fn put_path(path: Option<PathBuf>) {
  unsafe {
    match &path {
      Some(PathBuf) => PATH = path,
      _ => PATH = None,
    }
  };
}

fn get_path() -> Option<PathBuf> {
  unsafe {
    return PATH.clone();
  }
}

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
  match res {
    gtk::ResponseType::Accept => {
      let file = dialog_file_chooser.file().unwrap();
      //
      let file_path = file.path().unwrap();
      //
      let mut f = File::open(&file_path).unwrap();
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
      text_view.show();
      put_path(Some(file_path));
    }
    _ => put_path(None),
  };
  dialog_file_chooser.close();
}
//
fn handler_save_file(text_view: TextView) {
  //pega o texto do text_view
  let context = text_view.buffer().unwrap();
  let start = context.start_iter();
  let end = context.end_iter();
  //
  let text = context.text(&start, &end, true).unwrap();
  let buf = text.as_bytes();
  //
  let mut open = OpenOptions::new();
  let current_path = get_path();
  match current_path {
    Some(p) => {
      let mut file = open.write(true).truncate(true).open(p).unwrap();
      let _re = file.write(buf);
      file.flush();
    }
    None => handler_save_as(text_view),
  }
}
//
fn handler_save_as(text_view: TextView) {
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
  match res {
    gtk::ResponseType::Accept => {
      let file_name = dialog_file_chooser.filename().unwrap();
      //
      let context = text_view.buffer().unwrap();
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
        .open(&file_name)
        .unwrap();
      let _re = file.write(buf);
      file.flush();
      dialog_file_chooser.close();
      put_path(Some(file_name));
    }
    _ => put_path(None),
  }
  dialog_file_chooser.close();
}
//
fn handler_new_document(text_view: TextView) {
  let buffer = TextBuffer::new(Some(&TextTagTable::new()));
  buffer.set_text("");
  text_view.set_buffer(Some(&buffer));
  text_view.show();
}
//
fn handler_close_document(text_view: TextView) {
  text_view.hide();
}
//
fn handler_about() {
  todo!()
}
//
pub fn close() {
  gtk::main_quit();
}
//
pub fn builder_window(name: &str) -> Window {
  let glade_src = include_str!("mainWindow.glade");
  let builder = Builder::from_string(glade_src);
  //
  let window: Window = builder.object(name).unwrap();
  let text_view: TextView = builder_text_view(&builder, "text_area");
  //
  let button_new: ToolButton = builder_tool_button(&builder, "button_new");
  button_new.connect_clicked(clone!(@weak text_view => move |_| {
      handler_new_document(text_view);
  }));
  //
  let button_save: ToolButton = builder_tool_button(&builder, "button_save");
  button_save.connect_clicked(clone!(@weak text_view => move |_| {
      handler_save_file(text_view);
  }));
  //
  let button_save_as: ToolButton = builder_tool_button(&builder, "button_save_as");
  button_save_as.connect_clicked(clone!(@weak text_view => move |_| {
        handler_save_as(text_view);
        // unsafe { PATH = None };
        put_path(None);
  }));
  //
  let button_open: ToolButton = builder_tool_button(&builder, "button_open");
  button_open.connect_clicked(clone!(@weak text_view => move |_elem| {
      handler_open_file(text_view);
  }));
  //button_close
  let button_close: ToolButton = builder_tool_button(&builder, "button_close");
  button_close.connect_clicked(clone!(@weak text_view => move |_elem| {
      handler_close_document(text_view);
      // unsafe { PATH = None };
      put_path(None);
  }));
  //
  let menu_quit: MenuItem = builder_menu_item(&builder, "menu_quit").unwrap();
  menu_quit.connect_activate(|_| {
    close();
  });
  //
  let menu_open: MenuItem = builder_menu_item(&builder, "menu_open").unwrap();
  menu_open.connect_activate(clone!(@weak text_view => move |_ele| {
     handler_open_file(text_view);
  }));
  //
  let menu_save: MenuItem = builder_menu_item(&builder, "menu_save").unwrap();
  menu_open.connect_activate(clone!(@weak text_view => move |_elem| {
    handler_save_file(text_view);
  }));
  window
}
//
fn builder_text_view(builder: &Builder, name: &str) -> TextView {
  let text_view = builder.object(name).unwrap();
  text_view
}
//
fn builder_tool_button(builder: &Builder, name: &str) -> ToolButton {
  let tool_button = builder.object(name).unwrap();
  tool_button
}
//
fn builder_menu_item(builder: &Builder, name: &str) -> Option<MenuItem> {
  let menu_item = builder.object(name);
  menu_item
}
