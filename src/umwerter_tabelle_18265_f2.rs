use crate::trait_umwerter::Umwerter;
use std::collections::HashMap;
use std::vec::Vec;

use crate::tools;
use crate::errors::UmwerterError;

pub struct UmwerterTabelle18265F2<'a> {
    daten: Vec<HashMap<&'a str, f64>>,
}

impl<'a> UmwerterTabelle18265F2<'a> {
    pub fn new() -> Result<UmwerterTabelle18265F2<'a>,UmwerterError> {
        let mut tabelle = UmwerterTabelle18265F2 { daten: Vec::new() };
        let csv_data = include_str!("18265_F2.csv");
        tools::initialize(&tabelle.interne_einheiten(), &mut tabelle.daten, csv_data)?;
        Ok(tabelle)
    }
}

impl<'a> Umwerter<'a> for UmwerterTabelle18265F2<'a> {
    fn data(&self) -> &Vec<HashMap<&'a str, f64>> {
        &self.daten
    }

    fn externe_einheiten(&self) -> Vec<&'a str> {
        vec![
            "HV", "HRB", "HRF", "HR15T", "HR30T", "HR45T", "HBW",
        ]
    }

    fn interne_einheiten(&self) -> Vec<&'a str> {
        vec![
            "hv", "hrb", "hrf", "hr15t", "hr30t", "hr45t", "hbw", 
        ]
    }
}
