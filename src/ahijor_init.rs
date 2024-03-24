use super::ahijor_error::Error;
use serde::Serialize;
use std::fs;
use std::io::{self, Write};
use std::path::Path;
use std::process;

#[derive(Serialize)]
pub struct Ahijor {
    project_name: String,
    version: String,
    description: String,
}

pub fn create_project() -> Result<(), Error> {
    print!("What is the project name? : ");
    io::stdout().flush().unwrap();
    let mut project_name: String = String::new();
    io::stdin().read_line(&mut project_name).unwrap();
    let project_name: &str = project_name.trim();

    if Path::new(project_name).exists() {
        println!("Error : Already project directory name is {}", project_name);
        process::exit(1);
    }

    fs::create_dir(project_name).unwrap();
    fs::create_dir(format!("{}/pages", project_name)).unwrap();

    let ahijor_config: Ahijor = Ahijor {
        project_name: project_name.to_string(),
        version: "0.1.0".to_string(),
        description: "This is ahijor project!".to_string(),
    };

    let json: String = serde_json::to_string_pretty(&ahijor_config).unwrap();
    fs::write(format!("{}/ahijor.json", project_name), json)?;

    fs::write(
        format!("{}/pages/+page.md", project_name),
        "# Hello page.md",
    )?;
    let success_message: String = format!(
        r#"
Successfully created your project!!

cd {}
ahijor dev

Enjoy your development at ahijor!!
"#,
        project_name
    )
    .to_string();
    println!("{}", success_message);
    Ok(())
}
