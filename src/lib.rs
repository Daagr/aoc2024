pub fn input_file(example: bool) -> std::path::PathBuf {
    let exe = std::env::current_exe().unwrap();
    let project_dir = exe.ancestors().nth(2 /* 3? */).unwrap();
    let day = exe.file_stem().unwrap();
    let example = if example { "example" } else { "" };
    project_dir
        .join("data")
        .join(format!("{}{}.txt", day.to_string_lossy(), example))
}
