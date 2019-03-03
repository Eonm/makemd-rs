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
        error!("{}", String::from_utf8_lossy(&pandoc.stderr));
    }
}

pub fn create_pandoc_config(data: &Document, input_file: &str)-> Vec<String> {

    let input_files : Vec<String> = if input_file.ends_with(".md") {
        vec![input_file.to_string()]
    } else {
        data.inputs.clone()
    };

    let output_filename = util::get_filename(input_file); //withouth ext

    match &data.document_type {
        DocumentType::PDF => {
            let output = format!("{}{}.pdf", data.output, output_filename);
            create_pandoc_args(data.clone(), input_files, &output, None)
        }
        DocumentType::PRESENTATION => {
            let output = format!("{}{}.html", data.output, output_filename);
            create_pandoc_args(data.clone(), input_files, &output, Some(vec!["-t", "revealjs", "-V", "revealjs-url=./reveal.js-master"]))
        }
    }
}

pub fn create_pandoc_args(data: &Document, inputs: Vec<String>, output: &str, extra_args:Option<Vec<&str>>) -> Vec<String> {
    let mut config = vec!();
    config.append(&mut vec!("--toc", "-N", "-s"));
    config.append(&mut inputs.iter().map(|x| x.as_ref()).collect());
    config.push(&data.config);
    config.append(&mut vec!["-o", output]);

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
