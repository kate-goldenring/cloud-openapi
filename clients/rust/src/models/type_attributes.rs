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
pub enum TypeAttributes {
    #[serde(rename = "NotPublic")]
    NotPublic,
    #[serde(rename = "Public")]
    Public,
    #[serde(rename = "NestedPublic")]
    NestedPublic,
    #[serde(rename = "NestedPrivate")]
    NestedPrivate,
    #[serde(rename = "NestedFamily")]
    NestedFamily,
    #[serde(rename = "NestedAssembly")]
    NestedAssembly,
    #[serde(rename = "NestedFamANDAssem")]
    NestedFamAndAssem,
    #[serde(rename = "NestedFamORAssem")]
    NestedFamOrAssem,
    #[serde(rename = "SequentialLayout")]
    SequentialLayout,
    #[serde(rename = "ExplicitLayout")]
    ExplicitLayout,
    #[serde(rename = "LayoutMask")]
    LayoutMask,
    #[serde(rename = "Interface")]
    Interface,
    #[serde(rename = "Abstract")]
    Abstract,
    #[serde(rename = "Sealed")]
    Sealed,
    #[serde(rename = "SpecialName")]
    SpecialName,
    #[serde(rename = "RTSpecialName")]
    RtSpecialName,
    #[serde(rename = "Import")]
    Import,
    #[serde(rename = "Serializable")]
    Serializable,
    #[serde(rename = "WindowsRuntime")]
    WindowsRuntime,
    #[serde(rename = "UnicodeClass")]
    UnicodeClass,
    #[serde(rename = "AutoClass")]
    AutoClass,
    #[serde(rename = "CustomFormatClass")]
    CustomFormatClass,
    #[serde(rename = "HasSecurity")]
    HasSecurity,
    #[serde(rename = "ReservedMask")]
    ReservedMask,
    #[serde(rename = "BeforeFieldInit")]
    BeforeFieldInit,
    #[serde(rename = "CustomFormatMask")]
    CustomFormatMask,

}

impl ToString for TypeAttributes {
    fn to_string(&self) -> String {
        match self {
            Self::NotPublic => String::from("NotPublic"),
            Self::Public => String::from("Public"),
            Self::NestedPublic => String::from("NestedPublic"),
            Self::NestedPrivate => String::from("NestedPrivate"),
            Self::NestedFamily => String::from("NestedFamily"),
            Self::NestedAssembly => String::from("NestedAssembly"),
            Self::NestedFamAndAssem => String::from("NestedFamANDAssem"),
            Self::NestedFamOrAssem => String::from("NestedFamORAssem"),
            Self::SequentialLayout => String::from("SequentialLayout"),
            Self::ExplicitLayout => String::from("ExplicitLayout"),
            Self::LayoutMask => String::from("LayoutMask"),
            Self::Interface => String::from("Interface"),
            Self::Abstract => String::from("Abstract"),
            Self::Sealed => String::from("Sealed"),
            Self::SpecialName => String::from("SpecialName"),
            Self::RtSpecialName => String::from("RTSpecialName"),
            Self::Import => String::from("Import"),
            Self::Serializable => String::from("Serializable"),
            Self::WindowsRuntime => String::from("WindowsRuntime"),
            Self::UnicodeClass => String::from("UnicodeClass"),
            Self::AutoClass => String::from("AutoClass"),
            Self::CustomFormatClass => String::from("CustomFormatClass"),
            Self::HasSecurity => String::from("HasSecurity"),
            Self::ReservedMask => String::from("ReservedMask"),
            Self::BeforeFieldInit => String::from("BeforeFieldInit"),
            Self::CustomFormatMask => String::from("CustomFormatMask"),
        }
    }
}

impl Default for TypeAttributes {
    fn default() -> TypeAttributes {
        Self::NotPublic
    }
}




