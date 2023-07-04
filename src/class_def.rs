use crate::utils::*;
use petgraph::Graph;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::cell::RefCell;
use std::collections::hash_map::DefaultHasher;
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::{self, BufRead, Write};
use std::path::{Path, PathBuf};
use std::rc::Rc;

#[derive(Debug, Clone)]
pub enum Delimiter {
    Colon,
    Comma,
    Semicolon,
    Tab,
    VerticalBar,
}

impl Delimiter {
    fn as_char(&self) -> char {
        match *self {
            Delimiter::Tab => '\t',
            Delimiter::Colon => ':',
            Delimiter::Comma => ',',
            Delimiter::Semicolon => ';',
            Delimiter::VerticalBar => '|',
        }
    }
}

pub fn determine_delimiter(file_path: &str) -> Option<u8> {
    if let Ok(file) = File::open(file_path) {
        let reader = io::BufReader::new(file);

        if let Some(Ok(first_line)) = reader.lines().next() {
            let delimiter_count: Vec<(Delimiter, usize)> = vec![
                (Delimiter::Colon, first_line.matches(':').count()),
                (Delimiter::Comma, first_line.matches(',').count()),
                (Delimiter::Semicolon, first_line.matches(';').count()),
                (Delimiter::Tab, first_line.matches('\t').count()),
                (Delimiter::VerticalBar, first_line.matches('|').count()),
            ];

            if let Some((delimiter, _count)) =
                delimiter_count.iter().max_by_key(|(_, count)| *count)
            {
                Some(delimiter.as_char() as u8)
            } else {
                None
            }
        } else {
            None
        }
    } else {
        None
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Address {
    street: String,
    city: String,
    state: String,
    country: String,
    postal_code: String,
}

impl Address {
    pub fn new() -> Address {
        Default::default()
    }
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Gender {
    Male,
    Female,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum MaritalStatus {
    Single,
    Married(u64),
    Divorced(u64),
    Widowed(u64),
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct PersonInfo {
    id: u64,
    firstname: String,
    lastname: String,
    birthday: String,
    gender: Gender,
}

impl PersonInfo {
    fn new(firstname: &str, lastname: &str, birthday: &str, gender: Gender) -> PersonInfo {
        PersonInfo {
            id: PersonInfo::generate_id(firstname, lastname, birthday),
            firstname: firstname.to_string(),
            lastname: lastname.to_string(),
            birthday: birthday.to_string(),
            gender,
        }
    }

    fn generate_id(firstname: &str, lastname: &str, birthday: &str) -> u64 {
        let mut hasher = DefaultHasher::new();
        let input = format!("{}{}{}", firstname, lastname, birthday);
        input.hash(&mut hasher);
        hasher.finish()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Person {
    info: PersonInfo,
    address: Address,
    marital_status: MaritalStatus,
    // #[serde(
    //     serialize_with = "serialize_option_rc_person_info",
    //     deserialize_with = "deserialize_option_rc_person_info"
    // )]
    mother: Option<Rc<RefCell<Person>>>,
    // #[serde(
    //     serialize_with = "serialize_option_rc_person_info",
    //     deserialize_with = "deserialize_option_rc_person_info"
    // )]
    father: Option<Rc<RefCell<Person>>>,
}

mod my_serde {
    use super::{Address, MaritalStatus, Person, PersonInfo};
    use serde::{self, ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<S>(person: &Person, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Person", 5)?;
        state.serialize_field("info", &person.info)?;
        state.serialize_field("address", &person.address)?;
        state.serialize_field("marital_status", &person.marital_status)?;
        let mother = person
            .mother
            .clone()
            .map_or(None::<Person>, |p| Some(p.borrow().clone()));
        let father = person
            .father
            .clone()
            .map_or(None::<Person>, |p| Some(p.borrow().clone()));
        state.serialize_field("mother", &mother)?;
        state.serialize_field("father", &father)?;
        state.end()
    }
}

fn serialize_option_rc_person_info<S>(
    value: &Option<Rc<PersonInfo>>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    match value {
        Some(person) => person.serialize(serializer),
        None => serializer.serialize_none(),
    }
}

fn deserialize_option_rc_person_info<'de, D>(
    deserializer: D,
) -> Result<Option<Rc<PersonInfo>>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let deserialized: Option<PersonInfo> = Option::deserialize(deserializer)?;
    match deserialized {
        Some(person_info) => Ok(Some(Rc::new(person_info))),
        None => Ok(None),
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

impl Person {
    pub fn default(
        lastname: &str,
        firstname: &str,
        birthday: &str,
        gender: Gender,
    ) -> Rc<RefCell<Person>> {
        Rc::new(RefCell::new(Person {
            info: PersonInfo::new(firstname, lastname, birthday, gender),
            address: Address::new(),
            marital_status: MaritalStatus::Single,
            mother: None,
            father: None,
        }))
    }

    pub fn new(
        lastname: &str,
        firstname: &str,
        birthday: &str,
        gender: Gender,
        address: Option<Address>,
        marital_status: Option<MaritalStatus>,
        mother: Option<Rc<RefCell<Person>>>,
        father: Option<Rc<RefCell<Person>>>,
    ) -> Rc<RefCell<Person>> {
        Rc::new(RefCell::new(Person {
            info: PersonInfo::new(firstname, lastname, birthday, gender),
            address: address.unwrap_or(Address::new()),
            marital_status: marital_status.unwrap_or(MaritalStatus::Single),
            mother,
            father,
        }))
    }

    fn __marital_status__(&self, connect_to: u64) -> String {
        format!(
            "\nID_{0} [shape={1}, color={2}, label=\"\"];\nID_{3} -> ID_{0};\nID_{4} -> ID_{0};\n",
            concat_id(self.info.id > connect_to, connect_to, self.info.id),
            self.marital_status.get_shape(),
            self.marital_status.get_color(),
            self.info.id,
            connect_to,
        )
    }

    pub fn to_string(&self) -> String {
        format!(
            "ID: {}|{{{}|{}|{} }}",
            self.info.id, self.info.lastname, self.info.firstname, self.info.birthday
        )
    }

    fn get_couple_id(&self) -> Option<String> {
        match self.marital_status {
            MaritalStatus::Married(to) => Some(format!(
                "ID_{}",
                concat_id(self.info.id > to, self.info.id, to)
            )),
            MaritalStatus::Divorced(to) => Some(format!(
                "ID_{}",
                concat_id(self.info.id > to, self.info.id, to)
            )),
            MaritalStatus::Widowed(to) => Some(format!(
                "ID_{}",
                concat_id(self.info.id > to, self.info.id, to)
            )),
            _ => None,
        }
    }
    pub fn to_graphviz(&self) -> String {
        let mut result: String = format!(
            "ID_{} [shape=record, nojustify=true, color={}, label=\"ID_{}\\n{}\\n{}\\n{}|{{Status: {:?}",
            self.info.id,
            self.info.gender.get_color(),
            self.info.id,
            self.info.lastname,
            self.info.firstname,
            self.info.birthday,
            self.marital_status,
        );

        result.push_str("}\"];");

        result.push_str(&self.get_connection());
        result
    }

    fn hat_parent(&self) -> bool {
        self.mother.is_some() && self.father.is_some()
    }

    fn get_connection(&self) -> String {
        match self.marital_status {
            MaritalStatus::Married(to) => self.__marital_status__(to),
            MaritalStatus::Divorced(to) => self.__marital_status__(to),
            MaritalStatus::Widowed(to) => self.__marital_status__(to),
            _ => "".to_string(),
        }
    }

    pub fn to_json(&self) -> String {
        let mother_json = self
            .clone()
            .mother
            .map_or("None".to_string(), |m| m.to_json());
        let father_json = self
            .clone()
            .father
            .map_or("None".to_string(), |f| f.to_json());
        format!(
            "\n\"Person_{}\": info: {}\naddress: {}\nmarital_status: {}\nmother: {}\nfather: {}\n",
            self.info.id,
            serde_json::to_string(&self.info).unwrap(),
            serde_json::to_string(&self.address).unwrap(),
            serde_json::to_string(&self.marital_status).unwrap(),
            mother_json,
            father_json
        )
    }
}

pub trait RcPerson {
    // Setter
    fn set_address(&mut self, addr: Address) -> &mut Rc<RefCell<Person>>;
    fn set_marital_status(&mut self, status: MaritalStatus) -> &mut Rc<RefCell<Person>>;
    fn set_mother(&mut self, mother: &Rc<RefCell<Person>>) -> &mut Rc<RefCell<Person>>;
    fn set_father(&mut self, father: &Rc<RefCell<Person>>) -> &mut Rc<RefCell<Person>>;
    fn get_root_parents(&self, visited: &mut HashSet<u64>) -> Vec<Option<Rc<RefCell<Person>>>>;
    // Getter
    fn get_address(&self) -> Address;
    fn get_marital_status(&self) -> MaritalStatus;
    fn get_mother(&self) -> Option<Person>;
    fn get_rc_mother(&self) -> Option<Rc<RefCell<Person>>>;
    fn get_rc_father(&self) -> Option<Rc<RefCell<Person>>>;
    fn get_father(&self) -> Option<Person>;
    fn get_id(&self) -> u64;
    fn get_gender(&self) -> Gender;

    fn to_graphviz(&self) -> String;
    fn to_json(&self) -> String;

    fn has_parent(&self) -> bool;
    fn has_mother(&self) -> bool;
    fn has_father(&self) -> bool;

    fn mother_id(&self) -> Option<u64>;
    fn father_id(&self) -> Option<u64>;
}

impl RcPerson for Rc<RefCell<Person>> {
    fn get_root_parents(&self, visited: &mut HashSet<u64>) -> Vec<Option<Rc<RefCell<Person>>>> {
        visited.insert(self.get_id());
        if !self.has_parent() {
            return vec![Some(self.clone())];
        }
        remove_duplicated(
            &self.get_rc_mother().unwrap().get_root_parents(visited),
            &self.get_rc_father().unwrap().get_root_parents(visited),
        )
    }

    fn mother_id(&self) -> Option<u64> {
        let person = self.borrow().clone();
        if let Some(mother) = person.mother {
            return Some(mother.get_id());
        }
        None
    }

    fn father_id(&self) -> Option<u64> {
        let person = self.borrow().clone();
        if let Some(father) = person.father {
            return Some(father.get_id());
        }
        None
    }

    fn has_parent(&self) -> bool {
        self.get_mother().is_some() && self.get_father().is_some()
    }

    fn has_mother(&self) -> bool {
        self.get_mother().is_some()
    }

    fn has_father(&self) -> bool {
        self.get_father().is_some()
    }

    fn get_gender(&self) -> Gender {
        let person = self.borrow().clone();
        person.info.gender
    }

    fn to_graphviz(&self) -> String {
        let person = self.borrow().clone();
        person.to_json()
    }

    fn to_json(&self) -> String {
        let person = self.borrow().clone();
        person.to_json()
    }

    fn set_mother(&mut self, mother: &Rc<RefCell<Person>>) -> &mut Rc<RefCell<Person>> {
        self.borrow_mut().mother = Some(mother.clone());
        self
    }

    fn get_mother(&self) -> Option<Person> {
        let person = self.borrow().clone();
        if let Some(mother) = person.mother.clone() {
            return Some(mother.borrow().clone());
        }
        None
    }

    fn get_rc_mother(&self) -> Option<Rc<RefCell<Person>>> {
        let person = self.borrow().clone();
        person.mother
    }

    fn set_father(&mut self, father: &Rc<RefCell<Person>>) -> &mut Rc<RefCell<Person>> {
        self.borrow_mut().father = Some(father.clone());
        self
    }

    fn get_father(&self) -> Option<Person> {
        let person = self.borrow().clone();
        if let Some(father) = person.father.clone() {
            return Some(father.borrow().clone());
        }
        None
    }

    fn get_rc_father(&self) -> Option<Rc<RefCell<Person>>> {
        let person = self.borrow().clone();
        person.father
    }

    fn set_address(&mut self, addr: Address) -> &mut Rc<RefCell<Person>> {
        self.borrow_mut().address = addr;
        self
    }

    fn get_address(&self) -> Address {
        let person = self.borrow().clone();
        person.address
    }

    fn set_marital_status(&mut self, status: MaritalStatus) -> &mut Rc<RefCell<Person>> {
        (**self).borrow_mut().marital_status = status;
        self
    }

    fn get_marital_status(&self) -> MaritalStatus {
        let person = self.borrow().clone();
        person.marital_status
    }

    fn get_id(&self) -> u64 {
        let person = self.borrow().clone();
        person.info.id
    }
}
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct TreeNode<T: Eq + Clone> {
    pub value: T,
    pub children: Vec<TreeNode<T>>,
}

impl<T: Eq + Clone> TreeNode<T> {
    pub fn new(value: T) -> TreeNode<T> {
        TreeNode {
            value: value,
            children: Vec::new(),
        }
    }
    pub fn add_child(&mut self, child: TreeNode<T>) {
        if !self.children.contains(&child) {
            self.children.push(child);
        }
    }
}
#[derive(Debug, Default)]
pub struct PersonManager {
    counter: u64,
    persons: HashMap<u64, Rc<RefCell<Person>>>,
    pub tree: HashMap<u64, TreeNode<Rc<RefCell<Person>>>>,
    graph: Graph<&'static str, &'static str>,
}

impl PersonManager {
    pub fn new() -> Self {
        Default::default()
    }

    fn get_person(&self, id: u64) -> Option<&Rc<RefCell<Person>>> {
        self.persons.get(&id)
    }
    pub fn get_persons(&self) -> &HashMap<u64, Rc<RefCell<Person>>> {
        &self.persons
    }
    pub fn add_person_str<'a>(
        &'a mut self,
        lastname: &str,
        firstname: &str,
        birthday: &str,
        gender: Gender,
    ) -> &'a mut Rc<RefCell<Person>> {
        let p: Rc<RefCell<Person>> = Person::default(lastname, firstname, birthday, gender);
        let k = (*p).borrow().info.id;
        self.persons.entry(k).or_insert(p)
    }

    pub fn add_person(&mut self, person: &Rc<RefCell<Person>>) {
        let person_cloned = person.clone();
        self.persons
            .entry(person.get_id())
            .or_insert(person.clone());
        let node = TreeNode::new(person_cloned);
        self.tree.insert(person.get_id(), node.clone());

        if person.has_parent() {
            let mother_id = person.mother_id().unwrap();
            let father_id = person.father_id().unwrap();
            if let Some(mother) = self.tree.get_mut(&mother_id) {
                mother.add_child(node.clone());
            }
            if let Some(father) = self.tree.get_mut(&father_id) {
                father.add_child(node.clone());
            }
        }
    }

    pub fn set_mother(&mut self, person_id: u64, mother_id: u64) -> Result<(), String> {
        if person_id == mother_id {
            return Err(format!("You can't be your own mother"));
        }
        if let Some(person) = self.persons.get(&person_id) {
            if let Some(mother) = self.persons.get(&mother_id) {
                if mother.get_gender() == Gender::Female {
                    person.clone().set_mother(mother);
                    return Ok(());
                } else {
                    return Err(format!("We can't put the person with ID {mother_id} as the mother of the person with the ID {person_id} because he is not a female", ));
                }
            } else {
                return Err(format!("Person with ID {} not found", mother_id));
            }
        } else {
            Err(format!("Person with ID {} not found", person_id))
        }
    }

    pub fn set_father(&mut self, person_id: u64, father_id: u64) -> Result<(), String> {
        if person_id == father_id {
            return Err(format!("You can't be your own father"));
        }
        if let Some(person) = self.persons.get(&person_id) {
            if let Some(mother) = self.persons.get(&father_id) {
                if mother.get_gender() == Gender::Male {
                    person.clone().set_mother(mother);
                    return Ok(());
                } else {
                    return Err(format!("We can't put the person with ID {father_id} as the mother of the person with the ID {person_id} because he is not a male", ));
                }
            } else {
                return Err(format!("Person with ID {} not found", father_id));
            }
        } else {
            Err(format!("Person with ID {} not found", person_id))
        }
    }

    pub fn set_parent(
        &mut self,
        person_id: u64,
        mother_id: u64,
        father_id: u64,
    ) -> Result<(), String> {
        self.set_father(person_id, father_id)?;
        self.set_mother(person_id, mother_id)
    }

    pub fn update_marital_status(
        &mut self,
        person_id: u64,
        marital_status: MaritalStatus,
    ) -> Result<(), String> {
        if let Some(person) = self.persons.get_mut(&person_id) {
            person.set_marital_status(marital_status);
            Ok(())
        } else {
            Err(format!("Person with ID {} not found", person_id))
        }
    }

    pub fn get_person_marital_status(&self, person_id: u64) -> Option<MaritalStatus> {
        self.persons
            .get(&person_id)
            .map(|person| person.get_marital_status())
    }

    pub fn marry(&mut self, p1_id: u64, p2_id: u64) -> Result<(), String> {
        if let (Some(person1), Some(person2)) = (self.persons.get(&p1_id), self.persons.get(&p2_id))
        {
            if person1.get_gender() == person2.get_gender() {
                return Err(format!(
                    "Person with ID {p1_id} and {p2_id} must have different gender"
                ));
            }
            person1
                .clone()
                .set_marital_status(MaritalStatus::Married(p2_id));
            person2
                .clone()
                .set_marital_status(MaritalStatus::Married(p1_id));
        } else {
            return Err(format!("Person with ID {p1_id} or  {p2_id} not found"));
        }
        Ok(())
    }

    pub fn set_address(&mut self, person_id: u64, new_address: Address) {
        if let Some(person) = self.persons.get_mut(&person_id) {
            person.set_address(new_address);
        }
    }

    pub fn build_family_tree(&self) -> Option<TreeNode<Rc<RefCell<Person>>>> {
        let mut found_tree: bool = false;
        let mut root_nodes: Vec<TreeNode<Rc<RefCell<Person>>>> = Vec::new();

        for root_person in self.persons.values() {
            if !root_person.has_parent() {
                root_nodes.push(self.tree.get(&root_person.get_id()).unwrap().clone())
            }
            found_tree = true;
        }

        if !found_tree {
            return None;
        }

        if root_nodes.len() == 1 {
            Some(root_nodes.into_iter().next().unwrap())
        } else {
            // Create a dummy "Family" node
            let dummy_person = Person::new(
                "Ancestor",
                "",
                "",
                Gender::Male,
                Some(Address::new()),
                Some(MaritalStatus::Single),
                None,
                None,
            );
            let dummy_node: TreeNode<Rc<RefCell<Person>>> = TreeNode {
                value: dummy_person,
                children: root_nodes,
            };

            Some(dummy_node)
        }
        // todo!()
    }

    pub fn build_tree_recursive(
        &self,
        person_id: u64,
        visited: &mut HashSet<u64>,
    ) -> Vec<TreeNode<Rc<RefCell<Person>>>> {
        let mut children_nodes: Vec<TreeNode<Rc<RefCell<Person>>>> = Vec::new();

        if let Some(person) = self.persons.get(&person_id) {
            // for child_id in &person.children {
            //     if visited.contains(child_id) {
            //         continue;
            //     }
            //     visited.insert(*child_id);

            //     if let Some(child_person) = self.persons.get(child_id) {
            //         let child_node = TreeNode {
            //             value: child_person.clone(),
            //             children: self.build_tree_recursive(*child_id, visited),
            //         };
            //         children_nodes.push(child_node);
            //     }
            // }
        }

        children_nodes

        // todo!()
    }

    pub fn to_graphviz(&self, file_in: Option<PathBuf>) -> Result<(), String> {
        /*fs::create_dir_all("output").map_err(|e| e.to_string())?;

        let default_file = Path::new("output").join("tree.dot");
        let filename = file_in.unwrap_or(default_file.clone());
        let extension: Extension = Extension::from_path(&filename)?;

        let mut writer = fs::File::create(&default_file).map_err(|e| e.to_string())?;

        write!(&mut writer, "digraph Alf {{\n\n").map_err(|e| e.to_string())?;

        let mut unique_lines: HashSet<String> = HashSet::new();

        for person in self.persons.values() {
            let graphviz_output: String = person.to_graphviz();

            for line in graphviz_output.split('\n') {
                if unique_lines.insert(line.to_string()) {
                    writeln!(&mut writer, "{}", line).map_err(|e| e.to_string())?;
                }
            }
        }

        writeln!(&mut writer, "\n\n}}").map_err(|e| e.to_string())?;

        if filename != default_file {
            let dot_command: String = extension.get_dot_command(&default_file, &filename);
            let cmd_args: Vec<&str> = dot_command.split(' ').collect();

            let output = Command::new("dot")
                .args(cmd_args)
                .output()
                .map_err(|e| e.to_string())?;

            if !output.status.success() {
                return Err(format!(
                    "Graphviz failed with error: {}",
                    String::from_utf8_lossy(&output.stderr)
                ));
            }
        }

        Ok(()) */
        todo!()
    }

    pub fn to_json(&self, output_file: Option<PathBuf>) -> Result<(), String> {
        todo!()
        /*/let default_file = Path::new("output").join("tree.json");
        let filename = output_file.unwrap_or(default_file.clone());
        let extension = Extension::from_path(&filename)?;

        if extension != Extension::JSON {
            return Err(format!("The output file must be a JSON file"));
        }

        let mut writer = fs::File::create(&filename).map_err(|e| e.to_string())?;

        let persons_json: Vec<String> = self.persons.values().map(|p| p.to_json()).collect();

        let json_str_capacity = persons_json.iter().map(String::len).sum::<usize>()
            + persons_json.len() - 1  // Account for the commas between each person JSON
            + 3; // Account for the opening and closing braces and newline

        let mut json_str = String::with_capacity(json_str_capacity);
        json_str.push_str("{\n");
        json_str.push_str(&persons_json.join(","));
        json_str.push_str("\n}");

        write!(writer, "{}", json_str).map_err(|e| e.to_string())?;

        Ok(())*/
    }

    pub fn to_pdf(&self, output_file: Option<PathBuf>) -> Result<(), String> {
        Ok(self.to_graphviz(output_file)?)
    }
    pub fn to_svg(&self, output_file: Option<PathBuf>) -> Result<(), String> {
        Ok(self.to_graphviz(output_file)?)
    }
    pub fn to_png(&self, output_file: Option<PathBuf>) -> Result<(), String> {
        Ok(self.to_graphviz(output_file)?)
    }
    pub fn to_jpg(&self, output_file: Option<PathBuf>) -> Result<(), String> {
        Ok(self.to_graphviz(output_file)?)
    }
}
