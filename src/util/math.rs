/// Module containing basic pure rust math utils.

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

/// For a set of nD points calculate the NxN covariance matrix.
pub fn covariance_matrix(data: &Vec<Vec<f32>>) -> Option<Vec<Vec<f32>>> {
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
}
