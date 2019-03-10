use std::process::Command;
use crate::build::Document;
use crate::build::DocumentType;
use crate::util;


pub fn pandoc(args: Vec<String>) {
    let pandoc = Command::new("pandoc")
        .args(args)
        .output()
        .expect("failed to execute pandoc");

    if !pandoc.stderr.is_empty() {
        panic!("{}", String::from_utf8_lossy(&pandoc.stderr));
    }
}

pub fn create_pandoc_config(data: &Document, input: &str)-> Vec<String> {
    match &data.document_type {
        DocumentType::PDF => {
            create_pandoc_args(&data.clone(), input, Some(vec!["--toc"]))
        }
        DocumentType::PRESENTATION => {
            create_pandoc_args(&data.clone(), input, Some(vec!["-t", "revealjs", "-V", "revealjs-url=./reveal.js-master"]))
        }
    }
}

fn file_output_name(data: &Document, input: &str) -> String {
    let filename = util::get_filename(input).unwrap_or("no_filename".to_string());

    match data.document_type {
        DocumentType::PDF => format!("{}{}.pdf", data.output, filename),
        DocumentType::PRESENTATION => format!("{}{}.html", data.output, filename)
    }
}

pub fn create_pandoc_args(data: &Document, input: &str, extra_args:Option<Vec<&str>>) -> Vec<String> {
    let mut config :Vec<&str> = vec!();
    config.append(&mut vec!("-s"));

    if input.ends_with(".yml") {
        config.append(&mut data.inputs.iter().filter(|input| input.ends_with(".md")).map(|x| x.as_ref()).collect());
        config.push(&data.config);
    } else {
        config.push(input);
    }

    let output_name = &file_output_name(&data, input);
    config.append(&mut vec!["-o", output_name]);

    match extra_args {
        Some(mut args) => config.append(&mut args),
        None => ()
    }

    match data.bibliography.get_pandoc_args() {
        Some(bib) => config.append(&mut bib.clone()),
        None => ()
    };

    config.iter().map(|elem| elem.to_string()).collect::<Vec<String>>()
}

//-----------------------------------------------------------------------------
// Test
//-----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use crate::build;
    use crate::bibliography;
    use super::*;

    #[test]
    fn creating_pandoc_args_no_bib() {
        let document_struct = build::Document {
            document_type: DocumentType::PDF,
            inputs: vec!("md.md".to_string()),
            output: "pdf/".to_string(),
            config: "pdf.yml".to_string(),
            bibliography: bibliography::Bibliography {
                csl_file: None,
                csl_style: None,
                bib_dest: None,
                bib_src: None,
                z_user_id: None,
                z_group_id: None,
                z_api_key: None,
                z_collection: None,
                z_group_collection: None
            }
        };
        let pandoc_args = create_pandoc_args(&document_struct, &document_struct.config, None);
        assert_eq!(pandoc_args, vec!("-s", "md.md", "pdf.yml", "-o", "pdf/no_filename.pdf"))
    }

    #[test]
    fn creating_pandoc_args_bib() {
        let document_struct = build::Document {
            document_type: DocumentType::PDF,
            inputs: vec!("md.md".to_string()),
            output: "pdf/".to_string(),
            config: "pdf.yml".to_string(),
            bibliography: bibliography::Bibliography {
                csl_file: Some("style.bib".to_string()),
                csl_style: None,
                bib_dest: Some("bib.bib".to_string()),
                bib_src: None,
                z_user_id: None,
                z_group_id: None,
                z_api_key: None,
                z_collection: None,
                z_group_collection: None
            }
        };
        let pandoc_args = create_pandoc_args(&document_struct, &document_struct.config, None);
        assert_eq!(pandoc_args, vec!("-s", "md.md", "pdf.yml", "-o", "pdf/no_filename.pdf", "--filter", "pandoc-citeproc", "--bibliography", "bib.bib", "--csl", "style.bib"))
    }

    #[test]
    fn creating_pandoc_args_incomplet_bib() {
        let document_struct = build::Document {
            document_type: DocumentType::PDF,
            inputs: vec!("md.md".to_string()),
            output: "pdf/".to_string(),
            config: "pdf.yml".to_string(),
            bibliography: bibliography::Bibliography {
                csl_file: Some("style.bib".to_string()),
                csl_style: None,
                bib_dest: None,
                bib_src: None,
                z_user_id: None,
                z_group_id: None,
                z_api_key: None,
                z_collection: None,
                z_group_collection: None
            }
        };
        let pandoc_args = create_pandoc_args(&document_struct, &document_struct.config, None);
        assert_eq!(pandoc_args, vec!("-s", "md.md", "pdf.yml", "-o", "pdf/no_filename.pdf"))
    }

    #[test]
    fn creating_pandoc_args_incomplet_csl() {
        let document_struct = build::Document {
            document_type: DocumentType::PDF,
            inputs: vec!("md.md".to_string()),
            output: "pdf/".to_string(),
            config: "pdf.yml".to_string(),
            bibliography: bibliography::Bibliography {
                csl_file: None,
                csl_style: None,
                bib_dest: Some("bib.bib".to_string()),
                bib_src: None,
                z_user_id: None,
                z_group_id: None,
                z_api_key: None,
                z_collection: None,
                z_group_collection: None
            }
        };
        let pandoc_args = create_pandoc_args(&document_struct, &document_struct.config, None);
        assert_eq!(pandoc_args, vec!("-s", "md.md", "pdf.yml", "-o", "pdf/no_filename.pdf"))
    }

    #[test]
    fn creating_pandoc_extra_args() {
        let document_struct = build::Document {
            document_type: DocumentType::PDF,
            inputs: vec!("md.md".to_string()),
            output: "pdf/".to_string(),
            config: "pdf.yml".to_string(),
            bibliography: bibliography::Bibliography {
                csl_file: Some("style.bib".to_string()),
                csl_style: None,
                bib_dest: Some("bib.bib".to_string()),
                bib_src: None,
                z_user_id: None,
                z_group_id: None,
                z_api_key: None,
                z_collection: None,
                z_group_collection: None
            }
        };
        let extra_args = Some(vec!("extra", "args"));
        let pandoc_args = create_pandoc_args(&document_struct, &document_struct.config, extra_args);
        assert_eq!(pandoc_args, vec!("-s", "md.md", "pdf.yml", "-o", "pdf/no_filename.pdf", "extra", "args", "--filter", "pandoc-citeproc", "--bibliography", "bib.bib", "--csl", "style.bib"))
    }
}
