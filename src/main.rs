// use std::io::{stdin, stdout, Write};
// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use regex::Regex;
use std::{
    ffi::OsString,
    path::{Path, PathBuf},
    vec,
};

use dircpy::*;

use eframe::egui;
use walkdir::{DirEntry, WalkDir};

#[derive(Clone)]
struct SubFamily {
    path: PathBuf,
    documents: Vec<Document>,
}

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
    let subfamilies_to_file: Vec<Document> = Vec::new();

    let revision_regex = Regex::new(r"Rev\d{1,}");
    let number_regex = Regex::new(r"\d{1,}");

    let family = scan_families();
    println!("scanning for product families");
    for path in family {
        println!("{}", path.file_name().unwrap().to_str().unwrap());
        println!("subfamilies found:");
        let subfamilies = scan_subfamilies(path);
        for sf in subfamilies {
            let sf_struct = SubFamily {
                path: sf,
                documents: Vec::new(),
            };
            println!(
                "subfamily: {}",
                sf_struct.path.file_name().unwrap().to_str().unwrap()
            );
            let files = scan_files(sf_struct.path);
            for file in files {
                let document = Document {
                    index: 0,
                    path: file.clone(),
                    revision: 0,
                    name: file
                        .file_name()
                        .unwrap()
                        .to_os_string()
                        .into_string()
                        .unwrap(),
                };
                println!("{}", file.display());
            }
        }
        println!("next folder");
        println!("");
    }
}

fn is_directory(entry: &DirEntry) -> bool {
    entry.file_type().is_dir()
}

fn is_in_progress_folder(entry: &DirEntry) -> bool {
    entry.file_name().to_str().unwrap().contains("_InProgress")
}

fn scan_families() -> Vec<PathBuf> {
    // this function scans through all projects in the development folder to allow the user to
    // select the project they are updating the dhf for
    // let development_path = "H:\\Development";
    let development_path = "./test";
    let families = WalkDir::new(development_path)
        .min_depth(1)
        .max_depth(1)
        .into_iter()
        .filter_entry(|e| is_directory(e));

    let mut paths_to_return = Vec::new();
    for family in families {
        let path = family.unwrap().into_path();
        paths_to_return.push(path);
    }
    paths_to_return
}

fn scan_subfamilies(path: PathBuf) -> Vec<PathBuf> {
    // takes the path to a product family folder and scans the _InProgress directory for subfamily
    // folders

    // really ugly way of doing all thi but this makes it platform agnostic
    let in_progress_folder = WalkDir::new(path)
        .min_depth(1)
        .into_iter()
        .filter_entry(|entry| is_directory(entry) && is_in_progress_folder(entry));

    let in_progress_folder = in_progress_folder.last().unwrap().unwrap().into_path();

    let subfamilies = WalkDir::new(in_progress_folder)
        .min_depth(1)
        .into_iter()
        .filter_entry(|e| is_directory(e));

    let mut paths_to_return = Vec::new();
    for subfamily in subfamilies {
        let path = subfamily.unwrap().into_path();
        paths_to_return.push(path);
    }
    paths_to_return
}

fn scan_files(path: PathBuf) -> Vec<PathBuf> {
    // walks all the folders in the given directory. use this function to list all files in the
    // _InProgress directory of the selected project
    let files = WalkDir::new(path).min_depth(1).into_iter();
    // .filter_entry(|e| !is_directory(e));

    let mut paths_to_return = Vec::new();

    for entry in files {
        let result = entry.unwrap();
        if result.file_type().is_dir() {
            let path = result.into_path();
            paths_to_return.push(path);
        }
    }
    paths_to_return
}

fn collapse_to_complete_docs() {
    // scans the provided list of docs and returns a list of those docs where a word doc and a pdf
    // with the same name are present
}

fn backup(src: String, dest: String) {
    copy_dir(src, dest);
}

// fn make_doc_struct_from_path(path: PathBuf) -> Document {}
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
