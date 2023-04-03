use serde::Deserialize;
use crate::mets_rights::*;
use lax_derive::lax;
use getset::Getters;

/// Agent
/// mets:agent
/// http://www.loc.gov/METS/
/// L'élément mets:agent permet de mentionner la (ou les) personne(s) ayant contribué au document METS et de préciser son (leur) rôle. 
#[lax]
#[derive(Debug, Clone, Deserialize, Default, Getters)]
#[getset(get="pub")]
#[getset(get="pub")]
pub struct Agent {
    #[serde(rename = "@ROLE")]
    role: String,
    #[serde(rename = "@OTHERROLE")]
    otherrole: Option<String>,
    #[serde(rename = "$value")]
    values: Vec<AgentValues>
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename = "lowercase")]
pub enum AgentValues {
    #[serde(rename = "name")]
    Name(Name),
    #[serde(rename = "note")]
    Note(Note)
}

/// Autre identifiant de notice
/// mets:altRecordID
/// http://www.loc.gov/METS/
/// Identifiant de notice alternatif. Cet élément permet d'assigner des identifiants alternatifs au document METS. Ces identifiants s'ajoutent à l'identifiant primaire stocké dans l'attribut OBJID de la racine mets:mets . 
#[lax]
#[derive(Debug, Clone, Deserialize, Default, Getters)]
#[getset(get="pub")]
pub struct AltRecordID {
    #[serde(rename = "@ID")]
    id: String,
    #[serde(rename = "@TYPE")]
    r#type: Option<String>
}

/// Section des métadonnées de gestion
/// mets:amdSec
/// http://www.loc.gov/METS/
/// Section qui contient tous les blocs de métadonnées de gestion des entités TEF.
#[lax]
#[derive(Debug, Clone, Deserialize, Default, Getters)]
#[getset(get="pub")]
pub struct AmdSec {
    #[serde(rename = "$value")]
    values: Vec<AmdSecValues>
}

#[derive(Debug, Clone, Deserialize)]
pub enum AmdSecValues {
    #[serde(rename = "techMD")]
    TechMD(TechMD),
    #[serde(rename = "rightsMD")]
    RightsMD(RightsMD)
}

