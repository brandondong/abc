use std::{iter, time::Instant};

use permutohedron::heap_recursive;
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

// Parallelize the task by splitting into permutations of n-1 elements and then adding back the missing element later.
const DIGITS_MISSING_ELEMENT: [[usize; NUM_DIGITS - 1]; NUM_DIGITS] = {
    let mut a = [[0; NUM_DIGITS - 1]; NUM_DIGITS];
    let mut t = 0;
    while t < NUM_DIGITS {
        let mut i = 0;
        while i < NUM_DIGITS {
            if i < t {
                a[t][i] = i + 1;
            } else if i > t {
                a[t][i - 1] = i + 1;
            }
            i += 1;
        }
        t += 1;
    }
    a
};

fn main() {
    let now = Instant::now();
    let count_results = (0..NUM_DIGITS)
        .into_par_iter()
        .map(|t| {
            let mut count_results = 0;
            let mut digits = DIGITS_MISSING_ELEMENT[t];
            heap_recursive(&mut digits, |b| {
                let mut digits = DIGITS;
                heap_recursive(&mut digits, |c| {
                    // The heap_recursive inner loop constantly changes the first few elements of c. Reversing it helps with branch prediction???
                    // No idea why this formulation of b is fastest...
                    if all_digit_sum(b.iter().chain(iter::once(&(t + 1))), c.iter().rev()) {
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
