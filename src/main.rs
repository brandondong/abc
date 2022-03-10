use std::time::Instant;

use rayon::prelude::*;

const NUM_DIGITS: usize = 9;

const DIGITS: [usize; NUM_DIGITS] = {
    let mut a = [0; NUM_DIGITS];
    let mut i = 0;
    while i < NUM_DIGITS {
        a[i] = i + 1;
        i += 1;
    }
    a
};

// Parallelize the task by splitting into permutations of n-2 elements with the missing elements swapped to the end.
const DIGITS_SWAP_END: [[usize; NUM_DIGITS]; NUM_DIGITS * (NUM_DIGITS - 1)] = {
    let mut a = [[0; NUM_DIGITS]; NUM_DIGITS * (NUM_DIGITS - 1)];
    let mut skip1 = 0;
    let mut t = 0;
    while skip1 < NUM_DIGITS {
        let mut skip2 = 0;
        while skip2 < NUM_DIGITS {
            if skip1 != skip2 {
                let mut i = 0;
                let mut j = 0;
                while i < NUM_DIGITS {
                    if i != skip1 && i != skip2 {
                        a[t][j] = i + 1;
                        j += 1;
                    }
                    i += 1;
                }
                a[t][NUM_DIGITS - 2] = skip1 + 1;
                a[t][NUM_DIGITS - 1] = skip2 + 1;
                t += 1;
            }
            skip2 += 1;
        }
        skip1 += 1;
    }
    a
};

fn main() {
    let now = Instant::now();
    let count_results = DIGITS_SWAP_END
        .into_par_iter()
        .map(|mut digits_swap_end| {
            let mut count_results = 0;
            // Shuffle the first n-2 elements.
            heap_unrolled_(DIGITS.len() - 2, &mut digits_swap_end, &mut |b| {
                let mut digits = DIGITS;
                heap_unrolled_(DIGITS.len(), &mut digits, &mut |c| {
                    // The heap_recursive inner loop constantly changes the first few elements of c. Reversing it helps with branch prediction???
                    if all_digit_sum(b.iter(), c.iter().rev()) {
                        count_results += 1;
                    }
                });
            });
            count_results
        })
        .sum::<u32>();

    println!(
        "Finished: {} successful results after {} ms.",
        count_results,
        now.elapsed().as_millis()
    );
}

fn all_digit_sum<'a>(
    v1: impl Iterator<Item = &'a usize>,
    v2: impl Iterator<Item = &'a usize>,
) -> bool {
    // Mark digits outside the range as having been already encountered.
    const DIGITS_ENCOUNTERED: [bool; 10] = {
        let mut a = [false; 10];
        a[0] = true;
        let mut i = NUM_DIGITS + 1;
        while i < 10 {
            a[i] = true;
            i += 1;
        }
        a
    };
    let mut digits_encountered = DIGITS_ENCOUNTERED;
    let mut carry = 0;
    for (i1, i2) in v1.zip(v2) {
        let digit_sum = i1 + i2 + carry;
        let digit = digit_sum % 10;
        let encountered = digits_encountered.get_mut(digit).unwrap();
        if *encountered {
            return false;
        }
        *encountered = true;
        carry = digit_sum / 10;
    }
    carry == 0
}

// Extracted private helper function from permutohedron.
fn heap_unrolled_<T, F>(n: usize, xs: &mut [T], f: &mut F)
where
    F: FnMut(&mut [T]),
{
    debug_assert!(n >= 3);
    match n {
        3 => {
            // [1, 2, 3], [2, 1, 3], [3, 1, 2], [1, 3, 2], [2, 3, 1], [3, 2, 1]
            f(xs);
            xs.swap(0, 1);
            f(xs);
            xs.swap(0, 2);
            f(xs);
            xs.swap(0, 1);
            f(xs);
            xs.swap(0, 2);
            f(xs);
            xs.swap(0, 1);
            f(xs)
        }
        n => {
            for i in 0..n - 1 {
                heap_unrolled_(n - 1, xs, f);
                let j = if n % 2 == 0 { i } else { 0 };
                // One swap *between* each iteration.
                xs.swap(j, n - 1);
            }
            heap_unrolled_(n - 1, xs, f)
        }
    }
}
