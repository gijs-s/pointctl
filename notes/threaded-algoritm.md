# Small idea for threaded algorithm

The explanation mechanism already requires copies of the original data, it might be interesting to run the algorithm on a separate thread. This could be really nice when running the explanation algorithms from the frontend.

``` rust
enum Status<T> {
    // Algorithm is currently not running
    NotRunning,
    // The algorithm is currently running and between 0..1 done.
    Working(f32)
    // The algorithm is done and yields a result
    Done(T),
}

trait Background<AlgorithmParams, ResultType> {
    Arc<t>

    /// Start a algorithm and return if we can start
    fn start(params: AlogrithmParams) -> bool;

    /// Return if the algorithm is done running
    fn done() -> bool;

    /// Return if the algorithm is running
    fn is_running() -> bool;

    /// Return the progress of the algorithm if it is running. When done it will be 1.0
    fn progress() -> Option<f32>;

    /// Retrieve the result if it is available. When consumed the status will be reset to not running.
    fn result() -> Option<ResultType>;

    // Get the raw status
    fn status() -> Status<ResultType>;
}
```