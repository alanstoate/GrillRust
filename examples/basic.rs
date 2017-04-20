extern crate GrillRust;

use GrillRust::element_properties::*;
use GrillRust::structure::*;
use GrillRust::element::*;
use GrillRust::utils::*;

fn main() {
    let props = ElementProperties::new(200E+6, 2.133E-6, 80E+6, 2.03E-6);
    let mut structure = Structure::new();
    structure.add_node((0.0, 2.5), false);
    structure.add_node((5.0, 2.5), false);
    structure.add_node((7.5, 2.5), false);
    structure.add_node((5.0, 7.5), false);
    structure.add_node((5.0, 0.0), false);

    structure.add_element(0, 1, props);
    structure.add_element(1, 2, props);
    structure.add_element(1, 3, props);
    structure.add_element(1, 4, props);

    structure.add_element_load(0, -3.0, 0.0, LoadType::UDL);
    structure.add_element_load(2, -40.0, 2.50, LoadType::PointLoad);

    structure.run_calc();

    print_matrix(&structure.global_k_matrix);
}
