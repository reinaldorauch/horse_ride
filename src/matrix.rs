#[derive(Debug)]
pub struct Matrix {
    pub buf: Vec<i8>,
    pub cols: usize,
    pub rows: usize,
}

impl Matrix {
    pub fn new(rows: usize, cols: usize) -> Self {
        Matrix {
            buf: vec![0; rows * cols],
            cols,
            rows,
        }
    }

    pub fn new_with_initial(rows: usize, cols: usize, initial: i8) -> Self {
        Matrix {
            buf: vec![initial; rows * cols],
            rows,
            cols,
        }
    }

    pub fn set(self: &mut Self, x: usize, y: usize, val: i8) {
        self.buf[x * self.rows + y] = val;
    }

    pub fn get(&self, x: usize, y: usize) -> i8 {
        self.buf[x * self.rows + y]
    }
}

impl std::fmt::Display for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out = String::from("");

        for r in 0..self.rows {
            let slice = &self.buf[(self.cols * r)..((r + 1) * self.cols)];
            let row = slice
                .into_iter()
                .map(|v| format!("{:3}", v))
                .fold(String::from(""), |acc, x| format!("{}{}", acc, x));
            out.push_str(&row.to_owned());
            out.push('\n');
        }

        write!(f, "{}", out)
    }
}

// pub fn mochila(capacity: usize, weights: &Vec<usize>, values: &Vec<usize>, total: usize) -> usize {
//     let mut results = Matrix::new(capacity + 1, total + 1);

//     for i in 1..(total + 1) {
//         for c in 1..(capacity + 1) {
//             let new_val = if weights[i - 1] <= c {
//                 let with_item = values[i - 1] + results.get(i - 1, c - weights[i - 1]);
//                 let without_item = results.get(i - 1, c);
//                 if with_item >= without_item {
//                     with_item
//                 } else {
//                     without_item
//                 }
//             } else {
//                 results.get(i - 1, c)
//             };

//             results.set(i, c, new_val);
//         }
//     }

//     results.get(total, capacity)
// }
