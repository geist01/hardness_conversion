use crate::trait_umwerter::Umwerter;
use std::collections::HashMap;
use std::vec::Vec;

use crate::tools;
use crate::errors::UmwerterError;

pub struct UmwerterTabelle18265D2<'a> {
    daten: Vec<HashMap<&'a str, f64>>,
}

impl<'a> UmwerterTabelle18265D2<'a> {
    pub fn new() -> Result<UmwerterTabelle18265D2<'a>,UmwerterError> {
        let mut tabelle = UmwerterTabelle18265D2 { daten: Vec::new() };
        let csv_data = include_str!("18265_D2.csv");
        tools::initialize(&tabelle.interne_einheiten(), &mut tabelle.daten, csv_data)?;
        Ok(tabelle)
    }
}

impl<'a> Umwerter<'a> for UmwerterTabelle18265D2<'a> {
    fn data(&self) -> &Vec<HashMap<&'a str, f64>> {
        &self.daten
    }

    fn externe_einheiten(&self) -> Vec<&'a str> {
        vec![
	    "HV","HRC","HRA","HR45N","HR30N","HR15N",
        ]
    }

    fn interne_einheiten(&self) -> Vec<&'a str> {
        vec![
            "hv", "hrc", "hra", "hr45n", "hr30n", "hr15n", 
        ]
    }
}
