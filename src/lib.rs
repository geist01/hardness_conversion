mod trait_umwerter;
mod umwerter_tabelle_18265_a1;
mod umwerter_tabelle_18265_b2;
mod umwerter_tabelle_18265_b3;
mod umwerter_tabelle_18265_b4;
mod umwerter_tabelle_18265_c2;
mod umwerter_tabelle_18265_d2;
mod umwerter_tabelle_18265_f2;
mod tools;

use konstanten::UmwertungsTabelle;
use trait_umwerter::Umwerter;
use errors::UmwerterError;

pub mod errors;
pub mod konstanten;

pub fn bestimme_einheiten<'a>(tabelle: UmwertungsTabelle) -> Vec<&'a str> {
    let umwerter_trait = tools::bestimme_tabelle(tabelle).unwrap();
    umwerter_trait.externe_einheiten()
}

pub fn werte_um(
    wert: f64,
    externe_source_einheit: &str,
    externe_ziel_einheit: &str,
    tabelle: UmwertungsTabelle,
) -> Result<f64, UmwerterError> {
    if externe_source_einheit == externe_ziel_einheit {
        return Ok(wert);
    }

    let trait_tabelle = tools::bestimme_tabelle(tabelle)?;
    let interne_source_einheit = match trait_tabelle.konvert_einheit(externe_source_einheit) {
        Some(einheit) => einheit,
        None => {
            return Err(
                UmwerterError::QuellEinheitNichtVorhanden(externe_source_einheit.to_string()),
            )
        }
    };

    let interne_ziel_einheit = match trait_tabelle.konvert_einheit(externe_ziel_einheit) {
        Some(einheit) => einheit,
        None => {
            return Err(
                UmwerterError::ZielEinheitNichtVorhanden(externe_ziel_einheit.to_string()),
            )
        }
    };

    let (naeherung_unten_wert, zeile_unten_wert, naeherung_oben_wert, zeile_oben_wert) =
        tools::bestimme_naeherung(& *trait_tabelle, interne_source_einheit, interne_ziel_einheit, wert)?;

    if (naeherung_oben_wert - naeherung_unten_wert).abs() < std::f64::EPSILON && naeherung_unten_wert == 0.0 {
        return Err(UmwerterError::QuellWertAusserhalbUmwertungsnorm(wert));
    }

    let data = trait_tabelle.data();
    if zeile_unten_wert == zeile_oben_wert {
        match data[zeile_oben_wert].get(interne_ziel_einheit) {
            Some(w) => return Ok(*w),
            None => return Err(UmwerterError::ZielWertAusserhalbUmwertungsnorm(wert)),
        }
    }

    let ziel_oben_wert = match data[zeile_oben_wert].get(interne_ziel_einheit) {
        Some(w) => *w,
        None => return Err(UmwerterError::ZielWertAusserhalbUmwertungsnorm(wert)),
    };

    let ziel_unten_wert = match data[zeile_unten_wert].get(interne_ziel_einheit) {
        Some(w) => *w,
        None => return Err(UmwerterError::ZielWertAusserhalbUmwertungsnorm(wert)),
    };

    Ok(
        (ziel_oben_wert - ziel_unten_wert) / (naeherung_oben_wert - naeherung_unten_wert)
            * (wert - naeherung_unten_wert) + ziel_unten_wert,
    )
}
