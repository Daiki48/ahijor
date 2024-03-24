use super::ahijor_error::Error;

const HELP: &str = r#"
You will Know what commands you can execute with ahijor.

ahijor init
- Create a template for your project.

ahijor build
- Build content written in Markdown into HTML.

ahijor dev
- Start the local server.
"#;

pub fn help_display() -> Result<(), Error> {
    println!("{}", HELP);
    Ok(())
}
