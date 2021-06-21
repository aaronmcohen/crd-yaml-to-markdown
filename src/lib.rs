use std::error::Error;
use yaml_rust::Yaml;
use yaml_rust::YamlLoader;

fn open_api_to_table_row(
    prefix: &str,
    yaml: &yaml_rust::yaml::Hash,
) -> Result<String, Box<dyn Error>> {
    let mut markdown = String::new();
    for (key, value) in yaml {
        let label: String = match prefix {
            "" => key.as_str().unwrap().trim().to_string(),
            _ => {
                let key_string: &str = key.as_str().unwrap();
                format!("{}.{}", prefix, key_string)
            }
        };
        let value_type: &str = value.as_hash().unwrap()[&Yaml::String(String::from("type"))]
            .as_str()
            .ok_or("Could not fetch property type")?;
        match value_type {
            "object" => {
                match value
                    .as_hash()
                    .unwrap()
                    .get(&Yaml::String(String::from("description")))
                {
                    None => {}
                    Some(yaml) => {
                        let description: String =
                            yaml.as_str().ok_or("Could not fetch name")?.to_string();
                        let row: String = format!("| {} | {} |\n", label, description);
                        markdown.push_str(row.as_str());
                    }
                };
                match value
                    .as_hash()
                    .unwrap()
                    .get(&Yaml::String(String::from("properties")))
                {
                    None => {}
                    Some(_yaml) => {
                        let row = open_api_to_table_row(label.as_str(), _yaml.as_hash().unwrap())?;
                        markdown.push_str(row.as_str());
                    }
                };
            }
            "array" => {
                match value
                    .as_hash()
                    .unwrap()
                    .get(&Yaml::String(String::from("items")))
                {
                    None => {
                        let description: String = match value
                            .as_hash()
                            .unwrap()
                            .get(&Yaml::String(String::from("description")))
                        {
                            None => String::new(),
                            Some(yaml) => yaml.as_str().ok_or("Could not fetch name")?.to_string(),
                        };
                        let row: String = format!("| {} | {} |\n", label, description);
                        markdown.push_str(row.as_str());
                    }
                    Some(_yaml) => {
                        match _yaml
                            .as_hash()
                            .unwrap()
                            .get(&Yaml::String(String::from("properties")))
                        {
                            None => {
                                let description: String = match value
                                    .as_hash()
                                    .unwrap()
                                    .get(&Yaml::String(String::from("description")))
                                {
                                    None => String::new(),
                                    Some(yaml) => {
                                        yaml.as_str().ok_or("Could not fetch name")?.to_string()
                                    }
                                };
                                let row: String = format!("| {} | {} |\n", label, description);
                                markdown.push_str(row.as_str());
                            }
                            Some(_item) => {
                                let row = open_api_to_table_row(
                                    format!("{}[]", label).as_str(),
                                    _item.as_hash().unwrap(),
                                )?;
                                markdown.push_str(row.as_str());
                            }
                        };
                    }
                };
            }
            _ => {
                //                println!("{:?}", value);

                let description: String = match value
                    .as_hash()
                    .unwrap()
                    .get(&Yaml::String(String::from("description")))
                {
                    None => String::new(),
                    Some(yaml) => yaml.as_str().ok_or("Could not fetch name")?.to_string(),
                };
                let row: String = format!("| {} | {} |\n", label, description);
                markdown.push_str(row.as_str());
            }
        };
    }
    Ok(markdown)
}

