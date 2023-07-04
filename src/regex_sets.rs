use regex::{RegexSet };
use lazy_static::lazy_static;



lazy_static! {
     pub static ref ESTATE_TYPE_SET:RegexSet=RegexSet::new(&[

        r"\bappartement\b",
        r"\bmaison\b",
        r"\w*ex\b",
        r"\bstudio\b",
        r"\bprojet\b"
    ])
    .unwrap();
    
    pub static ref PRICE_SET: RegexSet = 
        RegexSet::new(&[
            r"(\d[\d\s]*\d)\s*€",
            r"\bprice\b",
            r"\beuro\b",
            r"\beuros\b",
        ])
        .unwrap();

    pub static ref  AREA_SET:RegexSet = RegexSet::new([
        r"(\d+)\s*m[2,²]"
    ]).unwrap();
    
    pub static ref ROOM_COUNT_SET: RegexSet = RegexSet::new([
        r"(\d+)\s*(?:pièces?|piece)",
        r"(\d+)\s*(?:pièces?|piece)",
        r"pi[è|e]ce[s]\b",
        r"\bt\d+\b",
        r"t[1-9]+bis",
        r"\bf[1-9]\b"
    ]).unwrap();

    pub static ref MEUBLE_SET: RegexSet = RegexSet::new([
        r"\bmeubl[é,e]\b",
        r"\bnu\b"

    ]).unwrap();

    
}

