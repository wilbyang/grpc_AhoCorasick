use aho_corasick::AhoCorasick;

fn main() {
    let patterns = &["apple", "maple", "Snapple"];
    let haystack = "Nobody likes maple in their apple flavored Snapple.";

    let ac = AhoCorasick::new(patterns);
    let mut matches = vec![];
    for mat in ac.find_iter(haystack) {
        matches.push((mat.pattern(), mat.start(), mat.end()));
    }
    print!("{:?}", matches);
}
