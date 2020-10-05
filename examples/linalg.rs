extern crate lapack;
extern crate openblas_src;

use lapack::*;

fn main() {
    let n = 3;
    let mut a = vec![3.0, 1.0, 1.0, 1.0, 3.0, 1.0, 1.0, 1.0, 3.0];
    let mut w = vec![0.0; n as usize];
    let mut work = vec![0.0; 4 * n as usize];
    let lwork = 4 * n;
    let mut info = 0;

    unsafe {
        dsyev(
            b'V', b'U', n, &mut a, n, &mut w, &mut work, lwork, &mut info,
        );
    }

    assert!(info == 0);
    for (one, another) in w.iter().zip(&[2.0, 2.0, 5.0]) {
        assert!((one - another).abs() < 1e-14);
    }
}
