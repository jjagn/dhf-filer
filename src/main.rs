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
use egui::{FontId, RichText, ScrollArea};
use walkdir::{DirEntry, WalkDir};

enum GUIState {
    SearchingFolder,
    FamilySelect,
    SubfamilySelect,
    DocumentSelect,
}

#[derive(Clone)]
struct SubFamily {
    path: PathBuf,
    documents: Vec<Document>,
    name: String,
    to_file: bool,
}

#[derive(Clone)]
struct Document {
    index: i32,
    path: PathBuf,
    revision: i32,
    name: String, // might need to be change
    to_add: bool,
    to_update: bool,
}

#[derive(Clone)]
struct Family {
    path: PathBuf,
    subfamilies: Vec<SubFamily>,
    name: String,
}

impl Default for Family {
    fn default() -> Self {
        Self {
            path: PathBuf::from("H:\\Development"),
            subfamilies: Vec::new(),
            name: "Default Product Family".to_string(),
        }
    }
}

struct MyApp {
    product_families: Vec<Family>,
    subfamilies_to_file: Vec<SubFamily>,
    state: GUIState,
    family_to_file: Family,
    documents_to_add: Vec<Document>,
    documents_to_update: Vec<Document>,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            product_families: Vec::new(),
            subfamilies_to_file: Vec::new(),
            state: GUIState::FamilySelect,
            family_to_file: Family::default(),
            documents_to_add: Vec::new(),
            documents_to_update: Vec::new(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            match self.state {
                GUIState::SearchingFolder => {
                    ui.heading("searching folder, wait a second");
                }
                GUIState::FamilySelect => {
                    ui.heading("select product family to file:");
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        for family in &self.product_families {
                            // println!("{}", &family.name);
                            if ui.button(&family.name).clicked() {
                                self.family_to_file = family.clone();
                                self.state = GUIState::SubfamilySelect;
                            }
                        }
                    });
                }
                GUIState::SubfamilySelect => {
                    ui.heading("select subfamilies to file:");
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        for subfamily in &mut self.family_to_file.subfamilies {
                            ui.horizontal(|ui| {
                                ui.label(&subfamily.name);
                                ui.with_layout(
                                    egui::Layout::right_to_left(egui::Align::Max),
                                    |ui| {
                                        ui.checkbox(&mut subfamily.to_file, "to file");
                                    },
                                );
                            });
                        }
                        if ui
                            .button("i have selected all the subfamilies i'd like to file")
                            .clicked()
                        {
                            for subfamily in &self.family_to_file.subfamilies {
                                if subfamily.to_file {
                                    self.subfamilies_to_file.push(subfamily.clone());
                                }
                            }
                            self.state = GUIState::DocumentSelect;
                        }
                    });
                }

                GUIState::DocumentSelect => {
                    // egui::Window::new("select documents to file")
                    // .vscroll(true)
                    // .show(ctx, |ui| {
                    ui.heading("select documents to file:");
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        for subfamily in &mut self.subfamilies_to_file {
                            ui.label(
                                RichText::new(&subfamily.name).font(FontId::proportional(20.0)),
                            );
                            // ui.label(&subfamily.name);
                            for document in &mut subfamily.documents {
                                ui.horizontal(|ui| {
                                    ui.label(&document.name);
                                    ui.with_layout(
                                        egui::Layout::right_to_left(egui::Align::Max),
                                        |ui| {
                                            ui.checkbox(&mut document.to_add, "add");
                                            ui.checkbox(&mut document.to_update, "update");
                                        },
                                    );
                                });
                            }
                            ui.separator();
                        }
                    });
                }
            }
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(720.0, 480.0)),
        ..Default::default()
    };

    let mut app = MyApp::default();

    let revision_regex = Regex::new(r"Rev\d{1,}");
    let number_regex = Regex::new(r"\d{1,}");

    let families = scan_families();
    println!("scanning for product families");
    for family in families {
        let mut family_struct = Family {
            path: family.clone(),
            subfamilies: Vec::new(),
            name: family
                .file_name()
                .unwrap()
                .to_os_string()
                .into_string()
                .unwrap(),
        };

        println!("{}", family.file_name().unwrap().to_str().unwrap());
        // search for subfamilies
        let result = scan_subfamilies(family.clone());
        let mut subfamily_paths: Vec<PathBuf> = Vec::new();

        match result {
            Some(result) => {
                println!("in progress subfamilies found:");
                subfamily_paths = result
            }
            None => {
                println!("no in progress subfamilies found");
            }
        }

        for sf in subfamily_paths {
            let mut sf_struct = SubFamily {
                path: sf.clone(),
                documents: Vec::new(),
                name: sf
                    .file_name()
                    .unwrap()
                    .to_os_string()
                    .into_string()
                    .unwrap(),
                to_file: false,
            };

            println!("subfolder: {}", sf_struct.name);
            let files = scan_files(sf_struct.path.clone());
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
                    to_update: false,
                    to_add: false,
                };
                // println!("document: {}", document.name);
                sf_struct.documents.push(document);
            }
            family_struct.subfamilies.push(sf_struct);
        }
        println!("");
        app.product_families.push(family_struct);
    }
    eframe::run_native("dhf filer", options, Box::new(|_cc| Box::new(app)))
}

fn is_directory(entry: &DirEntry) -> bool {
    entry.file_type().is_dir()
}

fn is_in_progress_folder(entry: &DirEntry) -> bool {
    entry.file_name().to_str().unwrap().contains("_InProgress")
}

