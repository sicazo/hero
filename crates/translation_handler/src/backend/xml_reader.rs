use std::collections::BTreeMap;
use quick_xml::events::Event;
use quick_xml::name::QName;
use quick_xml::Reader;

pub struct XmlReader;



impl XmlReader {
    pub fn read_name_attributes_and_value_tags(input_string: &str) -> BTreeMap<String, String> {
        let mut reader = Reader::from_str(input_string);
        reader.trim_text(true);
        let mut return_values: BTreeMap<String, String> = BTreeMap::new();

        let mut buf = Vec::new();
        let mut inside_data = false;
        let mut attribute_name = String::new();
        let mut inside_name = false;

        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(ref e)) if e.name() == QName(b"data") => {
                    for attr in e.attributes().filter_map(|a| a.ok()) {
                        if attr.key == QName(b"name") {
                            let name_value = attr.decode_and_unescape_value(&reader).unwrap();
                            attribute_name = name_value.to_string();
                            inside_data = true;
                        }
                    }
                }
                Ok(Event::Start(ref e)) if e.name() == QName(b"value") && inside_data => {
                    inside_name = true;
                }
                Ok(Event::End(ref e)) if e.name() == QName(b"value") && inside_data => {
                    inside_name = false;
                }
                Ok(Event::End(ref e)) if e.name() == QName(b"data") => {
                    inside_data = false;
                }
                Ok(Event::Text(e)) if inside_name => {
                    let value_content = e.unescape().unwrap();
                    return_values.insert(attribute_name, value_content.to_string());
                    attribute_name = String::new();

                }
                Ok(Event::Eof) => break,
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
            buf.clear();
        }
        return_values

    }

    pub fn get_resources(input_string: &str) -> Vec<String> {
        let mut resources: Vec<String> = Vec::new();
        let mut reader = Reader::from_str(input_string);
        reader.trim_text(true);

        let mut buf = Vec::new();

        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(ref e)) if e.name() == QName(b"EmbeddedResource") => {
                    for attr in e.attributes().filter_map(|a| a.ok()) {
                        if attr.key == QName(b"Update") {
                            let name_value = attr.decode_and_unescape_value(&reader).unwrap();
                            resources.push(name_value.to_string());
                        }
                    }
                }
                Ok(Event::Eof) => break,
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
            buf.clear();
        }


        resources
    }
}




#[cfg(test)]
mod tests {
    #[test]
    fn name_attributes_and_value_tags_are_read_correctly() {
        let xml = r#"
            <data name="Label_Building_Location" xml:space="preserve">
                <value>Building Location</value>
            </data>
            <data name="Label_BackgroundColor" xml:space="preserve">
                <value>Background Color</value>
            </data>
            "#;

        let response = super::XmlReader::read_name_attributes_and_value_tags(xml);

        assert_eq!(2, response.len());
        assert_eq!(Some(&"Building Location".to_string()), response.get("Label_Building_Location"));
        assert_eq!(Some(&"Background Color".to_string()), response.get("Label_BackgroundColor"));
        assert_eq!(None, response.get("Wrong Input"));
    }

    #[test]
    fn get_resources() {
        let xml = r#"
        <EmbeddedResource Update="Quotes\QuoteResources.en-US.resx">
            <LastGenOutput>QuoteResources.en-US.Designer.cs</LastGenOutput>
            <Generator>PublicResXFileCodeGenerator</Generator>
        </EmbeddedResource>
        <EmbeddedResource Update="Quotes\QuoteResources.resx">
            <Generator>PublicResXFileCodeGenerator</Generator>
            <LastGenOutput>QuoteResources.Designer.cs</LastGenOutput>
        </EmbeddedResource>
        <EmbeddedResource Update="Sms\SmsTextResources.resx">
            <Generator>PublicResXFileCodeGenerator</Generator>
            <LastGenOutput>SmsTextResources.Designer.cs</LastGenOutput>
        </EmbeddedResource>
        "#;

        let response = super::XmlReader::get_resources(xml);
        assert_eq!(3, response.len());
        assert_eq!(r#"Quotes\QuoteResources.en-US.resx"#, response[0]);
        assert_eq!(r#"Quotes\QuoteResources.resx"#, response[1]);
        assert_eq!(r#"Sms\SmsTextResources.resx"#, response[2]);
    }
}

