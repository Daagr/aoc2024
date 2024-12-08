pub fn input_file(example: bool) -> std::path::PathBuf {
    let exe = std::env::current_exe().unwrap();
    let project_dir = exe.ancestors().nth(2 /* 3? */).unwrap();
    let day = exe.file_stem().unwrap();
    let example = if example { "example" } else { "" };
    project_dir
        .join("data")
        .join(format!("{}{}.txt", day.to_string_lossy(), example))
}

pub fn concat_nums(a: u64, b: u64) -> u64 {
    let mut m = 10;
    while m <= b {
        m *= 10;
    }
    m * a + b
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_concat() {
        assert_eq!(concat_nums(1, 2), 12);
        assert_eq!(concat_nums(23, 45), 2345);
        assert_eq!(concat_nums(10, 2), 102);
        assert_eq!(concat_nums(100, 2), 1002);
        assert_eq!(concat_nums(1, 20), 120);
        assert_eq!(concat_nums(5, 200), 5200);
        assert_eq!(concat_nums(6, 10), 610);
    }
}

#[derive(Debug)]
pub struct Combs<'a, Elem> {
    len: usize,
    n: usize,
    elems: &'a [Elem],
}
impl<'a, Elem: Copy> Iterator for Combs<'a, Elem> {
    type Item = Vec<Elem>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.n >= self.elems.len().pow(self.len as u32) {
            return None;
        }
        let mut it = Vec::with_capacity(self.len);
        let mut n = self.n;
        for _ in 0..self.len {
            it.push(self.elems[n % self.elems.len()]);
            n /= self.elems.len();
        }
        self.n += 1;
        Some(it)
    }
}
impl<'a, Elem> Combs<'a, Elem> {
    pub fn new(len: usize, elems: &'a [Elem]) -> Self {
        Combs { len, n: 0, elems }
    }
}
#[cfg(test)]
mod tests_combs {
    use super::*;
    #[test]
    fn test_combs() {
        assert_eq!(
            Combs::new(1, &[1, 2]).collect::<Vec<_>>(),
            vec![vec![1], vec![2]]
        );
        assert_eq!(
            Combs::new(2, &[1, 2]).collect::<Vec<_>>(),
            vec![vec![1, 1], vec![2, 1], vec![1, 2], vec![2, 2]]
        );
        assert_eq!(
            Combs::new(1, &[1, 2, 3]).collect::<Vec<_>>(),
            vec![vec![1], vec![2], vec![3]]
        );
        assert_eq!(
            Combs::new(2, &[1, 2, 3]).collect::<Vec<_>>(),
            vec![
                vec![1, 1],
                vec![2, 1],
                vec![3, 1],
                vec![1, 2],
                vec![2, 2],
                vec![3, 2],
                vec![1, 3],
                vec![2, 3],
                vec![3, 3]
            ]
        );
    }
}
