use std::collections::{HashSet};

#[derive(Debug, Copy, Clone)]
pub struct RowOperation {
    pub row_operand_idx1: usize,
    pub row_operand_idx2: usize,
    pub destination_row_idx: usize,
    pub factor1: f64,
    pub factor2: f64,
}

pub struct Planner {
    pub input_operations: Vec<RowOperation>,
    pub optimized_operations: Vec<Vec<RowOperation>>,
}

impl Planner {
    pub fn new() -> Planner {
        Planner {
            input_operations: vec![],
            optimized_operations: vec![],
        }
    }

    pub fn add_operation(&mut self, operation: RowOperation) {
        self.input_operations.push(operation);
    }

    pub fn optimize(&mut self) {
        // Here goes our optimization algorithm.
        let mut seen_destination_indices = HashSet::<usize>::new();
        let mut current_parallel_operations = vec![];
        for op in self.input_operations.iter() {
            if seen_destination_indices.contains(&op.destination_row_idx) {
                // End the current parallel operations and start a new block. Also clear seen_destination_indices.
                self.optimized_operations.push(current_parallel_operations.clone());
                seen_destination_indices.clear();
                current_parallel_operations = vec![];
            }
            seen_destination_indices.insert(op.destination_row_idx);
            current_parallel_operations.push(*op);
            // println!("{:#?}", self.optimized_operations);
        }
        self.optimized_operations.push(current_parallel_operations);
    }
}