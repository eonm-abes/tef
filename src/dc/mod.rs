use getset::Getters;
use serde::{Deserialize, Serialize};

#[cfg(feature = "lax")]
use lax_derive::lax;

/// Couverture spatiale ou temporelle
/// dc:coverage
/// <http://purl.org/dc/elements/1.1/>
/// Ensemble des métadonnées relatives au périmètre ou au domaine d'application du contenu de la ressource
#[cfg_attr(feature = "lax", lax)]
#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq)]
pub struct Coverage(#[serde(rename = "$text")] String);

/// Identifiant
/// dc:identifier
/// <http://purl.org/dc/elements/1.1/>
/// Identifiant unique
#[cfg_attr(feature = "lax", lax)]
#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Getters)]
#[getset(get = "pub")]
pub struct Identifier {
    #[serde(rename = "@type")]
    r#type: String,
    #[serde(rename = "$text")]
    text: String,
}

/// Langue
/// dc:language
/// <http://purl.org/dc/elements/1.1/>
/// Langue de la thèse
#[cfg_attr(feature = "lax", lax)]
#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Getters)]
#[getset(get = "pub")]
pub struct Language {
    #[serde(rename = "@type")]
    r#type: String,
    #[serde(rename = "$text")]
    text: String,
}

/// Sujet
/// dc:subject
/// <http://purl.org/dc/elements/1.1/>
/// Sujet, discipline et/ou mots-clés attribués à la thèse
#[cfg_attr(feature = "lax", lax)]
#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Getters)]
#[getset(get = "pub")]
pub struct Subject {
    #[serde(rename = "@lang")]
    lang: Option<String>,
    #[serde(rename = "@type")]
    r#type: Option<String>,
    #[serde(rename = "$text")]
    text: String,
}

/// Titre et sous-titre
/// dc:title
/// <http://purl.org/dc/elements/1.1/>
/// Titre propre de la thèse et son sous-titre
#[cfg_attr(feature = "lax", lax)]
#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Getters)]
#[getset(get = "pub")]
pub struct Title {
    #[serde(rename = "@lang")]
    lang: String,
    #[serde(rename = "@type")]
    r#type: Option<String>,
    #[serde(rename = "$text")]
    text: String,
}

/// Type
/// dc:type
/// <http://purl.org/dc/elements/1.1/>
/// Type
#[cfg_attr(feature = "lax", lax)]
#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Getters)]
#[getset(get = "pub")]
pub struct Type {
    #[serde(rename = "@type")]
    r#type: Option<String>,
    #[serde(rename = "$text")]
    text: String,
}
