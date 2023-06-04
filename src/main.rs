use the_phoenicians_are_coming::PhoenicianTrader;

#[tokio::main]
async fn main() {
    let files = std::fs::read_dir("cases").unwrap();

    let mut threads = Vec::new();

    for file in files {
        threads.push(tokio::spawn(async {
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
        }));
    }

    for thread in threads {
        thread.await.unwrap();
    }
}
