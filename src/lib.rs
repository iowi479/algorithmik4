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

        self.sa.extend(0..self.input.len());

        let mut k = 1;
        let mut ra = self
            .input
            .iter()
            .map(|&c| c as usize)
            .collect::<Vec<usize>>();
        let mut temp_ra = vec![0; self.input.len()];

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

pub struct NaiveSuffixArray<'a> {
    pub input: &'a [u8],
    pub sa: Vec<usize>,
}

impl<'a> NaiveSuffixArray<'a> {
    pub fn new(input: &'a [u8]) -> Self {
        let mut sa = Vec::with_capacity(input.len());
        for i in 0..input.len() {
            sa.push(i);
        }

        sa.par_sort_unstable_by(|&a, &b| {
            let suffix_a = &input[a..];
            let suffix_b = &input[b..];
            suffix_a.cmp(suffix_b)
        });

        NaiveSuffixArray { input, sa }
    }

    pub fn search(&self, pattern: &[u8]) -> Vec<(usize, &str)> {
        let index = self.sa.binary_search_by(|i| {
            let suffix = &self.input[*i..];
            compare_suffix(pattern, suffix)
        });

        if let Err(_) = index {
            return Vec::new();
        }

        let mut results = Vec::new();
        let mut start_index = match index {
            Ok(idx) => idx,
            Err(_) => unreachable!(),
        };
        let mut end_index = start_index;

        loop {
            if start_index > 0 {
                let previous_index = start_index - 1;
                match compare_suffix(pattern, &self.input[self.sa[previous_index]..]) {
                    std::cmp::Ordering::Equal => {
                        start_index = previous_index;
                        continue;
                    }
                    _ => {
                        break;
                    }
                }
            }

            break;
        }

        loop {
            if end_index < self.sa.len() - 1 {
                let next_index = end_index + 1;
                match compare_suffix(pattern, &self.input[self.sa[next_index]..]) {
                    std::cmp::Ordering::Equal => {
                        end_index = next_index;
                        continue;
                    }
                    _ => {
                        break;
                    }
                }
            }

            break;
        }

        for i in start_index..=end_index {
            let start = self.sa[i];
            let mut end = usize::min(self.sa[i] + 100, self.input.len());
            let mut suffix = &self.input[start..end];
            let result = loop {
                if let Ok(valid_str) = std::str::from_utf8(suffix) {
                    break valid_str;
                } else {
                    end += 1;
                    suffix = &self.input[start..end];
                }
            };
            results.push((start, result));
        }

        results
    }
}

fn compare_suffix(pattern: &[u8], suffix: &[u8]) -> std::cmp::Ordering {
    for (i, pc) in pattern.iter().enumerate() {
        let sc = suffix.iter().nth(i);
        if sc.is_none() {
            return std::cmp::Ordering::Less;
        }
        let sc = sc.unwrap();
        if *pc < *sc {
            return std::cmp::Ordering::Greater;
        } else if *pc > *sc {
            return std::cmp::Ordering::Less;
        }
    }

    std::cmp::Ordering::Equal
}
