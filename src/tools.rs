extern crate csv;
extern crate regex;

use std::collections::HashMap;
use std::vec::Vec;
use self::regex::Regex;

use umwerter_tabelle_18265_a1::UmwerterTabelle18265A1;
use umwerter_tabelle_18265_b2::UmwerterTabelle18265B2;
use konstanten::UmwertungsTabelle;
use Umwerter;
use errors::*;

lazy_static! {
    static ref RE: Regex = Regex::new(r"[\(\)]").unwrap();
}

pub fn bestimme_tabelle<'a>(tabelle: &'a UmwertungsTabelle) -> Result<Box<dyn Umwerter<'a> + 'a>> {
    match tabelle {
        &UmwertungsTabelle::Iso18265A1 => {
            let trait_tabelle = UmwerterTabelle18265A1::new()?;
            Ok(Box::new(trait_tabelle))
        }
        &UmwertungsTabelle::Iso18265B2 => {
            let trait_tabelle = UmwerterTabelle18265B2::new()?;
            Ok(Box::new(trait_tabelle))
        }
    }
}

pub fn initialize<'a>(
    einheiten: &Vec<&'a str>,
    daten: &mut Vec<HashMap<&'a str, f64>>,
    csv_data: &'static str,
) -> Result<()> {
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b'\t')
        .flexible(true)
        .has_headers(false)
        .from_reader(csv_data.as_bytes());

    for result in rdr.records() {
        let record = result?;
        // println!("{:?}", record);

        let mut daten_zeile = HashMap::new();
        for i in 0..einheiten.len() {
            if let Ok(wert) = parse_element(&record[i]) {
                let einheit = einheiten[i];
                daten_zeile.insert(einheit, wert);
                // println!("Einheit: {0}, Wert: {1}", einheit, wert);
            }
        }
        daten.push(daten_zeile);
    }

    Ok(())
}

fn parse_element(element: &str) -> Result<f64> {
    let x = str::replace(element, ",", ".");
    Ok(RE.replace_all(x.as_str(), "").parse::<f64>()?)
}

fn kleinstes_element(umwerter: &dyn Umwerter, einheit: &str) -> (f64, usize) {
    let mut wert = 100000.0;
    let mut zeile = 0;

    for i in 0..umwerter.data().len() {
        if let Some(w) = umwerter.data()[i].get(einheit) {
            if *w <= wert {
                wert = *w;
                zeile = i;
            }
        }
    }

    (wert, zeile)
}

fn groesstes_element(umwerter: &dyn Umwerter, einheit: &str) -> (f64, usize) {
    let mut wert = 0.0;
    let mut zeile = 0;

    for i in 0..umwerter.data().len() {
        if let Some(w) = umwerter.data()[i].get(einheit) {
            if *w > wert {
                wert = *w;
                zeile = i;
            }
        }
    }

    (wert, zeile)
}

pub fn bestimme_naeherung(
    umwerter: &dyn Umwerter,
    quell_einheit: &str,
    ziel_einheit: &str,
    wert: f64,
) -> Result<(f64, usize, f64, usize)> {
    let (mut ausgangsbereich_unten, mut zeile_untere_naeherung) =
        kleinstes_element(umwerter, quell_einheit);
    let (mut ausgangsbereich_oben, mut zeile_obere_naeherung) =
        groesstes_element(umwerter, quell_einheit);

    // println!("unten {0}, oben {1}", ausgangsbereich_unten, ausgangsbereich_oben);
    if wert < ausgangsbereich_unten || wert > ausgangsbereich_oben {
        return Err(ErrorKind::QuellWertAusserhalbUmwertungsnorm(wert).into());
    }

    for i in 0..umwerter.data().len() {
        if umwerter.data()[i].get(quell_einheit).is_some() && umwerter.data()[i].get(ziel_einheit).is_some() {
            let w = umwerter.data()[i].get(quell_einheit).unwrap();
            if *w < wert {
                zeile_untere_naeherung = i;
                ausgangsbereich_unten = *w;
            } else if *w == wert {
                zeile_untere_naeherung = i;
                zeile_obere_naeherung = i;
                ausgangsbereich_unten = *w;
                ausgangsbereich_oben = *w;
                break;
            } else if *w > wert {
                zeile_obere_naeherung = i;
                ausgangsbereich_oben = *w;
                break;
            }
        }
    }

    Ok((
        ausgangsbereich_unten,
        zeile_untere_naeherung,
        ausgangsbereich_oben,
        zeile_obere_naeherung,
    ))
}
