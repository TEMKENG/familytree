mod class;
mod utils;
use class::{Address, Gender, MaritalStatus, Person, PersonManager, TreeNode};
use log::{error, info, warn};
use petgraph::dot::{Config, Dot};
use petgraph::{graph::NodeIndex, Graph};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;
use std::rc::Rc;
use utils::set_logger;

struct GraphManager {
    graph: Graph<&'static str, &'static str>,
}

impl GraphManager {
    fn new() -> Self {
        GraphManager {
            graph: Graph::new(),
        }
    }
    fn build_graph(&mut self, tree: &TreeNode<Rc<Person>>) {
        let node_label: &'static str = Box::leak(tree.value.to_string().into_boxed_str());
        let parent_node = self.graph.add_node(node_label);
        self.add_node(tree, parent_node);
    }
    fn add_node(&mut self, tree: &TreeNode<Rc<Person>>, parent_node: NodeIndex) {
        for child in &tree.children {
            let child_label: &'static str = Box::leak(child.value.to_string().into_boxed_str());
            let child_node = self.graph.add_node(child_label);
            self.graph.add_edge(parent_node, child_node, "child");
            self.add_node(child, child_node);
        }
    }
}

fn main() -> Result<(), String> {
    set_logger(None);

    // Create a PersonManager
    let mut person_manager: PersonManager = PersonManager::new();

    // Create dummy persons
    let person1: Person = Person {
        id: 1,
        gender: Gender::Male,
        first_name: "John".to_string(),
        last_name: "Doe".to_string(),
        birthday: "1990-01-01".to_string(),
        address: Address {
            street: "123 Main St".to_string(),
            city: "City1".to_string(),
            state: "State1".to_string(),
            country: "Country1".to_string(),
            postal_code: "12345".to_string(),
        },
        marital_status: MaritalStatus::Widowed(2),
        mother_id: None,
        father_id: None,
        children_id: vec![3],
    };

    let person2: Person = Person {
        id: 2,
        gender: Gender::Female,
        first_name: "Jane".to_string(),
        last_name: "Doe".to_string(),
        birthday: "1992-01-01".to_string(),
        address: Address {
            street: "456 Main St".to_string(),
            city: "City1".to_string(),
            state: "State1".to_string(),
            country: "Country1".to_string(),
            postal_code: "12345".to_string(),
        },
        marital_status: MaritalStatus::Widowed(1),
        mother_id: None,
        father_id: None,
        children_id: vec![3],
    };

    let person3: Person = Person {
        id: 3,
        gender: Gender::Male,
        first_name: "Child".to_string(),
        last_name: "Doe".to_string(),
        birthday: "2010-01-01".to_string(),
        address: Address {
            street: "789 Main St".to_string(),
            city: "City1".to_string(),
            state: "State1".to_string(),
            country: "Country1".to_string(),
            postal_code: "12345".to_string(),
        },
        marital_status: MaritalStatus::Single,
        mother_id: Some(2),
        father_id: Some(1),
        children_id: Vec::new(),
    };

    // Add persons to the PersonManager
    person_manager.add_person(person1)?;
    person_manager.add_person(person2)?;
    person_manager.add_person(person3)?;

    let mut graphviz_path: PathBuf = PathBuf::new();
    graphviz_path.push("output/tree.svg");
    if let Err(message) = person_manager.to_graphviz(Some(graphviz_path)) {
        error!("{message}");
    }
    let _json = person_manager.to_json(None)?;
    let json: HashMap<String, Person> =
        serde_json::from_str(&fs::read_to_string("output/tree.json").unwrap())
            .expect("JSON was not well-formatted");
    // Build the family tree
    for p in json.values() {
        // warn!("{:#?}", p);
    }
    let mut graph_manager: GraphManager = GraphManager::new();
    if let Some(tree) = person_manager.build_family_tree() {
        graph_manager.build_graph(&tree);
        let dot = format!(
            "{:?}",
            Dot::with_config(&graph_manager.graph, &[Config::EdgeNoLabel])
        );
        info!("{:#?}", tree);
    } else {
        println!("No family tree found.");
    }
    let test = "La vie est belle";
    warn!("{test}");
    let test = ["/C", "echo hello"];
    info!("{test:#?}");

    Ok(())
}
