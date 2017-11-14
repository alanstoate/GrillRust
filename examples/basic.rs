extern crate GrillRust;
extern crate rulinalg;

use GrillRust::element_properties::*;
use GrillRust::structure::*;
use GrillRust::element::*;
use GrillRust::utils::*;

use rulinalg::matrix::{Matrix, BaseMatrix};


fn main() {
    // Initialise Element properties
    let props = ElementProperties::new(426.6, 1.0, 167.1, 1.0);
    let mut structure = Structure::new();

    // Add nodes
    structure.add_node((0.0, 2.5), true);
    structure.add_node((5.0, 2.5), false);
    structure.add_node((7.5, 2.5), true);
    structure.add_node((5.0, 7.5), true);
    structure.add_node((5.0, 0.0), true);

    // Add elements
    structure.add_element(0, 1, props);
    structure.add_element(1, 2, props);
    structure.add_element(1, 3, props);
    structure.add_element(1, 4, props);

    // Add loads
    //structure.add_node_load(1, -5.0);

    structure.add_element_load(0, -3.0, 0.0, LoadType::UDL);
    structure.add_element_load(1, -5.0, 0.0, LoadType::PointLoad);
    structure.add_element_load(2, -40.0, 2.50, LoadType::PointLoad);

    structure.run_calc();

    println!("Load vector");
    print_matrix(&structure.global_loads);
    println!("\n");

    println!("Stiffness matrix");
    print_matrix(&structure.global_k_matrix);
    println!("\n");

    println!("Displacements");
    for i in structure.displacements.iter() {
        println!("{}", i);
    }
    println!("\n");
}
