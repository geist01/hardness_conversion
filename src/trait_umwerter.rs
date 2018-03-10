use std::collections::HashMap;
use std::vec::Vec;

pub trait Umwerter<'a> {
    fn data(&self) -> &Vec<HashMap<&'a str, f64>>;

    fn externe_einheiten(&self) -> Vec<&'a str>;
    fn interne_einheiten(&self) -> Vec<&'a str>;

    fn konvert_einheit(&self, externe_einheit: &'a str) -> Option<&'a str> {
        match self.externe_einheiten()
            .iter()
            .enumerate()
            .find(|&einheit| *einheit.1 == externe_einheit)
        {
            Some((index, _)) => {
                return Some(self.interne_einheiten()[index]);
            }

            None => return None,
        }
    }
}
