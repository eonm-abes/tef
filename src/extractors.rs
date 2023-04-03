impl<'a> From<&'a Mets> for Vec<&'a DmdSec> {
    fn from(value: &'a Mets) -> Self {
        value
            .values()
            .iter()
            .filter_map(|elem| match elem {
                MetsValues::DmdSec(dmd_sec) => Some(dmd_sec),
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
                MetsValues::AmdSec(dmd_sec) => Some(dmd_sec),
                _ => None,
            })
            .collect::<Vec<&AmdSec>>()
    }
}

impl<'a> From<&'a Mets> for Vec<&'a tef::ThesisAdmin> {
    fn from(value: &'a Mets) -> Self {
        let amd_secs: Vec<&AmdSec> = Vec::from(value);

        amd_secs
            .iter()
            .map(|amd_sec| {
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
                    .collect::<Vec<&tef::ThesisAdmin>>()
            })
            .flatten()
            .collect()
    }
}

impl<'a> From<&'a Mets> for Vec<&'a tef::ThesisRecord> {
    fn from(value: &'a Mets) -> Self {
        let dmd_secs: Vec<&DmdSec> = Vec::from(value);

        dmd_secs
            .iter()
            .filter_map(|md| match md.value().value().value() {
                XmlDataValues::ThesisRecord(record) => Some(record),
                _ => None,
            })
            .collect::<Vec<&tef::ThesisRecord>>()
    }
}


impl<'a> From<&'a Mets> for Option<&'a DateAccepted> {
    fn from(value: &'a Mets) -> Self {
        let thesis_admins: Vec<&tef::ThesisAdmin> = Vec::from(value);
        thesis_admins
            .iter()
            .map(|admin| admin.date_accepted())
            .collect::<Vec<&DateAccepted>>()
            .first()
            .copied()
    }
}

impl<'a> From<&'a Mets> for Vec<&'a Identifier> {
    fn from(value: &'a Mets) -> Self {
        let thesis_admins: Vec<&tef::ThesisAdmin> = Vec::from(value);
        thesis_admins
            .iter()
            .flat_map(|admin| admin.identifier())
            .collect::<Vec<&_>>()
    }
}

impl<'a> From<&'a Mets> for Option<&'a ThesisDegreeDiscipline> {
    fn from(value: &'a Mets) -> Self {
        let thesis_admins: Vec<&tef::ThesisAdmin> = Vec::from(value);
        thesis_admins
            .iter()
            .map(|admin| admin.thesis_degree())
            .map(|degree| degree.discipline())
            .collect::<Vec<&_>>()
            .first()
            .copied()
    }
}

impl<'a> From<&'a Mets> for Vec<&'a ThesisDegreeGrantor> {
    fn from(value: &'a Mets) -> Self {
        let thesis_admins: Vec<&tef::ThesisAdmin> = Vec::from(value);
        thesis_admins
            .iter()
            .map(|admin| admin.thesis_degree())
            .flat_map(|degree| degree.grantor())
            .collect::<Vec<&_>>()
    }
}

impl<'a> From<&'a Mets> for Option<&'a TheseSurTravaux> {
    fn from(value: &'a Mets) -> Self {
        let thesis_admins: Vec<&tef::ThesisAdmin> = Vec::from(value);
        thesis_admins
            .iter()
            .map(|admin| admin.these_sur_travaux())
            .collect::<Vec<&_>>()
            .first()
            .copied()
    }
}

impl<'a> From<&'a Mets> for Option<&'a Auteur> {
    fn from(value: &'a Mets) -> Self {
        let thesis_admins: Vec<&tef::ThesisAdmin> = Vec::from(value);
        thesis_admins
            .iter()
            .map(|admin| admin.auteur())
            .collect::<Vec<&_>>()
            .first()
            .copied()
    }
}

impl<'a> From<&'a Mets> for Vec<&'a DirecteurThese> {
    fn from(value: &'a Mets) -> Self {
        let thesis_admins: Vec<&tef::ThesisAdmin> = Vec::from(value);
        thesis_admins
            .iter()
            .flat_map(|admin| admin.directeur_these())
            .collect::<Vec<&_>>()
    }
}

