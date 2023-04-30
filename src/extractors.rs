use crate::dc::*;
use crate::dcterms::*;
use crate::mets::*;
use crate::tef::*;

#[cfg(feature = "gestion")]
use crate::gestion::star::StarGestion;
#[cfg(feature = "gestion")]
use crate::gestion::step::StepGestion;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum TefError {
    #[error("date accetped not found")]
    DateAcceptedNotFound,
    #[error("thesis degree discipline not found")]
    ThesisDegreeDisciplineNotFound,
    #[error("thesis degree grantor not found")]
    ThesisDegreeGrantorNotFound,
    #[error("theses sur travaux not found")]
    TheseSurTravauxNotFound,
    #[error("auteur not found")]
    AuteurNotFound,
    #[error("directeur de thèse not found")]
    DirecteurTheseNotFound,
    #[error("oai set not found")]
    OaiSetSpecNotFound,
    #[error("title not found")]
    TitleNotFound,
    #[error("alternative title not found")]
    AlternativeTitleNotFound,
    #[error("abstract not found")]
    AbstractNotFound,
    #[error("subject not found")]
    SubjectNotFound,
    #[error("language not found")]
    LanguageNotFound,
    #[error("element entree not found")]
    ElementdEntreeNotFound,
    #[error("star getsion not found")]
    StarGestionNotFound,
    #[error("step getsion not found")]
    StepGestionNotFound,
}

macro_rules! from_option {
    ($type:ty) => {
        impl<'a> From<&'a Mets> for Option<$type> {
            fn from(value: &'a Mets) -> Self {
                let result: Result<$type, _> = value.try_into();

                match result {
                    Ok(result) => Some(result),
                    Err(_) => None,
                }
            }
        }
    };
}

impl<'a> From<&'a Mets> for Vec<&'a DmdSec> {
    fn from(value: &'a Mets) -> Self {
        value
            .values()
            .iter()
            .filter_map(|elem| match elem {
                MetsValues::DmdSec(dmd_sec) => {
                    if dmd_sec != &DmdSec::default() {
                        Some(dmd_sec)
                    } else {
                        None
                    }
                }
                _ => None,
            })
            .collect::<Vec<&DmdSec>>()
    }
}

impl<'a> From<&'a Mets> for Vec<&'a AmdSec> {
    fn from(value: &'a Mets) -> Self {
        value
            .values()
            .iter()
            .filter_map(|elem| match elem {
                MetsValues::AmdSec(amd_sec) => {
                    if amd_sec != &AmdSec::default() {
                        Some(amd_sec)
                    } else {
                        None
                    }
                }
                _ => None,
            })
            .collect::<Vec<&AmdSec>>()
    }
}

impl<'a> From<&'a Mets> for Vec<&'a ThesisAdmin> {
    fn from(value: &'a Mets) -> Self {
        let amd_secs: Vec<&AmdSec> = Vec::from(value);

        amd_secs
            .iter()
            .flat_map(|amd_sec| {
                amd_sec
                    .values()
                    .iter()
                    .filter_map(|elem| match elem {
                        AmdSecValues::TechMD(md) => Some(md),
                        _ => None,
                    })
                    .filter_map(|md| match md.value().value().value() {
                        XmlDataValues::ThesisAdmin(admin) => Some(admin),
                        _ => None,
                    })
                    .filter(|elem| elem != &&ThesisAdmin::default())
                    .collect::<Vec<&ThesisAdmin>>()
            })
            .collect()
    }
}

impl<'a> From<&'a Mets> for Vec<&'a ThesisRecord> {
    fn from(value: &'a Mets) -> Self {
        let dmd_secs: Vec<&DmdSec> = Vec::from(value);

        dmd_secs
            .iter()
            .filter_map(|md| match md.value().value().value() {
                XmlDataValues::ThesisRecord(record) => Some(record),
                _ => None,
            })
            .filter(|elem| elem != &&ThesisRecord::default())
            .collect::<Vec<&ThesisRecord>>()
    }
}

