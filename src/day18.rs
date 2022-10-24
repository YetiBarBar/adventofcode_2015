#[derive(Clone)]
struct Matrix<const N: usize> {
    values: [[bool; N]; N],
}

impl<const N: usize> Matrix<N> {
    fn new() -> Self {
        Self {
            values: [[false; N]; N],
        }
    }

    fn get_x_y(&self, x: isize, y: isize) -> bool {
        *self
            .values
            .get(usize::try_from(y).unwrap())
            .unwrap()
            .get(usize::try_from(x).unwrap())
            .unwrap()
    }

    /// Retrieve neighboorhood
    ///
    /// # Panics
    ///
    /// If cannot convert usize to isize
    fn get_neighboorhood(&self, x: usize, y: usize) -> Vec<bool> {
        let x = isize::try_from(x).unwrap();
        let y = isize::try_from(y).unwrap();
        let n: isize = isize::try_from(N).unwrap();
        [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ]
        .iter()
        .filter(|(dx, dy)| x + *dx >= 0 && y + *dy >= 0 && x + *dx < n && y + *dy < n)
        .map(|(dx, dy)| (x + dx, y + dy))
        .map(|(pos_x, pos_y)| self.get_x_y(pos_x, pos_y))
        .collect()
    }

    fn count_sharp_neighboors(&self, x: usize, y: usize) -> usize {
        let neighb = self.get_neighboorhood(x, y);
        let res = neighb.iter().filter(|v| **v).count();

        res
    }
}

fn main() {
    let input = include_str!("../data/day18.data")
        .lines()
        .map(|s| s.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut matrix = Matrix::<100>::new();

    for y in 0..100 {
        for x in 0..100 {
            matrix.values[y][x] = input.get(y).unwrap().get(x).unwrap() == &'#';
        }
    }

    // Cloning for step 2
    let mut matrix2 = matrix.clone();

    for _ in 0..100 {
        let mut new_matrix = [[false; 100]; 100];
        for (y, line) in matrix.values.iter().enumerate() {
            for (x, current) in line.iter().enumerate() {
                let ctr = matrix.count_sharp_neighboors(x, y);
                if *current {
                    new_matrix[y][x] = ctr == 2 || ctr == 3;
                } else {
                    new_matrix[y][x] = ctr == 3;
                }
            }
        }
        matrix.values = new_matrix;
    }

    println!(
        "Part 1: {}",
        matrix
            .values
            .iter()
            .flat_map(|v| v.iter())
            .filter(|item| **item)
            .count()
    );

    matrix2.values[0][0] = true;
    matrix2.values[0][99] = true;
    matrix2.values[99][0] = true;
    matrix2.values[99][99] = true;
    for _ in 0..100 {
        let mut new_matrix = [[false; 100]; 100];
        for (y, line) in matrix2.values.iter().enumerate() {
            for (x, current) in line.iter().enumerate() {
                let ctr = matrix2.count_sharp_neighboors(x, y);
                if *current {
                    new_matrix[y][x] = ctr == 2 || ctr == 3;
                } else {
                    new_matrix[y][x] = ctr == 3;
                }
            }
        }
        // We need to fix edges on!
        new_matrix[0][0] = true;
        new_matrix[0][99] = true;
        new_matrix[99][0] = true;
        new_matrix[99][99] = true;
        matrix2.values = new_matrix;
    }

    println!(
        "Part 2: {}",
        matrix2
            .values
            .iter()
            .flat_map(|v| v.iter())
            .filter(|item| **item)
            .count()
    );
}
