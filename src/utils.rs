use nalgebra::{DMatrix, SVD};

pub fn pseudoinverse(matrix: &DMatrix<f64>) -> DMatrix<f64> {
    let svd = SVD::new(matrix.clone(), true, true);
    let (u, singular_values, v_t) = (svd.u.unwrap(), svd.singular_values, svd.v_t.unwrap());

    let tolerance = 1e-9;
    let sigma_inv = DMatrix::from_diagonal(&singular_values.map(|x| if x > tolerance { 1.0 / x } else { 0.0 }));

    // V * Sigma^+ * U^T
    &v_t.transpose() * sigma_inv * u.transpose()
}
