mod class;
mod class_def;
mod class_impl;
mod utils;
use class::PersonManager;
use log::{error, info, warn};
use std::collections::{HashMap, HashSet};
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;
use utils::*;
use class_def::*;

use std::path::Path;


fn get_dummy_manager() -> Result<PersonManager, String> {
    let mut manager = PersonManager::new();

    // Generation 1
    let person1 = Person {
        id: 1,
        first_name: "John".to_string(),
        last_name: "Doe".to_string(),
        birthday: "1990-01-01".to_string(),
        address: Address {
            street: "123 Main St".to_string(),
            city: "New York".to_string(),
            state: "NY".to_string(),
            country: "USA".to_string(),
            postal_code: "10001".to_string(),
        },
        gender: Gender::Male,
        marital_status: MaritalStatus::Married(2),
        mother_id: None,
        father_id: None,
        children_id: vec![3], // IDs of children in the next generation
    };
    manager.add_person(person1)?;

    let person2 = Person {
        id: 2,
        first_name: "Jane".to_string(),
        last_name: "Doe".to_string(),
        birthday: "1992-03-05".to_string(),
        address: Address {
            street: "123 Main St".to_string(),
            city: "New York".to_string(),
            state: "NY".to_string(),
            country: "USA".to_string(),
            postal_code: "10001".to_string(),
        },
        gender: Gender::Female,
        marital_status: MaritalStatus::Married(1),
        mother_id: None,
        father_id: None,
        children_id: vec![3], // No children in the next generation
    };
    manager.add_person(person2)?;

    let person3 = Person {
        id: 3,
        first_name: "David".to_string(),
        last_name: "Doe".to_string(),
        birthday: "1995-07-10".to_string(),
        address: Address {
            street: "123 Main St".to_string(),
            city: "New York".to_string(),
            state: "NY".to_string(),
            country: "USA".to_string(),
            postal_code: "10001".to_string(),
        },
        gender: Gender::Male,
        marital_status: MaritalStatus::Divorced(4),
        mother_id: Some(2),
        father_id: Some(1),
        children_id: vec![6, 5], // IDs of children in the next generation
    };
    manager.add_person(person3)?;

    // Generation 2
    let person4 = Person {
        id: 4,
        first_name: "Emily".to_string(),
        last_name: "Smith".to_string(),
        birthday: "1998-09-15".to_string(),
        address: Address {
            street: "456 Elm St".to_string(),
            city: "New York".to_string(),
            state: "NY".to_string(),
            country: "USA".to_string(),
            postal_code: "10002".to_string(),
        },
        gender: Gender::Female,
        marital_status: MaritalStatus::Divorced(3), // ID of the spouse in the same generation
        mother_id: None,
        father_id: None,
        children_id: vec![5, 6], // No children in the next generation
    };
    manager.add_person(person4)?;

    let person5 = Person {
        id: 5,
        first_name: "Michael".to_string(),
        last_name: "Smith".to_string(),
        birthday: "2000-12-20".to_string(),
        address: Address {
            street: "456 Elm St".to_string(),
            city: "New York".to_string(),
            state: "NY".to_string(),
            country: "USA".to_string(),
            postal_code: "10002".to_string(),
        },
        gender: Gender::Male,
        marital_status: MaritalStatus::Single,
        mother_id: Some(4),
        father_id: Some(3),
        children_id: vec![], // IDs of children in the next generation
    };
    manager.add_person(person5)?;

    // Generation 3
    let person6 = Person {
        id: 6,
        first_name: "Sarah".to_string(),
        last_name: "Johnson".to_string(),
        birthday: "1997-04-25".to_string(),
        address: Address {
            street: "789 Oak St".to_string(),
            city: "New York".to_string(),
            state: "NY".to_string(),
            country: "USA".to_string(),
            postal_code: "10003".to_string(),
        },
        gender: Gender::Female,
        marital_status: MaritalStatus::Single, // ID of the spouse in the same generation
        mother_id: Some(4),
        father_id: Some(3),
        children_id: vec![], // No children in the next generation
    };
    manager.add_person(person6)?;

    let person7 = Person {
        id: 7,
        first_name: "Olivia".to_string(),
        last_name: "Smith".to_string(),
        birthday: "2003-06-30".to_string(),
        address: Address {
            street: "456 Elm St".to_string(),
            city: "New York".to_string(),
            state: "NY".to_string(),
            country: "USA".to_string(),
            postal_code: "10002".to_string(),
        },
        gender: Gender::Female,
        marital_status: MaritalStatus::Single,
        mother_id: Some(4),
        father_id: Some(3),
        children_id: vec![], // No children in the next generation
    };
    manager.add_person(person7)?;

    let person8 = Person {
        id: 8,
        first_name: "William".to_string(),
        last_name: "Smith".to_string(),
        birthday: "2005-09-10".to_string(),
        address: Address {
            street: "456 Elm St".to_string(),
            city: "New York".to_string(),
            state: "NY".to_string(),
            country: "USA".to_string(),
            postal_code: "10002".to_string(),
        },
        gender: Gender::Male,
        marital_status: MaritalStatus::Single,
        mother_id: Some(2),
        father_id: Some(1),
        children_id: vec![], // No children in the next generation
    };
    manager.add_person(person8)?;

    Ok(manager)
}
fn main() -> Result<(), String> {
    utils::set_logger(None);

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

    Ok(())
}
