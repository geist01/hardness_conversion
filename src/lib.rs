#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate error_chain;

mod trait_umwerter;
mod umwerter_tabelle_18265_a1;
mod umwerter_tabelle_18265_b2;
mod tools;
pub mod errors;
pub mod konstanten;
use konstanten::UmwertungsTabelle;

use trait_umwerter::Umwerter;

use errors::*;

pub fn bestimme_einheiten<'a>(tabelle: &'a UmwertungsTabelle) -> Vec<&'a str> {
    let umwerter_trait = tools::bestimme_tabelle(tabelle).unwrap();
    umwerter_trait.externe_einheiten()
}

pub fn werte_um<'a>(
    wert: f64,
    externe_source_einheit: &str,
    externe_ziel_einheit: &str,
    tabelle: &'a UmwertungsTabelle,
) -> Result<f64> {
    if externe_source_einheit == externe_ziel_einheit {
        return Ok(wert);
    }

    let mut trait_tabelle = tools::bestimme_tabelle(&tabelle)?;
    let interne_source_einheit = match trait_tabelle.konvert_einheit(externe_source_einheit) {
        Some(einheit) => einheit,
        None => {
            return Err(
                ErrorKind::QuellEinheitNichtVorhanden(externe_source_einheit.to_string()).into(),
            )
        }
    };

    let interne_ziel_einheit = match trait_tabelle.konvert_einheit(externe_ziel_einheit) {
        Some(einheit) => einheit,
        None => {
            return Err(
                ErrorKind::ZielEinheitNichtVorhanden(externe_ziel_einheit.to_string()).into(),
            )
        }
    };

    let (naeherung_unten_wert, zeile_unten_wert, naeherung_oben_wert, zeile_oben_wert) =
        tools::bestimme_naeherung(&mut *trait_tabelle, interne_source_einheit, wert)?;

    if naeherung_oben_wert == naeherung_unten_wert && naeherung_unten_wert == 0.0 {
        return Err(ErrorKind::QuellWertAusserhalbUmwertungsnorm(wert).into());
    }

    let data = trait_tabelle.data();
    if zeile_unten_wert == zeile_oben_wert {
        match data[zeile_oben_wert].get(interne_ziel_einheit) {
            Some(w) => return Ok(*w),
            None => return Err(ErrorKind::ZielWertAusserhalbUmwertungsnorm(wert).into()),
        }
    }

    let ziel_oben_wert = match data[zeile_oben_wert].get(interne_ziel_einheit) {
        Some(w) => *w,
        None => return Err(ErrorKind::ZielWertAusserhalbUmwertungsnorm(wert).into()),
    };

    let ziel_unten_wert = match data[zeile_unten_wert].get(interne_ziel_einheit) {
        Some(w) => *w,
        None => return Err(ErrorKind::ZielWertAusserhalbUmwertungsnorm(wert).into()),
    };

    Ok(
        (ziel_oben_wert - ziel_unten_wert) / (naeherung_oben_wert - naeherung_unten_wert)
            * (wert - naeherung_unten_wert) + ziel_unten_wert,
    )
}
