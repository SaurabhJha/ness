pub mod planner;

use planner::{RowOperation, Planner};

struct Tensor {
    elements: Vec<f64>,
    dimensions: Vec<usize>,
}

impl Tensor
{
    fn new(elements: Vec<f64>, dimensions: Vec<usize>) -> Tensor {
        Tensor {
            elements,
            dimensions,
        }
    }

    fn get_row_slice(&self, row_idx: usize) -> &[f64] {
        let start_idx = row_idx * &self.dimensions[1];
        let end_idx = start_idx + &self.dimensions[1];
        &self.elements[start_idx..end_idx]
    }

    fn apply_gaussian_elimination(&mut self) {
        let mut planner = Planner::new();
        for pivot_row_idx in 0..self.dimensions[0] {
            let pivot_row = &self.get_row_slice(pivot_row_idx);
            let pivot = pivot_row[pivot_row_idx];
            for eliminate_row_idx in pivot_row_idx + 1..self.dimensions[0] {
                let eliminate_row = &self.get_row_slice(eliminate_row_idx);
                planner.add_operation(
                    RowOperation {
                        row_operand_idx1: pivot_row_idx,
                        row_operand_idx2: eliminate_row_idx,
                        destination_row_idx: eliminate_row_idx,
                        factor1: -1.0 * eliminate_row[pivot_row_idx] / pivot,
                        factor2: 1.0,
                    }
                );
            }
        }
        planner.optimize();
    }
}

#[test]
fn scratchpad() {
    let mut t = Tensor {
        elements: vec![1.0, -1.0, -1.0, 1.0, 0.0, -1.0, -2.0, 0.0, 2.0, 0.0, 2.0, 0.0, 3.0, -3.0, -2.0, 4.0],
        dimensions: vec![4, 4],
    };
    t.apply_gaussian_elimination();
}
