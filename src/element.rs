use generic_matrix::*;
use node::*;
use element_properties::*;
use std::rc::Rc;

pub enum LoadType{
    PointLoad,
    UDL,
}

pub struct Element {
    nodes: (Rc<Node>, Rc<Node>),
    properties: ElementProperties,
    pub local_k: Matrix<f64>,
    pub global_k: Matrix<f64>,
    pub t_matrix: Matrix<f64>,
    pub load: f64,
    pub load_pos: f64,
    pub load_type: LoadType, 
    p_local: Matrix<f64>,
    p_global: Matrix<f64>,
}

impl Element{
    pub fn new (ns: (Rc<Node>, Rc<Node>), props: ElementProperties, dim: usize)
        -> Element{
            Element{
                nodes: ns,
                properties: props,
                local_k: Matrix::zero(dim, dim),
                global_k: Matrix::zero(dim, dim),
                t_matrix: Matrix::zero(dim, dim),
                load: 0.0,
                load_pos: 0.0,
                load_type: LoadType::PointLoad,
                p_local: Matrix::zero(6, 1), 
                p_global: Matrix::zero(6, 1), 
            }
        }

    pub fn get_element_dof(&self) -> ((usize, usize, usize), 
                                      (usize, usize, usize)) {
        (self.nodes.0.g_dof, self.nodes.1.g_dof)
    }

    pub fn find_t_matrix(&mut self) {
        let node1 = self.nodes.0.clone();
        let node2 = self.nodes.1.clone();

        let l = node1.distance_from(node2.position);
        let (lx, ly) = node1.x_y_distance_from(node2.position);
        let cos_alpha = lx/l;
        let sin_alpha = ly/l; 
        let mut lambda = Matrix::zero(3, 3);

        lambda[(0,0)] = cos_alpha  ;
        lambda[(1,1)] = cos_alpha  ;
        lambda[(0,1)] = sin_alpha  ;
        lambda[(1,0)] = -sin_alpha ;
        lambda[(2,2)] = 1.0;

        for i in 0..2 {
            for j in 0..3 {
                for k in 0..3 {
                    self.t_matrix[(j + i * 3, k + i * 3)] = lambda[(j, k)];
                }
            }
        } 
    }

    pub fn find_global_k(&mut self) {
        let t_trans = self.t_matrix.trans();
        self.global_k = &t_trans * &self.local_k * &self.t_matrix;
    }

    pub fn add_load_to_global(&self, global: &mut Matrix<f64>) {
        let node1 = self.nodes.0.clone();
        let node2 = self.nodes.1.clone();

        global[(node1.g_dof.0 - 1, 0)] += self.p_global[(0, 0)];
        global[(node1.g_dof.1 - 1, 0)] += self.p_global[(1, 0)];
        global[(node1.g_dof.2 - 1, 0)] += self.p_global[(2, 0)];

        global[(node2.g_dof.0 - 1, 0)] += self.p_global[(3, 0)];
        global[(node2.g_dof.1 - 1, 0)] += self.p_global[(4, 0)];
        global[(node2.g_dof.2 - 1, 0)] += self.p_global[(5, 0)];
    }

    pub fn find_local_k (&mut self) {
        let node1 = self.nodes.0.clone();
        let node2 = self.nodes.1.clone();
        let l = node1.distance_from(node2.position);

        let ei_y = self.properties.e * self.properties.iy;
        let gj = self.properties.g * self.properties.j;

        let ei12_l3_y = (12.0 * ei_y) / (l * l * l);
        let ei6_l2_y  = (6.0 * ei_y) / (l * l);
        let ei4_l_y   = (4.0 * ei_y) / l ;     
        let ei2_l_y   = (2.0 * ei_y) / l;      
        let gj_l    = (gj) / l;

        self.local_k[(0,0)]    =  gj_l; 
        self.local_k[(3,3)]    =  gj_l; 

        self.local_k[(0,3)]    =  - gj_l; 
        self.local_k[(3,0)]    =  - gj_l; 

        self.local_k[(1,1)]    =  ei4_l_y; 
        self.local_k[(4,4)]    =  ei4_l_y; 

        self.local_k[(1,2)]    =  - ei6_l2_y; 
        self.local_k[(2,1)]    =  - ei6_l2_y; 
        self.local_k[(2,4)]    =  - ei6_l2_y; 
        self.local_k[(4,2)]    =  - ei6_l2_y;

        self.local_k[(1,5)]    =  ei6_l2_y; 
        self.local_k[(5,1)]    =  ei6_l2_y; 
        self.local_k[(4,5)]    =  ei6_l2_y; 
        self.local_k[(5,4)]    =  ei6_l2_y;

        self.local_k[(2,2)]    =  ei12_l3_y; 
        self.local_k[(5,5)]    =  ei12_l3_y; 
        self.local_k[(2,5)]    =  - ei12_l3_y; 
        self.local_k[(5,2)]    =  - ei12_l3_y;

        self.local_k[(4,1)]    =  ei2_l_y; 
        self.local_k[(1,4)]    =  ei2_l_y; 
    }

    pub fn calculate_fes (&mut self){
        let node1 = self.nodes.0.clone();
        let node2 = self.nodes.1.clone();

        let l = node1.distance_from(node2.position);
        let a = self.load_pos;
        let b = l - a;
        let pz = self.load;

        self.p_local[(0, 0)] =  0.0;        
        self.p_local[(3, 0)] =  0.0;        

        match self.load_type {
            LoadType::PointLoad => {
                self.p_local[(1, 0)] = -pz * (b * b) * a / (l * l);
                self.p_local[(2, 0)] = (pz * (b * b) * ((3.0 * a) + b)) / (l*l*l);
                self.p_local[(4, 0)] =  pz * (a * a) * b / (l * l);
                self.p_local[(5, 0)] = (pz * (a * a) * ((3.0 * b) + a)) / (l*l*l);
            },
            LoadType::UDL => {
                self.p_local[(1, 0)] = -pz * (l * l) / 12.0;
                self.p_local[(2, 0)] =  pz * l / 2.0;
                self.p_local[(4, 0)] =  pz * (l * l) / 12.0;
                self.p_local[(5, 0)] =  pz * l / 2.0;
            },
        }

        self.p_global = self.t_matrix.trans() * &self.p_local;
    }
}
