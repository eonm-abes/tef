use std::fmt::Display;

use crate::{dc::*, dcterms::*};
use getset::Getters;
use serde::{Deserialize, Serialize};

#[cfg(feature = "lax")]
use lax_derive::lax;

fn empty_string_as_none<'de, D, T>(de: D) -> Result<Option<T>, D::Error>
where
    D: serde::Deserializer<'de>,
    T: serde::Deserialize<'de> + Display + Clone,
{
    let opt_1 = Option::<T>::deserialize(de)?;

    match opt_1.clone() {
        None => Ok(None),
        Some(s) => {
            if s.to_string().is_empty() {
                Ok(None)
            } else {
                Ok(opt_1)
            }
        }
    }
}

/// Auteur
/// tef:auteur
/// <http://www.abes.fr/abes/documents/tef>
/// Auteur de la thèse
#[cfg_attr(feature = "lax", lax)]
#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Getters)]
#[getset(get = "pub")]
pub struct Auteur {
    nom: Nom,
    prenom: Prenom,
    #[serde(rename = "nomDeNaissance")]
    nom_de_naissance: Option<NomDeNaissance>,
    #[serde(rename = "dateNaissance")]
    #[serde(deserialize_with = "empty_string_as_none")]
    date_naissance: Option<DateNaissance>,
    #[serde(deserialize_with = "empty_string_as_none")]
    nationalite: Option<String>,
    #[serde(rename = "autoriteExterne")]
    autorite_externe: Option<Vec<AutoriteExterne>>,
}

/// Identifiant de notice d’autorité externe
/// tef:autoriteExterne
/// <http://www.abes.fr/abes/documents/tef>
/// Identifiant de notice d'autorité issue d'un système externe
#[cfg_attr(feature = "lax", lax)]
#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Getters)]
#[getset(get = "pub")]
pub struct AutoriteExterne {
    #[serde(rename = "@autoriteSource")]
    autorite_source: String,
    #[serde(rename = "$text")]
    text: String,
}

