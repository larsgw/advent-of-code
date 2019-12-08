use std::fs;

fn get_layers (file: &str, size: &(usize, usize)) -> Vec<String> {
    file
        .chars()
        .collect::<Vec<char>>()
        .chunks(size.0 * size.1)
        .map(|chunk| chunk.iter().collect::<String>())
        .collect()
}

fn count_color (layer: &str, color: i32) -> i32 {
    let mut count = 0;
    for pixel in layer.chars() {
        if parse_digit(&pixel) == color {
            count += 1
        }
    }
    count
}

fn calculate_checksum (file: &str, size: &(usize, usize)) -> i32 {
    let layers = get_layers(file, size);
    let layer = layers.iter().min_by_key(|layer| count_color(layer, 0)).unwrap();

    count_color(layer, 1) * count_color(layer, 2)
}

fn decode_image (file: &str, size: &(usize, usize)) -> String {
    let mut image = vec![2; size.0 * size.1];
    let layers = get_layers(file, size);

    for layer in layers {
        for (i, pixel) in layer.chars().enumerate() {
            match image[i] {
                2 => image[i] = parse_digit(&pixel),
                0 | 1 => {},
                _ => panic!("unkown color")
            }
        }
    }

    image
        .chunks(size.0)
        .map(|chunk| chunk
            .iter()
            .map(|pixel| match pixel {
                0 => String::from("░"),
                1 => String::from("█"),
                2 => String::from("▒"),
                _ => panic!("unkown color")
            })
            .collect::<Vec<String>>()
            .join("")
        )
        .collect::<Vec<String>>()
        .join("\n")
}

fn parse_digit (char: &char) -> i32 {
    char.to_digit(10).expect("cannot parse char as digit") as i32
}

fn main () {
	let file = fs::read_to_string("./input.txt").expect("unable to download file");
	let data = file.trim();
    let layer_size = (25, 6);

	println!(
		"star 8-1: {}
star 8-2:
{}",
		calculate_checksum(&data, &layer_size),
        decode_image(&data, &layer_size)
	);
}