impl<'a> TryFrom<&'a Mets> for &'a DateAccepted {
    type Error = TefError;

    fn try_from(value: &'a Mets) -> Result<Self, Self::Error> {
        let thesis_admins: Vec<&ThesisAdmin> = Vec::from(value);

        let result = thesis_admins
            .iter()
            .map(|admin| admin.date_accepted())
            .filter(|elem| elem != &&DateAccepted::default() && !elem.text().is_empty())
            .collect::<Vec<&DateAccepted>>();

        match result.first() {
            Some(date) => Ok(date),
            None => Err(TefError::DateAcceptedNotFound),
        }
    }
}

from_option!(&'a DateAccepted);

/// Implémentation qui respecte la cardinalité du TEF
impl<'a> TryFrom<&'a Mets> for Vec<&'a Identifier> {
    type Error = &'static str;

    fn try_from(value: &'a Mets) -> Result<Self, Self::Error> {
        let thesis_admins: Vec<&ThesisAdmin> = Vec::from(value);
        let result = thesis_admins
            .iter()
            .flat_map(|admin| admin.identifier())
            .collect::<Vec<&_>>();

        if result.is_empty() {
            Err("Failed to get data")
        } else {
            Ok(result)
        }
    }
}

from_option!(Vec<&'a Identifier>);

impl<'a> TryFrom<&'a Mets> for &'a ThesisDegreeDiscipline {
    type Error = TefError;

    fn try_from(value: &'a Mets) -> Result<Self, Self::Error> {
        let thesis_admins: Vec<&ThesisAdmin> = Vec::from(value);

        let result = thesis_admins
            .iter()
            .map(|admin| admin.thesis_degree())
            .map(|degree| degree.discipline())
            .filter(|elem| elem != &&ThesisDegreeDiscipline::default())
            .collect::<Vec<&_>>();

        match result.first() {
            Some(discipline) => Ok(discipline),
            None => Err(TefError::ThesisDegreeDisciplineNotFound),
        }
    }
}

from_option!(&'a ThesisDegreeDiscipline);

impl<'a> TryFrom<&'a Mets> for Vec<&'a ThesisDegreeGrantor> {
    type Error = TefError;

    fn try_from(value: &'a Mets) -> Result<Self, Self::Error> {
        let thesis_admins: Vec<&ThesisAdmin> = Vec::from(value);
        let result = thesis_admins
            .iter()
            .map(|admin| admin.thesis_degree())
            .flat_map(|degree| degree.grantor())
            .filter(|elem| elem != &&ThesisDegreeGrantor::default())
            .filter(|elem| elem.nom() != &Nom::default())
            .collect::<Vec<&_>>();

        if !result.is_empty() {
            Ok(result)
        } else {
            Err(TefError::ThesisDegreeGrantorNotFound)
        }
    }
}

from_option!(Vec<&'a ThesisDegreeGrantor>);

impl<'a> TryFrom<&'a Mets> for &'a TheseSurTravaux {
    type Error = TefError;

    fn try_from(value: &'a Mets) -> Result<Self, Self::Error> {
        let thesis_admins: Vec<&ThesisAdmin> = Vec::from(value);
        let result = thesis_admins
            .iter()
            .map(|admin| admin.these_sur_travaux())
            .filter(|elem| elem != &&TheseSurTravaux::default())
            .collect::<Vec<&_>>();

        match result.first() {
            Some(elem) => {
                if elem != &&TheseSurTravaux::default() {
                    Ok(elem)
                } else {
                    Err(TefError::TheseSurTravauxNotFound)
                }
            }
            None => Err(TefError::TheseSurTravauxNotFound),
        }
    }
}

from_option!(&'a TheseSurTravaux);

impl<'a> TryFrom<&'a Mets> for &'a Auteur {
    type Error = TefError;

    fn try_from(value: &'a Mets) -> Result<Self, Self::Error> {
        let thesis_admins: Vec<&ThesisAdmin> = Vec::from(value);
        let result = thesis_admins
            .iter()
            .map(|admin| admin.auteur())
            .filter(|elem| elem != &&Auteur::default())
            .collect::<Vec<&_>>();

        match result.first() {
            Some(elem) => Ok(elem),
            None => Err(TefError::AuteurNotFound),
        }
    }
}

