use std::process;

use crate::bibliography;
use crate::util;
use crate::download;
use crate::pandoc;
use crate::unzip;

extern crate serde_derive;

#[derive(Debug)]
pub enum DocumentType {
    PDF,
    PRESENTATION
}

//-----------------------------------------------------------------------------
// EnvData
//-----------------------------------------------------------------------------
#[derive(Deserialize, Debug)]
pub struct EnvData {
    pub md_src: Option<String>,
    pub pdf_dir: Option<String>,
    pub presentation_src: Option<String>,
    pub presentation_dir: Option<String>,
    pub git_book_src: Option<String>,
    pub git_book_dir: Option<String>,
    pub pdf_config: Option<String>,
    pub presentation_config: Option<String>,
    pub git_book_config: Option<String>,
    pub bibliography_src: Option<String>,
    pub bibliography: Option<String>,
    pub csl_style: Option<String>,
    pub csl_file: Option<String>,
    pub z_api_key: Option<String>,
    pub z_user_id: Option<String>,
    pub z_collection: Option<String>,
    pub z_group_id: Option<String>,
    pub z_group_collection: Option<String>,
    pub github_repo_name: Option<String>,
    pub github_user_name: Option<String>
}

//-----------------------------------------------------------------------------
// Build
//-----------------------------------------------------------------------------
pub trait Build {
    fn before_build(&self);
    fn build_individually(&self);
    fn build(&self) {
        println!("building");
        &self.before_build();
    }
}

//-----------------------------------------------------------------------------
// Document
//-----------------------------------------------------------------------------
#[derive(Debug)]
pub struct  Document {
    pub document_type: DocumentType,
    pub inputs: Vec<String>,
    pub output: String,
    pub config: String,
    pub bibliography: bibliography::Bibliography
}

impl Document {
    pub fn new (env_data:EnvData, document_type:DocumentType) -> Document {
        match document_type {
            DocumentType::PDF => Document {
                document_type: DocumentType::PDF,
                inputs: util::get_input_files(&env_data.md_src.clone().expect("No md src")),
                output: env_data.pdf_dir.clone().expect("Please give an output Dir"),
                config: env_data.pdf_config.clone().expect("Please give a configuration file"),
                bibliography: bibliography::Bibliography::new(env_data)
            },
            DocumentType::PRESENTATION => Document {
                document_type: DocumentType::PRESENTATION,
                inputs: util::get_input_files(&env_data.presentation_src.clone().unwrap()),
                output: env_data.presentation_dir.clone().expect("Please give an output Dir"),
                config: env_data.presentation_config.clone().expect("Please give a cnofiguration file"),
                bibliography: bibliography::Bibliography::new(env_data)
            }
        }
    }
}

impl Build for Document {
    fn before_build(&self) {
        util::mkdir_all(&self.output);
        self.bibliography.download_bibliography();
        self.bibliography.download_csl_file();
        download_revealjs(&self)
    }

    fn build(&self) {
        self.before_build();
        let pandoc_config = pandoc::create_pandoc_config(self, &self.config);
        pandoc::pandoc(pandoc_config.to_vec());
    }

    fn build_individually (&self) {
        self.before_build();
        for file in self.inputs.iter() {
            let pandoc_config = pandoc::create_pandoc_config(self, &file);
            pandoc::pandoc(pandoc_config.to_vec());
        }
    }
}

fn download_revealjs(data: &Document) {
    match data.document_type {
        DocumentType::PRESENTATION => {
            let revealjs_dest = format!("{}/reveal.zip", data.output);
            let revealjs = download::download_dont_replace("https://github.com/hakimel/reveal.js/archive/master.zip", &revealjs_dest, None);
            match revealjs {
                Err(_) => {
                    eprintln!("Failed to download reveal.js");
                    process::exit(1)
                },
                Ok(_) => ()
            };

            unzip::unzip(&revealjs_dest, &data.output);
        },
        _ => ()
    }
}
