// Utility functions

use generic_matrix::*;

pub fn print_matrix(m: &Matrix<f64>) {
    for row in 0..m.row() {
        let mut out: String = format!("{}: ", row);
        for col in 0..m.column() {
            out += &format!(" {:10.*} ,", 2, m[(row, col)]);
        }
        println!("{}", out);
    }
}

pub fn string_matrix(m: &Matrix<f64>) -> String {
    let mut out = String::new();
    for row in 0..m.row() {
        let mut outRow: String = format!("{}: ", row);
        for col in 0..m.column() {
            outRow += &format!(" {:10.*} ,", 2, m[(row, col)]);
        }
        out.push_str(&outRow);
        out.push_str("\n");
    }
    out
}
