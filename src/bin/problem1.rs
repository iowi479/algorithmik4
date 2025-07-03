use std::io::BufRead;

use algorithmik4::SuffixArray;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        println!("Usage: {} <input_file> <search_string>", args[0]);
        return;
    }
    let input_file = &args[1];
    let article_count: usize = args[2].parse().expect("Invalid article count");

    let mut input: Vec<u8> = Vec::new();
    let file = std::fs::File::open(input_file).expect("Could not open input file");
    let mut reader = std::io::BufReader::new(file);
    for _ in 0..article_count {
        while reader
            .read_until(b'\n', &mut input)
            .expect("Failed to read line")
            > 1
        {}
    }

    input.push(0); // Append end marker

    let sa = SuffixArray::new(&input);

    let searches = vec![
        "wechselwirken",
        "und",
        "die",
        "der",
        "das",
        "in",
        "zu",
        "von",
        "mit",
        "auf",
    ];

    for search in searches {
        let timestamp = std::time::Instant::now();
        let result = sa.search(search);
        let elapsed = timestamp.elapsed().as_micros();
        if let Some(res) = result {
            let s = String::from_utf8_lossy(&sa.input[res..]);
            println!("{}us\tSearch {} -> \"{}\"", elapsed, search, s);
        } else {
            println!("{}us\tSearch {} -> Not found", elapsed, search);
        }
    }
}
