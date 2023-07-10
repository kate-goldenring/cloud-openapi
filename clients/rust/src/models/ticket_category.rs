/*
 * Fermyon Cloud API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0
 * 
 * Generated by: https://openapi-generator.tech
 */


/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TicketCategory {
    #[serde(rename = "TechnicalHelp")]
    TechnicalHelp,
    #[serde(rename = "BillingHelp")]
    BillingHelp,
    #[serde(rename = "SomethingElse")]
    SomethingElse,

}

impl ToString for TicketCategory {
    fn to_string(&self) -> String {
        match self {
            Self::TechnicalHelp => String::from("TechnicalHelp"),
            Self::BillingHelp => String::from("BillingHelp"),
            Self::SomethingElse => String::from("SomethingElse"),
        }
    }
}

impl Default for TicketCategory {
    fn default() -> TicketCategory {
        Self::TechnicalHelp
    }
}




