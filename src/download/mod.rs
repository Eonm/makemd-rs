extern crate reqwest;
extern crate tempdir;

use std::path::Path;
use std::io::copy;
use std::fs::File;
use tempdir::TempDir;
use std::fs;

pub fn download(url:&str, filename:&str, api_key:Option<&str>) -> Result<(),reqwest::Response>{
    let tmp_dir = TempDir::new("example").unwrap();
    let target = url;

    let client = reqwest::Client::new();

    let mut response = match api_key {
        Some(key) => {
         client.get(target).header("Zotero-API-Key", key).send().expect("failed to download data")
        },
        None => client.get(target).send().expect("failed to download data")
    };

    if !&response.status().is_success() {
        return Err(response)
    }

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
    fs::copy(dest, filename).expect("failed to copy file");
    Ok(())
}

pub fn download_dont_replace(url:&str, filename:&str, api_key:Option<&str>) -> Result <(),reqwest::Response> {
    if !Path::new(filename).exists() {
        download(url, filename, api_key)
    } else {
        Ok(())
    }
}
