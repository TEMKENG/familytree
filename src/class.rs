use crate::class_def::*;
use crate::utils::*;
use petgraph::Graph;
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::fs;
use std::hash::Hash;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::rc::Rc;

#[derive(Debug, Eq, Hash, PartialEq)]
pub struct TreeNode<T: Hash + Eq> {
    pub value: T,
    pub children: Vec<TreeNode<T>>,
}
#[derive(Debug, Default)]
pub struct PersonManager {
    counter: u64,
    persons: HashMap<u64, Person>,
    relationships: HashMap<u64, TreeNode<u64>>,
    graph: Graph<&'static str, &'static str>,
}

impl PersonManager {
    pub fn new() -> Self {
        // PersonManager {
        //     counter: 0,
        //     persons: HashMap::new(),
        //     relationships: HashMap::new(),
        //     graph: Graph::new(),
        // }
        Default::default()
    }

    fn get_person(&self, id: u64) -> Option<&Person> {
        self.persons.get(&id)
    }
    pub fn get_persons(&self) -> &HashMap<u64, Person> {
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

    fn set_mother(&mut self, person_id: u64, mother_id: u64) -> Result<(), String> {
        if let Some(person) = self.persons.get_mut(&person_id) {
            person.mother_id = Some(mother_id);
            Ok(())
        } else {
            Err(format!("Person with ID {} not found", person_id))
        }
    }

    fn set_father(&mut self, person_id: u64, father_id: u64) -> Result<(), String> {
        if let Some(person) = self.persons.get_mut(&person_id) {
            person.father_id = Some(father_id);
            Ok(())
        } else {
            Err(format!("Person with ID {} not found", person_id))
        }
    }

    fn set_parent(&mut self, person_id: u64, mother_id: u64, father_id: u64) -> Result<(), String> {
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
        person_id: u64,
        marital_status: MaritalStatus,
    ) -> Result<(), String> {
        if let Some(person) = self.persons.get_mut(&person_id) {
            person.marital_status = marital_status;
            Ok(())
        } else {
            Err(format!("Person with ID {} not found", person_id))
        }
    }

    pub fn get_person_marital_status(&self, person_id: u64) -> Option<&MaritalStatus> {
        self.persons
            .get(&person_id)
            .map(|person| &person.marital_status)
    }

    pub fn marry(&mut self, p1_id: u64, p2_id: u64) -> Result<(), String> {
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

    pub fn get_person_mut(&mut self, person_id: &u64) -> Option<&mut Person> {
        self.persons.get_mut(person_id)
    }

    pub fn set_address(&mut self, person_id: u64, new_address: Address) {
        if let Some(person) = self.persons.get_mut(&person_id) {
            person.address = new_address;
        }
    }
    pub fn build_family_tree(&self) -> Option<TreeNode<Rc<Person>>> {
        let mut found_tree: bool = false;
        let mut visited: HashSet<u64> = HashSet::new();
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
        person_id: u64,
        visited: &mut HashSet<u64>,
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

    pub fn to_graphviz(&self, file_in: Option<PathBuf>) -> Result<(), String> {
        fs::create_dir_all("output").map_err(|e| e.to_string())?;

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

        Ok(())
    }

    pub fn to_json(&self, output_file: Option<PathBuf>) -> Result<(), String> {
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

        Ok(())
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
