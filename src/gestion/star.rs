use getset::Getters;
use lax_derive::lax;
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "lax", lax)]
#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Getters)]
#[getset(get = "pub")]
pub struct StarGestion {
    #[serde(rename = "@codeEtab")]
    code_etab: String,
    #[serde(rename = "@libEtab")]
    lib_etab: String,
    #[serde(rename = "@ppnEtab")]
    ppn_etab: String,
    #[serde(rename = "@enProd")]
    en_prod: String,
    #[serde(rename = "@ID_THESE")]
    id_these: String,
    #[serde(rename = "@scenarioEtab")]
    scenario_etab: String,
    #[serde(rename = "@etat")]
    etat: String,
    traitements: Traitements,
    workflow: Workflow,
}

#[cfg_attr(feature = "lax", lax)]
#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Getters)]
#[getset(get = "pub")]
pub struct Traitements {
    #[serde(rename = "@scenario")]
    scenario: String,
    entree: Entree,
    step: Vec<Step>,
    maj: Maj,
    facile: Facile,
    #[serde(rename = "ctrUrl")]
    ctr_url: CtrlUrl,
    #[serde(rename = "remonteeArchive")]
    remontee_archive: RemonteeArchive,
    purge: Purge,
    invalidation: Invalidation,
    sorties: Sorties,
}

#[cfg_attr(feature = "lax", lax)]
#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Getters)]
#[getset(get = "pub")]
pub struct Entree;

#[cfg_attr(feature = "lax", lax)]
#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Getters)]
#[getset(get = "pub")]
pub struct Step;

#[cfg_attr(feature = "lax", lax)]
#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Getters)]
#[getset(get = "pub")]
pub struct Maj;

#[cfg_attr(feature = "lax", lax)]
#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Getters)]
#[getset(get = "pub")]
pub struct Facile;

#[cfg_attr(feature = "lax", lax)]
#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Getters)]
#[getset(get = "pub")]
pub struct RemonteeArchive;

#[cfg_attr(feature = "lax", lax)]
#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Getters)]
#[getset(get = "pub")]
pub struct Invalidation;

#[cfg_attr(feature = "lax", lax)]
#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Getters)]
#[getset(get = "pub")]
pub struct Sorties {
    #[serde(rename = "@date")]
    date: String,
    cines: Cines,
    sudoc: Sudoc,
    diffusion: Diffusion,
}

#[cfg_attr(feature = "lax", lax)]
#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Getters)]
#[getset(get = "pub")]
pub struct Cines {
    #[serde(rename = "@numeroPAC")]
    numero_pac: String,
    #[serde(rename = "@dateCines")]
    date_cines: String,
    #[serde(rename = "@indicCines")]
    indic_cines: String,
    #[serde(rename = "@trace")]
    trace: String,
}

#[cfg_attr(feature = "lax", lax)]
#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Getters)]
#[getset(get = "pub")]
pub struct Sudoc;

#[cfg_attr(feature = "lax", lax)]
#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Getters)]
#[getset(get = "pub")]
pub struct Diffusion {
    #[serde(rename = "@urlPerenne")]
    url_perenne: String,
    #[serde(rename = "@conformitePolDiffusion")]
    conformite_pol_diffusion: String,
    #[serde(rename = "@typeDiffusion")]
    type_diffusion: String,
    #[serde(rename = "@restrictionTemporelleType")]
    restriction_temporelle_type: String,
    #[serde(rename = "@restrictionTemporelleFin")]
    restriction_temporelle_fin: String,
    #[serde(rename = "@embargoFin")]
    embargo_fin: String,
    #[serde(rename = "@confidentialiteFin")]
    confidentialite_fin: String,
}

#[cfg_attr(feature = "lax", lax)]
#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Getters)]
#[getset(get = "pub")]
pub struct Workflow;

#[cfg_attr(feature = "lax", lax)]
#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Getters)]
#[getset(get = "pub")]
pub struct CtrlUrl;

#[cfg_attr(feature = "lax", lax)]
#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Getters)]
#[getset(get = "pub")]
pub struct Purge;
