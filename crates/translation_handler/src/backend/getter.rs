use std::fs::read_to_string;
use crate::backend::xml_reader::XmlReader;

pub fn get_translations_from_location(location_path: &str) {
    let xml = read_to_string(location_path).expect("failed to read file");
    let response = XmlReader::read_name_attributes_and_value_tags(&xml);
    println!("{:?}", response);
}

pub fn get_resources_from_csproj(path: &str) -> Option<String> {
    if !path.ends_with(".csproj") {
        return None
    }

    let response = XmlReader::get_resources(path);
    println!("{:?}", response);

    Some("test".to_string())

}
