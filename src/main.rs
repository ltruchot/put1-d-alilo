use tracing::Level;

use std::sync::Arc;

use std::fs;

use dioxus::{prelude::dioxus_elements::FileEngine, prelude::*};

use slug::slugify;
use std::path::Path;

const _TAILWIND_URL: &str = manganis::mg!(file("./assets/tailwind.css"));
const _MAIN_CSS_URL: &str = manganis::mg!(file("./assets/main.css"));

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");

    let cfg = dioxus::desktop::Config::new();
    LaunchBuilder::desktop().with_cfg(cfg).launch(App);
}

#[component]
fn App() -> Element {
    let mut files_uploaded = use_signal(|| Vec::new() as Vec<String>);

    let read_files = move |file_engine: Arc<dyn FileEngine>| async move {
        let files = file_engine.files();
        for file_name in &files {
            files_uploaded.write().push(file_name.clone());
        }
    };

    let upload_files = move |evt: FormEvent| async move {
        if let Some(file_engine) = evt.files() {
            read_files(file_engine).await;
        }
    };
    let haveFiles = files_uploaded.read().len() > 0;

    // Build cool things ✌️

    rsx! {
    style { {include_str!("./../assets/tailwind.css")} }
    style { {include_str!("./../assets/main.css")} }
    div {
        { (!haveFiles).then(|| rsx!(
            div {
                label { class: "w-full text-gray-500 font-medium text-lg block mb-4", "Sélectionner les fichier mp3 à renommer" }
                input {
                    class: "w-full text-gray-500 font-medium text-lg bg-gray-100 file:cursor-pointer cursor-pointer file:border-0 file:py-3 file:px-4 file:mr-4 file:bg-gray-800 file:hover:bg-gray-700 file:text-white rounded block",
                    r#type: "file",
                    accept: ".mp3",
                    multiple: true,
                    onchange: upload_files,
                }
            }
        )) }
    }

        ul {
            class: "border border-gray-200 rounded overflow-hidden shadow-md",
            for file_path in files_uploaded.read().iter().rev() {
                li {
                    class: "px-4 py-2 bg-white hover:bg-sky-100 hover:text-sky-900 border-b last:border-none border-gray-200 transition-all duration-300 ease-in-out",
                    span {
                        {format!("{} -> ", file_path)}, strong { {process_path(file_path).unwrap_or_else(|| "invalid path".to_string()) }}
                     }
                }
            }
        }

        { haveFiles.then(|| rsx!(
            button {
            class: "w-full font-medium text-lg cursor-pointer border-0 py-3 px-4 mr-4 bg-gray-800 :hover:bg-gray-700 text-white rounded block",
            onclick: move |_| rename_files(&files_uploaded.read()),
            "Renommer les fichiers"
            }
        ))}

    }
}

fn get_filename_from_path(path: &str) -> Option<String> {
    Path::new(path).file_name()?.to_str().map(|s| s.to_string())
}

fn slugify_filename(filename: &str) -> String {
    let parts: Vec<&str> = filename.rsplitn(2, '.').collect();
    if parts.len() == 2 {
        format!("{}.{}", slugify(parts[1]), parts[0])
    } else {
        slugify(filename)
    }
}

fn process_path(path: &str) -> Option<String> {
    get_filename_from_path(path).map(|filename| slugify_filename(&filename))
}

fn rename_files(files: &[String]) {
    for file_path in files {
        if let Some(new_name) = process_path(file_path) {
            let new_path = Path::new(file_path).with_file_name(new_name);
            fs::rename(file_path, new_path).expect("Failed to rename file");
        }
    }
    std::process::exit(0);
}
