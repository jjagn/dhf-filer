// use std::io::{stdin, stdout, Write};
// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::{
    path::{Path, PathBuf},
    vec,
};

use eframe::egui;
use walkdir::{DirEntry, WalkDir};

#[derive(Clone)]
struct Document {
    index: i32,
    path: PathBuf,
    revision: i32,
    name: String, // might need to be changed
}

// fn main() -> Result<(), eframe::Error> {
//     env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
//     let options = eframe::NativeOptions {
//         initial_window_size: Some(egui::vec2(320.0, 240.0)),
//         ..Default::default()
//     };
//     eframe::run_native(
//         "dhf filer",
//         options,
//         Box::new(|_cc| Box::new(MyApp::default())),
//     )
// }
//
// struct MyApp {
//     name: String,
//     age: u32,
// }
//
// impl Default for MyApp {
//     fn default() -> Self {
//         Self {
//             name: "Arthur".to_owned(),
//             age: 42,
//         }
//     }
// }
//
// impl eframe::App for MyApp {
//     fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
//         egui::CentralPanel::default().show(ctx, |ui| {
//             ui.heading("dhf filer");
//             ui.horizontal(|ui| {
//                 let name_label = ui.label("Your name: ");
//                 ui.text_edit_singleline(&mut self.name)
//                     .labelled_by(name_label.id);
//             });
//             ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));
//             if ui.button("Click each year").clicked() {
//                 self.age += 1;
//             }
//             ui.label(format!("Hello '{}', age {}", self.name, self.age));
//         });
//     }
// }
//
//
fn main() {
    unimplemented!();
}

fn is_directory(entry: &DirEntry) -> bool {
    entry.file_type().is_dir()
}

fn scan_dirs() -> Vec<PathBuf> {
    let development_path = "H:\\Development";
    let projects = WalkDir::new(development_path)
        .min_depth(1)
        .max_depth(1)
        .into_iter()
        .filter_entry(|e| is_directory(e))
        .collect();

    let paths = Vec::new();

    return projects;
}

fn make_doc_struct_from_path(path: PathBuf) -> Document {}
// fn main() {
//     let mut strings = vec!["string1", "string2", "string3"];
//     let mut add_strings = vec![];
//     let mut update_strings = vec![];
//
//     for string in strings.iter() {
//         println!("Do you want to add or update '{}'?", string);
//         loop {
//             print!("Enter 'a' to add, 'u' to update, or 's' to skip: ");
//             stdout().flush().unwrap();
//             let mut input = String::new();
//             stdin().read_line(&mut input).unwrap();
//             match input.trim() {
//                 "a" => {
//                     add_strings.push(string.to_string());
//                     break;
//                 }
//                 "u" => {
//                     update_strings.push(string.to_string());
//                     break;
//                 }
//                 "s" => break,
//                 _ => println!("Invalid option"),
//             }
//         }
//     }
//
//     println!("Add strings: {:?}", add_strings);
//     println!("Update strings: {:?}", update_strings);
// }
