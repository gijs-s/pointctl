/// Module containing basic rust math utils, eigen values use lapack + openblas
extern crate lapack;
extern crate openblas_src;

/// Get the mean of a vector of floats
pub fn mean(data: &Vec<f32>) -> Option<f32> {
    let sum = data.iter().sum::<f32>();
    let count = data.len();
    match count {
        positive if positive > 0 => Some(sum / count as f32),
        _ => None,
    }
}

/// Get the variance of a vector of floats
pub fn variance(data: &Vec<f32>) -> Option<f32> {
    match (mean(&data), data.len()) {
        (Some(mean), count) if count > 0 => {
            let variance = data
                .iter()
                .map(|value| {
                    let diff = mean - (*value as f32);
                    diff * diff
                })
                .sum::<f32>()
                / count as f32;
            Some(variance)
        }
        _ => None,
    }
}

/// Get the covariance between 2nd points
pub fn covariance(x: &Vec<f32>, y: &Vec<f32>) -> Option<f32> {
    match (mean(x), mean(y), x.len() == y.len()) {
        // If the length of both vectors are equal and more than 0
        (Some(x_bar), Some(y_bar), true) => {
            let count = x.len();
            let covariance = x
                .iter()
                .zip(y)
                .map(|(x_i, y_i)| (*x_i - x_bar) * (*y_i - y_bar))
                .sum::<f32>()
                / count as f32;
            Some(covariance)
        }
        _ => None,
    }
}

/// Transpose a list of nd points into n vectors with ||points|| entries
fn transpose(data: &Vec<Vec<f32>>) -> Option<Vec<Vec<f32>>> {
    // If there are no points we can not create a covariance matrix
    if data.len() == 0 {
        return None;
    };

    // Get the dimension count n. Unwrap is safe because of the 0 check earlier
    let n = data.first().unwrap().len();
    // If any of the points have a different dimensionality we can not compute the covariance matrix
    if data.iter().any(|point| point.len() != n) {
        return None;
    }
    // Transpose the points
    let accumulator: Vec<Vec<f32>> = (0..n).into_iter().map(|_| Vec::<f32>::new()).collect();
    let transposed_data = data.iter().fold(accumulator, |mut acc, point| {
        for (acc_j, &p_j) in acc.iter_mut().zip(point) {
            acc_j.push(p_j)
        }
        acc
    });
    Some(transposed_data)
}

/// Calculate the eigenvalues given a series of nd points.
pub fn eigen_values_from_points(data: &Vec<Vec<f32>>) -> Option<Vec<f32>> {
    match covariance_matrix(&data) {
        Some(v) => match eigen_values(&v) {
            Some((value, _)) => Some(value),
            None => None,
        },
        None => None,
    }
}

/// For a set of nD points calculate the NxN covariance matrix.
fn covariance_matrix(data: &Vec<Vec<f32>>) -> Option<Vec<Vec<f32>>> {
    // transpose the points
    let transposed_data = match transpose(&data) {
        Some(v) => v,
        None => return None,
    };

    // Find the dimensionality of the points
    let n = transposed_data.len();

    // Create the actual covariance matrix
    let mut matrix: Vec<Vec<f32>> = Vec::<Vec<f32>>::new();

    // Fill it with values one row at the time
    for i in 0..n {
        let mut row = Vec::<f32>::new();
        for j in 0..n {
            match match i == j {
                true => variance(&transposed_data[i]),
                false => covariance(&transposed_data[i], &transposed_data[j]),
            } {
                Some(v) => row.push(v),
                None => return None,
            }
        }
        matrix.push(row);
    }
    // Actually return the matrix
    Some(matrix)
}

/// For a set of nD point calculate the variance within each dimension.
pub fn variance_per_dimension(data: &Vec<Vec<f32>>) -> Option<Vec<f32>> {
    // transpose the points
    let transposed_data = match transpose(&data) {
        Some(v) => v,
        None => return None,
    };

    // find the variance per dimension
    transposed_data.iter().map(|dim| variance(dim)).collect()
}

