use thiserror::Error;

#[derive(Error, Debug)]
pub enum UmwerterError {
    #[error("IO")]
    Io(#[from] ::std::io::Error),

    #[error("IO")]
    CsvError(#[from] csv::Error),
    #[error("IO")]
    ParseError(#[from] ::std::num::ParseFloatError),

    #[error("IO")]
    QuellEinheitNichtVorhanden(String),
    
    #[error("IO")]
    ZielEinheitNichtVorhanden(String),

    #[error("IO")]
    QuellWertAusserhalbUmwertungsnorm(f64),
    #[error("IO")]
    ZielWertAusserhalbUmwertungsnorm(f64),

    #[error("IO")]
    UmwertungstabelleUnbekannt(String),
}
