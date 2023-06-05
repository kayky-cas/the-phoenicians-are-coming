use the_phoenicians_are_coming::PhoenicianTrader;

fn main() {
    let mut files: Vec<_> = std::fs::read_dir("cases").unwrap().collect();

    files.sort_by_key(|file| {
        file.as_ref()
            .unwrap()
            .path()
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_owned()
    });

    for file in files {
        let path = file.unwrap().path();
        let input = std::fs::read_to_string(&path).unwrap();

        let phoenicians: PhoenicianTrader = input.parse().unwrap();

        let start = std::time::Instant::now();

        println!(
            "{:?}: {} in {:?}",
            path,
            phoenicians.last().unwrap(),
            start.elapsed()
        )
    }
}
