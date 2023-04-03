use quick_xml::de::from_str;

pub mod dc;
pub mod dcterms;
pub mod mads;
pub mod mets;
pub mod mets_rights;
pub mod tef;

use mets::*;
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    // let stef_string: String = fs::read_to_string("100054.xml")?.parse()?;
    // let stef_string: String = fs::read_to_string("320864.xml")?.parse()?;
    let stef_string: String = fs::read_to_string("stef.xml")?.parse()?;
    let stef: Mets = from_str(&stef_string)?;

    let date_accepted: Option<&DateAccepted> = (&stef).into();
    let title: Option<&Title> = (&stef).into();
    let subjects: Vec<&Subject> = (&stef).into();
    let abstracts: Vec<&Abstract> = (&stef).into();
    let language : Vec<&Language> = (&stef).into();
    let grantors : Vec<&ThesisDegreeGrantor> = (&stef).into();
    let discipline : Option<&ThesisDegreeDiscipline> = (&stef).into();
    let directeurs : Vec<&DirecteurThese> = (&stef).into();
    let rapporteurs : Vec<&Rapporteur> = (&stef).into();
    let president : Option<&PresidentJury> = (&stef).into();
    let membre_jury: Vec<&MembreJury> = (&stef).into();
    let ecoles_doctorales : Vec<&EcoleDoctorale> = (&stef).into();
    let partenaires_recherche : Vec<&PartenaireRecherche> = (&stef).into();
    let oai_set_specs : Vec<&OaiSetSpec> = (&stef).into();

    println!("\n date_accepted : {:?}", date_accepted);
    println!("\n title : {:?}", title);
    println!("\n subjects : {:?}", subjects);
    println!("\n abstracts : {:?}", abstracts);
    println!("\n language : {:?}", language);
    println!("\n discipline : {:?}", discipline);
    println!("\n grantors : {:?}", grantors);
    println!("\n directeurs : {:?}", directeurs);
    println!("\n rapporteurs : {:?}", rapporteurs);
    println!("\n president : {:?}", president);
    println!("\n membre_jury : {:?}", membre_jury);
    println!("\n ecoles_doctorales : {:?}", ecoles_doctorales);
    println!("\n partenaires_recherche : {:?}", partenaires_recherche);
    println!("\n oai_set_specs : {:?}", oai_set_specs);

    // println!("{:#?}", &stef);

    // let auteurs = EAuteurs::from(stef);
    // println!("{:#?}", auteurs);
    Ok(())
}

use ::tef::*;
use dcterms::*;
use dc::*;

