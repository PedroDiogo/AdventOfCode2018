extern crate regex;
use regex::Regex;
use std::env;
use std::fs::File;
use std::io::prelude::*;

struct Rectangle {
    x: u32,
    y: u32,
    width: u32,
    height: u32,
}

fn main() {
    let file_name = get_input_file_name();
    let inputs = read_input_file(file_name);
    let inputs = inputs.lines();
    let rectangles = transform_inputs_to_rectangles(inputs);

    let overlapping_area = get_overlapping_area(rectangles);
    println!("Overlapping Area = {}", overlapping_area);
}

// Boilerplate code
fn get_input_file_name() -> String {
    let args: Vec<String> = env::args().collect();
    let filename;
    match args.len() {
        1 => filename = String::from("input"),
        _ => filename = args[1].clone(),
    }
    return filename;
}

fn read_input_file(file_name: String) -> String {
    let mut file = File::open(file_name).expect("file not found");
    let mut all_inputs = String::new();
    file.read_to_string(&mut all_inputs)
        .expect("something went wrong reading the file");

    return all_inputs;
}

// Problem code
fn transform_inputs_to_rectangles(inputs: std::str::Lines) -> Vec<Rectangle> {
    let mut rectangles = Vec::<Rectangle>::new();
    let re = Regex::new(r"@ *(?P<x>\d+),(?P<y>\d+): *(?P<width>\d+)x(?P<height>\d+)$").unwrap();

    for line in inputs {
        let captures = re.captures(line).unwrap();
        
        rectangles.push(Rectangle {
            x: captures["x"].parse().unwrap(),
            y: captures["y"].parse().unwrap(),
            width: captures["width"].parse().unwrap(),
            height: captures["height"].parse().unwrap(),
        });
    }
    return rectangles;
}

fn get_overlapping_area(rectangles: Vec<Rectangle>) -> u32 {
    const CLOTH_SIDE : usize = 1000;
    let mut cloth = Vec::with_capacity(CLOTH_SIDE);
    for x in 0..CLOTH_SIDE {
        cloth.push(Vec::<u32>::with_capacity(CLOTH_SIDE));
        for y in 0..CLOTH_SIDE {
            cloth[x].push(0);
        }
    } 

    for rectangle in rectangles {
        for x in rectangle.x..rectangle.x+rectangle.width {
            for y in rectangle.y..rectangle.y+rectangle.height {
                cloth[x as usize][y as usize] += 1;
            }
        }
    }

    let mut overlapping_area = 0;
    for x in 0..CLOTH_SIDE {
        for y in 0..CLOTH_SIDE {
           if cloth[x as usize][y as usize] > 1 {
               overlapping_area += 1;
           } 
        }
    }
    return overlapping_area;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_transform_inputs_to_rectangles() {
        let input = "#10 @ 364,640: 26x10";
        let rectangles = transform_inputs_to_rectangles(input.lines());
        let rectangle = &rectangles[0];

        assert_eq!(1, rectangles.len());
        assert_eq!(364, rectangle.x);
        assert_eq!(640, rectangle.y);
        assert_eq!(26, rectangle.width);
        assert_eq!(10, rectangle.height);
    } 

    #[test]
    fn test_get_overlapping_area() {
        let mut rectangles = Vec::new();

        rectangles.push(Rectangle{x: 1, y: 3, width: 4, height: 4});
        rectangles.push(Rectangle{x: 3, y: 1, width: 4, height: 4});
        rectangles.push(Rectangle{x: 5, y: 5, width: 2, height: 2});

        assert_eq!(4, get_overlapping_area(rectangles));
    }
}