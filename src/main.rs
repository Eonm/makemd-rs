mod bibliography;
mod build;
mod defaultconfig;
mod download;
mod file_system;
mod newproject;
mod pandoc;
mod unzip;
mod util;
use crate::build::Build;
use crate::build::EnvData;

#[macro_use]
extern crate log;
extern crate env_logger;

extern crate envy;

#[macro_use]
extern crate serde_derive;
extern crate dotenv;

extern crate clap;
use clap::{App, Arg, SubCommand};

fn main() {
    defaultconfig::set_default_config();
    env_logger::init();

    let matches = App::new("MakeMD")
        .version("0.1.2")
        .author("Eonm <eon.mathis@gmail.com>")
        .about("Build document with pandoc")
        .subcommand(
            SubCommand::with_name("build")
                .arg(
                    Arg::with_name("individually")
                        .long("individually")
                        .short("i")
                        .help("Build a pdf")
                        .takes_value(false),
                )
                .arg(
                    Arg::with_name("pdf")
                        .long("pdf")
                        .help("Build a pdf")
                        .takes_value(false),
                )
                .arg(
                    Arg::with_name("presentation")
                        .long("presentation")
                        .help("Build a presentation")
                        .takes_value(false),
                ),
        )
        .subcommand(
            SubCommand::with_name("maintenance")
                .arg(
                    Arg::with_name("bibliography")
                        .long("update-bib")
                        .help("Update bibliography")
                        .takes_value(false),
                )
                .arg(
                    Arg::with_name("csl")
                        .long("update-csl")
                        .help("Update citation style file")
                        .takes_value(false),
                ),
        )
        .subcommand(
            SubCommand::with_name("init")
                .arg(
                    Arg::with_name("new")
                        .long("new")
                        .short("n")
                        .help("Create a new project")
                        .takes_value(false),
                )
                .arg(
                    Arg::with_name("force")
                        .long("force")
                        .help("Force to create a porject")
                        .takes_value(false),
                ),
        )
        .subcommand(SubCommand::with_name("lint"))
        .get_matches();

    // Build-----------------------------------------------------------------------
    if let Some(matches) = matches.subcommand_matches("build") {
        load_env_file();
        let env_data = envy::from_env::<EnvData>().expect("failed to parse .makemd file");
        if matches.is_present("pdf") {
            let pdf = build::Document::new(env_data, build::DocumentType::PDF);
            if matches.is_present("individually") {
                pdf.build_individually();
                println!("PDFs builded");
            } else {
                pdf.build();
                println!("PDF builded");
            }
        } else if matches.is_present("presentation") {
            let presentation = build::Document::new(env_data, build::DocumentType::PRESENTATION);
            if matches.is_present("individually") {
                presentation.build_individually();
                println!("Presentations builded");
            } else {
                presentation.build();
                println!("Presentation builded");
            }
        }
    };

    // Maintenance-----------------------------------------------------------------

    if let Some(matches) = matches.subcommand_matches("maintenance") {
        load_env_file();
        let env_data = envy::from_env::<EnvData>().expect("Failed to parse .makemd file");
        if matches.is_present("bibliography") {
            bibliography::Bibliography::new(env_data).download_bibliography_force();
            println!("Bibliography updated");
        } else if matches.is_present("csl") {
            bibliography::Bibliography::new(env_data).download_csl_file_force();
            println!("Csl file udpated");
        }
    };

    // Init------------------------------------------------------------------------
    if let Some(matches) = matches.subcommand_matches("init") {
        if matches.is_present("new") {
            match dotenv::from_filename(".makemd") {
                Ok(env) => {
                    if matches.is_present("force") {
                        newproject::init_project();
                        println!("Project created");
                    } else {
                        println!("You are already in a makemd project : {}", env.display());
                    }
                }
                Err(_e) => {
                    newproject::init_project();
                    println!("Project created");
                }
            }
        }
    };

    // Lint------------------------------------------------------------------------
}

use std::process;
fn load_env_file() {
    match dotenv::from_filename(".makemd") {
        Err(_e) => {
            eprintln!("\nYou aren't in a MakeMD project (Failed to read .makemd file)\nTry to init a project first");
            process::exit(1);
        }
        Ok(data) => data,
    };
}
