extern crate reqwest;
extern crate tempdir;

use std::path::Path;
use std::io::copy;
use std::fs::File;
use tempdir::TempDir;
use std::fs;
use std::process;

pub fn download(url:&str, filename:&str, api_key:Option<&str>) -> Result<(),reqwest::Response>{
    let tmp_dir = TempDir::new("example").expect("Failed to create a temp dir");
    let target = url;

    let client = reqwest::Client::new();

    let raw_response = match api_key {
        Some(key) => {
         client.get(target).header("Zotero-API-Key", key).send()
        },
        None => client.get(target).send()
    };

    let mut response = match raw_response {
        Ok(resp) => {
            if !&resp.status().is_success() {
                return Err(resp)
            } else {
                resp
            }
        },
        Err(e) => {
            eprintln!("Failed to download file : {}", e);
            process::exit(1)
        }
    };

    let dest = {
        let fname = response
            .url()
            .path_segments()
            .and_then(|segments| segments.last())
            .and_then(|name| if name.is_empty() { None } else { Some(name) })
            .unwrap_or("tmp.bin");

        let fname = tmp_dir.path().join(fname);
        fname
    };
    let mut tpm_file = File::create(&dest).unwrap();
    copy(&mut response, &mut tpm_file).unwrap();
    fs::copy(dest, filename).expect("Failed to copy file");
    Ok(())
}

pub fn download_dont_replace(url:&str, filename:&str, api_key:Option<&str>) -> Result <(),reqwest::Response> {
    if !Path::new(filename).exists() {
        download(url, filename, api_key)
    } else {
        Ok(())
    }
}
