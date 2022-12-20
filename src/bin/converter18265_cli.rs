use umwerter::konstanten;
use umwerter::errors::UmwerterError;

use anyhow::Result;
use clap::{Parser, ArgGroup};


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(group(
    ArgGroup::new("mode")
        .required(true)
        .args(["list", "hardnessvalue"]),
))]
#[command(group(
    ArgGroup::new("mode2")
        .requires_all(["source_unit", "destination_unit"])
        .args(["hardnessvalue"]),
))]
#[command(group(
    ArgGroup::new("mode3")
        .conflicts_with_all(["source_unit", "destination_unit"])
        .args(["list"]),
))]
struct Config {
    hardnessvalue : Option<f64>,

    #[arg(short, long)]
    source_unit : Option<String>,
    
    #[arg(short, long)]
    destination_unit : Option<String>,

    #[arg(short, long, value_enum, default_value_t = konstanten::UmwertungsTabelle::Iso18265A1)]
    table : konstanten::UmwertungsTabelle,
    
    #[arg(short, long, default_value_t = false)]
    verbose : bool,

    #[arg(short, long, default_value_t = false)]
    list : bool,

}

fn read_config() -> Result<Config,UmwerterError> {
    let config = Config::parse();
    println!("{:?}", config);
    Ok(config)
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
        config.hardnessvalue.unwrap(),
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
