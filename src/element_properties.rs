// The internal properties of the beam

#[derive(Copy, Clone)]
pub struct ElementProperties {
    pub e: f64,
    pub iy: f64,
    pub g: f64,
    pub j: f64,
}

impl ElementProperties {
    pub fn new(_e: f64, _iy: f64, _g: f64, _j: f64) -> ElementProperties {
        ElementProperties {
            e: _e,
            iy: _iy,
            g: _g,
            j: _j,
        }
    }

    pub fn default() -> ElementProperties {
        ElementProperties {
            e: 200E+6,
            iy: 1.406E-6,
            g: 80E+6,
            j: 1.061E-6,
        }
    }
}
