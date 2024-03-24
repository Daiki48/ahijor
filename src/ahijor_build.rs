mod stylesheet;
mod templates;
use super::ahijor_error::Error;
use stylesheet::STYLE;
use templates::BASE;

use pulldown_cmark::{Options, Parser};
use std::cell::RefCell;
use std::path::Path;
use tera::{Context, Tera};

const PAGES_DIR_NOT_FOUND: &str = r#"
++++++++++++++++++++++++++++++++++++++++++++++++++++++++

The pages directory does not exist.
Please create the pages directory in the root of the project.

++++++++++++++++++++++++++++++++++++++++++++++++++++++++
"#;

fn process_entry(tera: &mut Tera, entry: &std::fs::DirEntry) -> Result<(), Error> {
    if entry.file_type()?.is_dir() {
        if let Ok(entries) = std::fs::read_dir(entry.path()) {
            for entry in entries.flatten() {
                process_entry(tera, &entry)?;
            }
        }
        Ok(())
    } else if entry.file_name() == "+page.md" {
        let markdown: String = std::fs::read_to_string(entry.path()).map_err(Error::from)?;
        let html: String = markdown_to_html(markdown);

        let mut context: Context = Context::new();
        context.insert("content", &html);
        context.insert("STYLE", &STYLE);
        let output: Result<String, tera::Error> = tera.render("template", &context);

        let output_path: std::path::PathBuf = match entry.path().strip_prefix("pages") {
            Ok(stripped_path) => Path::new("build")
                .join(stripped_path)
                .with_extension("html"),
            Err(_) => {
                return Err(Error::Io(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Failed to strip prefix",
                )));
            }
        };
        match output {
            Ok(o) => {
                if let Some(parent) = output_path.parent() {
                    std::fs::create_dir_all(parent).map_err(Error::from)?;
                }
                std::fs::write(output_path.clone(), o)?;
                if let Some(file_name) = output_path.file_name() {
                    if file_name == "+page.html" {
                        let new_path: std::path::PathBuf = output_path.with_file_name("index.html");
                        std::fs::rename(output_path, new_path).map_err(Error::from)?;
                    }
                }
                Ok(())
            }
            Err(e) => {
                println!("Failed to render template : {}", e);
                Err(Error::Tera(e))
            }
        }
    } else {
        Err(Error::Io(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Not found +page.md file...",
        )))
    }
}

fn visit_dirs<F>(dir: &Path, cb: &mut F) -> Result<(), Error>
where
    F: FnMut(&mut std::fs::DirEntry) -> Result<(), Error>,
{
    println!("Current dir : {:?}", dir);
    if dir.is_dir() {
        let read_dir: std::fs::ReadDir = std::fs::read_dir(dir)?;
        for entry_result in read_dir {
            let mut entry: std::fs::DirEntry = entry_result?;
            cb(&mut entry)?;
            if entry.path().is_dir() {
                visit_dirs(&entry.path(), cb)?;
            }
        }
    }
    Ok(())
}

pub fn html_with_tera() -> Result<(), Error> {
    let pages_dir: &Path = Path::new("pages");
    if !pages_dir.is_dir() {
        println!("{}", PAGES_DIR_NOT_FOUND);
        return Err(Error::from(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Not found pages directory...",
        )));
    }
    let tera: RefCell<Tera> = RefCell::new(Tera::default());
    tera.borrow_mut().add_raw_template("template", BASE)?;

    let mut porcess_entry_closure = |entry: &mut std::fs::DirEntry| -> Result<(), Error> {
        let mut tera_guard: std::cell::RefMut<'_, Tera> = tera.borrow_mut();
        process_entry(&mut tera_guard, entry)
    };

    visit_dirs(Path::new("pages"), &mut porcess_entry_closure)?;
    println!("Created HTML in build directory!");

    // let src_favicon: &Path = Path::new("static/favicon.ico");
    // let dst_favicon: &Path = Path::new("build/favicon.ico");
    // std::fs::copy(src_favicon, dst_favicon)?;
    // println!("Copy favicon.");
    Ok(())
}

fn markdown_to_html(markdown: String) -> String {
    let mut options: Options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);

    let parser: Parser<'_> = Parser::new_ext(&markdown, options);
    let mut html_output: String = String::new();

    pulldown_cmark::html::push_html(&mut html_output, parser);
    html_output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_markdown_to_html() {
        let markdown: String = String::from("# Hello, World\n");
        let expected_html: String = String::from("<h1>Hello, World</h1>\n");
        let html: String = markdown_to_html(markdown);

        assert_eq!(html, expected_html);
    }
}
