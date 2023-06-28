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
    let person1 = Person::new("John",  "Doe", "1990-01-01");


    //  {
    //     id: 1,
        
    //     address: get_random_address(&addresses),
    //     gender: Gender::Male,
    //     marital_status: MaritalStatus::Married(2),
        
        
    //     children: vec![3], // IDs of children in the next generation
    // };
    // manager.add_person(person1)?;

    // let person2 = Person {
    //     id: 2,
    //     firstname: "Jane",
    //     lastname: "Doe",
    //     birthday: "1992-03-05",
    //     address: get_random_address(&addresses),
    //     gender: Gender::Female,
    //     marital_status: MaritalStatus::Married(1),
    //     
    //     
    //     children: vec![3], // No children in the next generation
    // };
    // manager.add_person(person2)?;

    // let person3 = Person {
    //     id: 3,
    //     firstname: "David",
    //     lastname: "Doe",
    //     birthday: "1995-07-10",
    //     address: get_random_address(&addresses),
    //     gender: Gender::Male,
    //     marital_status: MaritalStatus::Divorced(4),
    //     mother: Some(2),
    //     father: Some(1),
    //     children: vec![6, 5], // IDs of children in the next generation
    // };
    // manager.add_person(person3)?;

    // // Generation 2
    // let person4 = Person {
    //     id: 4,
    //     firstname: "Emily",
    //     lastname: "Smith",
    //     birthday: "1998-09-15",
    //     address: get_random_address(&addresses),
    //     gender: Gender::Female,
    //     marital_status: MaritalStatus::Divorced(3), // ID of the spouse in the same generation
    //     
    //     
    //     children: vec![5, 6], // No children in the next generation
    // };
    // manager.add_person(person4)?;

    // let person5 = Person {
    //     id: 5,
    //     firstname: "Michael",
    //     lastname: "Smith",
    //     birthday: "2000-12-20",
    //     address: get_random_address(&addresses),
    //     gender: Gender::Male,
    //     marital_status: MaritalStatus::Single,
    //     mother: Some(4),
    //     father: Some(3),
    //     children: vec![], // IDs of children in the next generation
    // };
    // manager.add_person(person5)?;

    // // Generation 3
    // let person6 = Person {
    //     id: 6,
    //     firstname: "Sarah",
    //     lastname: "Johnson",
    //     birthday: "1997-04-25",
    //     address: get_random_address(&addresses),
    //     gender: Gender::Female,
    //     marital_status: MaritalStatus::Single, // ID of the spouse in the same generation
    //     mother: Some(4),
    //     father: Some(3),
    //     children: vec![], // No children in the next generation
    // };
    // manager.add_person(person6)?;

    // let person7 = Person {
    //     id: 7,
    //     firstname: "Olivia",
    //     lastname: "Smith",
    //     birthday: "2003-06-30",
    //     address: get_random_address(&addresses),
    //     gender: Gender::Female,
    //     marital_status: MaritalStatus::Single,
    //     mother: Some(4),
    //     father: Some(3),
    //     children: vec![], // No children in the next generation
    // };
    // manager.add_person(person7)?;

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
    let addresses = get_dummy_addresses(None, None);

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
