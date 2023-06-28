use crate::utils::*;
use petgraph::Graph;
use serde::{Deserialize, Serialize, Deserializer, Serializer};
use std::cell::RefCell;
use std::collections::hash_map::DefaultHasher;
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::{self, BufRead, Write};
use std::path::{Path, PathBuf};
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Address {
    street: String,
    city: String,
    state: String,
    country: String,
    postal_code: String,
}

#[derive(Debug, Clone)]
pub enum Delimiter {
    Colon,
    Comma,
    Semicolon,
    Tab,
    VerticalBar,
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Person<'a> {
    id: u64,
    firstname: &'a str,
    lastname: &'a str,
    birthday: &'a str,
    address: Address,
    gender: Gender,
    marital_status: MaritalStatus,
    #[serde(serialize_with = "serialize_children", deserialize_with = "deserialize_children")]
    children: HashMap<u64, Rc<RefCell<Person<'a>>>>,
}

fn serialize_children<S>(children: &HashMap<u64, Rc<RefCell<Person>>>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let serializable_children: HashMap<u64, Person> = children.iter()
        .map(|(&id, child)| (id, child.borrow().clone()))
        .collect();
    serializable_children.serialize(serializer)
}

fn deserialize_children<'de, D>(deserializer: D) -> Result<HashMap<u64, Rc<RefCell<Person<'de>>>>, D::Error>
where
    D: Deserializer<'de>,
{
    let deserialized_children: HashMap<u64, Person> = Deserialize::deserialize(deserializer)?;
    let children = deserialized_children.into_iter()
        .map(|(id, child)| (id, Rc::new(RefCell::new(child))))
        .collect();
    Ok(children)
}

impl Address {
    pub fn new() -> Address {
        Default::default()
    }
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

impl<'a> Person<'a> {
    pub fn new(lastname: &'a str, firstname: &'a str, birthday: &'a str) -> Rc<RefCell<Person<'a>>> {
        Rc::new(RefCell::new(Person {
            id: Person::generate_id(firstname, lastname, birthday),
            firstname: firstname,
            lastname: lastname,
            birthday: birthday,
            address: Address::new(),
            gender: Gender::Male,
            marital_status: MaritalStatus::Single,
            children: HashMap::new(),
        }))
    }
    pub fn new_with_address(
        lastname: &'a str,
        firstname: &'a str,
        birthday: &'a str,
        address: Address,
    ) -> Rc<RefCell<Person<'a>>> {
        Rc::new(RefCell::new(Person {
            id: Person::generate_id(firstname, lastname, birthday),
            firstname: firstname,
            lastname: lastname,
            birthday: birthday,
            address: address,
            gender: Gender::Male,
            marital_status: MaritalStatus::Single,
            children: HashMap::new(),
        }))
    }

    fn add_child(parent: &Rc<RefCell<Person>>, child: Rc<RefCell<Person>>) {
        let id;
        {
            id = (*child.clone().borrow_mut()).id;
        }
        parent
            .borrow_mut()
            .children
            .entry(id)
            .or_insert_with(|| child);
    }

    fn __marital_status__(&self, connect_to: u64) -> String {
        format!(
            "\nID_{0} [shape={1}, color={2}, label=\"\"];\nID_{3} -> ID_{0};\nID_{4} -> ID_{0};\n",
            concat_id(self.id > connect_to, connect_to, self.id),
            self.marital_status.get_shape(),
            self.marital_status.get_color(),
            self.id,
            connect_to,
        )
    }


    pub fn to_string(person: &Rc<RefCell<Person>>) -> String {
        let p = person.borrow();
        format!(
            "ID: {}|{{{}|{}|{} }}",
            p.id, p.lastname, p.firstname, p.birthday
        )
    }

    fn get_couple_id(&self)-> Option<String>{
        match self.marital_status {
            MaritalStatus::Married(to) =>   Some(format!("ID_{}", concat_id(self.id > to, self.id, to))),
            MaritalStatus::Divorced(to) =>  Some(format!("ID_{}", concat_id(self.id > to, self.id, to))),
            MaritalStatus::Widowed(to) =>   Some(format!("ID_{}", concat_id(self.id > to, self.id, to))),
            _ => None,
        }
    }
    pub fn to_graphviz(person: &Rc<RefCell<Person>>) -> String {
        let p = (**person).borrow();
        let mut children_keys: Vec<u64> = p.children.clone().into_keys().collect();


        let mut result: String = format!(
            "ID_{} [shape=record, nojustify=true, color={}, label=\"ID_{}\\n{}\\n{}\\n{}|{{Status: {:?}|Children: {:?}",
            p.id,
            p.gender.get_color(),
            p.id,
            p.lastname,
            p.firstname,
            p.birthday,
            p.marital_status,
            children_keys
        );

        result.push_str("}\"];");

        if let Some(couple_id) = p.get_couple_id(){
            result.push_str(
                &children_keys.into_iter()
                .map(|key| format!("{couple_id} -> {key}\n"))
                .collect::<Vec<String>>().join("")
            );
        }

        result.push_str(&p.get_connection());
        result
    }

