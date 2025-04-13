mod utils;

use nalgebra::DMatrix;

fn main() {
    let data = vec![
        1.0, 2.0,
        3.0, 4.0,
        5.0, 6.0,
    ];
    let a = DMatrix::from_row_slice(3, 2, &data);

    let a_pinv = utils::pseudoinverse(&a);

    println!("Original Matrix:\n{}", a);
    println!("Pseudoinverse:\n{}", a_pinv);
}
