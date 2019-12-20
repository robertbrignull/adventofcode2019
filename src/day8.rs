fn get_input() -> Vec<u8> {
    return std::fs::read_to_string("src/day8_input").unwrap()
        .chars()
        .map(|x| x.to_digit(10).unwrap() as u8)
        .collect::<Vec<u8>>();
}

struct ImageLayer {
    pixels: Vec<Vec<u8>>,
}

struct Image {
    layers: Vec<ImageLayer>,
    width: usize,
    height: usize,
}

fn parse_images(width: usize, height: usize, data: Vec<u8>) -> Image {
    let pixels_per_layer = width * height;
    if data.len() % pixels_per_layer != 0 {
        println!("Number of pixels in data ({}) is not a multiple of number of pixels in a layer ({} = {} x {})",
                 data.len(), pixels_per_layer, width, height);
        std::process::exit(1);
    }
    let mut layers = Vec::new();
    for i in 0..(data.len() / pixels_per_layer) {
        let layer_start = pixels_per_layer * i;
        let layer_end = layer_start + pixels_per_layer;
        let layer_data = &data[layer_start..layer_end];

        let mut layer: Vec<Vec<u8>> = Vec::new();
        for y in 0..height {
            let row_start = width * y;
            let row_end = row_start + width;
            layer.push(layer_data[row_start..row_end].to_vec());
        }
        layers.push(ImageLayer {
            pixels: layer,
        });
    }
    return Image {
        layers,
        width,
        height,
    };
}

fn count_digit(image: &Image, layer: &ImageLayer, digit: u8) -> u32 {
    let mut count = 0;
    for y in 0..image.height {
        for x in 0..image.width {
            if layer.pixels[y][x] == digit {
                count += 1;
            }
        }
    }
    return count;
}

fn part1(input: &Image) {
    let best_layer = input.layers.iter()
        .min_by_key(|l| count_digit(input, l, 0))
        .unwrap();
    let result = count_digit(input, best_layer, 1) *
        count_digit(input, best_layer, 2);
    println!("part 1 result = {}", result);
}

fn render_pixel(image: &Image, x: usize, y: usize) -> char {
    for layer in &image.layers {
        let p = layer.pixels[y][x];
        if p == 1 {
            return '1';
        }
        if p == 0 {
            return ' ';
        }
    }
    println!("Could not determine pixel ({},{})", x, y);
    std::process::exit(1);
}

fn part2(input: &Image) {
    let mut pixels: Vec<String> = Vec::new();
    for y in 0..input.height {
        let mut row: Vec<char> = Vec::new();
        for x in 0..input.width {
            row.push(render_pixel(input, x, y));
        }
        pixels.push(row.iter().collect::<String>());
    }

    println!("part 2 result =");
    for row in pixels {
        println!("{}", row);
    }
}

pub fn run() {
    let input = parse_images(25, 6, get_input());
    part1(&input);
    part2(&input);
}
