// Utility functions

use rulinalg::matrix::{Matrix, BaseMatrix};

pub fn print_matrix(m: &Matrix<f64>) {
    for row in 0..m.rows() {
        let mut out: String = format!("{}: ", row);
        for col in 0..m.cols() {
            out += &format!(" {:10.*} ,", 2, m[[row, col]]);
        }
        println!("{}", out);
    }
}

pub fn string_matrix(m: &Matrix<f64>) -> String {
    let mut out = String::new();
    for row in 0..m.rows() {
        let mut out_row: String = format!("{}: ", row);
        for col in 0..m.cols() {
            out_row += &format!(" {:10.*} ,", 2, m[[row, col]]);
        }
        out.push_str(&out_row);
        out.push_str("\n");
    }
    out
}
