pub mod planner;

use planner::{Planner, RowOperation};

struct Tensor {
    elements: Vec<f64>,
    dimensions: Vec<usize>,
    planner: Planner,
}

impl Tensor {
    fn new(elements: Vec<f64>, dimensions: Vec<usize>) -> Tensor {
        Tensor {
            elements,
            dimensions,
            planner: Planner::new(),
        }
    }

    fn get_row(&self, row_idx: usize) -> &[f64] {
        let start_idx = row_idx * &self.dimensions[1];
        let end_idx = start_idx + &self.dimensions[1];
        &self.elements[start_idx..end_idx]
    }

    fn lu_decomposition(&mut self) {
        for pivot_row_idx in 0..self.dimensions[0] {
            let pivot_row = &self.get_row(pivot_row_idx);
            let pivot = pivot_row[pivot_row_idx];
            for eliminate_row_idx in pivot_row_idx + 1..self.dimensions[0] {
                let eliminate_row = &self.get_row(eliminate_row_idx);
                self.planner.add_operation(RowOperation {
                    row_operand_idx1: pivot_row_idx,
                    row_operand_idx2: eliminate_row_idx,
                    destination_row_idx: eliminate_row_idx,
                    factor1: -1.0 * eliminate_row[pivot_row_idx] / pivot,
                    factor2: 1.0,
                });
            }
        }
        self.planner.optimize();
        println!("{:#?}", self.planner.optimized_operations);
    }
}

#[test]
fn scratchpad() {
    let mut t = Tensor::new(
        vec![
            1.0, -1.0, -1.0, 1.0, 0.0, -1.0, -2.0, 0.0, 2.0, 0.0, 2.0, 0.0, 3.0, -3.0, -2.0, 4.0,
        ],
        vec![4, 4],
    );
    t.lu_decomposition();
}
