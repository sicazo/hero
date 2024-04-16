use quick_xml::{events::Event, reader::Reader, name::QName};

pub fn test() {
    let xml = r#"
    <data name="Label_Building_Location" xml:space="preserve">
    <value>Building Location</value>
  </data>
   <data name="Label_BackgroundColor" xml:space="preserve">
    <value>Background Color</value>
  </data>
  <data name="Label_Building_Area" xml:space="preserve">
    <value>Heatbale Living Area</value>
  </data>
  <data name="Label_Building_Floor" xml:space="preserve">
    <value>Floor</value>
  </data>
  <data name="Label_Building_Height" xml:space="preserve">
    <value>Building Height</value>
  </data>
  <data name="Label_Building_Location" xml:space="preserve">
    <value>Building Location</value>
  </data>
  <data name="Label_Building_PostalCode" xml:space="preserve">
    <value>Postal Code</value>
  </data>
  <data name="Label_Building_PostalCode_Help" xml:space="preserve">
    <value>Indicate where the building is located for which you want to receive an offer.</value>
  </data>
  <data name="Label_Building_Refurbishments" xml:space="preserve">
    <value>Additional Refurbishments</value>
  </data>
  <data name="Label_Building_Residents" xml:space="preserve">
    <value>Number of Residents</value>
  </data>
    "#;
    let mut reader = Reader::from_str(xml);
    reader.trim_text(true);

    let mut buf = Vec::new();
    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(ref e)) if e.name() == QName(b"data") => {
                // Iterate through attributes of the <data> tag
                for attr in e.attributes().filter_map(|a| a.ok()) {
                    if attr.key == QName(b"name") {
                        let name_value = attr.unescape_and_decode_value(&reader).unwrap();
                        println!("Name attribute: {}", name_value);  // Output the name attribute
                    }
                }
            }
            Ok(Event::Eof) => break, // exits the loop when reaching the end of the file
            Err(e) => {
                // Handle any errors that might occur
                println!("Error at position {}: {:?}", reader.buffer_position(), e);
                break;
            }
            _ => (), // Other events are ignored for this task
        }
    buf.clear();
    }
}
