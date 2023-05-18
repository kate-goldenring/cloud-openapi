/*
 * Fermyon.Cloud.Web
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0
 * 
 * Generated by: https://openapi-generator.tech
 */


/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum FieldAttributes {
    #[serde(rename = "PrivateScope")]
    PrivateScope,
    #[serde(rename = "Private")]
    Private,
    #[serde(rename = "FamANDAssem")]
    FamAndAssem,
    #[serde(rename = "Assembly")]
    Assembly,
    #[serde(rename = "Family")]
    Family,
    #[serde(rename = "FamORAssem")]
    FamOrAssem,
    #[serde(rename = "Public")]
    Public,
    #[serde(rename = "FieldAccessMask")]
    FieldAccessMask,
    #[serde(rename = "Static")]
    Static,
    #[serde(rename = "InitOnly")]
    InitOnly,
    #[serde(rename = "Literal")]
    Literal,
    #[serde(rename = "NotSerialized")]
    NotSerialized,
    #[serde(rename = "HasFieldRVA")]
    HasFieldRva,
    #[serde(rename = "SpecialName")]
    SpecialName,
    #[serde(rename = "RTSpecialName")]
    RtSpecialName,
    #[serde(rename = "HasFieldMarshal")]
    HasFieldMarshal,
    #[serde(rename = "PinvokeImpl")]
    PinvokeImpl,
    #[serde(rename = "HasDefault")]
    HasDefault,
    #[serde(rename = "ReservedMask")]
    ReservedMask,

}

impl ToString for FieldAttributes {
    fn to_string(&self) -> String {
        match self {
            Self::PrivateScope => String::from("PrivateScope"),
            Self::Private => String::from("Private"),
            Self::FamAndAssem => String::from("FamANDAssem"),
            Self::Assembly => String::from("Assembly"),
            Self::Family => String::from("Family"),
            Self::FamOrAssem => String::from("FamORAssem"),
            Self::Public => String::from("Public"),
            Self::FieldAccessMask => String::from("FieldAccessMask"),
            Self::Static => String::from("Static"),
            Self::InitOnly => String::from("InitOnly"),
            Self::Literal => String::from("Literal"),
            Self::NotSerialized => String::from("NotSerialized"),
            Self::HasFieldRva => String::from("HasFieldRVA"),
            Self::SpecialName => String::from("SpecialName"),
            Self::RtSpecialName => String::from("RTSpecialName"),
            Self::HasFieldMarshal => String::from("HasFieldMarshal"),
            Self::PinvokeImpl => String::from("PinvokeImpl"),
            Self::HasDefault => String::from("HasDefault"),
            Self::ReservedMask => String::from("ReservedMask"),
        }
    }
}

impl Default for FieldAttributes {
    fn default() -> FieldAttributes {
        Self::PrivateScope
    }
}



