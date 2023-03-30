use serde::Deserialize;

/// Description de personne
/// mads:description
/// http://www.loc.gov/mads/
/// Description d'une personne en texte libre
#[derive(Debug, Clone, Deserialize)]
pub struct Description {
    #[serde(rename = "$value")]
    value: String
}

/// Partie de nom
/// mads:namePart
/// http://www.loc.gov/mads/
/// Partie du nom complet d'une personne
#[derive(Debug, Clone, Deserialize)]
pub struct NamePart {
    #[serde(rename = "@type")]
    r#type: String,
    #[serde(rename = "$value")]
    value: String
}