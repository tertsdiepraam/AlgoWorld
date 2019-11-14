#![feature(proc_macro_hygiene)]
extern crate walkdir;
mod page;
use std::{
    env,
    fs::{File, create_dir_all},
    io::{Read, Write, BufRead, BufReader},
    collections::HashMap,
    path::{
        Path,
        PathBuf,
    },
};
use walkdir::WalkDir;
use page::{WikiPageToml, AlgorithmPage, CategoryPage, PageType};
use syntect::parsing::SyntaxSet;
use syntect::highlighting::ThemeSet;

fn get_file_extensions(filename: &str) -> std::io::Result<HashMap<String, String>> {
    let file_extensions_file = File::open(filename)?;
    let mut file_extensions = HashMap::new();
    for line in BufReader::new(file_extensions_file).lines() {
        let values : Vec<String> = line.unwrap().splitn(2, ",")
            .map(|val| String::from(val))
            .collect();
        file_extensions.insert(values[0].trim().to_owned(), values[1].trim().to_owned());
    }
    Ok(file_extensions)
}

fn get_pages(path: &String) -> std::io::Result<HashMap<String, (WikiPageToml, PathBuf)>> {
    let path = Path::new(&path);
    let mut pages = HashMap::new();
    for entry in WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.metadata().unwrap().is_file() && e.file_name().to_string_lossy().ends_with(".toml")) {
            let mut toml_file = File::open(entry.path()).unwrap();
            let mut toml_str = String::new();
            toml_file.read_to_string(&mut toml_str)?;
            let page: WikiPageToml = toml::from_str(toml_str.as_str()).unwrap();
            pages.insert(page.title.clone(), (page, entry.into_path()));
        }
    Ok(pages)
}

fn main() -> std::io::Result<()> {
    let args : Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Please enter a path");
        return Ok(());
    }

    let file_extensions = match get_file_extensions("file_extensions.csv") {
        Ok(x) => x,
        Err(_) => panic!("Syntax error in file_extensions.csv"),
    };
    
    let pages = get_pages(&args[1])?;

    let ss = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();
    let theme = &ts.themes["InspiredGitHub"];

    let url_lookup : HashMap<String, String> = pages
        .iter()
        .map(|(title, (page, _))| (title.clone(), page.url.clone()))
        .collect();

    println!("Generated files:");
    for (_, (page, path)) in pages {
        let write_path = Path::new(&page.url).with_extension("html");
        let contents = match page.page_type {
            PageType::Algorithm => AlgorithmPage::from(page, &path, &file_extensions)
                .expect(format!("Could not render page at {}", write_path.file_name().unwrap().to_string_lossy()).as_str())
                .render(&url_lookup, &ss, &theme)
                .into_string(),
            PageType::Category => CategoryPage::from(page, &path)
                .expect(format!("Could not render page at {}", write_path.file_name().unwrap().to_string_lossy()).as_str())
                .render(&url_lookup)
                .into_string(),
            _ => String::new(),
        };

        if let Some(d) = write_path.parent() {
            create_dir_all(d)?;
        }

        let mut write_file = File::create(&write_path).unwrap();
        write_file.write_all(contents.as_bytes())?;
        println!("- {}", write_path.display());
    }

    Ok(())
}
