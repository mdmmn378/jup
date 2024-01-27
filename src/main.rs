use clap::{ArgAction, Command, Error};
use colored::*;

mod core;
mod io_handler;

fn handle_cli(matches: clap::ArgMatches) -> Result<(), Error> {
    let file_name = matches.get_one::<String>("file").unwrap();

    if !std::path::Path::new(file_name).exists() {
        println!("{}", "File does not exist".red());
        std::process::exit(1);
    }

    if !file_name.ends_with(".ipynb") {
        println!("{}", "File must be a .ipynb file".red());
        std::process::exit(1);
    }

    let file_content = io_handler::read_file(file_name.clone()).unwrap();
    let markdown_content = core::convert_string_to_markdown(file_content).unwrap();

    if matches.get_flag("write") {
        let file_name_root = file_name.split('.').collect::<Vec<&str>>()[0];
        let file_name = format!("{}.md", file_name_root);
        io_handler::write_to_file(markdown_content, file_name.clone()).unwrap();
        println!("{}", format!("File {} written", file_name).green().bold());
    } else {
        io_handler::print_file(markdown_content).unwrap();
    }

    Ok(())
}

fn main() {
    let cmd = Command::new("jup").version("0.1.0").bin_name("jup");
    let cmd = cmd.arg(clap::Arg::new("file").required(true).index(1));
    let cmd = cmd.arg(
        clap::Arg::new("write")
            .short('w')
            .long("write")
            .help("Write to file")
            .value_parser(clap::value_parser!(bool))
            .required(false)
            .action(ArgAction::SetTrue),
    );

    let matches = cmd.get_matches();
    match matches.args_present() {
        true => {
            handle_cli(matches).unwrap();
        }
        false => {
            println!("{}", "No arguments provided".red());
            std::process::exit(1);
        }
    }
}
