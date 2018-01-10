extern crate csv;

error_chain!{
    foreign_links {
        Io(::std::io::Error);
        CsvError(csv::Error);
        ParseError(::std::num::ParseFloatError);
    }

    errors {
        QuellEinheitNichtVorhanden(t : String) {}            
        ZielEinheitNichtVorhanden(t : String) {}
        
        QuellWertAusserhalbUmwertungsnorm(wert : f64) {}
        ZielWertAusserhalbUmwertungsnorm(wert : f64) {}

        UmwertungstabelleUnbekannt(t : String) {}
    }

}
