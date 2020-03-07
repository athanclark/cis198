pub type Matrix = Vec<Vec<f32>>;

pub fn mat_mult(mat1: &Matrix, mat2: &Matrix) -> Matrix {
    let cols = mat2[0].len();
    let rows = mat1.len();
    assert_eq!(mat1[0].len(), mat2.len());
    let len = mat2.len();

    let mut acc: Matrix = vec![vec![0.;cols]; rows];

    for i in 0..rows {
        for j in 0..cols {
            let mut elem_acc: f32 = 0.;

            for k in 0..len {
                elem_acc += mat1[i][k] * mat2[k][j];
            }

            acc[i][j] = elem_acc;
        }
    }

    acc
}
