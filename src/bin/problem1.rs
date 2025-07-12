use algorithmik4::NaiveSuffixArray;
use std::io::BufRead;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        println!("Usage: {} <input_file> <search_string>", args[0]);
        return;
    }
    let input_file = &args[1];
    let article_count: usize = args[2]
        .parse()
        .expect(&format!("Invalid article count: {}", args[2]));

    let mut input: Vec<u8> = Vec::new();
    let file = std::fs::File::open(input_file).expect(&format!("Could not open: {}", input_file));
    let mut reader = std::io::BufReader::new(file);
    for _ in 0..article_count {
        while reader
            .read_until(b'\n', &mut input)
            .expect("Failed to read line")
            > 1
        {}
    }

    input.push(0); // Append end marker

    let timestamp = std::time::Instant::now();
    let nsa = NaiveSuffixArray::new(&input);
    println!(
        "Naive suffix array created in {}ms",
        timestamp.elapsed().as_millis()
    );

    println!("Now you can search for substrings in the input file:");

    loop {
        println!("Please enter a search string:");
        let search_string = match std::io::stdin().lock().lines().next() {
            Some(Ok(line)) => line,
            _ => break, // Exit on EOF or error
        };

        if search_string.trim().is_empty() {
            continue;
        }

        let timestamp = std::time::Instant::now();
        let nresult = nsa.search(search_string.as_bytes());
        let nelapsed = timestamp.elapsed().as_micros();
        if !nresult.is_empty() {
            println!("{}us\tNaive Search found {} hits.", nelapsed, nresult.len());
            for (i, s) in &nresult {
                println!("i = {} -> {}\n", i, s);
            }
            println!("{}us\tNaive Search found {} hits.", nelapsed, nresult.len());
        } else {
            println!("{}us\tNaiveSearch found no match", nelapsed);
        }
    }
}

// fn main() {
//     let args: Vec<String> = std::env::args().collect();
//     if args.len() != 3 {
//         println!("Usage: {} <input_file> <search_string>", args[0]);
//         return;
//     }
//     let input_file = &args[1];
//     let article_count: usize = args[2].parse().expect("Invalid article count");
//
//     let mut input: Vec<u8> = Vec::new();
//     let file = std::fs::File::open(input_file).expect("Could not open input file");
//     let mut reader = std::io::BufReader::new(file);
//     for _ in 0..article_count {
//         while reader
//             .read_until(b'\n', &mut input)
//             .expect("Failed to read line")
//             > 1
//         {}
//     }
//
//     input.push(0); // Append end marker
//     // let timestamp = std::time::Instant::now();
//     // let sa = SuffixArray::new(&input);
//     // println!(
//     //     "Suffix array created in {}ms",
//     //     timestamp.elapsed().as_millis()
//     // );
//
//     let timestamp = std::time::Instant::now();
//     let nsa = NaiveSuffixArray::new(&input);
//     println!(
//         "Naive suffix array created in {}ms",
//         timestamp.elapsed().as_millis()
//     );
//
//     // println!("Suffix Array:");
//     // for (i, &index) in sa.sa.iter().enumerate() {
//     //     let end = if index + 20 < sa.input.len() {
//     //         index + 20
//     //     } else {
//     //         sa.input.len()
//     //     };
//     //
//     //     let suffix = String::from_utf8_lossy(&sa.input[index..end]);
//     //     println!("{}: {}", i, suffix);
//     // }
//     //
//     let searches = vec![
//         "wechselwirken",
//         "und",
//         "die",
//         // "der",
//         // "das",
//         // "in",
//         // "zu",
//         // "von",
//         // "mit",
//         // "auf",
//     ];
//
//     // println!("{}", String::from_utf8_lossy(&sa.input));
//
//     for search in searches {
//         // let timestamp = std::time::Instant::now();
//         // let result = sa.search(search);
//         // let elapsed = timestamp.elapsed().as_micros();
//         // if let Some(res) = result {
//         //     let s = String::from_utf8_lossy(&sa.input[res..res + 50]);
//         //     println!("{}us\tSearch {} -> \"{}\"", elapsed, search, s);
//         // } else {
//         //     println!("{}us\tSearch {} -> Not found", elapsed, search);
//         // }
//
//         let timestamp = std::time::Instant::now();
//         let nresult = nsa.search(search.as_bytes());
//         let nelapsed = timestamp.elapsed().as_micros();
//
//         if !nresult.is_empty() {
//             println!("{}us\tNaive Search found {} hits.", nelapsed, nresult.len());
//             for (i, s) in nresult {
//                 println!("{} -> {}\n", i, s);
//             }
//         } else {
//             println!("{}us\tNaiveSearch found no match", nelapsed);
//         }
//     }
// }
