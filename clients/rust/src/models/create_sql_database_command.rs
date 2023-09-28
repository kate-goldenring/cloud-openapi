/*
 * Fermyon Cloud API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CreateSqlDatabaseCommand {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "appId", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub app_id: Option<Option<uuid::Uuid>>,
    #[serde(rename = "label", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub label: Option<Option<String>>,
}

impl CreateSqlDatabaseCommand {
    pub fn new(name: String) -> CreateSqlDatabaseCommand {
        CreateSqlDatabaseCommand {
            name,
            app_id: None,
            label: None,
        }
    }
}


