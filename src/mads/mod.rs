use serde::Deserialize;
use lax_derive::lax;
use getset::Getters;

/// Description de personne
/// mads:description
/// http://www.loc.gov/mads/
/// Description d'une personne en texte libre
#[lax]
#[derive(Debug, Clone, Deserialize, Default, Getters)]
#[getset(get="pub")]
pub struct Description {
    #[serde(rename = "$value")]
    value: String
}

/// Partie de nom
/// mads:namePart
/// http://www.loc.gov/mads/
/// Partie du nom complet d'une personne
#[lax]
#[derive(Debug, Clone, Deserialize, Default, Getters)]
#[getset(get="pub")]
pub struct NamePart {
    #[serde(rename = "@type")]
    r#type: String,
    #[serde(rename = "$value")]
    value: String
}