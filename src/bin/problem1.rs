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

    println!("Suffix Array:");
    for (i, &index) in sa.sa.iter().enumerate() {
        let end = if index + 20 < sa.input.len() {
            index + 20
        } else {
            sa.input.len()
        };

        let suffix = String::from_utf8_lossy(&sa.input[index..end]);
        println!("{}: {}", i, suffix);
    }

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

    println!("{}", String::from_utf8_lossy(&sa.input));

    for search in searches {
        let timestamp = std::time::Instant::now();
        let result = sa.search(search);
        let elapsed = timestamp.elapsed().as_micros();
        if let Some(res) = result {
            let s = String::from_utf8_lossy(&sa.input[res..res + 50]);
            println!("{}us\tSearch {} -> \"{}\"", elapsed, search, s);
        } else {
            println!("{}us\tSearch {} -> Not found", elapsed, search);
        }
    }
}
