use umwerter::konstanten;
use umwerter::errors::UmwerterError;

use anyhow::Result;

struct Config {
    source_unit : Option<String>,
    destination_unit : Option<String>,
    table : konstanten::UmwertungsTabelle,
    value : Option<f64>,
    verbose : bool,
    list : bool,
}

fn read_config() -> Result<Config,UmwerterError> {
    use clap::{Arg, App};
    
    let matches = App::new("Hardness Converter 18265")
        .version("1.0")
        .author("IB")
        .arg(Arg::with_name("table")
             .short("t")
             .long("table")
             .value_name("table")
             .help("Sets the table to use (default is A.1)")
             .takes_value(true))
        .arg(Arg::with_name("sourceunit")
             .short("s")
             .long("sunit")
             .value_name("sourceunit")
             .help("Sets the source unit to convert from")
             .required_unless("list")
             .takes_value(true))
        .arg(Arg::with_name("destinationunit")
             .short("d")
             .long("dunit")
             .value_name("destinationunit")
             .help("Sets the destination unit to convert to")
             .required_unless("list")
             .takes_value(true))
        .arg(Arg::with_name("value")
             .help("Sets the input value to convert")
             .required_unless("list")
             .index(1))
        .arg(Arg::with_name("verbose")
             .short("v")
             .multiple(false)
             .help("Sets the level of verbosity"))
        .arg(Arg::with_name("list")
             .short("l")
             .multiple(false)
             .help("List the available units of the current table"))
        .get_matches();

    let verbose = matches.occurrences_of("verbose") == 1;
    let list = matches.occurrences_of("list") == 1;
             
    let source_unit = matches.value_of("sourceunit").map(|s| s.to_string());
    let destination_unit = matches.value_of("destinationunit").map(|s| s.to_string());
    let table = konstanten::UmwertungsTabelle::kurzbezeichner_to_enum(matches.value_of("table").unwrap_or(konstanten::A1));
    
    if table.is_none() {
        return Err(UmwerterError::UmwertungstabelleUnbekannt(matches.value_of("table").unwrap().to_string()));
    }

    let mut value:Option<f64> = None;
    if let Some(v) = matches.value_of("value") {
        value = Some(v.parse::<f64>()?);
    }
    
    Ok(Config { source_unit, destination_unit, table : table.unwrap(), value, verbose, list })
}



fn list_units(config : Config) {
    let bezeichner = konstanten::UmwertungsTabelle::enum_to_kurzbezeichner(config.table);
    let einheiten = umwerter::bestimme_einheiten(config.table).join(",");
    println!("Available units in table {0}: {1}", bezeichner, einheiten);
}

fn convert(config : Config) {
    let source_unit = config.source_unit.unwrap();
    let destination_unit = config.destination_unit.unwrap();
    
    match umwerter::werte_um(
        config.value.unwrap(),
        &source_unit,
        &destination_unit,
        config.table
    ) {
        Ok(erg) => {
            if config.verbose {
                let bezeichner = konstanten::UmwertungsTabelle::enum_to_kurzbezeichner(config.table);
                println!("{0:.2} {1} - {2} - {3}",
                         erg,
                         &destination_unit,
                         bezeichner,
                         &source_unit
                );
                
            } else {
                println!("{0:.2}", erg)
            }
        },
        
        Err(e) => {
            match e {
                UmwerterError::QuellWertAusserhalbUmwertungsnorm(wert) |
                UmwerterError::ZielWertAusserhalbUmwertungsnorm(wert) => {
                    eprintln!("Conversion {0} {1} -> {2} not defined", wert, &source_unit, &destination_unit);
                    std::process::exit(3);
                    
                },
                UmwerterError::QuellEinheitNichtVorhanden(ref unit) | UmwerterError::ZielEinheitNichtVorhanden(ref unit) => {
                    eprintln!("Source or Destination Unit {0} unknown", unit);
                    std::process::exit(4);
                }
                _ => {}, 
            }
        }
    }
}    

fn main() {
    let config = read_config();

    match config {
        Err(e) => {
            match e {
                UmwerterError::ParseError(ref _e) => {
                    eprintln!("Invalid value - please enter a floating point number");
                    std::process::exit(1);
                },
                UmwerterError::UmwertungstabelleUnbekannt(ref table) => {
                    eprintln!("Conversion table {0} unknown", table);
                    std::process::exit(2);
                },
                _ => {},
            }
        },

        Ok(config) => {
            if config.list {
                list_units(config);
            } else {
                convert(config);
            }
        }        
    }
}
