use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Command {
    Move {
        r#type: String,
        amount: i32,
    },
    Jump {
        label: String,
    },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Section {
    pub name: String,
    pub commands: Vec<Command>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Program {
    pub sections: Vec<Section>,
} 