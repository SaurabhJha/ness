#[derive(Debug)]
pub struct RowOperation {
    pub row_operand_idx1: usize,
    pub row_operand_idx2: usize,
    pub destination_row_idx: usize,
    pub factor1: f64,
    pub factor2: f64,
}