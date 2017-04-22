// Represents a node within a structure

use rulinalg::matrix::Matrix;

pub struct Node {
    pub position: (f64, f64),
    pub g_dof: (usize, usize, usize),
    pub fixed: (bool, bool, bool),
    pub load: f64,
}

impl Node {
    pub fn new(pos: (f64, f64), n: usize) -> Node {
        let dof = ((n) * 3 + 1, (n) * 3 + 2, (n) * 3 + 3);
        Node {
            position: pos,
            g_dof: dof,
            fixed: (false, false, false),
            load: 0.0,
        }
    }

    pub fn x_y_distance_from(&self, other: (f64, f64)) -> (f64, f64) {
        let (x1, y1) = self.position;
        let (x2, y2) = other;
        (x2 - x1, y2 - y1)
    }

    pub fn distance_from(&self, other: (f64, f64)) -> f64 {
        let (dx, dy) = self.x_y_distance_from(other);
        (dx * dx + dy * dy).sqrt()
    }

    pub fn fixed_xyz(&mut self) -> &Node {
        self.fixed = (true, true, true);
        self
    }

    pub fn add_load_to_global(&self, global: &mut Matrix<f64>) {
        global[[self.g_dof.2 - 1, 0]] = global[[self.g_dof.2 - 1, 0]] + self.load;
    }
}
