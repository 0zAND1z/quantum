//! Gate library code (public for pedagogical reasons).

use ket::Ket;
use matrix::Matrix;

/// Represents a _quantum gate_: a quantum regster transformation.
///
/// This gate is tagged with a `width`, and contains a unitary matrix
/// representing the numerical transformation in the computational
/// basis.
///
/// This gate may be _applied_ to a ket to update the ket's state.
///
/// Currently we do not check whether the matrix is unitary.
///
/// See [Wikipedia](https://en.wikipedia.org/wiki/Quantum_computing#Operation)
/// for more information.
#[derive(Debug)]
pub struct Gate {
    width: usize,
    matrix: Matrix,
}

impl Gate {
    /// Construct a new quantum gate, given `width` and computational basis matrix.
    ///
    /// Currently we do not check whether the matrix is unitary.
    ///
    /// # Panics
    ///
    /// We panic if the supplied matrix is non-square or not of dimension `width`.
    pub fn new(width: usize, matrix: Matrix) -> Gate {
        assert_eq!(Ket::size(width), matrix.size());

        // TODO check that det(matrix) == 1

        Gate {
            width: width,
            matrix: matrix,
        }
    }

    /// Width of the gate.
    pub fn width(&self) -> usize {
        self.width
    }

    /// Representative matrix.
    pub fn matrix(&self) -> &Matrix {
        &self.matrix
    }
}
