mod class_def;
mod utils;
use class_def::*;
use csv::ReaderBuilder;
use log::{error, info, warn};
use rand::seq::SliceRandom;
use rand::Rng;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use utils::*;

fn get_dummy_addresses(filename: Option<&str>, delimiter: Option<u8>) -> Vec<Address> {
    let mut addr: Vec<Address> = Vec::new();
    // Path to the CSV file
    let filename = filename.unwrap_or("./input/addresse.csv");
    let delimiter = delimiter.unwrap_or(determine_delimiter(filename).unwrap_or(b';'));

    // Try to open the file
    if let Ok(file) = File::open(&filename) {
        // Create a CSV reader
        let mut csv_reader = ReaderBuilder::new().delimiter(delimiter).from_reader(file);
        // Read the CSV datasets
        for result in csv_reader.deserialize::<Address>() {
            if let Ok(record) = result {
                addr.push(record);
            } else {
                error!("Invalid line: {:#?}", result);
            }
        }
    } else {
        error!("Error when opening the file: {}", filename);
    }
    addr
}

fn get_random_address(addresses: &Vec<Address>) -> Address {
    let mut rng = rand::thread_rng();
    let index = rng.gen_range(1..addresses.len());
    addresses[index].clone()
}

fn get_dummy_manager(filename: Option<&str>) -> Result<PersonManager, String> {
    let addresses = get_dummy_addresses(filename, None);
    let mut manager = PersonManager::new();

    // Generation 1
    let mut binding = Person::default("John", "Doe", "1990-01-01", Gender::Male);
    let person1 = binding.set_address(get_random_address(&addresses));

    let mut person = Person::new(
        "Jane",
        "Doe",
        "1992-03-05",
        Gender::Female,
        None,
        Some(MaritalStatus::Married(person1.get_id())),
        None,
        None,
    );
    let person2 = person.set_address(get_random_address(&addresses));

    person1.set_marital_status(MaritalStatus::Married(person2.get_id()));
    manager.add_person(person1);
    manager.add_person(person2);

    let mut binding = Person::default("David", "Doe", "1995-07-10", Gender::Male);
    let person3 = binding
        .set_address(get_random_address(&addresses))
        .set_mother(&person2)
        .set_father(&person1);
    manager.add_person(person3);
    info!("{:#?}", person3.to_graphviz());
    info!("{:#?}", person3.to_json());

    manager.update_marital_status(person1.get_id(), MaritalStatus::Single)?;
    let status = manager.get_person_marital_status(person1.get_id()).unwrap();
    manager.set_mother(person3.get_id(), person2.get_id())?;
    manager.set_father(person3.get_id(), person1.get_id())?;
    manager.set_parent(person3.get_id(), person2.get_id(), person1.get_id())?;

    info!("{:?}", status);

    // 2 Family
    let mut binding = Person::new(
        "Emily",
        "Smith",
        "1998-09-15",
        Gender::Female,
        None,
        None,
        None,
        None,
    );
    let person4 = binding
        .set_address(get_random_address(&addresses))
        .set_marital_status(MaritalStatus::Single);

    manager.add_person(person4);
    manager.set_address(person4.get_id(), get_random_address(&addresses));
    info!("{:#?}", person4);
    info!("{:#?}", person3);

    let mut person5 = Person::new(
        "Michael",
        "Smith",
        "200-12-20",
        Gender::Male,
        Some(get_random_address(&addresses)),
        Some(MaritalStatus::Single),
        None,
        None,
    );
    let mut binding = Person::new(
        "Sarah",
        "Johnson",
        "1997-04-25",
        Gender::Female,
        Some(get_random_address(&addresses)),
        Some(MaritalStatus::Single),
        None,
        None,
    );
    let mut person6 = binding.set_mother(person4).set_father(&person5);
    manager.add_person(&person5);
    manager.add_person(person6);
    manager.marry(person4.get_id(), person5.get_id())?;

    let mut binding = Person::new("Olivia","Smith", "2003-06-30", Gender::Female, Some(get_random_address(&addresses)), Some(MaritalStatus::Single), None, None);
    let mut person7 = binding.set_mother(person3).set_father(person6);
    manager.add_person(person7);
    manager.marry(person3.get_id(), person6.get_id())?;
    let mut visited:  HashSet<u64> = HashSet::new();
    println!("get_root_parents\n{:#?}", person7.get_root_parents(&mut visited));
    println!("get_marital_status\n{:#?}", person6.get_marital_status());
    println!("visited\n{:#?}", visited);
    println!("build_family_tree\n{:#?}", manager.build_family_tree());
    
    
    // println!("{:#?}", manager.tree.get(&person1.get_id()));
    // let person8 = Person {
    //     id: 8,
    //     firstname: "William",
    //     lastname: "Smith",
    //     birthday: "2005-09-10",
    //     address: get_random_address(&addresses),
    //     gender: Gender::Male,
    //     marital_status: MaritalStatus::Single,
    //     mother: Some(2),
    //     father: Some(1),
    //     children: vec![], // No children in the next generation
    // };
    // manager.add_person(person8)?;

    Ok(manager)
}
fn main() -> Result<(), String> {
    utils::set_logger(None);
    let mut manager = get_dummy_manager(None)?;
    warn!("{:#?}", manager);
    /*
        let manager = get_dummy_manager()?;
        // Generate family tree
        manager.build_family_tree();

        // Output tree to Graphviz DOT file
        let mut json_file = PathBuf::new();
        let mut pdf_file = PathBuf::new();
        let mut png_file = PathBuf::new();
        let mut svg_file = PathBuf::new();
        let mut jpg_file = PathBuf::new();

        json_file.push("output/tree.json");
        png_file.push("output/tree.png");
        pdf_file.push("output/tree.pdf");
        svg_file.push("output/tree.svg");
        jpg_file.push("output/tree.jpg");
        let svg_result = manager.to_svg(Some(svg_file.clone()));
        let json_result = manager.to_json(Some(json_file));
        let png_result = manager.to_png(Some(png_file));
        let pdf_result = manager.to_pdf(Some(pdf_file));
        let jpg_result = manager.to_jpg(Some(jpg_file));
        info!("{:?}", svg_result);
        info!("{:?}", json_result);
        info!("{:?}", png_result);
        info!("{:?}", pdf_result);
        info!("{:?}", jpg_result);
        println!("Family tree generated successfully!");
    */
    Ok(())
}
