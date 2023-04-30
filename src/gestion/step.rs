use getset::Getters;
use lax_derive::lax;
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "lax", lax)]
#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Getters)]
#[getset(get = "pub")]
pub struct StepGestion {
    #[serde(rename = "@codeEtab")]
    code_etab: String,
    #[serde(rename = "@libEtab")]
    lib_etab: String,
    #[serde(rename = "@ppnEtab")]
    ppn_etab: String,
    #[serde(rename = "@enProdStep")]
    en_prod_step: String,
    #[serde(rename = "@ID_SUJET")]
    id_sujet: String,
    #[serde(rename = "@stepEtat")]
    step_etat: String,
    traitements: Traitements,
    workflow: Workflow,
}

#[cfg_attr(feature = "lax", lax)]
#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Getters)]
#[getset(get = "pub")]
pub struct Traitements {
    entree: Entree,
    maj: Maj,
    sorties: Sorties,
}

#[cfg_attr(feature = "lax", lax)]
#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Getters)]
#[getset(get = "pub")]
pub struct Maj;

#[cfg_attr(feature = "lax", lax)]
#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Getters)]
#[getset(get = "pub")]
pub struct Workflow;

#[cfg_attr(feature = "lax", lax)]
#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Getters)]
#[getset(get = "pub")]
pub struct Entree;

#[cfg_attr(feature = "lax", lax)]
#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Getters)]
#[getset(get = "pub")]
pub struct Sorties {
    nnt: Nnt,
    star: Star,
    sudoc: Sudoc,
    diffusion: Diffusion,
}

#[cfg_attr(feature = "lax", lax)]
#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Getters)]
#[getset(get = "pub")]
pub struct Diffusion;

#[cfg_attr(feature = "lax", lax)]
#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Getters)]
#[getset(get = "pub")]
pub struct Sudoc;

#[cfg_attr(feature = "lax", lax)]
#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Getters)]
#[getset(get = "pub")]
pub struct Nnt {
    #[serde(rename = "@dateNnt")]
    date_nnt: Option<String>,
    #[serde(rename = "@sourceNnt")]
    source_nnt: Option<String>,
    #[serde(rename = "@indicNnt")]
    indic_nnt: Option<String>,
    #[serde(rename = "@trace")]
    trace: Option<String>,
    #[serde(rename = "$text")]
    text: String,
}

#[cfg_attr(feature = "lax", lax)]
#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Getters)]
#[getset(get = "pub")]
pub struct Star;
