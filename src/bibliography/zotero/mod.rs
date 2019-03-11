use crate::bibliography::Bibliography;

pub fn create_zotero_url(bib_data: &Bibliography) -> Option<String> {
    match (
        &bib_data.z_user_id,
        &bib_data.z_api_key,
        &bib_data.z_collection,
    ) {
        (Some(user_id), Some(_api_key), Some(coll_id)) => {
            return Some(format!(
                "https://api.zotero.org/users/{}/collections/{}/items?format=biblatex",
                user_id, coll_id
            ));
        }
        (Some(user_id), Some(_api_key), None) => {
            return Some(format!(
                "https://api.zotero.org/users/{}/items?format=biblatex",
                user_id
            ));
        }
        _ => (),
    };

    match (&bib_data.z_group_id, &bib_data.z_group_collection) {
        (Some(z_group_id), Some(z_group_collection)) => {
            return Some(format!(
                "https://api.zotero.org/users/{}/collections/{}/items?format=biblatex",
                z_group_id, z_group_collection
            ));
        }
        (Some(z_group_id), None) => {
            return Some(format!(
                "https://api.zotero.org/users/{}/items?format=biblatex",
                z_group_id
            ));
        }
        _ => (),
    };

    None
}

pub fn check_zotero_api_error(response: &mut reqwest::Response) {
    let text = response.text().expect("No message from zotero API");
    match text.as_ref() {
        "Collection not found" => error!("Collection not found"),
        "An error occurred" => error!("An error occurred"),
        "Invalid key" => error!("Invalid Zotero api key"),
        "Forbidden" => error!("Forbidden zotero access"),
        _ => panic!("Zotero : an error occurred"),
    }
}
