# GrillRust
Calculates the load vector and stiffness matrix of a grillage using the matrix method which can then be used to calculate the nodal displacements and internal forces of the structure.
(I'm currently looking for a rust matrix library that supports Gaussian Elimination before this can be added)

A grillage is a structure with elements that are in the x-y plane and external forces in the z direction.

![alt text](https://github.com/alanstoate/GrillRust/blob/master/examples/basic/diagram.png "examples/basic diagram.png")

To run the basic example:
  * Install rust (https://www.rust-lang.org/en-US/install.html)
  * Clone this repository
  * Navigate to examples/basic/ and run with **cargo run**
  

### Usage
As in the example/basic to use GrillRust:

  * Create a new structure
  ```
  
  let mut structure = Structure::new();
  ```
  * Add the nodes
  ```
  structure.add_node((0.0, 2.5), false);
  structure.add_node((5.0, 2.5), false);
  structure.add_node((7.5, 2.5), false);
  structure.add_node((5.0, 7.5), false);
  structure.add_node((5.0, 0.0), false);
  ```
  
  * Add the elements with desired properties
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

## To-Do
* Add documentation to the library
* Use a library with Gaussian eleimnation support to the final step of the method
* Extend library to handle other structure types
