use crate::backend::xml_reader::XmlReader;
use std::collections::BTreeMap;
use std::fs::read_to_string;
use std::path::Path;

pub fn get_translations_from_location(location_path: &str) -> BTreeMap<String, String> {
    println!("Reading location at:    {}", location_path);
    let xml = read_to_string(location_path).expect("failed to read file");
    let response = XmlReader::read_name_attributes_and_value_tags(&xml);
    response
}

pub fn get_resources_from_csproj(path: &str) -> Option<Vec<String>> {
    if !path.ends_with(".csproj") {
        return None;
    }
    let xml = read_to_string(path.clone()).expect("failed to read file");
    let mut response = XmlReader::get_resources(&xml, path);
    response.retain(|file| {
        let path = std::path::Path::new(file);
        let filename = path
            .file_stem()
            .and_then(|name| name.to_str())
            .unwrap_or("");
        path.extension().map_or(false, |ext| ext == "resx")
            && !filename.contains('.')
            && path.exists()
    });
    Some(response)
}