/// Retrieve the eigen values from the covariance matrix using lapack dsyev.
/// It will return the eigen values asc with the vectors in the same order.
///
/// The routine computes all eigenvalues and, optionally, eigenvectors of an
/// n-by-n real symmetric matrix A. The eigenvector v(j) of A satisfies
///
/// A*v(j) = lambda(j)*v(j)
///
/// where lambda(j) is its eigenvalue. The computed eigenvectors are orthonormal.
/// https://software.intel.com/sites/products/documentation/doclib/mkl_sa/11/mkl_lapack_examples/dsyev_ex.c.htm
fn eigen_values(covariance_matrix: &Vec<Vec<f32>>) -> Option<(Vec<f32>, Vec<Vec<f32>>)> {
    // If there are no points we can not create a covariance matrix
    if covariance_matrix.len() == 0 {
        return None;
    };

    // Get the dimension count n. Unwrap is safe because of the 0 check earlier
    let n = covariance_matrix.first().unwrap().len();
    // If any of the points have a different dimensionality we can not compute the covariance matrix
    if covariance_matrix.iter().any(|row| row.len() != n) {
        return None;
    }

    let mut a = Vec::<f64>::new();
    for (i, row) in covariance_matrix.iter().enumerate() {
        for (j, entry) in row.iter().enumerate() {
            match j > i {
                true => a.push(0.0f64),
                false => a.push(*entry as f64),
            }
        }
    }

    let mut w = vec![0.0; n];
    let mut work = vec![0.0; 4 * n];
    let lwork = 4 * n as i32;
    let mut info = 0;

    // Numpy uses the slower more general ?geev versions, we do not need this because the covariance matrix
    // is symmetric and this is _much_ faster.
    unsafe {
        lapack::dsyev(
            b'V', b'U', n as i32, &mut a, n as i32, &mut w, &mut work, lwork, &mut info,
        );
    }

    if info != 0 {
        eprintln!("FFI call to lapack failed with code {:?}", info);
        return None;
    }

    let eigen_values: Vec<f32> = w.into_iter().map(|v| v as f32).collect();
    let mut eigen_vectors: Vec<Vec<f32>> = vec![Vec::<f32>::new(); n];
    for (index, x) in a.into_iter().enumerate() {
        eigen_vectors[index % n].push(x as f32);
    }

    Some((eigen_values, eigen_vectors))
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn correct_mean() {
        // Mean of 1,2,3 is 2.
        let input = vec![1.0f32, 2.0f32, 3.0f32];
        let expected = 2.0f32;
        let actual = mean(&input).unwrap();
        assert_relative_eq!(actual, expected, epsilon = 1.0e-5);

        // Empty list has no mean
        let input = Vec::<f32>::new();
        let expected = None;
        let actual = mean(&input);
        assert_eq!(actual, expected)
    }

    #[test]
    fn correct_variance() {
        let x = vec![1.0f32, 2.0, 3.0, 5.0, -1.0];
        let expected = 4.0f32;
        let actual = variance(&x).unwrap();
        assert_relative_eq!(actual, expected, epsilon = 1.0e-5);
    }

    #[test]
    fn correct_covariance() {
        let a = vec![1.0f32, 3.0, -1.0];
        let b = vec![1.0f32, 0.0, -1.0];
        let expected = 2.0f32 / 3.0f32;
        let actual = covariance(&a, &b).unwrap();
        assert_relative_eq!(actual, expected, epsilon = 1.0e-5);
    }

    #[test]
    fn covariance_of_self_is_variance() {
        let x = vec![1.0f32, 3.0, -1.0];
        let expected = variance(&x).unwrap();
        let actual = covariance(&x, &x).unwrap();
        assert_relative_eq!(actual, expected, epsilon = 1.0e-5);
    }

    #[test]
    fn correct_covariance_matrix() {
        let data = vec![vec![1.0f32, 1.0], vec![3.0f32, 0.0], vec![-1.0f32, -1.0]];
        let expected = vec![
            vec![8f32 / 3f32, 2f32 / 3f32],
            vec![2f32 / 3f32, 2f32 / 3f32],
        ];
        let actual = covariance_matrix(&data).unwrap();
        for x in 0..2 {
            for y in 0..2 {
                assert_relative_eq!(actual[x][y], expected[x][y], epsilon = 1.0e-5);
            }
        }
    }

    #[test]
    fn correct_eigen_values() {
        let input = vec![
            vec![8f32 / 3f32, 2f32 / 3f32],
            vec![2f32 / 3f32, 2f32 / 3f32],
        ];
        let expected_values = vec![0.464816255364607f32, 2.868517177309801];
        let (actual_values, _) = eigen_values(&input).unwrap();
        for (e, a) in expected_values.iter().zip(actual_values) {
            assert_relative_eq!(*e, a, epsilon = 1.0e-5);
        }
    }

    #[test]
    fn correct_eigen_values_2() {
        // based on https://stackoverflow.com/questions/32327760/how-to-use-dsyev-routine-to-calculate-eigenvalues
        let input = vec![
            vec![3.0, 2.0, 4.0],
            vec![2.0, 0.0, 2.0],
            vec![4.0, 2.0, 3.0]
        ];
        let expected_values = vec![-1.0, -1.0,  8.0];

        // Run the calculation
        let (actual_values, _) = eigen_values(&input).unwrap();

        // Check the eigen values
        for (e, a) in expected_values.iter().zip(actual_values) {
            assert_relative_eq!(*e, a, epsilon = 1.0e-5);
        }
    }

    #[test]
    fn correct_eigen_values_3() {
        let input = vec![
            vec![1.96, 0.0, 0.0, 0.0, 0.0],
            vec![-6.49, 3.80, 0.0, 0.0, 0.0],
            vec![-0.47, -6.39, 4.17, 0.0, 0.0],
            vec![-7.20, 1.50, -1.51, 5.70, 0.0],
            vec![-0.65, -6.34, 2.67, 1.80, -7.10],
        ];
        let expected_values = vec![
            -11.065575232626278f32,
            -6.228746693721885,
            0.8640280302358604,
            8.865457026577943,
            16.094836840924128,
        ];
        let expected_vectors = vec![
            vec![-0.298067, -0.607513, 0.4026200, -0.374481, 0.489637],
            vec![-0.507798, -0.287968, -0.4065856, -0.357169, -0.605255],
            vec![-0.081606, -0.384320, -0.659966, 0.500763, 0.399148],
            vec![-0.003589, -0.446730, 0.455290, 0.620365, -0.456374],
            vec![-0.804130, 0.448032, 0.172458, 0.310768, 0.162247],
        ];

        // Run the calculation
        let (actual_values, actual_vectors) = eigen_values(&input).unwrap();

        // Check the eigen values
        for (e, a) in expected_values.iter().zip(actual_values) {
            assert_relative_eq!(*e, a, epsilon = 1.0e-5);
        }
        // check the eigen vectors
        for x in 0..5 {
            for y in 0..5 {
                assert_relative_eq!(
                    actual_vectors[x][y],
                    expected_vectors[x][y],
                    epsilon = 1.0e-5
                );
            }
        }
    }

    #[test]
    fn correct_eigen_values_4() {
        let input = vec![
            vec![1.96, -6.49, -0.47, -7.20, -0.65],
            vec![-6.49, 3.80, -6.39, 1.50, -6.34],
            vec![-0.47, -6.39, 4.17, 1.51, 2.67],
            vec![-7.20, 1.50, -1.51, 5.70, 1.80],
            vec![-0.65, -6.34, 2.67, 1.80, -7.10],
        ];
        let expected_values = vec![
            -11.065575232626278f32,
            -6.228746693721885,
            0.8640280302358604,
            8.865457026577943,
            16.094836840924128,
        ];
        let expected_vectors = vec![
            vec![-0.298067, -0.607513, 0.4026200, -0.374481, 0.489637],
            vec![-0.507798, -0.287968, -0.4065856, -0.357169, -0.605255],
            vec![-0.081606, -0.384320, -0.659966, 0.500763, 0.399148],
            vec![-0.003589, -0.446730, 0.455290, 0.620365, -0.456374],
            vec![-0.804130, 0.448032, 0.172458, 0.310768, 0.162247],
        ];

        // Run the calculation
        let (actual_values, actual_vectors) = eigen_values(&input).unwrap();

        // Check the eigen values
        for (e, a) in expected_values.iter().zip(actual_values) {
            assert_relative_eq!(*e, a, epsilon = 1.0e-5);
        }
        // check the eigen vectors
        for x in 0..5 {
            for y in 0..5 {
                assert_relative_eq!(
                    actual_vectors[x][y],
                    expected_vectors[x][y],
                    epsilon = 1.0e-5
                );
            }
        }
    }
}