    fn is_child(&self) -> bool {
        // self.mother.is_some() && self.father.is_some()
        todo!()
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
        format!(
            "\n\"Person_{}\": {}\n",
            self.id,
            serde_json::to_string(self).unwrap()
        )
    }
    /// Generate a person ID basis of `lastname`, `firstname` and the `birthday`
    fn generate_id(firstname: &str, lastname: &str, birthday: &str) -> u64 {
        let mut hasher = DefaultHasher::new();
        let input = format!("{}{}{}", firstname, lastname, birthday);
        input.hash(&mut hasher);
        hasher.finish()
    }
    pub fn set_address(person: &Rc<RefCell<Person>>, address: Address) {
        person.borrow_mut().address = address;
    }
    fn set_mother(person: &Rc<RefCell<Person>>, mother_id: u64) {
        // person.borrow_mut().mother = Some(mother_id);
        todo!()
    }
    fn set_father(person: &Rc<RefCell<Person>>, father_id: u64) {
        // person.borrow_mut().father = Some(father_id);
        todo!()
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct TreeNode<T: Eq> {
    pub value: T,
    pub children: Vec<TreeNode<T>>,
}
#[derive(Debug, Default)]
pub struct PersonManager<'a> {
    counter: u64,
    persons: HashMap<u64, Rc<RefCell<Person<'a>>>>,
    relationships: HashMap<u64, TreeNode<u64>>,
    graph: Graph<&'static str, &'static str>,
}

impl<'a> PersonManager<'a> {
    pub fn new() -> Self {
        // PersonManager {
        //     counter: 0,
        //     persons: HashMap::new(),
        //     relationships: HashMap::new(),
        //     graph: Graph::new(),
        // }
        Default::default()
    }

    fn get_person(&self, id: u64) -> Option<&Rc<RefCell<Person<'a>>>> {
        self.persons.get(&id)
    }
    pub fn get_persons(&self) -> &HashMap<u64, Rc<RefCell<Person<'a>>>> {
        &self.persons
    }
    pub fn add_person(&mut self, person: Rc<RefCell<Person>>) -> Result<(), String> {
        todo!()
        /*
        let rc_person = person.borrow();
        if self.persons.contains_key(&rc_person.id) {
            return Err(format!("Person with ID {} already exists", rc_person.id));
        }
        if let Some(mother) = rc_person.mother {
            mother.borrow_mut().add_child(person);
        }
        if let Some(father) = rc_person.father {
            father.borrow_mut().add_child(person);
        }
        self.persons.insert(rc_person.id, person);
        self.counter += 1;

        Ok(())*/
    }

    fn set_mother(&mut self, person_id: u64, mother_id: u64) -> Result<(), String> {
        // if let Some(person) = self.persons.get(&person_id) {
        //     let mut rc_person = person.borrow_mut();
        //     if let Some(mother) = self.persons.get(&mother_id) {
        //         person.borrow_mut().set_mother(mother);
        //         let tree = BTreeMap::new();
        //         mother.borrow_mut().set_mother()
        //         rc_person.mother = Some(mother.clone());
        //         mother.borrow_mut().add_child(person);
        //     } else {
        //         return Err(format!("Person with ID {} not found", person_id));
        //     }
        //     Ok(())
        // } else {
        //     Err(format!("Person with ID {} not found", person_id))
        // }

        todo!()
    }

    fn set_father(&mut self, person_id: u64, father_id: u64) -> Result<(), String> {
        // if let Some(person) = self.persons.get_mut(&person_id) {
        //     person.father = Some(father_id);
        //     Ok(())
        // } else {
        //     Err(format!("Person with ID {} not found", person_id))
        // }

        todo!()
    }

    fn set_parent(&mut self, person_id: u64, mother_id: u64, father_id: u64) -> Result<(), String> {
        // if let Some(person) = self.persons.get_mut(&person_id) {
        //     person.mother = Some(mother_id);
        //     person.father = Some(father_id);
        //     Ok(())
        // } else {
        //     Err(format!("Person with ID {} not found", person_id))
        // }

        todo!()
    }

