mod build;
mod bibliography;
mod pandoc;
mod util;
mod download;
mod unzip;
use crate::build::EnvData;
use crate::build::Build;

#[macro_use]
extern crate log;
extern crate env_logger;

extern crate envy;

#[macro_use]
extern crate serde_derive;
extern crate dotenv;

extern crate clap;
use clap::{Arg, App, SubCommand};


fn main() {
    env_logger::init();
    let matches = App::new("MakeMD")
                          .version("0.1.0")
                          .author("Eonm <eon.mathis@gmail.com>")
                          .about("Build document with pandoc")
                          .subcommand(SubCommand::with_name("build")
                            .arg(Arg::with_name("individually")
                                .long("individually")
                                .short("i")
                                .help("Build a pdf")
                                .takes_value(false))
                            .arg(Arg::with_name("pdf")
                               .long("pdf")
                               .help("Build a pdf")
                               .takes_value(false))
                            .arg(Arg::with_name("presentation")
                                .long("presentation")
                                .help("Build a presentation")
                                .takes_value(false)))
                            .subcommand(SubCommand::with_name("maintenance")
                              .arg(Arg::with_name("bibliography")
                                 .long("update-bib")
                                 .help("Update bibliography")
                                 .takes_value(false))
                              .arg(Arg::with_name("csl")
                                  .long("update-csl")
                                  .help("Update citation style file")
                                  .takes_value(false)))
                            .subcommand(SubCommand::with_name("lint")
                              ).get_matches();

// Build-----------------------------------------------------------------------
    if let Some(matches) = matches.subcommand_matches("build") {
        dotenv::dotenv().expect("Failed to read .env file");
        let env_data = envy::from_env::<EnvData>().expect("failed to parse .env file");
        if matches.is_present("pdf") {
            println!("Building pdf");
            let pdf = build::Document::new(env_data, build::DocumentType::PDF);
            if matches.is_present("individually") {
                pdf.build_individually();
            } else {
                pdf.build();
            }
        } else if matches.is_present("presentation") {
            println!("Building presentation");
            let presentation = build::Document::new(env_data, build::DocumentType::PRESENTATION);
            if matches.is_present("individually") {
                presentation.build_individually();
            } else {
                presentation.build();
            }
        }
    };

// Maintenance-----------------------------------------------------------------
    if let Some(matches) = matches.subcommand_matches("maintenance") {
        dotenv::dotenv().expect("Failed to read .env file");
        let env_data = envy::from_env::<EnvData>().expect("failed to parse .env file");
        if matches.is_present("bibliography") {
            println!("Updating bibliography");
            bibliography::Bibliography::new(env_data).download_bibliography_force();
        } else if matches.is_present("csl") {
            bibliography::Bibliography::new(env_data).download_csl_file_force();
        }
    };

// Lint------------------------------------------------------------------------

}
