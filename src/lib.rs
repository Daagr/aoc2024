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
