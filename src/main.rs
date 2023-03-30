use quick_xml::de::from_str;
use serde::Deserialize;

mod dc;
mod dcterms;
mod mads;
mod mets;
mod metsRights;
mod tef;
use tef::*;

use mets::*;

use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    // let stef_string: String = fs::read_to_string("100054.xml")?.parse()?;
    // let stef_string: String = fs::read_to_string("320864.xml")?.parse()?;
    let stef_string: String = fs::read_to_string("stef.xml")?.parse()?;
    let stef: Mets = from_str(&stef_string)?;

    println!("{:#?}", &stef);
    
    // let auteurs = EAuteurs::from(stef);
    // println!("{:#?}", auteurs);
    Ok(())
}

