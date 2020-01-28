extern crate adblock;

use std::env;
use std::fs::File;
use std::io::{self, BufReader, BufRead};
use adblock::utils::rules_from_lists;
use adblock::engine::Engine;

fn read_file(filename: &str) -> Vec<String> {
    let file = File::open(filename).unwrap_or_else(|_err| {
        panic!("Could not open file {}", filename)
    });
    let reader = BufReader::new(file);
    reader.lines().filter_map(io::Result::ok).collect()
}

fn main() {

    // rules are copied from adblock-rust/benches/bench_cosmetic_matching.rs
    let rules = rules_from_lists(&vec![
        "data/uBlockOrigin/filters.txt".to_owned(),
        "data/uBlockOrigin/unbreak.txt".to_owned(),
        "data/easylist.txt".to_owned(),
    ]);

    let blocker = Engine::from_rules_debug(&rules);

    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        panic!("Please provide a base url, default, and nopatch url files!");
    }

    let base_url = &args[1];
    let default_urls_file = &args[2];
    let nopatch_urls_file = &args[3];

    // Analyzing default page load URLs
    let default_urls = read_file(&default_urls_file);

    let mut default_matched_count = 0;
    for url in default_urls {
        let blocker_result = blocker.check_network_urls(&url, &base_url, "");
        if blocker_result.matched {
            default_matched_count += 1
        }
    }
    println!("# of matched urls in the default page: {:?}", default_matched_count);

    // Analyzing Fawkes static template URLs
    let nopatch_urls = read_file(&nopatch_urls_file);

    let mut nopatch_matched_count = 0;
    for url in nopatch_urls {
        let blocker_result = blocker.check_network_urls(&url, &base_url, "");
        if blocker_result.matched {
            nopatch_matched_count += 1
        }
    }
    println!("# of matched urls in the static template: {:?}", nopatch_matched_count);

}

