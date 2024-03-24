mod ahijor_build;
mod ahijor_dev;
mod ahijor_error;
mod ahijor_help;
mod ahijor_init;

use ahijor_build::html_with_tera;
use ahijor_dev::start_local_server;
use ahijor_error::Error;
use ahijor_help::help_display;
use ahijor_init::create_project;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args: Vec<String> = std::env::args().collect();
    match args.get(1) {
        Some(command) => match command.as_str() {
            "init" => create_project()?,
            "build" => html_with_tera()?,
            "dev" => {
                html_with_tera()?;
                let server_handle: tokio::task::JoinHandle<Result<(), Error>> =
                    tokio::spawn(start_local_server());
                server_handle.await??;
            }
            "help" => help_display()?,
            _ => {
                println!("Unknown command : {}", command);
                println!("Try the <ahijor help>");
                return Ok(());
            }
        },
        None => {
            println!("Usage : ahijor <command>");
            return Ok(());
        }
    }
    Ok(())
}
