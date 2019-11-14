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
use syntect::{
    parsing::SyntaxSet,
    highlighting::Theme,
    html::highlighted_html_for_string,
};

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

    pub related: Option<Vec<String>>,
    pub categories: Option<Vec<String>>,
    // @TODO: Time & Space complexity

    // Category page specific
    pub subpages: Option<Vec<String>>,
}

pub struct AlgorithmPage {
    pub title: String,
    pub url: String,
    pub related: Vec<String>,
    pub categories: Vec<String>,
    pub information: Markup,
    pub implementations: HashMap<String, String>
}

fn base_page(content: Markup) -> Markup {
    html! {
        (DOCTYPE);
        head {
            meta charset="UTF-8";
            link
                rel="stylesheet"
                href="/base.css";
            link
                rel="stylesheet"
                href="https://cdn.jsdelivr.net/npm/katex@0.11.1/dist/katex.min.css"
                integrity="sha384-zB1R0rpPzHqg7Kpt0Aljp8JPLqbXI3bhnPWROx27a9N0Ll6ZP/+DiW/UqRcLbRjq"
                crossorigin="anonymous";
            script
                src="https://cdn.jsdelivr.net/npm/katex@0.11.1/dist/katex.min.js"
                integrity="sha384-y23I5Q6l+B6vatafAwxRu/0oK/79VlbSz7Q9aiSZUvyWYIYsd+qj+o24G5ZU2zJz"
                crossorigin="anonymous"{}
            script
                src="https://cdn.jsdelivr.net/npm/katex@0.11.1/dist/contrib/auto-render.min.js"
                integrity="sha384-kWPLUVMOks5AQFrykwIup5lo0m3iMkkHrD0uJ4H5cjeGihAutqP0yW0J6dpFiVkI"
                crossorigin="anonymous"{}
            script {
                (PreEscaped("document.addEventListener(\"DOMContentLoaded\", () => {renderMathInElement(document.body)});"))
            }
        }
        body {
            nav {
                "AlgoWorld";
                input .search type="text" placeholder="Search..." name="Search";
            }
            .page {(content)}
            footer {
                "Created by Terts Diepraam" br;
                "Source code hosted on ";
                a href="https://github.com/tertsdiepraam/AlgoWorld" {"Github"}
            }
        }
    }
}

fn generate_links(titles: &Vec<String>, url_lookup: &HashMap<String, String>) -> Markup {
    html! {
        @for (i, title) in titles.iter().enumerate() {
            @if i > 0 {", "}
            a href={"/" (url_lookup[title]) ".html"} {(title)}
        }
    }
}

fn render_markdown(path: &Path) -> std::io::Result<Markup> {
    let mut markdown = String::new();
    if let Ok(mut read_file) = File::open(path.with_extension("md")) {
        read_file.read_to_string(&mut markdown)?;
    }
    let options = ComrakOptions {
        ext_footnotes: true,
        unsafe_: true,
        ..ComrakOptions::default()
    };
    Ok(PreEscaped(markdown_to_html(&markdown, &options)))
}

impl AlgorithmPage {
    pub fn from(page_toml: WikiPageToml, path: &Path, file_extensions: &HashMap<String, String>) -> std::io::Result<AlgorithmPage> {
        Ok(AlgorithmPage {
            title: page_toml.title,
            url: page_toml.url,
            related: page_toml.related.unwrap_or(Vec::new()),
            categories: page_toml.categories.unwrap_or(Vec::new()),
            information: render_markdown(path)?,
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
                    }
                }
                implementations
            },
        })
    }

    pub fn render(&self, url_lookup: &HashMap<String, String>, ss: &SyntaxSet, theme: &Theme) -> Markup {
        base_page(html! {
            h1 {(self.title)}
            p {
                "Related: ";
                (generate_links(&self.related, url_lookup))
            }
            p {
                "Categories: ";
                (generate_links(&self.categories, url_lookup))
            }
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
                        h2 {
                            (title)
                        }
                        code {
                            (PreEscaped(highlighted_html_for_string(&content, &ss, &ss.find_syntax_by_name(title).unwrap(), &theme)))
                        }
                    }
                }
            }
            script src="/tab.js"{}
        })
    }
}

pub struct CategoryPage {
    pub title: String,
    pub url: String,
    pub related: Vec<String>,
    pub information: Markup,
    pub subpages: Vec<String>,
}

impl CategoryPage {
    pub fn from(page_toml: WikiPageToml, path: &Path) -> std::io::Result<CategoryPage> {
        Ok(CategoryPage {
            title: page_toml.title,
            url: page_toml.url,
            related: page_toml.related.unwrap_or(Vec::new()),
            information: render_markdown(path)?,
            subpages: page_toml.subpages.unwrap_or(Vec::new()),
        })
    }

    pub fn render(&self, url_lookup: &HashMap<String, String>) -> Markup {
        base_page(html! {
            h1 {(self.title)}
            #Information .wrapper {
                (self.information);
                h1 {"Pages"}
                @for title in &self.subpages {
                    p {a href={"/" (url_lookup[title]) ".html"} {(title)}}
                }
            }
        })
    }
}
