/*
 * Galaxy API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 22.05.1
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DatatypesMap {
    /// Dictionary mapping datatype's classes with their base classes
    #[serde(rename = "class_to_classes")]
    pub class_to_classes: ::std::collections::HashMap<String, ::std::collections::HashMap<String, bool>>,
    /// Dictionary mapping datatype's extensions with implementation classes
    #[serde(rename = "ext_to_class_name")]
    pub ext_to_class_name: ::std::collections::HashMap<String, String>,
}

impl DatatypesMap {
    pub fn new(class_to_classes: ::std::collections::HashMap<String, ::std::collections::HashMap<String, bool>>, ext_to_class_name: ::std::collections::HashMap<String, String>) -> DatatypesMap {
        DatatypesMap {
            class_to_classes,
            ext_to_class_name,
        }
    }
}


