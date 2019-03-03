use crate::util;
use crate::download;
use crate::build::EnvData;
use std::process;

use std::path::Path;

mod zotero;

#[derive(Debug)]
pub struct Bibliography {
    pub csl_file: Option<String>,
    pub csl_style: Option<String>,
    pub bib_dest: Option<String>,
    pub bib_src: Option<String>,
    pub z_user_id: Option<String>,
    pub z_group_id: Option<String>,
    pub z_api_key: Option<String>,
    pub z_collection: Option<String>,
    pub z_group_collection: Option<String>
}

impl Bibliography {
    pub fn new (env_data: EnvData) -> Bibliography {
        let bib = Bibliography {
            csl_file: env_data.csl_file,
            csl_style: env_data.csl_style,
            bib_dest: env_data.bibliography,
            bib_src: env_data.bibliography_src,
            z_user_id: env_data.z_user_id,
            z_group_id: env_data.z_group_id,
            z_api_key: env_data.z_api_key,
            z_collection: env_data.z_collection,
            z_group_collection: env_data.z_group_collection
        };

        match &bib.csl_file {
            Some(file) => util::mkdir_all(&file),
            None => (),
        };

        match &bib.bib_dest {
            Some(file) => util::mkdir_all(&file),
            None => (),
        };

        bib
    }

    pub fn download_csl_file_force(&self) {
        match (&self.csl_style, &self.csl_file) {
            (Some(csl_style), Some(csl_file)) => {
                let url = format!("https://raw.githubusercontent.com/citation-style-language/styles/master/{}.csl", csl_style);
                let result = download::download(&url, &csl_file, None);
                match result {
                    Err(mut e) => parse_csl_error(&mut e),
                    Ok(()) => ()
                }
            },
            _ => {
                eprintln!("Not enough data to download your .csl file. Check your .env file");
                process::exit(1)
            },
        }
    }

    pub fn download_csl_file(&self) {
        match &self.csl_file {
            Some(csl_file) => {
                if !Path::new(csl_file).exists() {
                    &self.download_csl_file_force();
                }
            }
            None => ()
        }
    }

    pub fn download_bibliography_force(&self) {
        let url = zotero::create_zotero_url(&self);
        match url {
            Some(url) => {
                let destination = match &self.bib_dest {
                    Some(data) => data,
                    None => {
                        eprintln!("BIBLIOGRAPHY variable is empty. Check your .env file");
                        process::exit(1)
                    }
                };

                let result = match &self.z_api_key {
                    Some(key) => download::download(&url, &destination, Some(key)),
                    None => download::download(&url, &destination, None)
                };

                match result {
                    Ok(_) => (),
                    Err(mut e) => zotero::check_zotero_api_error(&mut e),
                }
            },
            None => {
                eprintln!("Not enough data to download your .bib file. Check your .env file");
                process::exit(1)
            }
        }
    }

    pub fn download_bibliography (&self) {
        match &self.bib_dest {
            Some(bib_dest) => {
                if !Path::new(bib_dest).exists() {
                    &self.download_bibliography_force();
                }
            },
            None => (),
        }
    }

    pub fn get_pandoc_args(&self) -> Option<Vec<&str>> {
        match (&self.bib_dest, &self.csl_file) {
            (Some(bib), Some(csl)) => {
                let mut config = vec![];
                config.append(&mut vec!["--filter", "pandoc-citeproc", "--bibliography", bib.as_ref()]);
                config.append(&mut vec!["--csl", csl.as_ref()]);
                Some(config)
            },
            _ => None
        }
    }
}

pub fn parse_csl_error (response: &mut reqwest::Response) {
    let text = response.text().unwrap();
    match text.as_ref() {
        "404: Not Found\n" => {
            eprintln!("Bad csl filename");
            process::exit(1)
        }
        _ => panic!("CSL : an error occurred"),
    }
}