    pub fn set_person_marital_status(
        &mut self,
        person_id: u64,
        marital_status: MaritalStatus,
    ) -> Result<(), String> {
        // if let Some(person) = self.persons.get_mut(&person_id) {
        //     person.marital_status = marital_status;
        //     Ok(())
        // } else {
        //     Err(format!("Person with ID {} not found", person_id))
        // }
        todo!()
    }

    pub fn get_person_marital_status(&self, person_id: u64) -> Option<&MaritalStatus> {
        // self.persons
        //     .get(&person_id)
        //     .map(|person| &person.marital_status)

        todo!()
    }

    pub fn marry(&mut self, p1_id: u64, p2_id: u64) -> Result<(), String> {
        // if let Some(person1) = self.persons.get_mut(&p1_id) {
        //     person1.marital_status = MaritalStatus::Married(p2_id);
        // } else {
        //     return Err(format!("Person with ID {} not found", p1_id));
        // }

        // if let Some(person2) = self.persons.get_mut(&p2_id) {
        //     person2.marital_status = MaritalStatus::Married(p1_id);
        // } else {
        //     // Rollback the marital status change for person1 if person2 is not found
        //     if let Some(person1) = self.persons.get_mut(&p1_id) {
        //         person1.marital_status = MaritalStatus::Single;
        //     }
        //     return Err(format!("Person with ID {} not found", p2_id));
        // }

        // Ok(())

        todo!()
    }

    pub fn get_person_mut(&mut self, person_id: &u64) -> Option<&mut Person> {
        // self.persons.get_mut(person_id)

        todo!()
    }

    pub fn set_address(&mut self, person_id: u64, new_address: Address) {
        // if let Some(person) = self.persons.get_mut(&person_id) {
        //     person.address = new_address;
        // }
        todo!()
    }
    pub fn build_family_tree(&self) -> Option<TreeNode<Rc<Person>>> {
        /*let mut found_tree: bool = false;
        let mut visited: HashSet<u64> = HashSet::new();
        let mut root_nodes: HashSet<TreeNode<Rc<Person>>> = HashSet::new();

        for root_person in self.persons.values() {
            if root_person.children.is_empty() || visited.contains(&root_person.id) {
                continue;
            }

            found_tree = true;
            visited.insert(root_person.id);
            let root_node: TreeNode<Rc<Person>> = TreeNode {
                value: Rc::new(root_person.clone()),
                children: self.build_tree_recursive(root_person.id, &mut visited),
            };

            root_nodes.insert(root_node);
        }

        if !found_tree {
            return None;
        }

        if root_nodes.len() == 1 {
            Some(root_nodes.into_iter().next().unwrap())
        } else {
            // Create a dummy "Family" node
            let max_id = self.persons.keys().max().copied().unwrap_or(0) + 1;
            let dummy_person = Person {
                id: max_id,
                firstname: "Ancestor".to_string(),
                lastname: "".to_string(),
                birthday: "".to_string(),
                gender: Gender::Male,
                address: Address {
                    street: "".to_string(),
                    city: "".to_string(),
                    state: "".to_string(),
                    country: "".to_string(),
                    postal_code: "".to_string(),
                },
                marital_status: MaritalStatus::Single,
                mother: None,
                father: None,
                children: Vec::new(),
            };
            let dummy_node: TreeNode<Rc<Person>> = TreeNode {
                value: Rc::new(dummy_person),
                children: root_nodes.into_iter().collect(),
            };

            Some(dummy_node)
        }*/
        todo!()
    }

    pub fn build_tree_recursive(
        &self,
        person_id: u64,
        visited: &mut HashSet<u64>,
    ) -> Vec<TreeNode<Rc<Person>>> {
        /*let mut children_nodes: HashSet<TreeNode<Rc<Person>>> = HashSet::new();

        if let Some(person) = self.persons.get(&person_id) {
            for child_id in &person.children {
                if visited.contains(child_id) {
                    continue;
                }
                visited.insert(*child_id);

                if let Some(child_person) = self.persons.get(child_id) {
                    let child_node = TreeNode {
                        value: Rc::new(child_person.clone()),
                        children: self.build_tree_recursive(*child_id, visited),
                    };
                    children_nodes.insert(child_node);
                }
            }
        }

        children_nodes.into_iter().collect()*/

        todo!()
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
