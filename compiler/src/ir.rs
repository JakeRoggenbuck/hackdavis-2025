use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Command {
    pub r#type: String,
    pub amount: i32,
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