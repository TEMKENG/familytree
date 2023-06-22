use crate::class_def::*;
use crate::utils::*;
use std::collections::hash_map::DefaultHasher;
use std::path::Path;
use std::hash::{Hash, Hasher};

impl Address {
    pub fn new() -> Address {
        Default::default()
    }
}
impl Delimiter {
    fn as_char(&self) -> char {
        match *self {
            Delimiter::Comma => ',',
            Delimiter::Semicolon => ';',
            Delimiter::Tab => '\t',
        }
    }
}

impl Extension {
    pub fn from_str(s: &str) -> Result<Extension, String> {
        match s {
            "DOT" => Ok(Extension::DOT),
            "PNG" => Ok(Extension::PNG),
            "JPG" => Ok(Extension::JPG),
            "PDF" => Ok(Extension::PDF),
            "SVG" => Ok(Extension::SVG),
            "JSON" => Ok(Extension::JSON),
            _ => Err(format!("Extension '{}' is not yet supported", s)),
        }
    }

    pub fn from_path(path: &Path) -> Result<Extension, String> {
        let extension = path
            .extension()
            .and_then(|ext| ext.to_str())
            .ok_or_else(|| format!("File '{}' must have an extension", path.display()))?;

        Extension::from_str(extension.to_uppercase().as_str())
    }

    pub fn get_dot_command(&self, file_in: &Path, file_out: &Path) -> String {
        match self {
            Extension::PNG => format!("-Tpng {} -o {}", file_in.display(), file_out.display()),
            Extension::JPG => format!("-Tjpg {} -o {}", file_in.display(), file_out.display()),
            Extension::PDF => format!("-Tpdf {} -o {}", file_in.display(), file_out.display()),
            Extension::SVG => format!("-Tsvg {} -o {}", file_in.display(), file_out.display()),
            _ => format!("-Tsvg {} -o {}", file_in.display(), file_out.display()),
        }
    }
}

impl MaritalStatus {
    pub fn get_shape(&self) -> String {
        match self {
            MaritalStatus::Married(_) => "diamond".to_string(),
            MaritalStatus::Divorced(_) => "ellipse".to_string(),
            MaritalStatus::Widowed(_) => "triangle".to_string(),
            _ => "record".to_string(),
        }
    }

    pub fn get_color(&self) -> String {
        match self {
            MaritalStatus::Married(_) => "green".to_string(),
            MaritalStatus::Divorced(_) => "magenta".to_string(),
            MaritalStatus::Widowed(_) => "purple".to_string(),
            _ => "orange".to_string(),
        }
    }
}

impl Gender {
    pub fn get_color(&self) -> String {
        match self {
            Gender::Female => "red".to_string(),
            Gender::Male => "blue".to_string(),
        }
    }
    pub fn get_shape(&self) -> String {
        match self {
            Gender::Female => "red".to_string(),
            Gender::Male => "blue".to_string(),
        }
    }
}
impl Person {
    pub fn new(lastname: String, firstname: String, birthday: String) -> Self {
        Self {
            id: Person::generate_id(firstname.clone(), lastname.clone(), birthday.clone()),
            first_name: firstname,
            last_name: lastname,
            birthday: birthday,
            address: Address::new(),
            gender: Gender::Male,
            marital_status: MaritalStatus::Single,
            mother_id: None,
            father_id: None,
            children_id: Vec::new(),
        }
    }
    fn _marital_status(id_1: u64, id_2: u64, color: String, form: String) -> String {
        format!(
            "\nID_{0} [shape={1}, color={2}, label=\"\"];\nID_{3} -> ID_{0};\nID_{4} -> ID_{0};\n",
            concat_id(id_1 > id_2, id_2, id_1),
            form,
            color,
            id_1,
            id_2,
        )
    }

    pub fn to_string(&self) -> String {
        format!(
            "ID: {}|{{{}|{}|{} }}",
            self.id, self.last_name, self.first_name, self.birthday
        )
    }

    pub fn to_graphviz(&self) -> String {
        let mut result: String = format!(
            "ID_{} [shape=record, nojustify=true, color={}, label=\"ID_{}\\n{}\\n{}\\n{}|{{Status: {:?}|Mother: {:?}|Father: {:?}|Children: {:?}",
            self.id,
            self.gender.get_color(),
            self.id,
            self.last_name,
            self.first_name,
            self.birthday,
            self.marital_status,
            self.mother_id.unwrap_or_default(),
            self.father_id.unwrap_or_default(),
            self.children_id
        );

        result.push_str("}\"];");

        result.push_str(&self.get_connection());
        if self.is_child() {
            let tmp_str = format!(
                "\nID_{} -> ID_{};\n",
                concat_id(
                    self.mother_id.unwrap() > self.father_id.unwrap(),
                    self.father_id.unwrap(),
                    self.mother_id.unwrap()
                ),
                self.id
            );
            result.push_str(&tmp_str);
        }
        result
    }

    fn is_child(&self) -> bool {
        self.mother_id.is_some() && self.father_id.is_some()
    }

    fn get_connection(&self) -> String {
        match self.marital_status {
            MaritalStatus::Married(to) => Person::_marital_status(
                self.id,
                to,
                self.marital_status.get_color(),
                self.marital_status.get_shape(),
            ),
            MaritalStatus::Divorced(to) => Person::_marital_status(
                self.id,
                to,
                self.marital_status.get_color(),
                self.marital_status.get_shape(),
            ),
            MaritalStatus::Widowed(to) => Person::_marital_status(
                self.id,
                to,
                self.marital_status.get_color(),
                self.marital_status.get_shape(),
            ),
            _ => "".to_string(),
        }
    }

    pub fn to_json(&self) -> String {
        format!(
            "\n\"Person_{}\": {}\n",
            self.id,
            serde_json::to_string(self).unwrap()
        )
    }

    /// Generate a person ID basis of `last_name`, `first_name` and the `birthday`
    fn generate_id(firstname: String, lastname: String, birthday: String) -> u64 {
        let mut hasher = DefaultHasher::new();
        let input = format!("{}{}{}", firstname, lastname, birthday);
        input.hash(&mut hasher);
        hasher.finish()
    }
}
