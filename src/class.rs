use petgraph::Graph;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::ffi::OsStr;
use std::fmt::Debug;
use std::fs;
use std::hash::Hash;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::rc::Rc;
#[derive(Debug, PartialEq)]
pub enum Extension {
    DOT,
    PNG,
    PDF,
    SVG,
    JSON,
}

impl Extension {
    fn from_str(s: &str) -> Result<Extension, String> {
        match s {
            "DOT" => Ok(Extension::DOT),
            "PNG" => Ok(Extension::PNG),
            "PDF" => Ok(Extension::PDF),
            "SVG" => Ok(Extension::SVG),
            "JSON" => Ok(Extension::JSON),
            _ => Err(format!("Extension '{}' is not yet supported", s)),
        }
    }

    fn from_path(path: &Path) -> Result<Extension, String> {
        let extension = path
            .extension()
            .and_then(|ext| ext.to_str())
            .ok_or_else(|| {
                format!(
                    "Extension::from_path: File '{}' must have an extension",
                    path.display()
                )
            })?;

        Extension::from_str(extension.to_uppercase().as_str())
    }

    fn get_dot_command(&self, file_in: &Path, file_out: &Path) -> String {
        match self {
            Extension::PNG => format!("-Tpng {} -o {}", file_in.display(), file_out.display()),
            Extension::PDF => format!("-Tpdf {} -o {}", file_in.display(), file_out.display()),
            Extension::SVG => format!("-Tsvg {} -o {}", file_in.display(), file_out.display()),
            // Extension::JSON => format!("-Tjson {} -o {}", file_in.display(), file_out.display()),
            _ => format!("-Tsvg {} -o {}", file_in.display(), file_out.display()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Gender {
    Male,
    Female,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MaritalStatus {
    Single,
    Married(u32),
    Divorced(u32),
    Widowed(u32),
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
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Address {
    pub street: String,
    pub city: String,
    pub state: String,
    pub country: String,
    pub postal_code: String,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct Person {
    pub id: u32,
    pub first_name: String,
    pub last_name: String,
    pub birthday: String,
    pub address: Address,
    pub gender: Gender,
    pub marital_status: MaritalStatus,
    pub mother_id: Option<u32>,
    pub father_id: Option<u32>,
    pub children_id: Vec<u32>,
}
pub fn get_extension(filename: &str) -> Option<&str> {
    Path::new(filename).extension().and_then(OsStr::to_str)
}

fn ternary<T: Debug>(condition: bool, a: T, b: T) -> T {
    if condition {
        a
    } else {
        b
    }
}

fn concat_id<T: Debug>(condition: bool, id_1: T, id_2: T) -> String {
    if condition {
        format!("{:?}_{:?}", id_1, id_2)
    } else {
        format!("{:?}_{:?}", id_2, id_1)
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
    fn _marital_status(id_1: u32, id_2: u32, color: String, form: String) -> String {
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
}
#[derive(Debug, Eq, Hash, PartialEq)]
pub struct TreeNode<T: Hash + Eq> {
    pub value: T,
    pub children: Vec<TreeNode<T>>,
}

pub struct PersonManager {
    counter: u32,
    persons: HashMap<u32, Person>,
    relationships: HashMap<u32, TreeNode<u32>>,
    graph: Graph<&'static str, &'static str>,
}

impl PersonManager {
    pub fn new() -> Self {
        PersonManager {
            counter: 0,
            persons: HashMap::new(),
            relationships: HashMap::new(),
            graph: Graph::new(),
        }
    }

    fn get_person(&self, id: u32) -> Option<&Person> {
        self.persons.get(&id)
    }
    pub fn get_persons(&self) -> &HashMap<u32, Person> {
        &self.persons
    }
    pub fn add_person(&mut self, person: Person) -> Result<(), String> {
        if self.persons.contains_key(&person.id) {
            return Err(format!("Person with ID {} already exists", person.id));
        }
        if let Some(mother_id) = person.mother_id {
            if let Some(mother) = self.persons.get_mut(&mother_id) {
                if !mother.children_id.contains(&person.id) {
                    mother.children_id.push(person.id);
                }
            } else {
                return Err(format!("Mother with ID {} not found", mother_id));
            }
        }
        if let Some(father_id) = person.father_id {
            if let Some(father) = self.persons.get_mut(&father_id) {
                if !father.children_id.contains(&person.id) {
                    father.children_id.push(person.id);
                }
            } else {
                return Err(format!("Father with ID {} not found", father_id));
            }
        }
        self.persons.insert(person.id, person);
        self.counter += 1;

        Ok(())
    }

    fn set_mother(&mut self, person_id: u32, mother_id: u32) -> Result<(), String> {
        if let Some(person) = self.persons.get_mut(&person_id) {
            person.mother_id = Some(mother_id);
            Ok(())
        } else {
            Err(format!("Person with ID {} not found", person_id))
        }
    }

    fn set_father(&mut self, person_id: u32, father_id: u32) -> Result<(), String> {
        if let Some(person) = self.persons.get_mut(&person_id) {
            person.father_id = Some(father_id);
            Ok(())
        } else {
            Err(format!("Person with ID {} not found", person_id))
        }
    }

    fn set_parent(&mut self, person_id: u32, mother_id: u32, father_id: u32) -> Result<(), String> {
        if let Some(person) = self.persons.get_mut(&person_id) {
            person.mother_id = Some(mother_id);
            person.father_id = Some(father_id);
            Ok(())
        } else {
            Err(format!("Person with ID {} not found", person_id))
        }
    }

    pub fn set_person_marital_status(
        &mut self,
        person_id: u32,
        marital_status: MaritalStatus,
    ) -> Result<(), String> {
        if let Some(person) = self.persons.get_mut(&person_id) {
            person.marital_status = marital_status;
            Ok(())
        } else {
            Err(format!("Person with ID {} not found", person_id))
        }
    }

    pub fn get_person_marital_status(&self, person_id: u32) -> Option<&MaritalStatus> {
        self.persons
            .get(&person_id)
            .map(|person| &person.marital_status)
    }

    pub fn marry(&mut self, p1_id: u32, p2_id: u32) -> Result<(), String> {
        if let Some(person1) = self.persons.get_mut(&p1_id) {
            person1.marital_status = MaritalStatus::Married(p2_id);
        } else {
            return Err(format!("Person with ID {} not found", p1_id));
        }

        if let Some(person2) = self.persons.get_mut(&p2_id) {
            person2.marital_status = MaritalStatus::Married(p1_id);
        } else {
            // Rollback the marital status change for person1 if person2 is not found
            if let Some(person1) = self.persons.get_mut(&p1_id) {
                person1.marital_status = MaritalStatus::Single;
            }
            return Err(format!("Person with ID {} not found", p2_id));
        }

        Ok(())
    }

    pub fn get_person_mut(&mut self, person_id: &u32) -> Option<&mut Person> {
        self.persons.get_mut(person_id)
    }

    pub fn build_family_tree(&self) -> Option<TreeNode<Rc<Person>>> {
        let mut found_tree: bool = false;
        let mut visited: HashSet<u32> = HashSet::new();
        let mut root_nodes: HashSet<TreeNode<Rc<Person>>> = HashSet::new();

        for root_person in self.persons.values() {
            if root_person.children_id.is_empty() || visited.contains(&root_person.id) {
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
                first_name: "Ancestor".to_string(),
                last_name: "".to_string(),
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
                mother_id: None,
                father_id: None,
                children_id: Vec::new(),
            };
            let dummy_node: TreeNode<Rc<Person>> = TreeNode {
                value: Rc::new(dummy_person),
                children: root_nodes.into_iter().collect(),
            };

            Some(dummy_node)
        }
    }

    pub fn build_tree_recursive(
        &self,
        person_id: u32,
        visited: &mut HashSet<u32>,
    ) -> Vec<TreeNode<Rc<Person>>> {
        let mut children_nodes: HashSet<TreeNode<Rc<Person>>> = HashSet::new();

        if let Some(person) = self.persons.get(&person_id) {
            for child_id in &person.children_id {
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

        children_nodes.into_iter().collect()
    }
    /// Generates a Graphviz representation of the tree and writes it to a file.
    ///
    /// This function generates a Graphviz representation of the tree using the DOT language and writes it to a file.
    /// The file format is determined based on the provided `graphviz_path` or defaults to "output/tree.dot" if not specified.
    /// The function creates the necessary output directory if it doesn't exist.
    ///
    /// # Arguments
    ///
    /// * `graphviz_path` - Optional path for the Graphviz output file. If not provided, the default path "output/tree.dot" is used.
    ///
    /// # Errors
    ///
    /// This function returns an error if:
    /// * The provided `graphviz_path` doesn't have an extension.
    /// * The Graphviz output file cannot be created or written to.
    /// * The "dot" command execution fails when converting the DOT file to the desired format.
    ///
    /// # Example
    ///
    /// ```rust
    /// use tree::PersonManager;
    ///
    /// let tree = PersonManager::new();
    /// tree.to_graphviz(Some("output/tree.png")).unwrap();
    /// ```
    pub fn to_graphviz(&self, graphviz_path: Option<PathBuf>) -> Result<(), String> {
        fs::create_dir_all("output").map_err(|e| e.to_string())?;

        let default_file = Path::new("output").join("tree.dot");
        let filename = graphviz_path.unwrap_or(default_file.clone());
        let extension: Extension = Extension::from_path(&filename)?;

        let mut writer = fs::File::create(&filename).map_err(|e| e.to_string())?;

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

        Ok(())
    }
    /// Generates a JSON representation of the tree and writes it to a file.
    ///
    /// This function generates a JSON representation of the tree and writes it to a file.
    /// The output file format must be a JSON file. The file format and path are determined based on the provided `output_file`
    /// or defaults to "output/tree.json" if not specified.
    /// The function creates the necessary output directory if it doesn't exist.
    ///
    /// # Arguments
    ///
    /// * `output_file` - Optional path for the JSON output file. If not provided, the default path "output/tree.json" is used.
    ///
    /// # Errors
    ///
    /// This function returns an error if:
    /// * The provided `output_file` doesn't have a JSON extension.
    /// * The JSON output file cannot be created or written to.
    ///
    /// # Returns
    ///
    /// This function returns the generated JSON string on success.
    ///
    /// # Example
    ///
    /// ```rust
    /// use my_tree::Tree;
    ///
    /// let tree = Tree::new();
    /// let json_str = tree.to_json(Some("output/tree.json")).unwrap();
    /// println!("Generated JSON: {}", json_str);
    /// ```
    pub fn to_json(&self, output_file: Option<PathBuf>) -> Result<String, String> {
        let default_file = Path::new("output").join("tree.json");
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

        Ok(json_str)
    }

    pub fn to_pdf(&self, output_file: Option<PathBuf>) -> Result<String, String> {
        todo!()
    }
    pub fn to_svg(&self, output_file: Option<PathBuf>) -> Result<String, String> {
        todo!()
    }
    pub fn to_png(&self, output_file: Option<PathBuf>) -> Result<String, String> {
        todo!()
    }
}
