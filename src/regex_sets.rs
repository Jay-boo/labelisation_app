use regex::{RegexSet };
use lazy_static::lazy_static;

lazy_static! {
     pub static ref ESTATE_TYPE_SET:RegexSet=RegexSet::new(&[

        r"\bappartement\b",
        r"\bmaison\b",
        r"\w*ex\b",
    ])
    .unwrap();
    
    pub static ref PRICE_SET: RegexSet = 
        RegexSet::new(&[
            r"(\d[\d\s]*\d)\s*€",
            r"\bprice\b",
        ])
        .unwrap();
    
}

