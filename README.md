# GrillRust
Calculates the load vector and stiffness matrix of a grillage using the matrix method of structural analysis which can then be used to calculate the nodal displacements and internal forces of the structure.

A grillage is a structure with elements that are in the x-y plane and external forces in the z direction.

![alt text](https://github.com/alanstoate/GrillRust/blob/master/examples/img/diagram.png "examples/img/diagram.png")

To run the basic example:
  * Install rust (https://www.rust-lang.org/en-US/install.html)
  * Clone this repository with ``` git clone https://github.com/alanstoate/GrillRust.git ``` 
  * Run ```cargo run --example basic```

This will output the load vector, stiffness matrix and displacement vector of the structure.
  

### Usage
As in the example/basic to use GrillRust:

  * Create a new structure
  ```
  
  let mut structure = Structure::new();
  ```
  * Add the nodes (passing true as the second argument indicates a fixed node)
  ```
  structure.add_node((0.0, 2.5), true);
  structure.add_node((5.0, 2.5), false);
  structure.add_node((7.5, 2.5), true);
  structure.add_node((5.0, 7.5), true);
  structure.add_node((5.0, 0.0), true);
  ```
  
  * Add the elements by specifying their nodes and desired properties
  ```
  let props = ElementProperties::new(200E+6, 2.133E-6, 80E+6, 2.03E-6);
  structure.add_element(0, 1, props);
  structure.add_element(1, 2, props);
  structure.add_element(1, 3, props);
  structure.add_element(1, 4, props);
  ```
  
  * Add the loads to the structure
  ```
  structure.add_element_load(0, -3.0, 0.0, LoadType::UDL);
  structure.add_element_load(2, -40.0, 2.50, LoadType::PointLoad);
  ```
  
  * Run the calculation
  ```
  structure.run_calc();
  ```
