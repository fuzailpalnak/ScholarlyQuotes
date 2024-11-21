use serde::{Deserialize, Serialize};
use structsy_derive::{queries, Persistent};

#[derive(Persistent, Serialize, Deserialize, Debug)]
pub struct Quote {
    pub author: String,
    pub text: String,
    pub reference: String,
}

impl Quote {
    pub fn new(author: String, text: String, reference: String) -> Quote {
        Quote {
            author: author,
            text: text,
            reference: reference,
        }
    }
}

#[queries(Quote)]
pub trait QuoteQuery {
    fn by_author(self, author: String) -> Self;
}