fn extended_family_search_filter(entry: &DirEntry, found_families: &mut Vec<PathBuf>) -> bool {
    let name = entry.file_name().to_str().unwrap();
    let filtered = !name.contains("Design")
    // !name.contains("DHF & Tech File Word Docs")
    && !name.contains("Project Info")
    // && !name.contains("Tech File")
    && !name.contains("zz")
    && !name.contains("A - ")
    && !name.contains("Active Projects")
    && !entry.file_type().is_file()
    && !name.contains("Bioengineer")
    && !name.contains("!")
    && !name.contains("Obsolete")
    && !name.contains("DHF_")
    && !(found_families.contains(&entry.path().to_path_buf()));

    if !filtered {
        found_families.push(
            entry
                .path()
                .parent()
                .unwrap()
                .strip_prefix("H:\\Development")
                .unwrap()
                .to_path_buf(),
        )
    }
    filtered
}

// fn extended_family_search_filter_2(entry: &DirEntry) -> bool {
//     let descendants = WalkDir::new(entry.path())
//         .min_depth(1)
//         .max_depth(2)
//         .into_iter()
//         .filter_entry(|e| extended_family_search_filter(e));
//     for descendant in descendants {
//         if descendant
//             .unwrap()
//             .file_name()
//             .to_str()
//             .unwrap()
//             .contains("DHF & Tech File Word Docs")
//         {
//             return true;
//         }
//     }
//     false
// }

fn is_valid_file(entry: &DirEntry) -> bool {
    let name = entry.file_name().to_str().unwrap();
    (name.contains(".docx") || name.contains(".pdf") || name.contains(".xlsx"))
        && !name.contains("~$")
}

fn subfamily_garbage_filter(entry: &DirEntry) -> bool {
    let name = entry.file_name().to_str().unwrap();
    !name.contains("_Archive")
}

fn scan_families() -> Vec<PathBuf> {
    // this function scans through all projects in the development folder to allow the user to
    // select the project they are updating the dhf for
    let development_path = "H:\\Development";
    let mut families_found: Vec<PathBuf> = Vec::new();
    // let development_path = "./test";
    let pass_1 = WalkDir::new(development_path)
        .sort_by_file_name()
        .min_depth(1)
        .max_depth(3)
        .into_iter()
        .filter_entry(|e| extended_family_search_filter(e, &mut families_found))
        .filter(|e| {
            let path = e
                .as_ref()
                .unwrap()
                .file_name()
                .to_os_string()
                .into_string()
                .unwrap();
            // println!("path: {}", path);
            path.contains("DHF & Tech File Word Docs")
        });

    // for family in families_found {
    //     println!("family: {}", family.display());
    // }
    // let families = Vec::new();

    // for r in pass_1 {
    //     println!("result: {}", r.unwrap().path().parent().unwrap().display());
    // }

    // for result in pass_1 {
    //     let result = result.unwrap();
    //     println!("result: {}", result.file_name().to_str().unwrap());
    //     let descendants = WalkDir::new(result.path())
    //         .min_depth(1)
    //         .max_depth(1)
    //         .into_iter().filter(predicate);
    //     for descendant in descendants {
    //         let descendant = descendant.unwrap();
    //         println!(
    //             "descendant: {}",
    //             descendant.file_name().to_str().unwrap()
    //         )
    //         let pass_2 = WalkDir::new(descendant).min_depth(1).max_depth(depth)
    //     }
    // }

    let mut paths_to_return = Vec::new();
    for r in pass_1 {
        let result = r.unwrap();
        println!("result: {}", result.path().parent().unwrap().display());
        let path = result.path().parent().unwrap().to_path_buf();
        paths_to_return.push(path);
    }
    paths_to_return
}

fn scan_subfamilies(path: PathBuf) -> Option<Vec<PathBuf>> {
    // takes the path to a product family folder and scans the _InProgress directory for subfamily
    // folders

    // really ugly way of doing all this but this makes it platform agnostic
    // let in_progress_folder = WalkDir::new(path)
    //     .min_depth(1)
    //     .into_iter()
    //     .filter_entry(|entry| is_directory(entry));

    // let folders = WalkDir::new(path)
    //     .min_depth(1)
    //     .into_iter()
    //     .filter_entry(|e| is_directory(e));

    // println!("path: {}", path.display());

    let in_progress_folder = path.join("DHF & Tech File Word Docs\\_InProgress");

    if !in_progress_folder.exists() {
        return None;
    }

    // println!("{}", in_progress_folder.display());

    // let in_progress_folder = in_progress_folder.last()?.unwrap().into_path();
    // println!("got past");
    //
    // let subfamily_regex = regex::new(r"(\D\d*)*")

    let subfamilies = WalkDir::new(in_progress_folder)
        .sort_by_file_name()
        .min_depth(1)
        .max_depth(1)
        .into_iter()
        .filter_entry(|e| is_directory(e));

    let mut paths_to_return = Vec::new();
    for subfamily in subfamilies {
        let path = subfamily.unwrap().into_path();
        paths_to_return.push(path);
    }
    Some(paths_to_return)
}

fn scan_files(path: PathBuf) -> Vec<PathBuf> {
    let files = WalkDir::new(path)
        .sort_by_file_name()
        .min_depth(1)
        .into_iter()
        .filter_entry(|e| is_valid_file(e));
    // .filter_entry(|e| !is_directory(e));

    let mut paths_to_return = Vec::new();

    for entry in files {
        let result = entry.unwrap();
        if !result.file_type().is_dir() {
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
