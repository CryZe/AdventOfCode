extern crate image;

use image::RgbaImage;
use std::ops::{Index, IndexMut};
use std::path::{Path, PathBuf};
use std::io::prelude::*;
use std::fs::File;
use std::env::args;
use std::cmp::min;
use std::borrow::Cow::*;

#[derive(Clone)]
struct Lights {
    lights: Box<[bool]>,
}

impl Lights {
    fn new() -> Self {
        Lights { lights: Box::new([false; 100 * 100]) }
    }

    fn parse(input: &str) -> Self {
        let mut lights = Lights::new();

        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                lights[(x, y)] = c == '#';
            }
        }

        lights
    }

    fn get_next_frame(&self) -> Self {
        let mut new_frame = self.clone();

        for y in 0..100 {
            for x in 0..100 {
                new_frame[(x, y)] = self.get_new_light_status(x, y);
            }
        }

        new_frame
    }

    fn get_new_light_status(&self, x: usize, y: usize) -> bool {
        let mut neighbors = 0;

        for iy in y.saturating_sub(1)..min(100, y + 2) {
            for ix in x.saturating_sub(1)..min(100, x + 2) {
                if !(x == ix && y == iy) {
                    neighbors += if self[(ix, iy)] {
                        1
                    } else {
                        0
                    };
                }
            }
        }

        let old_status = self[(x, y)];

        if old_status {
            neighbors == 2 || neighbors == 3
        } else {
            neighbors == 3
        }
    }

    fn activate_corners(&mut self) {
        self[(0, 0)] = true;
        self[(99, 0)] = true;
        self[(0, 99)] = true;
        self[(99, 99)] = true;
    }

    fn write_to_image(&self) -> RgbaImage {
        let mut image = RgbaImage::new(100, 100);

        for (x, y, pixel) in image.enumerate_pixels_mut() {
            pixel.data = if self[(x as usize, y as usize)] {
                [0xFF, 0xFF, 0xFF, 0xFF]
            } else {
                [0x00, 0x00, 0x00, 0xFF]
            };
        }

        image
    }

    fn count_active_lights(&self) -> usize {
        self.lights.iter().cloned().fold(0, |a, i| {
            a +
            if i {
                1
            } else {
                0
            }
        })
    }
}

impl Index<(usize, usize)> for Lights {
    type Output = bool;

    fn index<'a>(&'a self, idx: (usize, usize)) -> &'a bool {
        let (x, y) = idx;
        &self.lights[x + 100 * y]
    }
}

impl IndexMut<(usize, usize)> for Lights {
    fn index_mut<'a>(&'a mut self, idx: (usize, usize)) -> &'a mut bool {
        let (x, y) = idx;
        &mut self.lights[x + 100 * y]
    }
}

fn read_file(path: &Path) -> String {
    let mut input = String::new();
    let mut file = File::open(path).expect("File could not be found.");
    file.read_to_string(&mut input).expect("File could not be read.");
    input
}

fn mutate_lights(initial: &Lights, n: usize, path: &Path, active_corners: bool) -> Lights {
    let mut path = PathBuf::from(path);

    let mut lights = Borrowed(initial);
    if active_corners {
        lights.to_mut().activate_corners();
    }

    for i in 0..n {
        lights = Owned(lights.get_next_frame());
        if active_corners {
            lights.to_mut().activate_corners();
        }
        let image = lights.write_to_image();

        path.push(format!("{:02}.png", i));
        let _ = image.save(&path);
        path.pop();
    }

    lights.into_owned()
}

fn main() {
    let mut path = PathBuf::from(args().nth(0).unwrap());
    path.pop();
    path.push("input.txt");
    let input = read_file(&path);

    let original = Lights::parse(&input);

    path.pop();
    path.push("images");
    let lights = mutate_lights(&original, 100, &path, false);
    let active_lights = lights.count_active_lights();
    println!("{} lights are still turned on.", active_lights);

    path.pop();
    path.push("images-corners");
    let lights = mutate_lights(&original, 100, &path, true);
    let active_lights = lights.count_active_lights();
    println!("{} lights are still turned on.", active_lights);
}

#[test]
fn test_indexing() {
    let mut lights = Lights::new();
    lights[(50, 50)] = true;

    assert_eq!(lights[(50, 50)], true);
    assert_eq!(lights[(50, 51)], false);
}
