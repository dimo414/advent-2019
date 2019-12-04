const PUZZLE_INPUT: (u32, u32) = (172930, 683082);

pub fn advent() {
    let (first, second) = find_passwords();
    println!("First Hints: {}", first);
    println!("Second Hint: {}", second);
}

fn find_passwords() -> (u32, u32) {
    let mut first = 0;
    let mut second = 0;
    for n in PUZZLE_INPUT.0..PUZZLE_INPUT.1 {
        if never_decrease(n) && two_adjacent(n) {
            first+=1;
            if exactly_two_adjacent(n) {
                second += 1;
            }
        }
    }
    (first, second)
}

fn never_decrease(n: u32) -> bool {
    let mut n = n;
    while n > 0 {
        if n % 10 < n / 10 % 10 {
            return false;
        }
        n /= 10;
    }
    true
}

fn two_adjacent(n: u32) -> bool {
    let mut n = n;
    while n > 0 {
        if n % 10 == n / 10 % 10 {
            break;
        }
        n /= 10;
    }
    n != 0 // loop ended early
}

fn exactly_two_adjacent(n: u32) -> bool {
    if n == 0 { return false; }
    let mut n = n;
    while n % 10 != n / 10 % 10 {
        n /= 10;
    }
    if n % 10 == n / 10 % 10 && n % 10 != n / 100 % 10 {
        return true;
    }
    let d = n % 10;
    while n % 10 == d && n > 0 {
        n /= 10;
    }
    // in case a subsequent run is exactly two
    exactly_two_adjacent(n)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn puzzle_input() {
        assert_eq!(find_passwords(), (1675, 1142));
    }

    parameterized_test! { facts, (n, nd, ta, eta), {
        assert_eq!(never_decrease(n), nd);
        assert_eq!(two_adjacent(n), ta);
        assert_eq!(exactly_two_adjacent(n), eta);
    }}
    facts! {
        a: (122345, true, true, true),
        b: (111123, true, true, false),
        c: (135679, true, false, false),
        d: (111111, true, true, false),
        e: (223450, false, true, true),
        f: (123789, true, false, false),

        g: (112233, true, true, true),
        h: (123444, true, true, false),
        i: (111122, true, true, true),
    }
}