/// Division
/// mets:div
/// http://www.loc.gov/METS/
/// Dans TEF, chaque mets:div de la carte de structure ( mets:structMap ) représente une entité du modèle TEF (la thèse, une version, une édition ou une ressource externe).
#[lax]
#[derive(Debug, Clone, Deserialize, Default, Getters)]
#[getset(get="pub")]
pub struct Div {
    #[serde(rename = "@TYPE")]
    r#type: String,
    #[serde(rename = "@DMDID")]
    dmdid: Option<String>,
    #[serde(rename = "@ADMID")]
    amdid: Option<String>,
    #[serde(rename = "@ID")]
    id: Option<String>,
    #[serde(rename = "@CONTENTIDS")]
    contentids: Option<String>,
    #[serde(rename = "$value")]
    values: Vec<DivValues>
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DivValues {
    Div(Box<Div>),
    Fptr(Fptr)
}

/// Bloc de métadonnées descriptives
/// mets:dmdSec
/// http://www.loc.gov/METS/
/// Bloc contenant les métadonnées descriptives d'une entité TEF
#[lax]
#[derive(Debug, Clone, Deserialize, Default, Getters)]
#[getset(get="pub")]
pub struct DmdSec {
    #[serde(rename = "@ID")]
    id: String,
    #[serde(rename = "@CREATED")]
    created: Option<String>,
    #[serde(rename = "$value")]
    value: MdWrap,
}

/// Fichier
/// mets:file
/// http://www.loc.gov/METS/
/// Fichier informatique composant une édition électronique 
#[lax]
#[derive(Debug, Clone, Deserialize, Default, Getters)]
#[getset(get="pub")]
pub struct File {
    #[serde(rename = "@ID")]
    id: String,
    #[serde(rename = "@MIMETYPE")]
    mimetype: Option<String>,
    #[serde(rename = "@ADMID")]
    admid: String,
    #[serde(rename = "@USE")]
    r#use: Option<String>,
    #[serde(rename = "$value")]
    value: FLocat,
}

/// Groupe de fichiers
/// mets:fileGrp
/// http://www.loc.gov/METS/
/// Cet élément permet de regrouper des fichiers ( mets:file ) 
#[lax]
#[derive(Debug, Clone, Deserialize, Default, Getters)]
#[getset(get="pub")]
pub struct FileGrp {
    #[serde(rename = "@USE")]
    r#use: Option<String>,
    #[serde(rename = "@ID")]
    id: Option<String>,
    #[serde(rename = "$value")]
    values: Vec<FileGrpValues>
}

#[derive(Debug, Clone, Deserialize)]
pub enum FileGrpValues {
    #[serde(rename = "file")]
    File(File),
    #[serde(rename = "fileGrp")]
    FileGrp(FileGrp)
}

/// Section des fichiers
/// mets:fileSec
/// http://www.loc.gov/METS/
/// Inventaire de tous les fichiers de toutes les éditions de la thèse 
#[lax]
#[derive(Debug, Clone, Deserialize, Default, Getters)]
#[getset(get="pub")]
pub struct FileSec {
    #[serde(rename = "@ID")]
    id: Option<String>,
    #[serde(rename = "$value")]
    value: Vec<FileGrp>
}

/// Emplacement du fichier
/// mets:FLocat
/// http://www.loc.gov/METS/
/// Cet élément pointe vers l'emplacement d'un fichier. 
#[lax]
#[derive(Debug, Clone, Deserialize, Default, Getters)]
#[getset(get="pub")]
pub struct FLocat {
    #[serde(rename = "@LOCTYPE")]
    loctype: String,
    #[serde(rename = "@OTHERLOCTYPE")]
    otherloctype: Option<String>,
    #[serde(rename = "@href")]
    href: String,
}

/// Pointeur de fichier
/// mets:fptr
/// http://www.loc.gov/METS/
/// L'élément mets:fptr associe un élément mets:div avec le(s) fichier(s) qui y correspond(ent).
#[lax]
#[derive(Debug, Clone, Deserialize, Default, Getters)]
#[getset(get="pub")]
pub struct Fptr {
    #[serde(rename = "@FILEID")]
    fileid: Option<String>
}

/// Enveloppe de métadonnées
/// mets:mdWrap
/// http://www.loc.gov/METS/
/// mets:mdWrap est un élément générique utilisé tout au long du schéma METS. Il permet de placer des métadonnées provenant de n'importe quel schéma dans un document METS. Dans TEF, ces métadonnées sont encodées en XML, via l'élément mets:xmlData . 
#[lax]
#[derive(Debug, Clone, Deserialize, Default, Getters)]
#[getset(get="pub")]
pub struct MdWrap {
    #[serde(rename = "$value")]
    value: XmlData
}

/// Document TEF
/// mets:mets
/// http://www.loc.gov/METS/
/// Cet élément est la racine d'une notice TEF.
#[lax]
#[derive(Debug, Clone, Deserialize, Default, Getters)]
#[getset(get="pub")]
pub struct Mets {
    #[serde(rename = "@OBJID")]
    objid: Option<String>,
    #[serde(rename = "@PROFILE")]
    profile: Option<String>,
    #[serde(rename = "$value")]
    values: Vec<MetsValues>
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum MetsValues {
    MetsHdr(MetsHdr),
    DmdSec(DmdSec),
    AmdSec(AmdSec),
    FileSec(FileSec),
    StructMap(StructMap)
}

/// en-tête METS
/// mets:metsHdr
/// http://www.loc.gov/METS/
/// Cet élément contient les métadonnées sur le document METS lui même.
#[lax]
#[derive(Debug, Clone, Deserialize, Default, Getters)]
#[getset(get="pub")]
pub struct MetsHdr {
    #[serde(rename="@CREATEDATE")]
    createdate: Option<String>,
    #[serde(rename="@LASTMODDATE")]
    lastmoddate: Option<String>,
    #[serde(rename = "@RECORDSTATUS")]
    recordstatus: Option<String>,
    #[serde(rename = "$value")]
    values: Vec<MetsHdrValues>,
}

#[derive(Debug, Clone, Deserialize)]
pub enum MetsHdrValues {
    #[serde(rename = "agent")]
    Agent(Agent),
    #[serde(rename = "altRecordID")]
    AltRecordID(AltRecordID)
}

/// Nom de l'agent METS
/// mets:name
/// http://www.loc.gov/METS/
/// Nom complet de l'agent (auteur, éditeur, ...) intervenant sur le document METS.
#[lax]
#[derive(Debug, Clone, Deserialize, Default)]
pub struct Name(#[serde(rename = "$value")] String);

/// Note sur l'agent METS
/// mets:note
/// http://www.loc.gov/METS/
/// Toutes informations complémentaires sur les activités de l'agent (auteur, éditeur, ... ) intervenant sur la notice METS. 
#[lax]
#[derive(Debug, Clone, Deserialize, Default)]
pub struct Note(#[serde(rename = "$value")] String);

/// Bloc de métadonnées de droits
/// mets:rightsMD
/// http://www.loc.gov/METS/
/// Bloc contenant les métadonnées de droits qui s'appliquent à une entité TEF
#[lax]
#[derive(Debug, Clone, Deserialize, Default, Getters)]
#[getset(get="pub")]
pub struct RightsMD {
    #[serde(rename = "@ID")]
    id: Option<String>,
    #[serde(rename = "$value")]    
    value: MdWrap
}

/// Carte de structure
/// mets:structMap
/// http://www.loc.gov/METS/
/// La carte de structure établit l'inventaire de toutes les entités TEF (la thèse, une version, une édition ou une ressource externe). 
#[lax]
#[derive(Debug, Clone, Deserialize, Default, Getters)]
#[getset(get="pub")]
pub struct StructMap {
    #[serde(rename = "@TYPE")]
    r#type: String,
    #[serde(rename = "$value")] 
    value: Vec<Div>
}

/// Bloc de métadonnées techniques ou administratives
/// mets:techMD
/// http://www.loc.gov/METS/
/// Un bloc mets:techMD contient soit les métadonnées administratives qui se rapportent à la thèse, soit les métadonnées de conservation qui se rapportent à chaque fichier de l'édition d'archivage.
#[lax]
#[derive(Debug, Clone, Deserialize, Default, Getters)]
#[getset(get="pub")]
pub struct TechMD {
    #[serde(rename = "@ID")]
    id: String,
    #[serde(rename = "$value")] 
    value: MdWrap
}

/// Enveloppe des métadonnées XML
/// mets:xmlData
/// http://www.loc.gov/METS/
/// Élément contenant les métadonnées encodées en XML. 
#[lax]
#[derive(Debug, Clone, Deserialize, Default, Getters)]
#[getset(get="pub")]
pub struct XmlData{
    #[serde(rename = "$value")]
    value: XmlDataValues
}

use super::tef::*;

#[derive(Debug, Clone, Deserialize)]
#[derive(Default)]
pub enum XmlDataValues {
    #[serde(rename = "thesisRecord")]
    ThesisRecord(ThesisRecord),
    #[serde(rename = "version")]
    Version(Version),
    #[serde(rename = "edition")]
    Edition(Edition),
    #[serde(rename = "ressourceExterneDescription")]
    RessourceExterneDescription(RessourceExterneDescription),
    #[serde(rename = "thesisAdmin")]
    ThesisAdmin(ThesisAdmin),
    #[serde(rename = "meta_fichier")]
    MetaFichier(MetaFichier),
    #[serde(rename = "RightsDeclarationMD")]
    RightsDeclarationMD(RightsDeclarationMD),
    #[serde(other)]
    #[default]
    Other
}


