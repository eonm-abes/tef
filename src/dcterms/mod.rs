use getset::Getters;
use serde::{Deserialize, Serialize};

#[cfg(feature = "lax")]
use lax_derive::lax;

/// Résumé
/// dcterms:abstract
/// <http://purl.org/dc/terms/>
/// Résumé de la thèse
#[cfg_attr(feature = "lax", lax)]
#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Getters)]
#[getset(get = "pub")]
pub struct Abstract {
    #[serde(rename = "@lang")]
    lang: String,
    #[serde(rename = "$text")]
    text: String,
}

/// Titre et sous-titres traduits
/// dcterms:alternative
/// <http://purl.org/dc/terms/>
/// Traduction du titre et du sous-titre de la thèse
#[cfg_attr(feature = "lax", lax)]
#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Getters)]
#[getset(get = "pub")]
pub struct Alternative {
    #[serde(rename = "@lang")]
    lang: String,
    #[serde(rename = "$text")]
    text: String,
}

/// Date de soutenance
/// dcterms:dateAccepted
/// <http://purl.org/dc/terms/>
/// Date de soutenance de la thèse
#[cfg_attr(feature = "lax", lax)]
#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Getters)]
#[getset(get = "pub")]
pub struct DateAccepted {
    #[serde(rename = "@type")]
    r#type: String,
    #[serde(rename = "$text")]
    text: String,
}

/// Taille de fichier
/// dcterms:extent
/// <http://purl.org/dc/terms/>
/// Taille de fichier informatique en octets
#[cfg_attr(feature = "lax", lax)]
#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq)]
pub struct Extent(#[serde(rename = "$text")] String);

/// Date de publication
/// dcterms:issued
/// <http://purl.org/dc/terms/>
/// Date de publication d'une édition de la thèse
#[cfg_attr(feature = "lax", lax)]
#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Getters)]
#[getset(get = "pub")]
pub struct Issued {
    #[serde(rename = "@type")]
    r#type: String,
    #[serde(rename = "$text")]
    text: String,
}

/// Type de fichier
/// dcterms:medium
/// <http://purl.org/dc/terms/>
/// Type de fichier informatique
#[cfg_attr(feature = "lax", lax)]
#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Getters)]
#[getset(get = "pub")]
pub struct Medium {
    #[serde(rename = "@type")]
    r#type: String,
    #[serde(rename = "$text")]
    text: String,
}

/// Remplace
/// dcterms:replaces
/// <http://purl.org/dc/terms/>
/// Renvoie à une autre ressource que la ressource décrite remplace ou à laquelle elle succède
#[cfg_attr(feature = "lax", lax)]
#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Getters)]
#[getset(get = "pub")]
pub struct Replaces {
    #[serde(rename = "@type")]
    r#type: String,
    #[serde(rename = "$text")]
    text: String,
}

/// Couverture spatiale du sujet
/// dcterms:spatial
/// <http://purl.org/dc/terms/>
/// Couverture spatiale du sujet traité dans la thèse
#[cfg_attr(feature = "lax", lax)]
#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Getters)]
#[getset(get = "pub")]
pub struct Spatial {
    #[serde(rename = "@type")]
    r#type: Option<String>,
    #[serde(rename = "@lang")]
    lang: Option<String>,
    #[serde(rename = "$text")]
    text: String,
}

/// Table des matières
/// dcterms:tableOfContents
/// <http://purl.org/dc/terms/>
/// Table des matières de la thèse
#[cfg_attr(feature = "lax", lax)]
#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Getters)]
#[getset(get = "pub")]
pub struct TableOfContents {
    #[serde(rename = "@lang")]
    lang: String,
    #[serde(rename = "$text")]
    text: String,
}

/// Couverture temporelle du sujet
/// dcterms:temporal
/// <http://purl.org/dc/terms/>
/// Couverture temporelle du sujet traité dans la thèse
#[cfg_attr(feature = "lax", lax)]
#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Getters)]
#[getset(get = "pub")]
pub struct Temporal {
    #[serde(rename = "@type")]
    r#type: Option<String>,
    #[serde(rename = "@lang")]
    lang: String,
    #[serde(rename = "$text")]
    text: String,
}