impl<'a> From<&'a Mets> for Vec<&'a Rapporteur> {
    fn from(value: &'a Mets) -> Self {
        let thesis_admins: Vec<&tef::ThesisAdmin> = Vec::from(value);
        thesis_admins
            .iter()
            .flat_map(|admin| admin.rapporteur())
            .collect::<Vec<&_>>()
    }
}

impl<'a> From<&'a Mets> for Option<&'a PresidentJury> {
    fn from(value: &'a Mets) -> Self {
        let thesis_admins: Vec<&tef::ThesisAdmin> = Vec::from(value);
        thesis_admins
            .iter()
            .flat_map(|admin| admin.president_jury())
            .collect::<Vec<&_>>()
            .first()
            .copied()
    }
}

impl<'a> From<&'a Mets> for Vec<&'a MembreJury> {
    fn from(value: &'a Mets) -> Self {
        let thesis_admins: Vec<&tef::ThesisAdmin> = Vec::from(value);
        thesis_admins
            .iter()
            .flat_map(|admin| admin.membre_jury())
            .collect::<Vec<&_>>()
    }
}

impl<'a> From<&'a Mets> for Vec<&'a EcoleDoctorale> {
    fn from(value: &'a Mets) -> Self {
        let thesis_admins: Vec<&tef::ThesisAdmin> = Vec::from(value);
        thesis_admins
            .iter()
            .flat_map(|admin| admin.ecole_doctorale())
            .collect::<Vec<&_>>()
    }
}

impl<'a> From<&'a Mets> for Vec<&'a PartenaireRecherche> {
    fn from(value: &'a Mets) -> Self {
        let thesis_admins: Vec<&tef::ThesisAdmin> = Vec::from(value);
        thesis_admins
            .iter()
            .flat_map(|admin| admin.partenaire_recherche())
            .collect::<Vec<&_>>()
    }
}

impl<'a> From<&'a Mets> for Vec<&'a OaiSetSpec> {
    fn from(value: &'a Mets) -> Self {
        let thesis_admins: Vec<&tef::ThesisAdmin> = Vec::from(value);
        thesis_admins
            .iter()
            .flat_map(|admin| admin.oai_set_spec())
            .collect::<Vec<&_>>()
    }
}

impl<'a> From<&'a Mets> for Option<&'a Title> {
    fn from(value: &'a Mets) -> Self {
        let thesis_records : Vec<&tef::ThesisRecord> = Vec::from(value);
        thesis_records
            .iter()
            .map(|admin| admin.title())
            .collect::<Vec<&_>>()
            .first()
            .copied()
    }
}

impl<'a> From<&'a Mets> for Vec<&'a Alternative> {
    fn from(value: &'a Mets) -> Self {
        let thesis_records : Vec<&tef::ThesisRecord> = Vec::from(value);
        thesis_records
            .iter()
            .flat_map(|admin| admin.alternative())
            .flatten()
            .collect::<Vec<&_>>()
    }
}

impl<'a> From<&'a Mets> for Vec<&'a Abstract> {
    fn from(value: &'a Mets) -> Self {
        let thesis_records : Vec<&tef::ThesisRecord> = Vec::from(value);
        thesis_records
            .iter()
            .flat_map(|admin| admin.r#abstract())
            .flatten()
            .collect::<Vec<&_>>()
    }
}

impl<'a> From<&'a Mets> for Vec<&'a Subject> {
    fn from(value: &'a Mets) -> Self {
        let thesis_records : Vec<&tef::ThesisRecord> = Vec::from(value);
        thesis_records
            .iter()
            .flat_map(|admin| admin.subject())
            .flatten()
            .collect::<Vec<&_>>()
    }
}

impl<'a> From<&'a Mets> for Vec<&'a Language> {
    fn from(value: &'a Mets) -> Self {
        let thesis_records : Vec<&tef::ThesisRecord> = Vec::from(value);
        thesis_records
            .iter()
            .flat_map(|admin| admin.language())
            .collect::<Vec<&_>>()
    }
}
