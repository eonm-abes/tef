use getset::Getters;
use serde::{Deserialize, Serialize};

#[cfg(feature = "lax")]
use lax_derive::lax;

/// Description de personne
/// mads:description
/// http://www.loc.gov/mads/
/// Description d'une personne en texte libre
#[cfg_attr(feature = "lax", lax)]
#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Getters)]
#[getset(get = "pub")]
pub struct Description {
    #[serde(rename = "$value")]
    value: String,
}

/// Partie de nom
/// mads:namePart
/// http://www.loc.gov/mads/
/// Partie du nom complet d'une personne
#[cfg_attr(feature = "lax", lax)]
#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Getters)]
#[getset(get = "pub")]
pub struct NamePart {
    #[serde(rename = "@type")]
    r#type: String,
    #[serde(rename = "$value")]
    value: String,
}
