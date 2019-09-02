extern crate maud;
extern crate toml;
extern crate comrak;
extern crate walkdir;
use maud::{html, Markup, DOCTYPE, PreEscaped};
use serde::Deserialize;
use std::{
    collections::HashMap,
    fs::File,
    path::Path,
    io::Read,
};
use walkdir::WalkDir;
use comrak::{markdown_to_html, ComrakOptions};
use syntect::parsing::SyntaxSet;
use syntect::highlighting::Theme;
use syntect::html::highlighted_html_for_string;

#[derive(Deserialize, Debug)]
pub enum PageType {
    Algorithm,
    Category,
    Generic,
}

#[derive(Deserialize, Debug)]
pub struct WikiPageToml {
    pub title: String,
    pub page_type: PageType,
    pub url: String,

    pub related_pages: Option<Vec<String>>,
    pub categories: Option<Vec<String>>,
    // @TODO: Time & Space complexity

    // Category page specific
    pub subpages: Option<Vec<String>>,
}

pub struct AlgorithmPage {
    pub title: String,
    pub url: String,
    pub related_pages: Vec<String>,
    pub categories: Vec<String>,
    pub information: Markup,
    pub implementations: HashMap<String, String>
}

fn base_page(content: Markup, css: &String) -> Markup {
    html! {
        (DOCTYPE);
        head {
            meta charset="UTF-8";
            style {(css)}
        }
        body {
            nav {"Homepage etc."}
            .page {(content)}
            footer {
                "Created by Terts Diepraam" br;
                "Source code hosted on ";
                a href="https://github.com/tertsdiepraam/AlgoWorld" {"Github"}
            }
        }
    }
}

impl AlgorithmPage {
    pub fn from(page_toml: WikiPageToml, path: &Path, file_extensions: &HashMap<String, String>) -> std::io::Result<AlgorithmPage> {
        Ok(AlgorithmPage {
            title: page_toml.title,
            url: page_toml.url,
            related_pages: page_toml.related_pages.unwrap_or(Vec::new()),
            categories: page_toml.categories.unwrap_or(Vec::new()),
            information: {
                let mut read_file = File::open(path.with_extension("md"))?;
                let mut markdown = String::new();
                read_file.read_to_string(&mut markdown)?;
                let options = ComrakOptions::default();
                PreEscaped(markdown_to_html(&markdown, &options))
            },
            implementations: {
                let implementations_vec = WalkDir::new(path.parent().unwrap())
                    .max_depth(1)
                    .into_iter()
                    .filter_map(|e| e.ok())
                    .filter(|e| e.metadata().unwrap().is_file());

                let mut implementations = HashMap::new();
                for entry in implementations_vec {
                    let p = entry.into_path();
                    let extension = p
                        .extension()
                        .expect("Failed to get extension")
                        .to_str()
                        .expect("Failed to turn extension into a str");
                    if let Some(title) = file_extensions.get(extension) {
                        let mut content_file = File::open(p)?;
                        let mut content = String::new();
                        content_file.read_to_string(&mut content)?;
                        implementations.insert(title.to_owned(), content);
                        println!("{:?}", implementations);
                    }
                }
                implementations
            },
        })
    }

    pub fn render(&self, css: &String, tab_js: &String, ss: &SyntaxSet, theme: &Theme) -> Markup {
        base_page(html! {
            h1 {(self.title)}
            div {
                button .tablink #defaultOpen onclick="openTab('Information', this)" {
                    "Information"
                }

                @if !self.implementations.is_empty() {
                    button .tablink onclick="openTab('Implementations', this)" {
                        "Implementations"
                    }
                }
            }

            #Information .tabcontent .wrapper {
                (self.information)
            }

            @if !self.implementations.is_empty() {
                #Implementations .tabcontent .wrapper {
                    @for (title, content) in &self.implementations {
                        h1 {
                            (title)
                        }
                        pre {
                            code {
                                (PreEscaped(highlighted_html_for_string(&content, &ss, &ss.find_syntax_by_name(title).unwrap(), &theme)))
                            }
                        }
                    }
                }
            }
            script {(PreEscaped(tab_js))}
        }, &css)
    }
}

pub struct CategoryPage {
    pub title: String,
    pub url: String,
    pub related_pages: Vec<String>,
    pub information: Markup,
    pub pages: Vec<String>,
}
