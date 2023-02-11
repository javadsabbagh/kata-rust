use std::collections::{HashMap, HashSet};
use std::vec;

struct Company {
    name: String,
    ceo: Option<String>,
}

impl Company {
    fn new(name: &str, ceo: &str) -> Self {
        let ceo = match ceo {
            "" => None,
            name => Some(name.to_string()),
        };

        Self {
            name: name.to_string(),
            ceo,
        }
    }

    fn get_ceo(&self) -> Option<String> {
        self.ceo.clone()
    }
}

fn main() {
    let company_vec = vec![
        Company::new("Abc", "Adam"),
        Company::new("Barzin Pardaz", "Javad"),
        Company::new("Another sample", ""),
        Company::new("Alphabet", "Sandra"),
    ];

    print!("{:?}", Company::get_ceo(&company_vec[0]));

    let all_the_ceos = company_vec
        .into_iter()
        .filter_map(|c| c.get_ceo())
        .collect::<Vec<String>>();

    println!("{:?}", all_the_ceos)
}
