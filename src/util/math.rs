//! Module containing basic rust math functions.

use std::cmp::Ordering;

/// Get the mean of a vector of floats
pub fn mean(data: &[f32]) -> Option<f32> {
    let sum = data.iter().sum::<f32>();
    let count = data.len();
    match count {
        positive if positive > 0 => Some(sum / count as f32),
        _ => None,
    }
}

/// Get the sample variance of a vector of floats
pub fn sample_variance(data: &[f32]) -> Option<f32> {
    match (mean(&data), data.len()) {
        (Some(mean), count) if count > 0 => {
            let variance = data
                .iter()
                .map(|value| {
                    let diff = mean - (*value as f32);
                    diff * diff
                })
                .sum::<f32>()
                / (count - 1) as f32;
            Some(variance)
        }
        _ => None,
    }
}

/// Get the sample variance of a vector of floats
pub fn variance(data: &[f32]) -> Option<f32> {
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

/// Get the sample covariance between 2nd points
pub fn covariance(x: &[f32], y: &[f32]) -> Option<f32> {
    match (mean(x), mean(y), x.len() == y.len()) {
        // If the length of both vectors are equal and more than 0
        (Some(x_bar), Some(y_bar), true) => {
            let count = x.len();
            let covariance = x
                .iter()
                .zip(y)
                .map(|(x_i, y_i)| (*x_i - x_bar) * (*y_i - y_bar))
                .sum::<f32>()
                / (count - 1) as f32;
            Some(covariance)
        }
        _ => None,
    }
}

/// Transpose a list of nd points into n vectors with ||points|| entries
fn transpose(data: &[Vec<f32>]) -> Option<Vec<Vec<f32>>> {
    // If there are no points we can not create a covariance matrix
    if data.is_empty() {
        return None;
    };

    // Get the dimension count n. Unwrap is safe because of the 0 check earlier
    let n = data.first().unwrap().len();
    // If any of the points have a different dimensionality we can not compute the covariance matrix
    if data.iter().any(|point| point.len() != n) {
        return None;
    }
    // Transpose the points
    let accumulator: Vec<Vec<f32>> = (0..n).map(|_| Vec::<f32>::new()).collect();
    let transposed_data = data.iter().fold(accumulator, |mut acc, point| {
        for (acc_j, &p_j) in acc.iter_mut().zip(point) {
            acc_j.push(p_j)
        }
        acc
    });
    Some(transposed_data)
}

/// Calculate the eigenvalues given a series of nd points.
pub fn eigen_values_from_points(data: &[Vec<f32>]) -> Option<Vec<f32>> {
    match covariance_matrix(data) {
        Some(v) => match eigen_values(v) {
            Some((value, _)) => Some(value),
            None => None,
        },
        None => None,
    }
}

/// For a set of nD points calculate the NxN covariance matrix.
pub fn covariance_matrix(data: &[Vec<f32>]) -> Option<na::DMatrix<f32>> {
    // transpose the points
    let transposed_data = match transpose(&data) {
        Some(v) => v,
        None => return None,
    };

    // Find the dimensionality of the points
    let n = transposed_data.len();

    // Create the actual covariance matrix
    let mut data: Vec<f32> = Vec::<f32>::new();

    // Fill it with values one row at the time
    for i in 0..n {
        for j in 0..n {
            match match i.cmp(&j) {
                Ordering::Greater => Some(0.0),
                Ordering::Equal => sample_variance(&transposed_data[i]),
                Ordering::Less => covariance(&transposed_data[i], &transposed_data[j]),
            } {
                Some(v) => data.push(v),
                None => return None,
            }
        }
    }
    // Actually return the matrix
    let matrix = na::DMatrix::from_vec(n, n, data);
    Some(matrix)
}

/// Retrieve the eigen values from the covariance matrix using lapack ssyev.
/// It will return the eigen values asc with the vectors in the same order.
///
/// The routine computes all eigenvalues and, optionally, eigenvectors of an
/// n-by-n real symmetric matrix A. The eigenvector v(j) of A satisfies
///
/// A*v(j) = lambda(j)*v(j)
///
/// where lambda(j) is its eigenvalue. The computed eigenvectors are orthonormal.
/// https://software.intel.com/sites/products/documentation/doclib/mkl_sa/11/mkl_lapack_examples/ssyev_ex.c.htm
pub fn eigen_values(covariance_matrix: na::DMatrix<f32>) -> Option<(Vec<f32>, na::DMatrix<f32>)> {
    // If there are no points we can not create a covariance matrix
    if covariance_matrix.is_empty() {
        return None;
    };

    // TODO: there are often NaN as eigen values. Why does  this happen?
    let eig = na::linalg::SymmetricEigen::new(covariance_matrix);
    let eigen_values = eig.eigenvalues.data.as_vec().clone();
    let eigen_vectors = eig.eigenvectors;

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
    fn correct_sample_variance() {
        let x = vec![1.0f32, 2.0, 3.0, 5.0, -1.0];
        let expected = 5.0f32;
        let actual = sample_variance(&x).unwrap();
        assert_relative_eq!(actual, expected, epsilon = 1.0e-5);
    }

    #[test]
    fn correct_sample_covariance() {
        let a = vec![1.0f32, 3.0, -1.0];
        let b = vec![1.0f32, 0.0, -1.0];
        let expected = 1f32;
        let actual = covariance(&a, &b).unwrap();
        assert_relative_eq!(actual, expected, epsilon = 1.0e-5);
    }

    #[test]
    fn covariance_of_self_is_variance() {
        let x = vec![1.0f32, 3.0, -1.0];
        let expected = sample_variance(&x).unwrap();
        let actual = covariance(&x, &x).unwrap();
        assert_relative_eq!(actual, expected, epsilon = 1.0e-5);
    }

    #[test]
    fn correct_covariance_matrix() {
        let data = vec![vec![1.0f32, 1.0], vec![3.0f32, 0.0], vec![-1.0f32, -1.0]];
        let expected = na::DMatrix::from_vec(2, 2, vec![4f32, 1f32, 0f32, 1f32]);
        let actual = covariance_matrix(&data).unwrap();
        for x in 0..4 {
            assert_relative_eq!(actual.index(x), expected.index(x), epsilon = 1.0e-5);
        }
    }

    #[test]
    fn correct_eigen_values() {
        let input = na::DMatrix::from_vec(2, 2, vec![4f32, 1f32, 1f32, 1f32]);
        let expected_values = vec![4.3027754, 0.6972244];
        let (actual_values, _) = eigen_values(input).unwrap();
        for (e, a) in expected_values.iter().zip(actual_values) {
            assert_relative_eq!(*e, a, epsilon = 1.0e-6);
        }
    }

    #[test]
    fn correct_eigen_values_2() {
        // based on https://stackoverflow.com/questions/32327760/how-to-use-dsyev-routine-to-calculate-eigenvalues
        let input = na::DMatrix::from_vec(3, 3, vec![3.0, 2.0, 4.0, 2.0, 0.0, 2.0, 4.0, 2.0, 3.0]);
        let expected_values = vec![8.0, -1.0, -1.0];

        // Run the calculation
        let (actual_values, _) = eigen_values(input).unwrap();

        // Check the eigen values
        for (e, a) in expected_values.iter().zip(actual_values) {
            assert_relative_eq!(*e, a, epsilon = 1.0e-5);
        }
    }

    #[test]
    fn correct_eigen_values_3() {
        let input = na::DMatrix::from_vec(
            5,
            5,
            vec![
                1.96, 0.0, 0.0, 0.0, 0.0, -6.49, 3.80, 0.0, 0.0, 0.0, -0.47, -6.39, 4.17, 0.0, 0.0,
                -7.20, 1.50, -1.51, 5.70, 0.0, -0.65, -6.34, 2.67, 1.80, -7.10,
            ],
        );
        let expected_values = vec![1.96f32, 3.8, 4.17, 5.7, -7.1];
        let expected_vectors = na::DMatrix::<f32>::from_vec(
            5,
            5,
            vec![
                1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0,
            ],
        );

        // Run the calculation
        let (actual_values, actual_vectors) = eigen_values(input).unwrap();

        // Check the eigen values
        for (e, a) in expected_values.iter().zip(actual_values) {
            assert_relative_eq!(*e, a, epsilon = 1.0e-5);
        }

        // check the eigen vectors
        for x in 0..25 {
            assert_relative_eq!(
                actual_vectors.index(x),
                expected_vectors.index(x),
                epsilon = 1.0e-5
            );
        }
    }

    #[test]
    /// Last sanity check with real world values, this matches the result as computed by numpy
    fn correct_covariance_to_eigen() {
        let input = vec![
            vec![
                6.7f32, 0.13, 0.57, 6.6, 0.056, 60.0, 150.0, 0.99548, 2.96, 0.43, 9.4, 6.0,
            ],
            vec![
                6.9, 0.21, 0.81, 1.1, 0.137, 52.0, 123.0, 0.9932, 3.03, 0.39, 9.2, 6.0,
            ],
            vec![
                6.6, 0.25, 0.42, 11.3, 0.049, 77.0, 231.0, 0.9966, 3.24, 0.52, 9.5, 6.0,
            ],
            vec![
                6.4, 0.44, 0.44, 14.4, 0.048, 29.0, 228.0, 0.99955, 3.26, 0.54, 8.8, 7.0,
            ],
            vec![
                6.0, 0.22, 0.25, 11.1, 0.056, 112.0, 177.0, 0.9961, 3.08, 0.36, 9.4, 6.0,
            ],
            vec![
                6.4, 0.28, 0.43, 7.1, 0.045, 60.0, 221.0, 0.9952, 3.09, 0.45, 9.4, 6.0,
            ],
            vec![
                6.6, 0.27, 0.49, 7.8, 0.049, 62.0, 217.0, 0.9959, 3.17, 0.45, 9.4, 6.0,
            ],
            vec![
                6.3, 0.27, 0.46, 11.75, 0.037, 61.0, 212.0, 0.9971, 3.25, 0.53, 9.5, 6.0,
            ],
            vec![
                7.2, 0.29, 0.4, 13.6, 0.045, 66.0, 231.0, 0.9977, 3.08, 0.59, 9.6, 6.0,
            ],
            vec![
                6.4, 0.25, 0.74, 7.8, 0.045, 52.0, 209.0, 0.9956, 3.21, 0.42, 9.2, 6.0,
            ],
            vec![
                6.4, 0.25, 0.74, 7.8, 0.045, 52.0, 209.0, 0.9956, 3.21, 0.42, 9.2, 6.0,
            ],
            vec![
                7.2, 0.21, 0.34, 11.9, 0.043, 37.0, 213.0, 0.9962, 3.09, 0.5, 9.6, 6.0,
            ],
            vec![
                7.9, 0.26, 0.33, 10.3, 0.039, 73.0, 212.0, 0.9969, 2.93, 0.49, 9.5, 6.0,
            ],
            vec![
                6.2, 0.3, 0.49, 11.2, 0.058, 68.0, 215.0, 0.99656, 3.19, 0.6, 9.4, 6.0,
            ],
            vec![
                7.9, 0.26, 0.41, 15.15, 0.04, 38.0, 216.0, 0.9976, 2.96, 0.6, 10.0, 6.0,
            ],
            vec![
                7.3, 0.23, 0.41, 14.6, 0.048, 73.0, 223.0, 0.99863, 3.16, 0.71, 9.4, 6.0,
            ],
            vec![
                7.9, 0.26, 0.33, 10.3, 0.039, 73.0, 212.0, 0.9969, 2.93, 0.49, 9.5, 6.0,
            ],
            vec![
                7.3, 0.23, 0.41, 14.6, 0.048, 73.0, 223.0, 0.99863, 3.16, 0.71, 9.4, 6.0,
            ],
            vec![
                6.2, 0.25, 0.42, 8.0, 0.049, 53.0, 206.0, 0.99586, 3.16, 0.47, 9.1, 6.0,
            ],
            vec![
                6.5, 0.22, 0.72, 6.8, 0.042, 33.0, 168.0, 0.9958, 3.12, 0.36, 9.2, 6.0,
            ],
            vec![
                7.3, 0.22, 0.41, 15.4, 0.05, 55.0, 191.0, 1.0, 3.32, 0.59, 8.9, 6.0,
            ],
            vec![
                7.3, 0.22, 0.41, 15.4, 0.05, 55.0, 191.0, 1.0, 3.32, 0.59, 8.9, 6.0,
            ],
        ];
        let mut actual = eigen_values_from_points(&input).unwrap();
        actual.sort_by(|a, b| b.partial_cmp(&a).unwrap_or(std::cmp::Ordering::Equal));
        let expected = vec![
            751.11456,
            321.00198,
            8.82669,
            0.31167442,
            0.054736782,
            0.027621135,
            0.007874934,
            0.0027378371,
            0.0014962169,
            0.0005472675,
            0.000050775572,
            0.000000020481371,
        ];

        // Check the eigen values
        for (e, a) in expected.iter().zip(actual) {
            assert_relative_eq!(*e, a, epsilon = 1.0e-5);
        }
    }
}
