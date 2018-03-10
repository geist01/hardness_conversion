use trait_umwerter::Umwerter;
use std::collections::HashMap;
use std::vec::Vec;

use tools;
use errors::*;

pub struct UmwerterTabelle18265A1<'a> {
    daten: Vec<HashMap<&'a str, f64>>,
}

impl<'a> UmwerterTabelle18265A1<'a> {
    pub fn new() -> Result<UmwerterTabelle18265A1<'a>> {
        let mut tabelle = UmwerterTabelle18265A1 { daten: Vec::new() };
        let csv_data = include_str!("18265_A1.csv");
        tools::initialize(&tabelle.interne_einheiten(), &mut tabelle.daten, csv_data)?;
        Ok(tabelle)
    }
}

impl<'a> Umwerter<'a> for UmwerterTabelle18265A1<'a> {
    fn data(&self) -> &Vec<HashMap<&'a str, f64>> {
        &self.daten
    }

    fn externe_einheiten(&self) -> Vec<&'a str> {
        vec![
            "MPa", "HV 10", "HBW", "HRB", "HRF", "HRC", "HRA", "HRD", "HR15N", "HR30N", "HR45N"
        ]
    }

    fn interne_einheiten(&self) -> Vec<&'a str> {
        vec![
            "mpa", "hv", "hbw", "hrb", "hrf", "hrc", "hra", "hrd", "hr15n", "hr30n", "hr45n"
        ]
    }
}
