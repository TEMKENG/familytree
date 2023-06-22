use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub struct Address {
    pub street: String,
    pub city: String,
    pub state: String,
    pub country: String,
    pub postal_code: String,
}

#[derive(Debug)]
pub enum Delimiter {
    Comma,
    Semicolon,
    Tab,
}

#[derive(Debug, PartialEq)]
pub enum Extension {
    DOT,
    PNG,
    JPG,
    PDF,
    SVG,
    JSON,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Gender {
    Male,
    Female,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MaritalStatus {
    Single,
    Married(u64),
    Divorced(u64),
    Widowed(u64),
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct Person {
    pub id: u64,
    pub first_name: String,
    pub last_name: String,
    pub birthday: String,
    pub address: Address,
    pub gender: Gender,
    pub marital_status: MaritalStatus,
    pub mother_id: Option<u64>,
    pub father_id: Option<u64>,
    pub children_id: Vec<u64>,
}