/// Autre format de fichier
/// tef:autreFormatFichier
/// <http://www.abes.fr/abes/documents/tef>
/// Format de fichier informatique qui n'est pas prévu dans la liste des valeurs que peut prendre l'élément tef:formatFichier
#[cfg_attr(feature = "lax", lax)]
#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq)]
pub struct AutoriteFormatFichier(#[serde(rename = "$text")] pub String);

/// Identifiant d'un bloc de données d'autorité internes
/// tef:autoriteInterne
/// <http://www.abes.fr/abes/documents/tef>
/// Identifiant d'un bloc de données d'autorité tef:MADSAuthority
#[cfg_attr(feature = "lax", lax)]
#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq)]
pub struct AutoriteInterne(#[serde(rename = "$text")] pub String);

/// Avis du jury
/// tef:avisJury
/// <http://www.abes.fr/abes/documents/tef>
/// Avis du jury autorisant ou non la diffusion de la thèse après la soutenance
#[cfg_attr(feature = "lax", lax)]
#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq)]
pub struct AvisJury(#[serde(rename = "$text")] pub String);

/// Date de naissance
/// tef:dateNaissance
/// <http://www.abes.fr/abes/documents/tef>
/// Date de naissance de l'auteur de la thèse
#[cfg_attr(feature = "lax", lax)]
#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq)]
pub struct DateNaissance(#[serde(rename = "$text")] pub String);

impl std::fmt::Display for DateNaissance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Directeur de thèse
/// tef:directeurThèse
/// <http://www.abes.fr/abes/documents/tef>
/// Personne qui encadre et oriente le travail du doctorant
#[cfg_attr(feature = "lax", lax)]
#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Getters)]
#[getset(get = "pub")]
pub struct DirecteurThese {
    nom: Nom,
    prenom: Prenom,
    #[serde(rename = "autoriteInterne")]
    autorite_interne: Option<Vec<AutoriteInterne>>,
    #[serde(rename = "autoriteExterne")]
    autorite_externe: Option<Vec<AutoriteExterne>>,
}

/// Ecole doctorale
/// tef:ecoleDoctorale
/// <http://www.abes.fr/abes/documents/tef>
/// Ecole doctorale au sein de laquelle s’est déroulée la recherche du doctorant
#[cfg_attr(feature = "lax", lax)]
#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Getters)]
#[getset(get = "pub")]
pub struct EcoleDoctorale {
    nom: Nom,
    #[serde(rename = "autoriteInterne")]
    autorite_interne: Option<Vec<AutoriteInterne>>,
    #[serde(rename = "autoriteExterne")]
    autorite_externe: Option<Vec<AutoriteExterne>>,
}

/// Editeur
/// tef:editeur
/// <http://www.abes.fr/abes/documents/tef>
/// Organisme responsable de l’édition électronique de la thèse
#[cfg_attr(feature = "lax", lax)]
#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Getters)]
#[getset(get = "pub")]
pub struct Editeur {
    nom: Nom,
    place: Place,
    #[serde(rename = "autoriteInterne")]
    autorite_interne: Option<Vec<AutoriteInterne>>,
    #[serde(rename = "autoriteExterne")]
    autorite_externe: Option<Vec<AutoriteExterne>>,
}

/// Edition
/// tef:edition
/// <http://www.abes.fr/abes/documents/tef>
/// Ensemble des métadonnées relatives à une édition électronique de la thèse
#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Getters)]
#[getset(get = "pub")]
pub struct Edition {
    #[serde(rename = "$value")]
    values: Vec<EditionValues>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum EditionValues {
    Medium(Medium),
    Extent(Extent),
    Identifier(Identifier),
    Issued(Issued),
    Replaces(Replaces),
    Editeur(Editeur),
    #[serde(other)]
    Other,
}

#[cfg_attr(feature = "lax", lax)]
#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Getters)]
#[getset(get = "pub")]
pub struct ElementdEntree {
    #[serde(rename = "@autoriteExterne")]
    autorite_externe: Option<String>,
    #[serde(rename = "@autoriteSource")]
    autorite_source: Option<String>,
    #[serde(rename = "$text")]
    text: String,
}

/// Encodage
/// tef:encodage
/// <http://www.abes.fr/abes/documents/tef>
/// Type d'encodage du fichier
#[cfg_attr(feature = "lax", lax)]
#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Getters)]
#[getset(get = "pub")]
pub struct Encodage {
    #[serde(rename = "$text")]
    text: EncodageValues,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq)]
pub enum EncodageValues {
    ASCII,
    #[serde(rename = "Latin 1")]
    Latin1,
    #[default]
    Unicode,
}

/// Format de fichier
/// tef:formatFichier
/// <http://www.abes.fr/abes/documents/tef>
/// Format du fichier
#[cfg_attr(feature = "lax", lax)]
#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Getters)]
#[getset(get = "pub")]
pub struct FormatFichier {
    #[serde(rename = "$text")]
    text: FormatFichierValues,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq)]
pub enum FormatFichierValues {
    OpenDocument,
    PDF,
    #[serde(rename = "PDF/A")]
    PDFA,
    HTML,
    RTF,
    TXT,
    XML,
    JPEG,
    GIF,
    PNG,
    TIFF,
    MP3,
    MPEG,
    QuickTime,
    #[serde(rename = "autreFormat")]
    #[default]
    AutreFormat,
}

/// Autorité
/// tef:MADSAuthority
/// <http://www.abes.fr/abes/documents/tef>
/// Données d'autorité minimales relatives à une personne (morale ou physique) mentionnée dans la notice
#[cfg_attr(feature = "lax", lax)]
#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Getters)]
#[getset(get = "pub")]
pub struct MADSAuthority {
    #[serde(rename = "@authorityID")]
    authority_id: String,
    #[serde(rename = "@type")]
    r#type: String,
    #[serde(rename = "$value")]
    value: PersonMADS,
}

/// Manque
/// tef:manque
/// <http://www.abes.fr/abes/documents/tef>
/// Indique une partie manquante en cas de version incomplète.
#[cfg_attr(feature = "lax", lax)]
#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Getters)]
#[getset(get = "pub")]
pub struct Manque {
    #[serde(rename = "ressourceID")]
    ressource_id: Option<RessourceID>,
    #[serde(rename = "noteVersion")]
    note_version: Option<NoteVersion>,
}

/// Membre du jury
/// tef:membreJury
/// <http://www.abes.fr/abes/documents/tef>
/// Personne chargée d'évaluer une thèse au moment de sa soutenance
#[cfg_attr(feature = "lax", lax)]
#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Getters)]
#[getset(get = "pub")]
pub struct MembreJury {
    nom: Nom,
    prenom: Prenom,
    #[serde(rename = "autoriteInterne")]
    autorite_interne: Option<Vec<AutoriteInterne>>,
    #[serde(rename = "autoriteExterne")]
    autorite_externe: Option<Vec<AutoriteExterne>>,
}

/// Métadonnées du fichier
/// tef:meta_fichier
/// <http://www.abes.fr/abes/documents/tef>
/// Ensemble des métadonnées techniques caractérisant un fichier informatique
#[cfg_attr(feature = "lax", lax)]
#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Getters)]
#[getset(get = "pub")]
pub struct MetaFichier {
    // divergeances entre le schéma et la réalité
    encodage: Option<Encodage>,
    #[serde(rename = "formatFichier")]
    format_fichier: FormatFichier,
    #[serde(rename = "noteFichier")]
    note_fichier: Option<NoteFichier>,
    #[serde(rename = "structureFichier")]
    // divergeances entre le schéma et la réalité
    structure_fichier: Option<StructureFichier>,
    taille: Taille,
}

/// Nationalité
/// tef:nationalite
/// <http://www.abes.fr/abes/documents/tef>
/// Nationalité de l'auteur de la thèse
#[cfg_attr(feature = "lax", lax)]
#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq)]
pub struct Nationalite(#[serde(rename = "$text")] pub String);

/// Nom
/// tef:nom
/// <http://www.abes.fr/abes/documents/tef>
/// Nom de famille d'une personne physique ou nom d'une personne morale
#[cfg_attr(feature = "lax", lax)]
#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq)]
pub struct Nom(#[serde(rename = "$text")] pub String);

/// Nom de naissance
/// tef:nomDeNaissance
/// <http://www.abes.fr/abes/documents/tef>
/// Nom de famille de l'auteur de la thèse avant un éventuel changement de son état-civil, par mariage par exemple.
#[cfg_attr(feature = "lax", lax)]
#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq)]
pub struct NomDeNaissance(#[serde(rename = "$text")] pub String);

/// Note sur le fichier
/// tef:noteFichier
/// <http://www.abes.fr/abes/documents/tef>
/// Note portant sur le fichier informatique
#[cfg_attr(feature = "lax", lax)]
#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq)]
pub struct NoteFichier(#[serde(rename = "$text")] pub String);

/// Note sur la version
/// tef:noteVersion
/// <http://www.abes.fr/abes/documents/tef>
/// Note décrivant sous forme de texte libre le contenu manquant en cas de version incomplète de la thèse
#[cfg_attr(feature = "lax", lax)]
#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq)]
pub struct NoteVersion(#[serde(rename = "$text")] pub String);

/// Set OAI
/// tef:oai_setSpec
/// <http://www.abes.fr/abes/documents/tef>
/// Set(s) OAI auquel(s) est rattachée la thèse
#[cfg_attr(feature = "lax", lax)]
#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq)]
pub struct OaiSetSpec(#[serde(rename = "$text")] pub String);

/// Partenaire de recherche
/// tef:partenaireRecherche
/// <http://www.abes.fr/abes/documents/tef>
/// Entreprise ou organisme, public ou privé, ayant participé à la réalisation de la thèse par la mise à disposition de moyens.
#[cfg_attr(feature = "lax", lax)]
#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Getters)]
#[getset(get = "pub")]
pub struct PartenaireRecherche {
    #[serde(rename = "@type")]
    r#type: PartenaireRechercheValues,
    #[serde(rename = "@autreType")]
    autre_type: Option<String>,
    nom: Nom,
    #[serde(rename = "autoriteInterne")]
    autorite_interne: Option<Vec<AutoriteInterne>>,
    #[serde(rename = "autoriteExterne")]
    autorite_externe: Option<Vec<AutoriteExterne>>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Hash, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum PartenaireRechercheValues {
    Laboratoire,
    Entreprise,
    Fondation,
    EquipeRecherche,
    #[serde(other)]
    AutreType,
}

impl Default for PartenaireRechercheValues {
    fn default() -> Self {
        Self::AutreType
    }
}

use super::mads::*;

/// Personne
/// tef:personMADS
/// <http://www.abes.fr/abes/documents/tef>
/// Données d'autorité minimales relatives à une personne (morale ou physique) mentionnée dans la notice
#[cfg_attr(feature = "lax", lax)]
#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Getters)]
#[getset(get = "pub")]
pub struct PersonMADS {
    name_part: Vec<NamePart>,
    description: Description,
}

/// Lieu d'édition
/// tef:place
/// <http://www.abes.fr/abes/documents/tef>
/// Adresse de l'organisme responsable d'une édition électronique de la thèse (ville)
#[cfg_attr(feature = "lax", lax)]
#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq)]
pub struct Place(#[serde(rename = "$text")] pub String);

/// Prénom
/// tef:prenom
/// <http://www.abes.fr/abes/documents/tef>
/// Prénom d'une personne physique
#[cfg_attr(feature = "lax", lax)]
#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq)]
pub struct Prenom(#[serde(rename = "$text")] pub String);

/// Président du jury
/// tef:presidentJury
/// <http://www.abes.fr/abes/documents/tef>
/// Personne présidant le jury lors de la soutenance de la thèse et chargée de donner l'avis du jury
#[cfg_attr(feature = "lax", lax)]
#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Getters)]
#[getset(get = "pub")]
pub struct PresidentJury {
    nom: Nom,
    prenom: Prenom,
    #[serde(rename = "autoriteInterne")]
    autorite_interne: Option<Vec<AutoriteInterne>>,
    #[serde(rename = "autoriteExterne")]
    autorite_externe: Option<Vec<AutoriteExterne>>,
}

/// Rapporteur
/// tef:rapporteur
/// <http://www.abes.fr/abes/documents/tef>
/// Personne qui rend compte du contenu de la thèse
#[cfg_attr(feature = "lax", lax)]
#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Getters)]
#[getset(get = "pub")]
pub struct Rapporteur {
    nom: Nom,
    prenom: Prenom,
    #[serde(rename = "autoriteInterne")]
    autorite_interne: Option<Vec<AutoriteInterne>>,
    #[serde(rename = "autoriteExterne")]
    autorite_externe: Option<Vec<AutoriteExterne>>,
}

/// Description de ressource externe
/// tef:ressourceExterneDescription
/// <http://www.abes.fr/abes/documents/tef>
/// Description de ressource externe
#[cfg_attr(feature = "lax", lax)]
#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq)]
// A pour enfant n'importe quel(s) élément(s) Dublin Core (simple et qualifié).
pub struct RessourceExterneDescription(std::collections::HashMap<String, String>);

/// Identifiant de ressource externe
/// tef:ressourceID
/// <http://www.abes.fr/abes/documents/tef>
/// Identifiant de ressource externe décrite en cas de version expurgée de la thèse
#[cfg_attr(feature = "lax", lax)]
#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq)]
pub struct RessourceID(#[serde(rename = "$text")] pub String);

/// Structure du fichier
/// tef:structureFichier
/// <http://www.abes.fr/abes/documents/tef>
/// Structure du fichier informatique
#[cfg_attr(feature = "lax", lax)]
#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq)]
pub struct StructureFichier(#[serde(rename = "$text")] pub String);

#[cfg_attr(feature = "lax", lax)]
#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Getters)]
#[getset(get = "pub")]
pub struct Subdivision {
    #[serde(rename = "@autoriteSource")]
    autorite_source: Option<String>,
    #[serde(rename = "@autoriteExterne")]
    autorite_externe: Option<String>,
    #[serde(rename = "@type")]
    r#type: String,
    #[serde(rename = "$text")]
    text: String,
}

/// Sujet Rameau
/// tef:sujetRameau
/// <http://www.abes.fr/abes/documents/tef>
/// Vedette Rameau
#[cfg_attr(feature = "lax", lax)]
#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Getters)]
#[getset(get = "pub")]
pub struct SujetRameau {
    #[serde(rename = "$value")]
    values: Vec<Option<SujetRameauValues>>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum SujetRameauValues {
    VedetteRameauPersonne(VedetteRameauPersonne),
    VedetteRameauCollectivite(VedetteRameauCollectivite),
    VedetteRameauFamille(VedetteRameauFamille),
    VedetteRameauAuteurTitre(VedetteRameauAuteurTitre),
    VedetteRameauTitre(VedetteRameauTitre),
    VedetteRameauNomCommun(VedetteRameauNomCommun),
    VedetteRameauNomGeographique(VedetteRameauNomGeographique),
    VedetteRameauGenreForme(VedetteRameauGenreForme),
}

/// Vedette Rameau personne physique
/// vedetteRameauPersonne
/// <http://www.abes.fr/abes/documents/tef>
/// Vedette Rameau se rapportant à une personne physique sujet de la thèse
#[cfg_attr(feature = "lax", lax)]
#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Getters)]
#[getset(get = "pub")]
pub struct VedetteRameauPersonne {
    #[serde(rename = "elementdEntree")]
    element_entree: ElementdEntree,
    #[serde(rename = "subdivision")]
    subdivision: Option<Vec<Subdivision>>,
}

/// Vedette Rameau Collectivité
/// vedetteRameauCollectivite
/// <http://www.abes.fr/abes/documents/tef>
/// Vedette Rameau se rapportant à une personne morale ou à un congrès sujet de la thèse
#[cfg_attr(feature = "lax", lax)]
#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Getters)]
#[getset(get = "pub")]
pub struct VedetteRameauCollectivite {
    #[serde(rename = "elementdEntree")]
    element_entree: ElementdEntree,
    #[serde(rename = "subdivision")]
    subdivision: Option<Vec<Subdivision>>,
}

/// Vedette Rameau Famille
/// vedetteRameauFamille
/// <http://www.abes.fr/abes/documents/tef>
/// Vedette Rameau se rapportant à une famille sujet de la thèse
#[cfg_attr(feature = "lax", lax)]
#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Getters)]
#[getset(get = "pub")]
pub struct VedetteRameauFamille {
    #[serde(rename = "elementdEntree")]
    element_entree: ElementdEntree,
    #[serde(rename = "subdivision")]
    subdivision: Option<Vec<Subdivision>>,
}

/// Vedette Rameau Auteur-Titre
/// vedetteRameauAuteurTitre
/// <http://www.abes.fr/abes/documents/tef>
/// Vedette Rameau se rapportant à une oeuvre et à son auteur
#[cfg_attr(feature = "lax", lax)]
#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Getters)]
#[getset(get = "pub")]
pub struct VedetteRameauAuteurTitre {
    #[serde(rename = "elementdEntree")]
    element_entree: ElementdEntree,
    #[serde(rename = "subdivision")]
    subdivision: Option<Vec<Subdivision>>,
}

/// Vedette Rameau Titre
/// vedetteRameauTitre
/// <http://www.abes.fr/abes/documents/tef>
/// Vedette Rameau se rapportant à une œuvre sujet de la thèse
#[cfg_attr(feature = "lax", lax)]
#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Getters)]
#[getset(get = "pub")]
pub struct VedetteRameauTitre {
    #[serde(rename = "elementdEntree")]
    element_entree: ElementdEntree,
    #[serde(rename = "subdivision")]
    subdivision: Option<Vec<Subdivision>>,
}

/// Vedette Rameau Nom Commun
/// tef:vedetteRameauNomCommun
/// <http://www.abes.fr/abes/documents/tef>
/// Vedette Rameau se rapportant à une œuvre sujet de la thèse
#[cfg_attr(feature = "lax", lax)]
#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Getters)]
#[getset(get = "pub")]
pub struct VedetteRameauNomCommun {
    #[serde(rename = "elementdEntree")]
    element_entree: ElementdEntree,
    #[serde(rename = "subdivision")]
    subdivision: Option<Vec<Subdivision>>,
}

/// Vedette Rameau Nom Géographique
/// vedetteRameauNomGeographique
/// <http://www.abes.fr/abes/documents/tef>
/// Vedette Rameau se rapportant à une entité géographique sujet de la thèse
#[cfg_attr(feature = "lax", lax)]
#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Getters)]
#[getset(get = "pub")]
pub struct VedetteRameauNomGeographique {
    #[serde(rename = "elementdEntree")]
    element_entree: ElementdEntree,
    #[serde(rename = "subdivision")]
    subdivision: Option<Vec<Subdivision>>,
}

#[cfg_attr(feature = "lax", lax)]
#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Getters)]
#[getset(get = "pub")]
pub struct VedetteRameauGenreForme {
    #[serde(rename = "elementdEntree")]
    element_entree: ElementdEntree,
    #[serde(rename = "subdivision")]
    subdivision: Option<Vec<Subdivision>>,
}

/// Taille du fichier en octets
/// tef:tailleFichier
/// <http://www.abes.fr/abes/documents/tef>
/// Taille du fichier informatique en octets
#[cfg_attr(feature = "lax", lax)]
#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq)]
pub struct Taille(#[serde(rename = "$text")] pub String);

/// Thèse sur travaux
/// tef:theseSurTravaux
/// <http://www.abes.fr/abes/documents/tef>
/// Thèse constituée en partie d'articles publiés par le doctorant avant la soutenance de sa thèse.
#[cfg_attr(feature = "lax", lax)]
#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq)]
pub struct TheseSurTravaux(#[serde(rename = "$text")] pub String);

/// Diplôme de doctorat
/// tef:thesis.degree
/// <http://www.abes.fr/abes/documents/tef>
/// Ensemble des métadonnées spécifiques au diplôme obtenu à l’issue d’une soutenance de thèse
#[cfg_attr(feature = "lax", lax)]
#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Getters)]
#[getset(get = "pub")]
pub struct ThesisDegree {
    #[serde(rename = "thesis.degree.discipline")]
    discipline: ThesisDegreeDiscipline,
    #[serde(rename = "thesis.degree.grantor")]
    grantor: Vec<ThesisDegreeGrantor>,
    #[serde(rename = "thesis.degree.level")]
    level: ThesisDegreeLevel,
    #[serde(rename = "thesis.degree.name")]
    name: Option<ThesisDegreeName>,
    // step
    #[cfg(feature = "gestion")]
    #[serde(rename = "datePremiereInscriptionDoctorat")]
    date_premiere_inscription_doctorat: Option<DatePremiereInscriptionDoctorat>,
    #[cfg(feature = "gestion")]
    #[serde(rename = "dateInscriptionEtab")]
    date_inscription_etab: Option<DateInscriptionEtab>,
    #[cfg(feature = "gestion")]
    #[serde(rename = "contratDoctoral")]
    contrat_doctoral: Option<String>,
}

/// Discipline
/// tef:thesis.degree.discipline
/// <http://www.abes.fr/abes/documents/tef>
/// Discipline de la thèse
#[cfg_attr(feature = "lax", lax)]
#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Getters)]
#[getset(get = "pub")]
pub struct ThesisDegreeDiscipline {
    #[serde(rename = "@lang")]
    lang: String,
    #[serde(rename = "$text")]
    text: String,
}

/// Etablissement de soutenance
/// tef:thesis.degree.grantor
/// <http://www.abes.fr/abes/documents/tef>
/// Etablissement de soutenance
#[cfg_attr(feature = "lax", lax)]
#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Getters)]
#[getset(get = "pub")]
pub struct ThesisDegreeGrantor {
    nom: Nom,
    #[serde(rename = "autoriteInterne")]
    autorite_interne: Option<Vec<AutoriteInterne>>,
    #[serde(rename = "autoriteExterne")]
    autorite_externe: Option<Vec<AutoriteExterne>>,
}

/// Type de doctorat
/// tef:thesis.degree.level
/// <http://www.abes.fr/abes/documents/tef>
/// Type de doctorat obtenu
#[cfg_attr(feature = "lax", lax)]
#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Getters)]
#[getset(get = "pub")]
pub struct ThesisDegreeLevel {
    #[serde(rename = "$text")]
    text: ThesisDegreeLevelValues,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub enum ThesisDegreeLevelValues {
    #[serde(rename = "Doctorat d'Etat")]
    DoctoratEtat,
    #[serde(rename = "Doctorat")]
    Doctorat,
    #[serde(rename = "Doctorat de troisième cycle")]
    Doctorat3eCycle,
}

impl Default for ThesisDegreeLevelValues {
    fn default() -> Self {
        Self::Doctorat
    }
}

/// Titre obtenu
/// tef:thesis.degree.name
/// <http://www.abes.fr/abes/documents/tef>
/// Titre obtenu
#[cfg_attr(feature = "lax", lax)]
#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq)]
pub struct ThesisDegreeName(#[serde(rename = "$text")] pub String);

/// Métadonnées administratives
/// tef:thesisAdmin
/// <http://www.abes.fr/abes/documents/tef>
/// Ensemble des métadonnées administratives de la thèse
#[cfg_attr(feature = "lax", lax)]
#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Getters)]
#[getset(get = "pub")]
pub struct ThesisAdmin {
    auteur: Auteur,
    identifier: Vec<Identifier>,
    #[serde(rename = "dateAccepted")]
    date_accepted: DateAccepted,
    #[serde(rename = "thesis.degree")]
    thesis_degree: ThesisDegree,
    #[serde(rename = "theseSurTravaux")]
    these_sur_travaux: TheseSurTravaux,
    #[serde(rename = "avisJury")]
    avis_jury: AvisJury,
    #[serde(rename = "directeurThese")]
    directeur_these: Vec<DirecteurThese>,
    #[serde(rename = "presidentJury")]
    president_jury: Option<PresidentJury>,
    #[serde(rename = "membreJury")]
    membre_jury: Vec<MembreJury>,
    rapporteur: Vec<Rapporteur>,
    #[serde(rename = "ecoleDoctorale")]
    ecole_doctorale: Vec<EcoleDoctorale>,
    #[serde(rename = "partenaireRecherche")]
    partenaire_recherche: Vec<PartenaireRecherche>,
    #[serde(rename = "oaiSetSpec")]
    oai_set_spec: Vec<OaiSetSpec>,
    #[serde(rename = "MADSAuthority ")]
    mads_authority: Option<Vec<MADSAuthority>>,
    // step
    #[cfg(feature = "gestion")]
    #[serde(rename = "vie")]
    vie: Vie,
}

#[cfg_attr(feature = "lax", lax)]
#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Getters)]
#[getset(get = "pub")]
pub struct DatePremiereInscriptionDoctorat {
    #[serde(rename = "@type")]
    r#type: Option<String>,
    #[serde(rename = "$text")]
    text: String,
}

#[cfg_attr(feature = "lax", lax)]
#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Getters)]
#[getset(get = "pub")]
pub struct DateInscriptionEtab {
    #[serde(rename = "@type")]
    r#type: Option<String>,
    #[serde(rename = "$text")]
    text: String,
}

#[cfg_attr(feature = "lax", lax)]
#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Getters)]
#[getset(get = "pub")]
pub struct Vie {
    #[serde(rename = "@derogationDixAns")]
    derogation_dix_ans: String,
    #[serde(rename = "@commentDixAns")]
    comment_dix_ans: Option<String>,
    #[serde(rename = "soutenancePrevue")]
    soutenance_prevue: SoutenancePrevue,
    #[serde(rename = "dateAbandon")]
    date_abandon: Option<String>,
}

#[cfg_attr(feature = "lax", lax)]
#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Getters)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct SoutenancePrevue {
    date_prevue: Option<String>,
    heure_prevue: Option<String>,
    lieu_prevue: Option<String>,
    // publiciteSoutenance
}

/// Description de la thèse
/// tef:thesisRecord
/// <http://www.abes.fr/abes/documents/tef>
/// Ensemble des métadonnées descriptives relatives à la thèse
#[cfg_attr(feature = "lax", lax)]
#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Getters)]
#[getset(get = "pub")]
pub struct ThesisRecord {
    coverage: Option<Vec<Coverage>>,
    title: Title,
    alternative: Option<Vec<Alternative>>,
    subject: Option<Vec<Subject>>,
    #[serde(rename = "sujetRameau")]
    sujet_rameau: Option<SujetRameau>,
    #[serde(rename = "abstract")]
    r#abstract: Option<Vec<Abstract>>,
    #[serde(rename = "type")]
    r#type: Vec<Type>,
    language: Vec<Language>,
}

/// Version
/// tef:version
/// <http://www.abes.fr/abes/documents/tef>
/// Ensemble des métadonnées descriptives relatives à une version incomplète de la thèse
#[cfg_attr(feature = "lax", lax)]
#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Getters)]
#[getset(get = "pub")]
pub struct Version {
    manque: Vec<Manque>,
    replaces: Vec<Replaces>,
}
