#![feature(proc_macro_hygiene)]

extern crate walkdir;
mod page;
use std::{
    env,
    fs::{File, create_dir_all},
    io::{Read, Write, BufRead, BufReader},
    collections::HashMap,
    path::Path,
};
use walkdir::WalkDir;
use page::{WikiPageToml, AlgorithmPage, CategoryPage, PageType};
use syntect::parsing::SyntaxSet;
use syntect::highlighting::ThemeSet;


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

    let mut tab_js_file = File::open("tab.js")?;
    let mut tab_js = String::new();
    tab_js_file.read_to_string(&mut tab_js)?;

    let mut pages = HashMap::new();

    for entry in WalkDir::new(root_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.metadata().unwrap().is_file() && e.file_name().to_string_lossy().ends_with(".toml")) {
            let mut toml_file = File::open(entry.path()).unwrap();
            let mut toml_str = String::new();
            toml_file.read_to_string(&mut toml_str)?;
            let page: WikiPageToml = toml::from_str(toml_str.as_str()).unwrap();
            pages.insert(page.title.clone(), (page, entry.into_path()));
    }

    // println!("{:?}", pages);

    println!("Generated files:");

    let ss = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();

    for (_title, (page, path)) in pages {
        let write_path = Path::new(&page.url).with_extension("html");
        let contents = match page.page_type {
            PageType::Algorithm => AlgorithmPage::from(page, &path, &file_extensions)
                .unwrap()
                .render(&css, &tab_js, &ss, &ts.themes["InspiredGitHub"])
                .into_string(),
            /* PageType::Category => CategoryPage::from(page, &path)
                .unwrap()
                .render(&css)
                .into_string(),*/
            _ => String::new(),
        };

        if let Some(d) = write_path.parent() {
            create_dir_all(d)?;
        }
        let mut write_file = File::create(&write_path).unwrap();
        write_file.write_all(contents.as_bytes())?;
        println!("- {}", write_path.display());
    }
    // process_file(entry.path(), &css, &tab_js, &file_extensions).unwrap();

    Ok(())
}
