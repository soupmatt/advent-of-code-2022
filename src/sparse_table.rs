use std::cmp::Ordering::*;
use std::{collections::BTreeMap, fmt::Display};

pub struct SparseTable<T>
where
    T: Clone,
{
    data: BTreeMap<(usize, usize), T>,
    default: T,
    row_min: usize,
    row_max: usize,
    col_min: usize,
    col_max: usize,
}

impl<T: Clone> SparseTable<T> {
    pub fn new(default: T) -> Self {
        Self::new_with_start_point(default, (0, 0))
    }

    pub fn new_with_start_point(default: T, (row, col): (usize, usize)) -> Self {
        let data = BTreeMap::new();
        SparseTable {
            data,
            default,
            row_min: row,
            row_max: row,
            col_min: col,
            col_max: col,
        }
    }

    pub fn insert(&mut self, (row, col): (usize, usize), value: T) -> Option<T> {
        self.row_max = row.max(self.row_max);
        self.col_min = col.min(self.col_min);
        self.col_max = col.max(self.col_max);
        self.data.insert((row, col), value)
    }

    pub fn get(&self, (row, col): (usize, usize)) -> &T {
        self.data.get(&(row, col)).unwrap_or(&self.default)
    }

    pub fn col_min(&self) -> &usize {
        &self.col_min
    }

    pub fn col_max(&self) -> &usize {
        &self.col_max
    }

    pub fn row_min(&self) -> &usize {
        &self.row_min
    }

    pub fn row_max(&self) -> &usize {
        &self.row_max
    }
}

impl<T: Clone + PartialEq> SparseTable<T> {
    pub fn from_vector_data(default: T, vec_data: Vec<Vec<T>>) -> Self {
        let mut table = SparseTable::new(default.clone());

        vec_data.iter().enumerate().for_each(|(r, row)| {
            row.iter().enumerate().for_each(|(c, v)| {
                if *v != default {
                    table.insert((r, c), v.clone());
                }
            })
        });

        table
    }
}

impl<T: Clone + Display> Display for SparseTable<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let row_num_width = self.row_max.to_string().len();
        let mut data_iter = self.data.iter();
        let mut cell = data_iter.next();
        for r in self.row_min..=self.row_max {
            write!(f, "{:row_num_width$} ", r)?;
            for c in self.col_min..=self.col_max {
                let res = match cell {
                    Some((k, v)) => match (r, c).cmp(k) {
                        Less => write!(f, "{}", self.default),
                        Equal => {
                            let result = write!(f, "{}", v);
                            cell = data_iter.next();
                            result
                        }
                        Greater => panic!("Something went wrong!"),
                    },
                    None => write!(f, "{}", self.default),
                };
                res?
            }
            writeln!(f)?
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    #[test]
    fn test_sparse_table() {
        let mut table = SparseTable::new(0);

        assert_eq!(*table.get((0, 0)), 0);
        assert_eq!(*table.get((27, 4399)), 0);

        table.insert((3, 5), 7);
        table.insert((0, 2), 2);
        table.insert((1, 3), 3);
        table.insert((4, 2), 9);

        assert_eq!(*table.get((3, 5)), 7);
        assert_eq!(*table.get((0, 2)), 2);
        assert_eq!(*table.get((1, 3)), 3);
        assert_eq!(*table.get((4, 2)), 9);

        assert_eq!(*table.get((0, 0)), 0);
        assert_eq!(*table.get((2, 1)), 0);

        assert_eq!(*table.get((3, 4)), 0);
        assert_eq!(table.insert((3, 4), 4), None);
        assert_eq!(*table.get((3, 4)), 4);
        assert_eq!(table.insert((3, 4), 6), Some(4));
        assert_eq!(*table.get((3, 4)), 6);
    }

    #[test]
    fn test_sparse_table_display() {
        let table = SparseTable::new(0);

        assert_eq!(
            format!("{}", table),
            indoc! { "
            0 0
            "
            }
        );
    }

    #[test]
    fn test_sparse_table_display_medium() {
        let mut table = SparseTable::new(0);

        table.insert((3, 5), 7);
        table.insert((0, 2), 2);
        table.insert((1, 3), 3);
        table.insert((4, 2), 9);
        table.insert((3, 4), 4);

        assert_eq!(
            format!("{}", table),
            indoc! { "
            0 002000
            1 000300
            2 000000
            3 000047
            4 009000
            "
            }
        );
    }

    #[test]
    fn test_sparse_table_display_large() {
        let mut table = SparseTable::new(0);

        table.insert((13, 5), 7);
        table.insert((0, 2), 2);
        table.insert((19, 17), 3);
        table.insert((8, 19), 9);
        table.insert((3, 4), 4);

        assert_eq!(
            format!("{}", table),
            indoc! { "
             0 00200000000000000000
             1 00000000000000000000
             2 00000000000000000000
             3 00004000000000000000
             4 00000000000000000000
             5 00000000000000000000
             6 00000000000000000000
             7 00000000000000000000
             8 00000000000000000009
             9 00000000000000000000
            10 00000000000000000000
            11 00000000000000000000
            12 00000000000000000000
            13 00000700000000000000
            14 00000000000000000000
            15 00000000000000000000
            16 00000000000000000000
            17 00000000000000000000
            18 00000000000000000000
            19 00000000000000000300
            "
            }
        );
    }
}
