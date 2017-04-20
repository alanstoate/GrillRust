use std::vec::Vec;
use std::rc::Rc;
use generic_matrix::*;
use element::*;
use node::*;
use element_properties::*;

pub struct Structure {
    pub nodes: Vec<Rc<Node>>,
    pub elements: Vec<Element>,
    pub global_k_matrix: Matrix<f64>,
    pub global_loads: Matrix<f64>,
}

impl Structure {
    // Creates a default Structure
    pub fn new() -> Structure {
        Structure {
            nodes: Vec::new(),
            elements: Vec::new(),
            global_k_matrix: Matrix::zero(10, 10),
            global_loads: Matrix::zero(1, 1),
        }
    }

    pub fn add_node(&mut self, pos: (f64, f64), fixed: bool) {
        let len = self.nodes.len();
        let mut node = Node::new(pos, len);
        if fixed {
            node.fixed_xyz();
        }
        self.nodes.push(Rc::new(node));
        self.global_k_matrix = Matrix::zero((len + 1) * 3, (len + 1) * 3);
        self.global_loads = Matrix::zero((len + 1) * 3, 1);
    }

    // Adds a load to the specified node
    pub fn add_node_load(&mut self, node: usize, load: f64) {
        let node = Rc::get_mut(&mut self.nodes[node]);
        match node {
            Some(n) => n.load = load,
            None => println!("Couldnt add load"),
        }
    }

    // Adds an load to the specified element
    pub fn add_element_load(&mut self, element: usize, load: f64, distance: f64, ltype: LoadType) {
        self.elements[element].load = load;
        self.elements[element].load_pos = distance;
        self.elements[element].load_type = ltype;
        self.elements[element].calculate_fes();

    }

    // Adds an element to the structure
    pub fn add_element(&mut self, n1: usize, n2: usize, props: ElementProperties) {
        let mut element = Element::new((self.nodes[n1].clone(), self.nodes[n2].clone()), props, 6);

        element.find_t_matrix();
        element.find_local_k();
        element.find_global_k();
        self.add_element_k_to_global(&element);
        self.elements.push(element);
    }

    fn add_element_k_to_global(&mut self, element: &Element) {
        let (d1, d2) = element.get_element_dof();
        let mut g_dof: Vec<usize> = Vec::new();
        g_dof.push(d1.0);
        g_dof.push(d1.1);
        g_dof.push(d1.2);
        g_dof.push(d2.0);
        g_dof.push(d2.1);
        g_dof.push(d2.2);

        for i in 0..6 {
            for j in 0..6 {
                let (i_g, j_g) = (g_dof[i] - 1, g_dof[j] - 1);
                self.global_k_matrix[(i_g, j_g)] += element.global_k[(i, j)];
            }
        }
    }

    // Calculates and prints the global load and stifness matrices of the structure
    pub fn run_calc(&mut self) {
        for node in &self.nodes {
            if node.fixed == (true, true, true) {
                let d0 = node.g_dof.0 - 1;
                let d1 = node.g_dof.1 - 1;
                let d2 = node.g_dof.2 - 1;

                self.global_k_matrix[(d0, d0)] = 999999.0;
                self.global_k_matrix[(d1, d1)] = 999999.0;
                self.global_k_matrix[(d2, d2)] = 999999.0;
            }

            node.add_load_to_global(&mut self.global_loads);
        }

        for element in &self.elements {
            element.add_load_to_global(&mut self.global_loads);
        }
    }
}
