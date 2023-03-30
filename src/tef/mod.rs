use serde::Deserialize;
use crate::{dc::*, dcterms::*};

/// Auteur
/// tef:auteur
/// http://www.abes.fr/abes/documents/tef
/// Auteur de la thèse
#[derive(Debug, Clone, Deserialize)]
pub struct Auteur {
    nom: Nom,
    prenom: Prenom,
    #[serde(rename = "nomDeNaissance")]
    nom_de_naissance: Option<NomDeNaissance>,
    #[serde(rename = "dateNaissance")]
    date_naissance: Option<DateNaissance>,
    nationalite: Option<String>,
    #[serde(rename = "autoriteExterne")]
    autorite_externe: Option<Vec<AutoriteExterne>>
}

/// Identifiant de notice d’autorité externe
/// tef:autoriteExterne
/// http://www.abes.fr/abes/documents/tef
/// Identifiant de notice d'autorité issue d'un système externe 
#[derive(Debug, Clone, Deserialize)]
pub struct AutoriteExterne {
    #[serde(rename = "@autoriteSource")]
    autorite_source: String,
    #[serde(rename = "$text")]
    text: String
}

/// Autre format de fichier
/// tef:autreFormatFichier
/// http://www.abes.fr/abes/documents/tef
/// Format de fichier informatique qui n'est pas prévu dans la liste des valeurs que peut prendre l'élément tef:formatFichier 
#[derive(Debug, Clone, Deserialize)]
pub struct AutoriteFormatFichier(#[serde(rename = "$text")] String);

/// Identifiant d'un bloc de données d'autorité internes
/// tef:autoriteInterne
/// http://www.abes.fr/abes/documents/tef
/// Identifiant d'un bloc de données d'autorité tef:MADSAuthority 
#[derive(Debug, Clone, Deserialize)]
pub struct AutoriteInterne(#[serde(rename = "$text")] String);

/// Avis du jury
/// tef:avisJury
/// http://www.abes.fr/abes/documents/tef
/// Avis du jury autorisant ou non la diffusion de la thèse après la soutenance 
#[derive(Debug, Clone, Deserialize)]
pub struct AvisJury(#[serde(rename = "$text")] String);

/// Date de naissance
/// tef:dateNaissance
/// http://www.abes.fr/abes/documents/tef
/// Date de naissance de l'auteur de la thèse
#[derive(Debug, Clone, Deserialize)]
pub struct DateNaissance(#[serde(rename = "$text")] String);

/// Directeur de thèse
/// tef:directeurThèse
/// http://www.abes.fr/abes/documents/tef
/// Personne qui encadre et oriente le travail du doctorant 
#[derive(Debug, Clone, Deserialize)]
pub struct DirecteurThese {
    nom: Nom,
    prenom: Prenom,
    #[serde(rename = "autoriteInterne")]
    autorite_interne: Option<Vec<AutoriteInterne>>,
    #[serde(rename = "autoriteExterne")]
    autorite_externe: Option<Vec<AutoriteExterne>>
}

/// Ecole doctorale
/// tef:ecoleDoctorale
/// http://www.abes.fr/abes/documents/tef
/// Ecole doctorale au sein de laquelle s’est déroulée la recherche du doctorant
#[derive(Debug, Clone, Deserialize)]
pub struct EcoleDoctorale {
    nom: Nom,
    #[serde(rename = "autoriteInterne")]
    autorite_interne: Option<Vec<AutoriteInterne>>,
    #[serde(rename = "autoriteExterne")]
    autorite_externe: Option<Vec<AutoriteExterne>>
}

/// Editeur
/// tef:editeur
/// http://www.abes.fr/abes/documents/tef
/// Organisme responsable de l’édition électronique de la thèse 
#[derive(Debug, Clone, Deserialize)]
pub struct Editeur {
    nom: Nom,
    place: Place,
    #[serde(rename = "autoriteInterne")]
    autorite_interne: Option<Vec<AutoriteInterne>>,
    #[serde(rename = "autoriteExterne")]
    autorite_externe: Option<Vec<AutoriteExterne>>
}

/// Edition
/// tef:edition
/// http://www.abes.fr/abes/documents/tef
/// Ensemble des métadonnées relatives à une édition électronique de la thèse
#[derive(Debug, Clone, Deserialize)]
pub struct Edition {
    #[serde(rename = "$value")]
    values: Vec<EditionValues>
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all="lowercase")]
pub enum EditionValues {
    Medium(Medium),
    Extent(Extent),
    Identifier(Identifier),
    Issued(Issued),
    Replaces(Replaces),
    Editeur(Editeur),
}

#[derive(Debug, Clone, Deserialize)]
pub struct ElementdEntree;

/// Encodage
/// tef:encodage
/// http://www.abes.fr/abes/documents/tef
/// Type d'encodage du fichier
#[derive(Debug, Clone, Deserialize)]
pub struct Encodage {
    #[serde(rename = "$text")]
    text: EncodageValues
}

#[derive(Debug, Clone, Deserialize)]
pub enum EncodageValues {
    ASCII,
    #[serde(rename = "Latin 1")]
    Latin1,
    Unicode
}

/// Format de fichier
/// tef:formatFichier
/// http://www.abes.fr/abes/documents/tef
/// Format du fichier
#[derive(Debug, Clone, Deserialize)]
pub struct FormatFichier {
    #[serde(rename = "$text")]
    text: FormatFichierValues
}

#[derive(Debug, Clone, Deserialize)]
pub enum FormatFichierValues {
    OpenDocument,
    PDF,
    #[serde(rename= "PDF/A")]
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
    AutreFormat
}

/// Autorité
/// tef:MADSAuthority
/// http://www.abes.fr/abes/documents/tef
/// Données d'autorité minimales relatives à une personne (morale ou physique) mentionnée dans la notice 
#[derive(Debug, Clone, Deserialize)]
pub struct MADSAuthority {
    #[serde(rename = "@authorityID")]
    authority_id: String,
    #[serde(rename = "@type")]
    r#type: String,
    #[serde(rename = "$value")]
    value: PersonMADS
}

/// Manque
/// tef:manque
/// http://www.abes.fr/abes/documents/tef
/// Indique une partie manquante en cas de version incomplète. 
#[derive(Debug, Clone, Deserialize)]
pub struct Manque {
    #[serde(rename = "ressourceID")]
    ressource_id: Option<RessourceID>,
    #[serde(rename = "noteVersion")]
    note_version: Option<NoteVersion>
}

/// Membre du jury
/// tef:membreJury
/// http://www.abes.fr/abes/documents/tef
/// Personne chargée d'évaluer une thèse au moment de sa soutenance
#[derive(Debug, Clone, Deserialize)]
pub struct MembreJury {
    nom: Nom,
    prenom: Prenom,
    #[serde(rename = "autoriteInterne")]
    autorite_interne: Option<Vec<AutoriteInterne>>,
    #[serde(rename = "autoriteExterne")]
    autorite_externe: Option<Vec<AutoriteExterne>>
}

/// Métadonnées du fichier
/// tef:meta_fichier
/// http://www.abes.fr/abes/documents/tef
/// Ensemble des métadonnées techniques caractérisant un fichier informatique 
#[derive(Debug, Clone, Deserialize)]
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
    taille: Taille
}

/// Nationalité
/// tef:nationalite
/// http://www.abes.fr/abes/documents/tef
/// Nationalité de l'auteur de la thèse
#[derive(Debug, Clone, Deserialize)]
pub struct Nationalite(#[serde(rename = "$text")] String);

/// Nom
/// tef:nom
/// http://www.abes.fr/abes/documents/tef
/// Nom de famille d'une personne physique ou nom d'une personne morale
#[derive(Debug, Clone, Deserialize)]
pub struct Nom(#[serde(rename = "$text")] String);

/// Nom de naissance
/// tef:nomDeNaissance
/// http://www.abes.fr/abes/documents/tef
/// Nom de famille de l'auteur de la thèse avant un éventuel changement de son état-civil, par mariage par exemple. 
#[derive(Debug, Clone, Deserialize)]
pub struct NomDeNaissance(#[serde(rename = "$text")] String);

/// Note sur le fichier
/// tef:noteFichier
/// http://www.abes.fr/abes/documents/tef
/// Note portant sur le fichier informatique
#[derive(Debug, Clone, Deserialize)]
pub struct NoteFichier(#[serde(rename = "$text")] String);

/// Note sur la version
/// tef:noteVersion
/// http://www.abes.fr/abes/documents/tef
/// Note décrivant sous forme de texte libre le contenu manquant en cas de version incomplète de la thèse
#[derive(Debug, Clone, Deserialize)]
pub struct NoteVersion(#[serde(rename = "$text")] String);

/// Set OAI
/// tef:oai_setSpec
/// http://www.abes.fr/abes/documents/tef
/// Set(s) OAI auquel(s) est rattachée la thèse
#[derive(Debug, Clone, Deserialize)]
pub struct OaiSetSpec(#[serde(rename = "$text")] String);

/// Partenaire de recherche
/// tef:partenaireRecherche
/// http://www.abes.fr/abes/documents/tef
/// Entreprise ou organisme, public ou privé, ayant participé à la réalisation de la thèse par la mise à disposition de moyens.
#[derive(Debug, Clone, Deserialize)]
pub struct PartenaireRecherche {
    #[serde(rename = "@type")]
    r#type: String,
    #[serde(rename = "@autreType")]
    autre_type: Option<String>,
    nom: String,
    #[serde(rename = "autoriteInterne")]
    autorite_interne: Option<Vec<AutoriteInterne>>,
    #[serde(rename = "autoriteExterne")]
    autorite_externe: Option<Vec<AutoriteExterne>>
}

use super::mads::*;

/// Personne
/// tef:personMADS
/// http://www.abes.fr/abes/documents/tef
/// Données d'autorité minimales relatives à une personne (morale ou physique) mentionnée dans la notice 
#[derive(Debug, Clone, Deserialize)]
pub struct PersonMADS {
    name_part: Vec<NamePart>,
    description: Description
}

/// Lieu d'édition
/// tef:place
/// http://www.abes.fr/abes/documents/tef
/// Adresse de l'organisme responsable d'une édition électronique de la thèse (ville) 
#[derive(Debug, Clone, Deserialize)]
pub struct Place(#[serde(rename = "$text")] String);

/// Prénom
/// tef:prenom
/// http://www.abes.fr/abes/documents/tef
/// Prénom d'une personne physique
#[derive(Debug, Clone, Deserialize)]
pub struct Prenom(#[serde(rename = "$text")] String);

/// Président du jury
/// tef:presidentJury
/// http://www.abes.fr/abes/documents/tef
/// Personne présidant le jury lors de la soutenance de la thèse et chargée de donner l'avis du jury 
#[derive(Debug, Clone, Deserialize)]
pub struct PresidentJury {
    nom: Nom,
    prenom: Prenom,
    #[serde(rename = "autoriteInterne")]
    autorite_interne: Option<Vec<AutoriteInterne>>,
    #[serde(rename = "autoriteExterne")]
    autorite_externe: Option<Vec<AutoriteExterne>>
}

/// Rapporteur
/// tef:rapporteur
/// http://www.abes.fr/abes/documents/tef
/// Personne qui rend compte du contenu de la thèse
#[derive(Debug, Clone, Deserialize)]
pub struct Rapporteur {
    nom: Nom,
    prenom: Prenom,
    #[serde(rename = "autoriteInterne")]
    autorite_interne: Option<Vec<AutoriteInterne>>,
    #[serde(rename = "autoriteExterne")]
    autorite_externe: Option<Vec<AutoriteExterne>>
}

/// Description de ressource externe
/// tef:ressourceExterneDescription
/// http://www.abes.fr/abes/documents/tef
/// Description de ressource externe
#[derive(Debug, Clone, Deserialize)]
// A pour enfant n'importe quel(s) élément(s) Dublin Core (simple et qualifié). 
pub struct RessourceExterneDescription(std::collections::HashMap<String, String>);

/// Identifiant de ressource externe
/// tef:ressourceID
/// http://www.abes.fr/abes/documents/tef
/// Identifiant de ressource externe décrite en cas de version expurgée de la thèse
#[derive(Debug, Clone, Deserialize)]
pub struct RessourceID(#[serde(rename = "$text")] String);

/// Structure du fichier
/// tef:structureFichier
/// http://www.abes.fr/abes/documents/tef
/// Structure du fichier informatique
#[derive(Debug, Clone, Deserialize)]
pub struct StructureFichier(#[serde(rename = "$text")] String);

#[derive(Debug, Clone, Deserialize)]
pub struct Subdivision;

#[derive(Debug, Clone, Deserialize)]
pub struct SujetRameau;

/// Taille du fichier en octets
/// tef:tailleFichier
/// http://www.abes.fr/abes/documents/tef
/// Taille du fichier informatique en octets
#[derive(Debug, Clone, Deserialize)]
pub struct Taille(#[serde(rename = "$text")] String);

/// Thèse sur travaux
/// tef:theseSurTravaux
/// http://www.abes.fr/abes/documents/tef
/// Thèse constituée en partie d'articles publiés par le doctorant avant la soutenance de sa thèse.
#[derive(Debug, Clone, Deserialize)]
pub struct TheseSurTravaux(#[serde(rename = "$text")] String);

/// Diplôme de doctorat
/// tef:thesis.degree
/// http://www.abes.fr/abes/documents/tef
/// Ensemble des métadonnées spécifiques au diplôme obtenu à l’issue d’une soutenance de thèse
#[derive(Debug, Clone, Deserialize)]
pub struct ThesisDegree {
    #[serde(rename = "thesis.degree.discipline")]
    discipline: ThesisDegreeDiscipline,
    #[serde(rename = "thesis.degree.grantor")]
    grantor: Vec<ThesisDegreeGrantor>,
    #[serde(rename = "thesis.degree.level")]
    level: ThesisDegreeLevel,
    #[serde(rename = "thesis.degree.name")]
    name: Option<ThesisDegreeName>
}

/// Discipline
/// tef:thesis.degree.discipline
/// http://www.abes.fr/abes/documents/tef
/// Discipline de la thèse

#[derive(Debug, Clone, Deserialize)]
pub struct ThesisDegreeDiscipline{
    #[serde(rename = "@lang")]
    lang: String,
    #[serde(rename = "$text")]
    text: String
}

/// Etablissement de soutenance
/// tef:thesis.degree.grantor
/// http://www.abes.fr/abes/documents/tef
/// Etablissement de soutenance
#[derive(Debug, Clone, Deserialize)]
pub struct ThesisDegreeGrantor {
    nom: Nom,
    #[serde(rename = "autoriteInterne")]
    autorite_interne: Option<Vec<AutoriteInterne>>,
    #[serde(rename = "autoriteExterne")]
    autorite_externe: Option<Vec<AutoriteExterne>>
}

/// Type de doctorat
/// tef:thesis.degree.level
/// http://www.abes.fr/abes/documents/tef
/// Type de doctorat obtenu
#[derive(Debug, Clone, Deserialize)]
pub struct ThesisDegreeLevel {
    #[serde(rename = "$text")]
    text: ThesisDegreeLevelValues
}

#[derive(Debug, Clone, Deserialize)]
pub enum ThesisDegreeLevelValues {
    #[serde(rename= "Doctorat d'Etat")]
    DoctoratEtat,
    #[serde(rename= "Doctorat")]
    Doctorat,
    #[serde(rename = "Doctorat de troisième cycle")]
    Doctorat3eCycle
}

/// Titre obtenu
/// tef:thesis.degree.name
/// http://www.abes.fr/abes/documents/tef
/// Titre obtenu
#[derive(Debug, Clone, Deserialize)]
pub struct ThesisDegreeName(#[serde(rename = "$text")] String);

/// Métadonnées administratives
/// tef:thesisAdmin
/// http://www.abes.fr/abes/documents/tef
/// Ensemble des métadonnées administratives de la thèse
#[derive(Debug, Clone, Deserialize)]
pub struct ThesisAdmin {
    auteur: Auteur,
    identifier: Vec<Identifier>,
    #[serde(rename ="dateAccepted")]
    date_accepted: DateAccepted,
    #[serde(rename ="thesis.degree")]
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
    mads_authority: Option<Vec<MADSAuthority>>
}

/// Description de la thèse
/// tef:thesisRecord
/// http://www.abes.fr/abes/documents/tef
/// Ensemble des métadonnées descriptives relatives à la thèse
#[derive(Debug, Clone, Deserialize)]
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
    language: Vec<Language>
}

#[derive(Debug, Clone, Deserialize)]    
pub struct VedetteRameauNomCommun;

/// Version
/// tef:version
/// http://www.abes.fr/abes/documents/tef
/// Ensemble des métadonnées descriptives relatives à une version incomplète de la thèse 
#[derive(Debug, Clone, Deserialize)]
pub struct Version {
    manque: Vec<Manque>,
    replaces: Vec<Replaces>
}