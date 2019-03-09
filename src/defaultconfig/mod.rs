use crate::file_system;
use std::env;
use std::fs;
use std::process;
use std::io;

pub fn set_default_config() {
    let defaultconfig_path = env::home_dir().expect("Failed to get Home Dir");
    match create_config_dir(&defaultconfig_path) {
        Err(err) => {
            eprintln!("Failed to create config dir : {}", err);
            process::exit(1)
        }
        _ => ()
    };

    match populate_default_env_file(&defaultconfig_path) {
        Err(err) => {
            eprintln!("Failed to populate default .makemd file : {}", err);
            process::exit(1)
        }
        _ => ()
    }

    match create_template_dir(&defaultconfig_path) {
        Err(err) => {
            eprintln!("Failed to create default template dir : {}", err);
            process::exit(1)
        }
        _ => ()
    }

    match populate_templates(&defaultconfig_path) {
        Err(err) => {
            eprintln!("Failed to populate default templates : {}", err);
            process::exit(1)
        }
        _ => ()
    }
}

fn default_env_file() -> &'static str {
r#"MD_SRC=./md/
PDF_DIR=./pdf/
PRESENTATION_SRC=./md/
PRESENTATION_DIR=./presentation/
GIT_BOOK_SRC=./md/
GIT_BOOK_DIR=./gitbook/
PDF_CONFIG=./config/pdf.yml
PRESENTATION_CONFIG=./config/presentation.yml
GIT_BOOK_CONFIG=./config/gitbook.yml
"#
}

fn default_yml_file() -> &'static str {
r#"---
title: Your title
author: Your name
bibliography: #
csl: #
documentclass: book #report, memoir, article
classoption: twoside #oneside
link-citations: true
header-includes:
  - \usepackage[unicode=true]{hyperref}
---
"#
}

fn populate_default_env_file(home_dir: &std::path::PathBuf) -> Result<(), io::Error> {
    let default_env_file_path = home_dir.join(".makemd/env.rc");
    file_system::create_file_with_content(&default_env_file_path, default_env_file())?;
    Ok(())
}

fn populate_templates(home_dir: &std::path::PathBuf) -> Result<(), io::Error> {
    let template_list: Vec<&str> =  vec!("PDF_CONFIG", "GIT_BOOK_CONFIG", "PRESENTATION_CONFIG");

    for template in template_list {
        let template_path = home_dir.join(".makemd/templates").join(template);
        file_system::create_file_with_content(&template_path, default_yml_file())?;
    }
    Ok(())
}

fn create_config_dir(home_dir: &std::path::PathBuf) -> Result<(), io::Error> {
    fs::create_dir_all(home_dir.join(".makemd"))?;
    Ok(())
}

fn create_template_dir(home_dir: &std::path::PathBuf) -> Result<(), io::Error> {
    fs::create_dir_all(home_dir.join(".makemd/templates"))?;
    Ok(())
}