from_option!(&'a Auteur);

impl<'a> TryFrom<&'a Mets> for Vec<&'a DirecteurThese> {
    type Error = TefError;

    fn try_from(value: &'a Mets) -> Result<Self, Self::Error> {
        let thesis_admins: Vec<&ThesisAdmin> = Vec::from(value);
        let result = thesis_admins
            .iter()
            .flat_map(|admin| admin.directeur_these())
            .filter(|elem| elem != &&DirecteurThese::default() && !elem.nom().0.is_empty())
            .collect::<Vec<&_>>();

        if !result.is_empty() {
            Ok(result)
        } else {
            Err(TefError::DirecteurTheseNotFound)
        }
    }
}

from_option!(Vec<&'a DirecteurThese>);

impl<'a> From<&'a Mets> for Option<Vec<&'a Rapporteur>> {
    fn from(value: &'a Mets) -> Self {
        let thesis_admins: Vec<&ThesisAdmin> = Vec::from(value);
        let result = thesis_admins
            .iter()
            .flat_map(|admin| admin.rapporteur())
            .filter(|elem| elem != &&Rapporteur::default() && !elem.nom().0.is_empty())
            .collect::<Vec<&_>>();

        if !result.is_empty() {
            Some(result)
        } else {
            None
        }
    }
}

impl<'a> From<&'a Mets> for Option<&'a PresidentJury> {
    fn from(value: &'a Mets) -> Self {
        let thesis_admins: Vec<&ThesisAdmin> = Vec::from(value);
        thesis_admins
            .iter()
            .flat_map(|admin| admin.president_jury())
            .filter(|elem| elem != &&PresidentJury::default() && !elem.nom().0.is_empty())
            .collect::<Vec<&_>>()
            .first()
            .copied()
    }
}

impl<'a> From<&'a Mets> for Option<Vec<&'a MembreJury>> {
    fn from(value: &'a Mets) -> Self {
        let thesis_admins: Vec<&ThesisAdmin> = Vec::from(value);
        let result = thesis_admins
            .iter()
            .flat_map(|admin| admin.membre_jury())
            .filter(|elem| elem != &&MembreJury::default() && !elem.nom().0.is_empty())
            .collect::<Vec<&_>>();

        if !result.is_empty() {
            Some(result)
        } else {
            None
        }
    }
}

impl<'a> From<&'a Mets> for Option<Vec<&'a EcoleDoctorale>> {
    fn from(value: &'a Mets) -> Self {
        let thesis_admins: Vec<&ThesisAdmin> = Vec::from(value);
        let result = thesis_admins
            .iter()
            .flat_map(|admin| admin.ecole_doctorale())
            .filter(|elem| elem != &&EcoleDoctorale::default())
            .filter(|elem| elem.nom() != &Nom::default() && !elem.nom().0.is_empty())
            .collect::<Vec<&_>>();

        if !result.is_empty() {
            Some(result)
        } else {
            None
        }
    }
}

impl<'a> From<&'a Mets> for Option<Vec<&'a PartenaireRecherche>> {
    fn from(value: &'a Mets) -> Self {
        let thesis_admins: Vec<&ThesisAdmin> = Vec::from(value);
        let result = thesis_admins
            .iter()
            .flat_map(|admin| admin.partenaire_recherche())
            .filter(|elem| elem != &&PartenaireRecherche::default())
            .filter(|elem| elem.nom() != &Nom::default() && !elem.nom().0.is_empty())
            .collect::<Vec<&_>>();

        if !result.is_empty() {
            Some(result)
        } else {
            None
        }
    }
}

impl<'a> TryFrom<&'a Mets> for Vec<&'a OaiSetSpec> {
    type Error = TefError;

    fn try_from(value: &'a Mets) -> Result<Self, Self::Error> {
        let thesis_admins: Vec<&ThesisAdmin> = Vec::from(value);
        let result = thesis_admins
            .iter()
            .flat_map(|admin| admin.oai_set_spec())
            .filter(|elem| elem != &&OaiSetSpec::default())
            .collect::<Vec<&_>>();

        if !result.is_empty() {
            Ok(result)
        } else {
            Err(TefError::OaiSetSpecNotFound)
        }
    }
}

