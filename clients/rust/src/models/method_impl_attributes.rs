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
pub enum MethodImplAttributes {
    #[serde(rename = "IL")]
    Il,
    #[serde(rename = "Native")]
    Native,
    #[serde(rename = "OPTIL")]
    Optil,
    #[serde(rename = "Runtime")]
    Runtime,
    #[serde(rename = "Unmanaged")]
    Unmanaged,
    #[serde(rename = "NoInlining")]
    NoInlining,
    #[serde(rename = "ForwardRef")]
    ForwardRef,
    #[serde(rename = "Synchronized")]
    Synchronized,
    #[serde(rename = "NoOptimization")]
    NoOptimization,
    #[serde(rename = "PreserveSig")]
    PreserveSig,
    #[serde(rename = "AggressiveInlining")]
    AggressiveInlining,
    #[serde(rename = "AggressiveOptimization")]
    AggressiveOptimization,
    #[serde(rename = "InternalCall")]
    InternalCall,
    #[serde(rename = "MaxMethodImplVal")]
    MaxMethodImplVal,

}

impl ToString for MethodImplAttributes {
    fn to_string(&self) -> String {
        match self {
            Self::Il => String::from("IL"),
            Self::Native => String::from("Native"),
            Self::Optil => String::from("OPTIL"),
            Self::Runtime => String::from("Runtime"),
            Self::Unmanaged => String::from("Unmanaged"),
            Self::NoInlining => String::from("NoInlining"),
            Self::ForwardRef => String::from("ForwardRef"),
            Self::Synchronized => String::from("Synchronized"),
            Self::NoOptimization => String::from("NoOptimization"),
            Self::PreserveSig => String::from("PreserveSig"),
            Self::AggressiveInlining => String::from("AggressiveInlining"),
            Self::AggressiveOptimization => String::from("AggressiveOptimization"),
            Self::InternalCall => String::from("InternalCall"),
            Self::MaxMethodImplVal => String::from("MaxMethodImplVal"),
        }
    }
}

impl Default for MethodImplAttributes {
    fn default() -> MethodImplAttributes {
        Self::Il
    }
}



