pub mod fetcher;

fn main() {
    let mut fetcher = fetcher::Fetcher::new();
    let info = fetcher.fetch_json();

    println!("{}", info);
}
