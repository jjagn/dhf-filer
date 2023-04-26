// use std::io::{stdin, stdout, Write};
// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use regex::Regex;
use std::{
    ffi::OsString,
    fmt,
    fs::rename,
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
enum DocType {
    PDF,
    WordDoc,
    Other,
}

impl fmt::Display for DocType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DocType::PDF => write!(f, "pdf"),
            DocType::WordDoc => write!(f, "word doc"),
            DocType::Other => write!(f, "other"),
        }
    }
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
    doc_type: DocType,
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
                    if ui.button("go back").clicked() {
                        self.state = GUIState::FamilySelect;
                    }
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        for subfamily in &mut self.family_to_file.subfamilies {
                            ui.horizontal(|ui| {
                                // ui.checkbox(&mut subfamily.to_file, "");
                                ui.label(&subfamily.name);
                                ui.with_layout(
                                    egui::Layout::right_to_left(egui::Align::Max),
                                    |ui| {
                                        ui.checkbox(&mut subfamily.to_file, "to file");
                                    },
                                );
                            });
                            ui.separator();
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
                    if ui.button("go back").clicked() {
                        self.subfamilies_to_file.clear();
                        self.state = GUIState::SubfamilySelect;
                    }
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        for subfamily in &mut self.subfamilies_to_file {
                            // ui.label(
                            // RichText::new(&subfamily.name).font(FontId::proportional(20.0)),
                            // );
                            ui.collapsing(
                                RichText::new(&subfamily.name).font(FontId::proportional(14.0)),
                                |ui| {
                                    for document in &mut subfamily.documents {
                                        ui.horizontal(|ui| {
                                            // ui.checkbox(&mut document.to_add, "add");
                                            // ui.checkbox(&mut document.to_update, "update");
                                            ui.label(&document.name);
                                            ui.with_layout(
                                                egui::Layout::right_to_left(egui::Align::Max),
                                                |ui| {
                                                    ui.checkbox(&mut document.to_add, "add");
                                                    // ui.checkbox(&mut document.to_update, "update");
                                                },
                                            );
                                        });
                                        ui.separator();
                                    }
                                },
                            );
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

            // println!("subfolder: {}", sf_struct.name);
            let files = scan_files(sf_struct.path.clone());
            for file in files {
                let file_name = name_string_from_dir_entry(&file);
                let document = Document {
                    index: 0,
                    path: file.clone(),
                    revision: {
                        match find_revision_from_path_buf(&file) {
                            Some(i) => i,
                            None => 0,
                        }
                    },
                    doc_type: doc_type_from_string(&file_name),
                    name: file_name,
                    to_update: false,
                    to_add: false,
                };
                println!("document: {}", document.name);
                println!("path: {}", document.path.display());
                println!("revision: {}", document.revision);
                println!("type: {}", document.doc_type);
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

fn doc_type_from_string(name: &String) -> DocType {
    if name.contains(".docx") {
        DocType::WordDoc
    } else if name.contains(".pdf") {
        DocType::PDF
    } else {
        DocType::Other
    }
}

fn find_revision_from_path_buf(file: &PathBuf) -> Option<i32> {
    let revision_regex = Regex::new(r"Rev\d{1,}").unwrap();
    let number_regex = Regex::new(r"\d{1,}").unwrap();
    let file_name = file.file_name().unwrap().to_str().unwrap();
    let rev_position = revision_regex.find(file_name);
    match rev_position {
        Some(v) => {
            let rev_text = &file_name[v.start()..v.end()];
            let number_position = number_regex.find(rev_text).unwrap();
            let rev: Result<i32, _> =
                rev_text[number_position.start()..number_position.end()].parse();
            if let Ok(r) = rev {
                Some(r)
            } else {
                None
            }
        }
        None => None,
    }
}

fn is_in_progress_folder(entry: &DirEntry) -> bool {
    entry.file_name().to_str().unwrap().contains("_InProgress")
}

fn name_string_from_dir_entry(entry: &PathBuf) -> String {
    entry
        .file_name()
        .unwrap()
        .to_os_string()
        .into_string()
        .unwrap()
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

    // println!("path: {}", path.display());

    let in_progress_folder = path.join("DHF & Tech File Word Docs\\_InProgress");

    if !in_progress_folder.exists() {
        return None;
    }

    let subfamilies = WalkDir::new(in_progress_folder)
        .sort_by_file_name()
        .min_depth(1)
        //make this controlled by a setting
        .max_depth(2)
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

// fn add_doc(doc: Document, family: Family) -> Result<String, String> {
//     doc.path;
//     rename(from, to)
// }
//
// fn update_doc(doc: Document, family: Family) -> Result<String, String> {}

fn match_doc_name_to_dhf_path() {}

// fn match_doc_name_to_tech_file_path(doc_path: PathBuf) -> PathBuf {}
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