from_option!(Vec<&'a OaiSetSpec>);

impl<'a> TryFrom<&'a Mets> for &'a Title {
    type Error = TefError;

    fn try_from(value: &'a Mets) -> Result<Self, Self::Error> {
        let thesis_records: Vec<&ThesisRecord> = Vec::from(value);
        let result = thesis_records
            .iter()
            .map(|admin| admin.title())
            .filter(|elem| elem != &&Title::default())
            .filter(|elem| !elem.text().is_empty() && !elem.lang().is_empty())
            .collect::<Vec<&_>>();

        match result.first() {
            Some(title) => Ok(title),
            None => Err(TefError::TitleNotFound),
        }
    }
}

from_option!(&'a Title);

impl<'a> TryFrom<&'a Mets> for Vec<&'a Alternative> {
    type Error = TefError;

    fn try_from(value: &'a Mets) -> Result<Self, Self::Error> {
        let thesis_records: Vec<&ThesisRecord> = Vec::from(value);
        let result = thesis_records
            .iter()
            .flat_map(|admin| admin.alternative())
            .flatten()
            .filter(|elem| elem != &&Alternative::default())
            .filter(|elem| !elem.text().is_empty() && !elem.lang().is_empty())
            .collect::<Vec<&_>>();

        if !result.is_empty() {
            Ok(result)
        } else {
            Err(TefError::AlternativeTitleNotFound)
        }
    }
}

