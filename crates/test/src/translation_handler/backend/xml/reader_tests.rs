use translation_handler::backend::xml::XmlHandler;
use std::collections::BTreeMap;

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

    let response = XmlHandler::read_name_attributes_and_value_tags(xml);

    assert_eq!(2, response.len());
    //*
    //assert_eq!(
    //   Some(&"Building Location".to_string()),
    //response.get("Label_Building_Location")
    //);
    //assert_eq!(
    //Some(&"Background Color".to_string()),
    //response.get("Label_BackgroundColor")
    //);
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

    let response = XmlHandler::get_resources(xml, "");
    assert_eq!(3, response.len());
    
    // With empty path, the resources should be returned as-is without parent path
    assert_eq!("Quotes\\QuoteResources.en-US.resx", response[0]);
    assert_eq!("Quotes\\QuoteResources.resx", response[1]);
    assert_eq!("Sms\\SmsTextResources.resx", response[2]);
}