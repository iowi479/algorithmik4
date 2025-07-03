use rayon::prelude::*;

pub struct SuffixArray<'a> {
    pub input: &'a [u8],
    pub sa: Vec<usize>,
}

impl<'a> SuffixArray<'a> {
    pub fn new(input: &'a [u8]) -> Self {
        let input_length = input.len();
        let sa = Vec::with_capacity(input_length);
        let mut a = SuffixArray { input, sa };
        a.build();
        a
    }

    fn build(&mut self) {
        let mut timestamp = std::time::Instant::now();

        println!(
            "Building suffix array for input of length {}.",
            self.input.len()
        );

        let mut k = 1;
        let mut ra = Vec::with_capacity(self.input.len());
        let mut temp_ra = Vec::with_capacity(self.input.len());

        for (i, c) in self.input.iter().enumerate() {
            self.sa.push(i);
            ra.push(*c as usize);
            temp_ra.push(0);
        }

        self.sa.sort();

        let elapsed = timestamp.elapsed().as_millis();
        println!("Initial setup done. Time: {}ms", elapsed);

        while k < self.input.len() {
            timestamp = std::time::Instant::now();
            println!("Step with k = {} until n = {}. ", k, self.input.len());
            self.sa.par_sort_by(|&a, &b| match ra[a].cmp(&ra[b]) {
                std::cmp::Ordering::Equal => {
                    let a2 = if a + k < ra.len() { ra[a + k] } else { 0 };
                    let b2 = if b + k < ra.len() { ra[b + k] } else { 0 };
                    a2.cmp(&b2)
                }
                other => other,
            });

            let elapsed = timestamp.elapsed().as_millis();
            println!("Sorting done. Time: {}ms", elapsed);

            let mut counter = 0;
            temp_ra[self.sa[0]] = counter;

            for i in 1..self.input.len() {
                if ra[self.sa[i]] != ra[self.sa[i - 1]] {
                    counter += 1;
                } else {
                    if ra[self.sa[i] + k] != ra[self.sa[i - 1] + k] {
                        counter += 1;
                    }
                }

                temp_ra[self.sa[i]] = counter;
            }

            let elapsed = timestamp.elapsed().as_millis() - elapsed;
            println!("Temp ranks calculated. Time: {}ms", elapsed);

            std::mem::swap(&mut ra, &mut temp_ra);

            let elapsed = timestamp.elapsed().as_millis();
            println!("Step completed. Time: {}ms", elapsed);
            if ra[*self.sa.last().unwrap()] == self.input.len() - 1 {
                println!("All suffixes have unique ranks, stopping.");
                break;
            }

            k <<= 1;
        }
    }

    pub fn search(&self, pattern: &str) -> Option<usize> {
        match self.sa.binary_search_by(|i| {
            let suffix = &self.input[*i..];

            for (j, pc) in pattern.bytes().enumerate() {
                let sc = suffix.iter().nth(j);
                if sc.is_none() {
                    return std::cmp::Ordering::Less;
                }
                let sc = *sc.unwrap();
                if pc < sc {
                    return std::cmp::Ordering::Greater;
                } else if pc > sc {
                    return std::cmp::Ordering::Less;
                }
            }

            println!("Pattern '{}' found at index {}", pattern, i);
            std::cmp::Ordering::Equal
        }) {
            Ok(index) => Some(self.sa[index]),
            Err(_) => None,
        }
    }
}
