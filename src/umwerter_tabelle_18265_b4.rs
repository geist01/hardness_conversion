use crate::trait_umwerter::Umwerter;
use std::collections::HashMap;
use std::vec::Vec;

use crate::tools;
use crate::errors::UmwerterError;

pub struct UmwerterTabelle18265B4<'a> {
    daten: Vec<HashMap<&'a str, f64>>,
}

impl<'a> UmwerterTabelle18265B4<'a> {
    pub fn new() -> Result<UmwerterTabelle18265B4<'a>,UmwerterError> {
        let mut tabelle = UmwerterTabelle18265B4 { daten: Vec::new() };
        let csv_data = include_str!("18265_B4.csv");
        tools::initialize(&tabelle.interne_einheiten(), &mut tabelle.daten, csv_data)?;
        Ok(tabelle)
    }
}

impl<'a> Umwerter<'a> for UmwerterTabelle18265B4<'a> {
    fn data(&self) -> &Vec<HashMap<&'a str, f64>> {
        &self.daten
    }

    fn externe_einheiten(&self) -> Vec<&'a str> {
        vec![
            "HV", "HBW", "HRC", "HRA", "HR45N", "HR30N", "HR15N", "HRB", "HRF", "HR45T", "HR30T",
            "HR15T", "MPa",
        ]
    }

    fn interne_einheiten(&self) -> Vec<&'a str> {
        vec![
            "hv", "hbw", "hrc", "hra", "hr45n", "hr30n", "hr15n", "hrb", "hrf", "hr45t", "hr30t",
            "hr15t", "mpa",
        ]
    }
}
