struct Matrix {
    elements: Vec<f64>,
    dimensions: (usize, usize),
}

impl Matrix {
    fn new(elements: Vec<f64>, dimensions: (usize, usize)) -> Matrix {
        Matrix {
            elements,
            dimensions,
        }
    }

    fn get_element(&self, row_idx: usize, col_idx: usize) -> f64 {
        let ncols = self.dimensions.1;
        self.elements[row_idx * ncols + col_idx]
    }

    fn get_pivot_row_idx(&self, col_idx: usize) -> usize {
        let mut column_elements = vec![];
        for row_idx in 0..self.dimensions.0 {
            column_elements.push(self.get_element(row_idx, col_idx));
        }

        let mut pivot_row_idx = 0;
        let mut max_element = column_elements[pivot_row_idx].abs();
        for col_idx in 1..self.dimensions.1 {
            if column_elements[col_idx].abs() < max_element && column_elements[col_idx] != 0.0 {
                pivot_row_idx = col_idx;
                max_element = column_elements[pivot_row_idx].abs();
            } else if max_element == 0.0 {
                pivot_row_idx = col_idx;
                max_element = column_elements[pivot_row_idx].abs();
            }
        }
        pivot_row_idx
    }

    fn get_row(&self, row_idx: usize) -> Vec<f64> {
        let mut row = vec![];
        for col_idx in 0..self.dimensions.1 {
            row.push(self.get_element(row_idx, col_idx));
        }
        row
    }

    fn construct_rank_one_lu(&self, row_idx: usize, col_idx: usize) -> RankOneMatrix {
        let row = self.get_row(row_idx);
        let mut col = vec![];
        let pivot = self.get_element(row_idx, col_idx);
        for r_idx in 0..self.dimensions.0 {
            col.push(self.get_element(r_idx, col_idx) / pivot);
        }
        RankOneMatrix { row, col }
    }

    fn subtract_with_rank_one_matrix(&self, rank_one_matrix: &RankOneMatrix) -> Matrix {
        let mut new_matrix_elements = vec![];
        for row_idx in 0..self.dimensions.0 {
            let main_matrix_row = self.get_row(row_idx);
            let rank_one_matrix_row = rank_one_matrix.get_row(row_idx);
            for col_idx in 0..self.dimensions.1 {
                new_matrix_elements.push(main_matrix_row[col_idx] - rank_one_matrix_row[col_idx]);
            }
        }
        Matrix {
            elements: new_matrix_elements,
            dimensions: (self.dimensions.0, self.dimensions.1),
        }
    }

    fn lu(&mut self) -> Vec<RankOneMatrix> {
        let mut result = vec![];
        for col_idx in 0..self.dimensions.1 {
            let row_idx = self.get_pivot_row_idx(col_idx);
            let rank_one_lu = self.construct_rank_one_lu(row_idx, col_idx);
            *self = self.subtract_with_rank_one_matrix(&rank_one_lu);
            result.push(rank_one_lu);
        }
        result
    }
}

#[derive(Debug)]
struct RankOneMatrix {
    row: Vec<f64>,
    col: Vec<f64>,
}

impl RankOneMatrix {
    fn new(row: Vec<f64>, col: Vec<f64>) -> RankOneMatrix {
        RankOneMatrix { row, col }
    }

    fn get_row(&self, row_idx: usize) -> Vec<f64> {
        let mut row = vec![];
        for col_idx in 0..self.row.len() {
            row.push(self.row[col_idx] * self.col[row_idx]);
        }
        row
    }
}

#[test]
fn scratchpad() {
    let mut t = Matrix::new(
        vec![
            1.0, -1.0, -1.0, 1.0,
            2.0, 0.0, 2.0, 0.0,
            0.0, -1.0, -2.0, 0.0,
            3.0, -3.0, -2.0, 4.0,
        ],
        (4, 4),
    );
    t.lu();
}
