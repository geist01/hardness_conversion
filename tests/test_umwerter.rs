use umwerter::konstanten;
use umwerter::errors;

#[test]
fn umwerter_18265_a1() {
    let mut erg = umwerter::werte_um(260f64, "MPa", "HV 10", &konstanten::UmwertungsTabelle::Iso18265A1);
    assert!(erg.is_ok());
    assert_eq!(erg.unwrap(), 81.66666666666667);

    erg = umwerter::werte_um(260f64, "MPa", "HRF", &konstanten::UmwertungsTabelle::Iso18265A1);
    assert!(erg.is_err());
    assert!(match erg.unwrap_err() { errors::UmwerterError::ZielWertAusserhalbUmwertungsnorm(_) => true, _ => false });

    erg = umwerter::werte_um(200f64, "MPa", "HV 10", &konstanten::UmwertungsTabelle::Iso18265A1);
    assert!(erg.is_err());
    assert!(match erg.unwrap_err() { errors::UmwerterError::QuellWertAusserhalbUmwertungsnorm(_) => true, _ => false });

    erg = umwerter::werte_um(200f64, "XXX", "HV 10", &konstanten::UmwertungsTabelle::Iso18265A1);
    assert!(erg.is_err());
    assert!(match erg.unwrap_err() { errors::UmwerterError::QuellEinheitNichtVorhanden(_) => true, _ => false });

    erg = umwerter::werte_um(200f64, "MPa", "XXX", &konstanten::UmwertungsTabelle::Iso18265A1);
    assert!(erg.is_err());
    assert!(match erg.unwrap_err() { errors::UmwerterError::ZielEinheitNichtVorhanden(_) => true, _ => false });

}

#[test]
fn umwerter_18265_b2() {
    let erg = umwerter::werte_um(215f64, "HV", "HBW", &konstanten::UmwertungsTabelle::Iso18265B2);
    assert!(erg.is_ok());
    assert_eq!(erg.unwrap(), 210f64);
}
