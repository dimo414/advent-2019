use std::fs;

const IMAGE_DIM: (usize, usize) = (25, 6);

pub fn advent() {
    let layers = read_data();

    println!("Checksum: {}", checksum(&layers));

    println!("Password:");
    println!("{}", render_image(&decode_image(&layers, IMAGE_DIM.0, IMAGE_DIM.1), 25));
}

fn read_data() -> Vec<String> {
    partition(fs::read_to_string("data/day8.txt").expect("Cannot open").trim(), IMAGE_DIM.0*IMAGE_DIM.1)
}

fn partition(input: &str, size: usize) -> Vec<String> {
    let mut ret = Vec::new();
    let mut cur = String::new();
    for c in input.chars() {
        if cur.len() >= size {
            ret.push(cur);
            cur = String::new();
        }
        cur.push(c);
    }
    if !cur.is_empty() {
        ret.push(cur);
    }
    ret
}

fn checksum(layers: &Vec<String>) -> usize {
    let min_zero_layer = layers.iter()
        .min_by_key(|l| l.chars().filter(|&c| c == '0').count())
        .expect("Invalid");
    let ones = min_zero_layer.chars().filter(|&c| c == '1').count();
    let twos = min_zero_layer.chars().filter(|&c| c == '2').count();
    ones*twos
}

fn decode_image(layers: &Vec<String>, width: usize, height: usize) -> String {
    // This could be a fold, e.g. `.fold("2".repeat(25*6), merge_layer)`, but it's done as a loop
    // here so the intermediate stages can be printed
    let mut image = "2".repeat(width*height);
    for layer in layers {
        if cfg!(debug_assertions) {
            println!("{}\u{001B}[{}A", render_image(&image, width), height);
            std::thread::sleep(std::time::Duration::from_millis(25));
        }
        image = merge_layer(image, &layer);
    }
    image
}

fn merge_layer(floor: String, next: &str) -> String {
    let mut ret = String::new();
    for pixel in floor.chars().zip(next.chars()) {
        let new_pixel = match pixel.0 {
            '0' | '1' => pixel.0,
            '2' => pixel.1,
            _ => panic!(format!("Unexpected: {:?}", pixel)),
        };
        ret.push(new_pixel);
    }
    ret
}

fn render_image(image: &str, width: usize) -> String {
    // Note that █ and ▒ are multi-bit characters, so we can't do any more naive string splitting
    // once they're introduced into the string.
    partition(&image, width).join("\n")
        .replace("0", " ").replace("1", "█").replace("2", "▒")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_file() {
        assert!(!read_data().is_empty());
    }

    #[test]
    fn decode() {
        let layers = vec!("0222".to_string(), "1122".into(), "2212".into(), "0000".into());
        assert_eq!(decode_image(&layers,2, 2), "0110");
    }
}