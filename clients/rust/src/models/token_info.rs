/*
 * Fermyon.Cloud.Web
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TokenInfo {
    #[serde(rename = "token")]
    pub token: String,
    #[serde(rename = "refreshToken")]
    pub refresh_token: String,
    #[serde(rename = "expiration")]
    pub expiration: String,
}

impl TokenInfo {
    pub fn new(token: String, refresh_token: String, expiration: String) -> TokenInfo {
        TokenInfo {
            token,
            refresh_token,
            expiration,
        }
    }
}