from_option!(Vec<&'a Alternative>);

impl<'a> TryFrom<&'a Mets> for Vec<&'a Abstract> {
    type Error = TefError;

    fn try_from(value: &'a Mets) -> Result<Self, Self::Error> {
        let thesis_records: Vec<&ThesisRecord> = Vec::from(value);

        let result = thesis_records
            .iter()
            .flat_map(|admin| admin.r#abstract())
            .flatten()
            .filter(|elem| elem != &&Abstract::default())
            .filter(|elem| !elem.text().is_empty() && !elem.lang().is_empty())
            .collect::<Vec<&_>>();

        if !result.is_empty() {
            Ok(result)
        } else {
            Err(TefError::AbstractNotFound)
        }
    }
}

from_option!(Vec<&'a Abstract>);

impl<'a> TryFrom<&'a Mets> for Vec<&'a Subject> {
    type Error = TefError;

    fn try_from(value: &'a Mets) -> Result<Self, Self::Error> {
        let thesis_records: Vec<&ThesisRecord> = Vec::from(value);

        let result = thesis_records
            .iter()
            .flat_map(|admin| admin.subject())
            .flatten()
            .filter(|elem| elem != &&Subject::default())
            .filter(|elem| !elem.text().is_empty())
            .collect::<Vec<&_>>();

        if !result.is_empty() {
            Ok(result)
        } else {
            Err(TefError::SubjectNotFound)
        }
    }
}

from_option!(Vec<&'a Subject>);

impl<'a> TryFrom<&'a Mets> for Vec<&'a Language> {
    type Error = TefError;

    fn try_from(value: &'a Mets) -> Result<Self, Self::Error> {
        let thesis_records: Vec<&ThesisRecord> = Vec::from(value);

        let result = thesis_records
            .iter()
            .flat_map(|elem| elem.language())
            .filter(|elem| elem != &&Language::default())
            .collect::<Vec<&_>>();

        if !result.is_empty() {
            Ok(result)
        } else {
            Err(TefError::LanguageNotFound)
        }
    }
}

from_option!(Vec<&'a Language>);

impl<'a> From<&'a Mets> for Option<&'a SujetRameau> {
    fn from(value: &'a Mets) -> Self {
        let thesis_records: Vec<&ThesisRecord> = Vec::from(value);

        thesis_records
            .iter()
            .filter(|elem| elem != &&&ThesisRecord::default())
            .flat_map(|elem| elem.sujet_rameau())
            .filter(|elem| elem != &&SujetRameau::default())
            .collect::<Vec<&_>>()
            .first()
            .copied()
    }
}

impl TryFrom<&SujetRameauValues> for ElementdEntree {
    type Error = TefError;

    fn try_from(value: &SujetRameauValues) -> Result<Self, Self::Error> {
        let element_entree = match value {
            SujetRameauValues::VedetteRameauPersonne(elem) => elem.element_entree().clone(),
            SujetRameauValues::VedetteRameauCollectivite(elem) => elem.element_entree().clone(),
            SujetRameauValues::VedetteRameauFamille(elem) => elem.element_entree().clone(),
            SujetRameauValues::VedetteRameauAuteurTitre(elem) => elem.element_entree().clone(),
            SujetRameauValues::VedetteRameauTitre(elem) => elem.element_entree().clone(),
            SujetRameauValues::VedetteRameauNomCommun(elem) => elem.element_entree().clone(),
            SujetRameauValues::VedetteRameauNomGeographique(elem) => elem.element_entree().clone(),
            SujetRameauValues::VedetteRameauGenreForme(elem) => elem.element_entree().clone(),
        };

        if element_entree != ElementdEntree::default() {
            Ok(element_entree)
        } else {
            Err(TefError::ElementdEntreeNotFound)
        }
    }
}

impl From<&SujetRameauValues> for Option<Vec<Subdivision>> {
    fn from(value: &SujetRameauValues) -> Self {
        let subdivisions = match value {
            SujetRameauValues::VedetteRameauPersonne(elem) => elem.subdivision().clone(),
            SujetRameauValues::VedetteRameauCollectivite(elem) => elem.subdivision().clone(),
            SujetRameauValues::VedetteRameauFamille(elem) => elem.subdivision().clone(),
            SujetRameauValues::VedetteRameauAuteurTitre(elem) => elem.subdivision().clone(),
            SujetRameauValues::VedetteRameauTitre(elem) => elem.subdivision().clone(),
            SujetRameauValues::VedetteRameauNomCommun(elem) => elem.subdivision().clone(),
            SujetRameauValues::VedetteRameauNomGeographique(elem) => elem.subdivision().clone(),
            SujetRameauValues::VedetteRameauGenreForme(elem) => elem.subdivision().clone(),
        };

        match subdivisions {
            Some(subdivisions) => {
                let result: Vec<Subdivision> = subdivisions
                    .into_iter()
                    .filter(|elem| elem != &Subdivision::default())
                    .collect();
                if !result.is_empty() {
                    Some(result)
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}

#[cfg(feature = "gestion")]
impl<'a> TryFrom<&'a Mets> for &'a StarGestion {
    type Error = TefError;

    fn try_from(value: &'a Mets) -> Result<Self, Self::Error> {
        let thesis_records: Vec<&DmdSec> = Vec::from(value);

        let result = thesis_records
            .iter()
            .filter_map(|md| match md.value().value().value() {
                XmlDataValues::StarGestion(record) => Some(record),
                _ => None,
            })
            .filter(|elem| elem != &&StarGestion::default())
            .collect::<Vec<&StarGestion>>();

        match result.first() {
            Some(result) => Ok(result),
            None => Err(TefError::StarGestionNotFound),
        }
    }
}

#[cfg(feature = "gestion")]
from_option!(&'a StarGestion);

#[cfg(feature = "gestion")]
impl<'a> TryFrom<&'a Mets> for &'a StepGestion {
    type Error = TefError;

    fn try_from(value: &'a Mets) -> Result<Self, Self::Error> {
        let thesis_records: Vec<&DmdSec> = Vec::from(value);

        let result = thesis_records
            .iter()
            .filter_map(|md| match md.value().value().value() {
                XmlDataValues::StepGestion(record) => Some(record),
                _ => None,
            })
            .filter(|elem| elem != &&StepGestion::default())
            .collect::<Vec<&StepGestion>>();

        match result.first() {
            Some(result) => Ok(result),
            None => Err(TefError::StepGestionNotFound),
        }
    }
}

#[cfg(feature = "gestion")]
from_option!(&'a StepGestion);
