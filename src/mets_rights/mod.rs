use serde::Deserialize;
use lax_derive::lax;

/// Descritpion d'une restriction d'usage
/// metsRights:ConstraintDescription
/// http://cosimo.stanford.edu/sdr/metsrights/
/// Description d'une condition qui vient restreindre les permissions accordées
#[lax]
#[derive(Debug, Clone, Deserialize, Default)]
pub struct ConstraintDescription(#[serde(rename = "$value")] String);

/// Restriction d'usage
/// metsRights:Constraints
/// http://cosimo.stanford.edu/sdr/metsrights/
/// Condition qui vient restreindre les permissions accordées 
#[lax]
#[derive(Debug, Clone, Deserialize, Default)]
pub struct Constraints {
    #[serde(rename = "@CONSTRAINTTYPE")]
    constrainttype: String,
    #[serde(rename = "@OTHERCONSTRAINTTYPE")]
    otherconstrainttype: Option<String>,
    #[serde(rename = "$value")]
    values: Vec<ConstraintDescription>
}

/// Contexte
/// metsRights:Context
/// http://cosimo.stanford.edu/sdr/metsrights/
/// Autorisations et contraintes dans un contexte d'usage donné. Définition des droits par types de public.
#[lax]
#[derive(Debug, Clone, Deserialize, Default)]
pub struct Context {
    #[serde(rename = "@CONTEXTCLASS")]
    contextclass: String,
    #[serde(rename = "@OTHERCONTEXTTYPE")]
    othercontexttype: Option<String>,
    #[serde(rename = "$value")]
    values: Vec<ContextValues>
}

#[derive(Debug, Clone, Deserialize)]
pub enum ContextValues {
    Permissions(Permissions),
    Constraints(Constraints)
}

/// Autorisations
/// metsRights:Permissions
/// http://cosimo.stanford.edu/sdr/metsrights/
/// Description des modalités d’utilisation de la thèse par le détenteur des droits dans un contexte donné
#[lax]
#[derive(Debug, Clone, Deserialize, Default)]
pub struct Permissions {
    #[serde(rename = "@DISCOVER")]
    discover: Option<bool>,
    #[serde(rename = "@DISPLAY")]
    display: Option<bool>,
    #[serde(rename = "@COPY")]
    copy: Option<bool>,
    #[serde(rename = "@DUPLICATE")]
    duplicate: Option<bool>,
    #[serde(rename = "@MODIFY")]
    modify: Option<bool>,
    #[serde(rename = "@DELETE")]
    delete: Option<bool>,
    #[serde(rename = "@PRINT")]
    print: Option<bool>,
    #[serde(rename = "@OTHER")]
    other: Option<bool>,
    #[serde(rename = "@OTHERPERMITTYPE")]
    otherpermittype: Option<String>
}

/// Déclaration
/// metsRights:RightsDeclaration
/// http://cosimo.stanford.edu/sdr/metsrights/
/// Description libre relative aux droits
#[lax]
#[derive(Debug, Clone, Deserialize, Default)]
pub struct RightsDeclaration(#[serde(rename = "$value")] String);

/// Ensemble des métadonnées de droits
/// metsRights:RightsDeclarationMD
/// http://cosimo.stanford.edu/sdr/metsrights/
/// Ensemble des métadonnées de droits
#[lax]
#[derive(Debug, Clone, Deserialize, Default)]
pub struct RightsDeclarationMD {
    #[serde(rename = "$value")]
    values: Vec<RightsDeclarationMDValues>
}

#[derive(Debug, Clone, Deserialize)]
pub enum RightsDeclarationMDValues {
    RightsDeclaration(RightsDeclaration),
    Context(Context),
    RightsHolder(RightsHolder),
}

/// Titulaire des droits
/// metsRights:RightsHolder
/// http://cosimo.stanford.edu/sdr/metsrights/
/// Informations sur les personnes ou institutions titulaires de droits sur une ressource externe intégrée en tout ou partie à la thèse
#[lax]
#[derive(Debug, Clone, Deserialize, Default)]
pub struct RightsHolder(#[serde(rename = "$value")] Vec<RightsHolderName>);

/// Nom du titulaire des droits sur une ressource externe
/// metsRights:RightsHolderName
/// http://cosimo.stanford.edu/sdr/metsrights/
/// Nom et prénom du détenteur des droits s'il s'agit d'une personne physique. Nom s'il s'agit d'une personne morale. 
#[lax]
#[derive(Debug, Clone, Deserialize, Default)]
pub struct RightsHolderName(#[serde(rename = "$value")] String);