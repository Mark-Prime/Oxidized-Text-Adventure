use std::error::Error;
use std::fmt::{self, Display, Formatter};

#[derive(Debug, PartialEq)]
pub enum ItemError {
    NoName,
}

impl Error for ItemError {}

impl Display for ItemError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        use ItemError::*;
        match self {
            NoName => write!(f, "No name given to item"),
        }
    }
}

pub struct Item {
    pub name: String,
    description: String,
    pub count: i32
}

impl Item {
    pub fn new(item_name: &str, desc: &str) -> Result<Item, ItemError> {
        if item_name.len() == 0 {
            return Err(ItemError::NoName)
        }

        if desc.len() == 0 {
            return Ok(Item { 
                name: item_name.to_string(), 
                description: "A mysterious object".to_string(),
                count: 1
            })
        }

        return Ok(Item { 
            name: item_name.to_string(), 
            description: desc.to_string(),
            count: 1
        })
    }

    pub fn describe(&self) -> String {
        return self.description.clone()
    }
}