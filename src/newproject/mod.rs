use std::fs;
use std::path::Path;
use std::env;
use crate::build::EnvData;
use std::io;
use std::process;

pub fn init_project() {

    let home_dir = env::home_dir().expect("failed to get Home Dir");
    let defaultconfig_path = home_dir.join(".makemd");

    match import_default_env(&defaultconfig_path) {
        Err(err) => {
            eprintln!("Failed to import default .makemd file : {}", err);
            process::exit(1)
        }
        _ => ()
    }
    dotenv::from_filename(".makemd").expect("Failed to read .makemd file");
    let env_data = envy::from_env::<EnvData>().expect("failed to parse .makemd file");

    match scaffold_config_dirs(env_data.clone()) {
        Err(err) => {
            eprintln!("Failed to scaffold config dir : {}", err);
            process::exit(1)
        }
        _ => ()
    };

    match import_templates_files(&defaultconfig_path, env_data) {
        Err(err) => {
            eprintln!("Failed to import template file : {}", err);
            process::exit(1)
        }
        _ => ()
    }

}

fn import_default_env(config_path:  &std::path::PathBuf) -> Result<(), io::Error> {
    fs::copy(config_path.join("env.rc"), ".makemd")?;
    Ok(())
}

fn scaffold_config_dirs(env_data: EnvData) -> Result<(), io::Error> {
    fs::create_dir_all(env_data.clone().md_src.unwrap_or("".to_string()))?;
    fs::create_dir_all(env_data.git_book_src.unwrap_or("".to_string()))?;
    fs::create_dir_all(env_data.presentation_src.unwrap_or("".to_string()))?;

    let pdf_config_path = env_data.pdf_config.unwrap_or("".to_string());
    let pdf_config_dir = Path::new(&pdf_config_path).parent().unwrap();

    let presentation_config_path = env_data.presentation_config.unwrap_or("".to_string());
    let presentation_config_dir = Path::new(&presentation_config_path).parent().unwrap();

    let git_book_config_path = env_data.git_book_config.unwrap_or("".to_string());
    let git_book_config_dir = Path::new(&git_book_config_path).parent().unwrap();

    fs::create_dir_all(pdf_config_dir)?;
    fs::create_dir_all(presentation_config_dir)?;
    fs::create_dir_all(git_book_config_dir)?;
    Ok(())
}

fn import_templates_files(config_path:  &std::path::PathBuf, env_data: EnvData) -> Result<(), io::Error> {
    let pdf_config_path = env_data.pdf_config.unwrap_or("".to_string());
    let git_book_config_path = env_data.git_book_config.unwrap_or("".to_string());
    let presentation_config_path =  env_data.presentation_config.unwrap_or("".to_string());

    fs::copy(config_path.join("templates").join("PDF_CONFIG"), pdf_config_path)?;
    fs::copy(config_path.join("templates").join("PRESENTATION_CONFIG"), presentation_config_path)?;
    fs::copy(config_path.join("templates").join("GIT_BOOK_CONFIG"), git_book_config_path)?;
    Ok(())
}