pub fn yaml_to_markdown(content: &str) -> Result<String, Box<dyn Error>> {
    let mut markdown = String::new();
    let docs: Vec<Yaml> = match YamlLoader::load_from_str(content) {
        Ok(yaml_docs) => yaml_docs,
        Err(error) => return Err(Box::new(error)),
    };
    for doc in docs {
        let spec: &yaml_rust::yaml::Hash =
            doc["spec"].as_hash().ok_or("Could not fetch metadata")?;
        let group: String = spec[&Yaml::String(String::from("group"))]
            .as_str()
            .ok_or("Could not fetch group")?
            .to_string();
        let names: &yaml_rust::yaml::Hash = spec[&Yaml::String(String::from("names"))]
            .as_hash()
            .ok_or("Could not fetch metadata")?;
        let name: String = names[&Yaml::String(String::from("kind"))]
            .as_str()
            .ok_or("Could not fetch name")?
            .to_string();
        let versions: &yaml_rust::yaml::Array = spec[&Yaml::String(String::from("versions"))]
            .as_vec()
            .ok_or("Could not fetch versions")?;
        for version in versions {
            let version_hash = version.as_hash().unwrap();
            let version_name: String = version_hash[&Yaml::String(String::from("name"))]
                .as_str()
                .ok_or("Could not fetch version name")?
                .to_string();
            markdown.push_str(format!("### {} ({}/{})\n\n", name, group, version_name).as_str());

            let schema: &yaml_rust::yaml::Hash = match doc["apiVersion"]
                .as_str()
                .ok_or("Could not fetch apiVersion")?
            {
                "apiextensions.k8s.io/v1beta1" => spec[&Yaml::String(String::from("validation"))]
                    .as_hash()
                    .ok_or("Validation is required and not present.")?,

                _ => version_hash[&Yaml::String(String::from("schema"))]
                    .as_hash()
                    .ok_or("schema is required and not present.")?,
            };
            let open_api_v3_schema: &yaml_rust::yaml::Hash = schema
                [&Yaml::String(String::from("openAPIV3Schema"))]
                .as_hash()
                .ok_or("Validation is required and not present.")?;
            let desciption: String =
                match open_api_v3_schema.get(&Yaml::String(String::from("description"))) {
                    Some(desc) => String::from(desc.as_str().unwrap()),
                    None => String::new(),
                };
            markdown.push_str(desciption.as_str());
            markdown.push_str("| Name | Descrption |\n");
            markdown.push_str("| ---- | ---------- |\n");
            let open_api_v3_schema_properties: &yaml_rust::yaml::Hash = open_api_v3_schema
                [&Yaml::String(String::from("properties"))]
                .as_hash()
                .ok_or("OpenSchema properties is required and not present.")?;
            //Address scenario where top level kubernetes components are defined.
            let crd_properties: &yaml_rust::yaml::Hash = match open_api_v3_schema
                .get(&Yaml::String(String::from("spec")))
            {
                Some(yaml) => yaml.as_hash().unwrap()[&Yaml::String(String::from("properties"))]
                    .as_hash()
                    .ok_or("OpenSchema properties is required and not present.")?,
                None => open_api_v3_schema_properties,
            };
            let crd_spec: &yaml_rust::yaml::Hash = crd_properties
                [&Yaml::String(String::from("spec"))]
                .as_hash()
                .ok_or("OpenSchema spec is required and not present.")?;
            if crd_properties.contains_key(&Yaml::String(String::from("properties"))) {
                let crd_spec_properties: &yaml_rust::yaml::Hash = crd_spec
                    [&Yaml::String(String::from("properties"))]
                    .as_hash()
                    .ok_or("OpenSchema spec properties are required and not present.")?;
                let row_data: String = open_api_to_table_row("", crd_spec_properties)?;
                markdown.push_str(row_data.as_str());
            }
            if crd_properties.contains_key(&Yaml::String(String::from("status"))) {
                let crd_status: &yaml_rust::yaml::Hash = crd_properties
                    [&Yaml::String(String::from("status"))]
                    .as_hash()
                    .ok_or("OpenSchema status is required and not present.")?;
                let crd_status_properties: &yaml_rust::yaml::Hash = crd_status
                    [&Yaml::String(String::from("properties"))]
                    .as_hash()
                    .ok_or("OpenSchema status properties are required and not present.")?;
                markdown.push_str("\n\n#### Status");
                markdown.push_str("\n| Name | Descrption |\n");
                markdown.push_str("| ---- | ---------- |\n");
                let status_row_data: String = open_api_to_table_row("", crd_status_properties)?;
                markdown.push_str(status_row_data.as_str());
            }
        }
    }
    Ok(markdown)
}
