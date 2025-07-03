use algorithmik4::SuffixArray;

fn main() {
    // let input = b"GATAGACA";
    // let searches = vec![
    //     "GATAGACA", "GATAGAC", "TAGACA", "\0", // End marker
    //     // not found
    //     "GATAGACX", "TAGACAGX",
    // ];

    let s = "Hello Ö World!";
    let input = s.as_bytes();
    let searches = vec![
        "Hell", "o Ö", "!", "\0", // End marker
        // not found
        "Hellow", "Woearld",
    ];

    let mut input: Vec<u8> = input.to_vec();

    input.push(0); // Append end marker

    let sa = SuffixArray::new(&input);

    dbg!(&sa.sa);

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
