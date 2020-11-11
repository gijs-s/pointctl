//! A customer search method that returns the indexes of all the points within a certain
//! radius.

/// Build in party imports
use std::collections::HashSet;

/// Third party imports
use vpsearch::{BestCandidate, MetricSpace};

/// Add custom search for finding the index of multiple points in a radius
/// The index of all point with a euclidean distance strictly less than
/// `max_distance` will be returned.
pub struct RadiusBasedNeighborhood<Item: MetricSpace<Impl>, Impl> {
    max_distance: Item::Distance,
    ids: HashSet<usize>,
}

impl<Item: MetricSpace<Impl>, Impl> RadiusBasedNeighborhood<Item, Impl> {
    /// Helper function for creating the RadiusBasedNeighborhood struct.
    /// Here `max_distance` is an exclusive upper bound to the euclidean distance.
    #[allow(dead_code)]
    fn new(max_distance: Item::Distance) -> Self {
        RadiusBasedNeighborhood {
            max_distance,
            ids: HashSet::<usize>::new(),
        }
    }
}

/// Best candidate definitions that tracks of the index all the points
/// within the radius of `distance` as specified in the `RadiusBasedNeighborhood`.
impl<Item: MetricSpace<Impl> + Clone, Impl> BestCandidate<Item, Impl>
    for RadiusBasedNeighborhood<Item, Impl>
{
    type Output = HashSet<usize>;

    #[inline]
    fn consider(
        &mut self,
        _: &Item,
        distance: Item::Distance,
        candidate_index: usize,
        _: &Item::UserData,
    ) {
        // If the distance is lower than the bound we include the index
        // in the result.
        if distance < self.max_distance {
            self.ids.insert(candidate_index);
        }
    }

    #[inline]
    fn distance(&self) -> Item::Distance {
        self.max_distance
    }
    fn result(self, _: &Item::UserData) -> Self::Output {
        self.ids
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use vpsearch::Tree;

    #[derive(Clone, Debug)]
    struct PointN {
        data: Vec<f32>,
    }

    /// Point structure that will end up in the tree
    impl PointN {
        pub fn new(data: &Vec<f32>) -> Self {
            PointN { data: data.clone() }
        }
    }

    /// The searching function
    impl MetricSpace for PointN {
        type UserData = ();
        type Distance = f32;

        fn distance(&self, other: &Self, _: &Self::UserData) -> Self::Distance {
            self.data
                .iter()
                .zip(other.data.iter())
                .map(|(s, o)| (s - o).powi(2))
                .sum::<f32>()
                .sqrt()
        }
    }

    /// Get the tree definition used in testing
    fn get_test_tree() -> Tree<PointN> {
        let points = vec![
            PointN::new(&vec![2.0, 3.0]),
            PointN::new(&vec![0.0, 1.0]),
            PointN::new(&vec![4.0, 5.0]),
        ];
        Tree::new(&points)
    }

    #[test]
    /// Search with a distance of 0, expect no points to be returned
    fn find_vpsearch_radius_0() {
        let tree = get_test_tree();
        let expected = HashSet::new();
        let actual: HashSet<usize> = tree.find_nearest_custom(
            &PointN::new(&vec![1.0, 2.0]),
            &(),
            RadiusBasedNeighborhood::new(0.0f32),
        );
        assert_eq!(actual, expected);
    }

    #[test]
    /// Search with a distance of 100, expect all points to be returned
    fn find_vpsearch_radius_100() {
        let tree = get_test_tree();
        let expected = [0, 1, 2].iter().cloned().collect::<HashSet<usize>>();
        let actual: HashSet<usize> = tree.find_nearest_custom(
            &PointN::new(&vec![1.0, 2.0]),
            &(),
            RadiusBasedNeighborhood::new(100.0f32),
        );
        assert_eq!(actual, expected)
    }
}
