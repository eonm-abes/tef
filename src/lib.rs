use mets::Mets;

pub mod dc;
pub mod dcterms;
pub mod mads;
pub mod mets;
pub mod mets_rights;
pub mod tef;

#[cfg(feature = "extractors")]
pub mod extractors;

#[cfg(feature = "gestion")]
pub mod gestion;

pub fn parse_tef(input: &str) -> Result<Mets, Box<dyn std::error::Error>> {
    Ok(quick_xml::de::from_str(input)?)
}
