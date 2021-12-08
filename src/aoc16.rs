pub fn advent() {
    let input = read_data();
    let sequence = to_vec(input.trim());

    println!("FFT Test: {}", to_str(&fft(&sequence)[..8]));

    let offset: usize = input[..7].parse().expect("Invalid");
    println!("FFT Run:  {}", to_str(&fast_fft(&repeat(&sequence), offset)));
}

fn read_data() -> String {
    std::fs::read_to_string("data/day16.txt").expect("Cannot open")
}

fn to_vec(s: &str) -> Vec<u32> {
    s.chars().map(|c| c.to_digit(10).expect("Invalid")).collect()
}

fn to_str(v: &[u32]) -> String { v.iter().map(|n| n.to_string()).collect() }

// TODO this is a bit inefficient, since we'll then skip more than half the vec
fn repeat(sequence: &[u32]) -> Vec<u32> {
    sequence.iter().cloned().cycle().take(sequence.len()*10000).collect()
}

fn apply_pattern(input: &[u32], offset: usize, pattern: usize) -> u32 {
    let mut sum = 0;
    for (i, &item) in input.iter().enumerate() {
        sum += item as i32 * pattern_at_index(i+offset, pattern);
    }
    sum.abs() as u32 % 10
}

const BASE_PATTERN: [i32; 4] = [0, 1, 0, -1];
fn pattern_at_index(index: usize, pattern: usize) -> i32 {
    let index = index + 1; // offset by 1
    let repeat = pattern + 1; // 1-indexed
    BASE_PATTERN[(index / repeat) % BASE_PATTERN.len()]
}

fn phase(input: &[u32], offset: usize) -> Vec<u32> {
    let mut ret = Vec::new();
    for i in 0..input.len() {
        ret.push(apply_pattern(input, offset, i));
    }
    ret
}

fn fft(sequence: &[u32]) -> Vec<u32> {
    let mut result = sequence.to_vec();
    for _ in 0..100 {
        result = phase(&result, 0);
    }
    result
}

fn sum_digits(input: &[u32]) -> Vec<u32> {
    let mut ret = Vec::new();
    let mut sum = 0;
    for n in input.iter().rev() {
        sum = (sum + *n) % 10;
        ret.push(sum);
    }
    ret.reverse();
    ret
}

// This works as long as offset is more than halfway down the list, since this essentially makes
// the pattern all 1's, and each digit is defined only by later digits, not earlier ones.
fn fast_fft(sequence: &[u32], offset: usize) -> Vec<u32> {
    assert!(offset > sequence.len() / 2);
    let mut result: Vec<_> = sequence.iter().cloned().skip(offset).collect();
    for _ in 0..100 {
        result = sum_digits(&result);
        //result = phase(&result, index);
    }
    result.into_iter().take(8).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pattern_for(index: usize) -> Vec<i32> {
        (0..4*(index+1)).map(|i| pattern_at_index(i, index)).collect()
    }

    parameterized_test::create!{ patterns, (n, expected), {
        assert_eq!(pattern_for(n), expected);
    }}
    patterns! {
        p0: (0, vec!(1, 0, -1, 0)),
        p1: (1, vec!(0, 1, 1, 0, 0, -1, -1, 0)),
        p2: (2, vec!(0, 0, 1, 1, 1, 0, 0, 0, -1, -1, -1, 0)),
    }

    #[test]
    fn example1() {
        let input = vec!(1,2,3,4,5,6,7,8);
        let p1 = phase(&input, 0);
        let p2 = phase(&p1, 0);
        let p3 = phase(&p2, 0);
        let p4 = phase(&p3, 0);
        assert_eq!(p1, vec!(4,8,2,2,6,1,5,8));
        assert_eq!(p2, vec!(3,4,0,4,0,4,3,8));
        assert_eq!(p3, vec!(0,3,4,1,5,5,1,8));
        assert_eq!(p4, vec!(0,1,0,2,9,4,9,8));
    }

    parameterized_test::create!{ phases, (input, expected), {
        let state = to_vec(input);
        let expected = to_vec(expected);
        assert_eq!(fft(&state)[..8], expected[..]);
    }}
    phases! {
        a: ("80871224585914546619083218645595", "24176176"),
        b: ("19617804207202209144916044189917", "73745418"),
        c: ("69317163492948606335995924319873", "52432133"),
    }

    parameterized_test::create!{ fast_phases, (input, offset, expected), {
        let input = to_vec(input);
        let expected = to_vec(expected);
        assert_eq!(fast_fft(&repeat(&input), offset), expected);
    }}
    fast_phases! {
        a: ("03036732577212944063491565474664", 303673, "84462026"),
        b: ("02935109699940807407585447034323", 293510, "78725270"),
        c: ("03081770884921959731165446850517", 308177, "53553731"),
    }
}