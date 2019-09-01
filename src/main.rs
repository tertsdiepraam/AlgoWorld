#![feature(proc_macro_hygiene)]

extern crate comrak;
extern crate walkdir;
extern crate maud;
use std::env;
use std::path::{Path, PathBuf};
use std::fs::{File, create_dir_all};
use std::io::{Read, Write, BufRead, BufReader};
use std::collections::HashMap;
use comrak::{markdown_to_html, ComrakOptions};
use walkdir::WalkDir;
use maud::{DOCTYPE, html, Markup, PreEscaped};
use titlecase::titlecase;

fn inject_content(path: &Path, story: String, implementations: &HashMap<String, String>, subpages: &Vec<(String, String)>, css: String) -> Markup {
    html! {
        (DOCTYPE)
        head {
            meta charset="UTF-8";
            style {
                (css)
            }
        }
        body {
            nav {
                @for ancestor in path.ancestors()
                    .collect::<Vec<&Path>>()
                    .into_iter()
                    .skip(1)
                    .rev() {
                        @if let Some(name) = ancestor.file_name() {
                            a href={(ancestor.to_string_lossy()) (".html")} {
                                (name.to_string_lossy())
                            }
                            "/"
                        }
                    }
            }

            .wrapper {
                (PreEscaped(story))
            }

            @if !implementations.is_empty() {
                .wrapper {
                    @for (title, content) in implementations {
                        h2 {
                            (title)
                        }
                        pre {
                            code."language-lang=python" {
                                (content)
                            }
                        }
                    }
                }
            }

            @if !subpages.is_empty() {
                .wrapper {
                    h2 {"Subpages"}
                    ul {
                        @for (name,path) in subpages {
                            li {
                                a href={(path) ".html"} {(name)}
                            }
                        }
                    }
                }
            }

            footer {
                "Created by Terts Diepraam"
                    br;
                "Source code hosted on "
                    a href="" {"Github"}
            }
        }
    }
}

fn process_file(path: &Path, css: &String, file_extensions: &HashMap<String, String>) -> std::io::Result<()> {
    let mut read_file = File::open(path)?;
    let mut markdown = String::new();
    read_file.read_to_string(&mut markdown)?;

    let options = ComrakOptions::default();
    let html = markdown_to_html(&markdown, &options);

    let implementations_vec = WalkDir::new(path.parent().unwrap())
        .max_depth(1)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.metadata().unwrap().is_file() && e.path().file_stem() == path.file_stem());

    let mut implementations = HashMap::new();
    for entry in implementations_vec {
        let path = entry.into_path();
        if let Some(title) = file_extensions.get(path
                                                 .extension()
                                                 .expect("Failed to get extension")
                                                 .to_str()
                                                 .expect("Failed to turn into str")) {
            let mut contents_file = File::open(path)?;
            let mut contents = String::new();
            contents_file.read_to_string(&mut contents)?;
            implementations.insert(title.to_owned(), contents);
        }
    }

    let mut subpages = Vec::new();
    for entry in WalkDir::new(path.parent().unwrap())
        .min_depth(1)
        .max_depth(1)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.metadata().unwrap().is_dir()) {
            let name = entry.file_name().to_str().unwrap().to_owned();
            let mut rel_path = PathBuf::from(path.parent().unwrap().file_name().unwrap());
            rel_path.push(name.as_str());
            subpages.push((titlecase(&name.clone().replace("_", " ")), String::from(rel_path.to_string_lossy())));
        }

    // let write_path = path.parent().unwrap().with_extension("html");
    let write_path = Path::new("wiki")
        .join(path.strip_prefix("wiki_src").expect("Could not strip prefix: \"wiki_src\""))
        .parent().unwrap()
        .with_extension("html");
    
    println!("{}", write_path.display());
    
    create_dir_all(write_path.parent().unwrap()).expect("Could not create directory");
    let mut write_file = File::create(write_path)?;
    write_file.write_all(inject_content(
        &path,
        html,
        &implementations,
        &subpages,
        css.to_string()
    ).into_string().as_bytes())?;

    Ok(())
}

fn main() -> std::io::Result<()> {
    let args : Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Please enter a path");
        return Ok(());
    }
    let root_path = Path::new(&args[1]);

    let file_extensions_file = File::open("file_extensions.csv")?;
    let mut file_extensions = HashMap::new();
    for line in BufReader::new(file_extensions_file).lines() {
        let values : Vec<String> = line.unwrap().splitn(2, ",")
            .map(|val| String::from(val))
            .collect();
        file_extensions.insert(values[0].trim().to_owned(), values[1].trim().to_owned());
    }

    let mut css_file = File::open("base.css")?;
    let mut css = String::new();
    css_file.read_to_string(&mut css)?;

    for entry in WalkDir::new(root_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.metadata().unwrap().is_file())
        .filter(|e| e.file_name().to_string_lossy().ends_with(".md")) {
        process_file(entry.path(), &css, &file_extensions).unwrap();
    }

    Ok(())
}
