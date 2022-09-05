use std::fs::File; 
use std::io::{BufWriter, Write};

use crate::grafik_error::GrafikError;

pub struct Colour {
    r: u8, 
    g: u8, 
    b: u8, 
    a: f32,
}

impl Colour {
    pub fn new(r: u8, g: u8, b: u8, a: f32) -> Self {
        Self {
            r: r, 
            g: g, 
            b: b,
            a: a,
        }
    }

    pub fn red(self: &Self) -> &u8 {
        &self.r
    }

    pub fn green(self: &Self) -> &u8 {
        &self.g
    }

    pub fn blue(self: &Self) -> &u8 {
        &self.b
    } 
    
    pub fn alpha(self: &Self) -> &f32 {
        &self.a
    }

    pub fn colour_string(self: &Self) -> String {
        format!("{} {} {}", self.red(), self.green(), self.blue())
    }
}

pub struct Canvas <'a> {
    w: usize, 
    h: usize,
    c: &'a Colour,
}

impl<'a> Canvas <'a>{
    pub fn new(w: usize, h: usize, c: &'a Colour) -> Self {
        Self { w: w, h: h, c: c,}
    }

    pub fn width(self: &Self) -> &usize {
        &self.w
    }

    pub fn height(self: &Self) -> &usize {
        &self.h
    }

    pub fn colour(self: &Self) -> &Colour {
        &self.c
    }

    pub fn image_init(self: &Self) -> Vec<String> {
        vec![format!("\n{}", &self.colour().colour_string()); self.width() * self.height()]
    }
    pub fn file_ppm3(self: &Self, file_name: &str, image: Vec<String>) -> Result<(), GrafikError> {
        let file = File::create(file_name)?;
        let mut buffer = BufWriter::new(file);
        
        let mut data = format!("P3\n{} {}\n255", self.width(), self.height()); 
        let s = image.into_iter().collect::<String>(); 
        data = format!("{}{}", data, s);
        
        buffer.write_all(data.as_bytes())?;        
        Ok(())
    }
}


pub struct Point <'a>{
    x: usize, 
    y: usize, 
    c: &'a Colour, 
}

impl<'a> Point <'a> {
    pub fn new(x: usize, y: usize, c: &'a Colour) -> Self {
        Self {
            x: x, 
            y: y, 
            c: c, 
        }
    }

    pub fn x(self: &Self) -> &usize {
        &self.x
    }

    pub fn y(self: &Self) -> &usize {
        &self.y
    }

    pub fn colour(self: &Self) -> &Colour {
        &self.c
    }

    pub fn add(self: &Self, canvas: &Canvas, mut image: Vec<String>) -> Vec<String> {
        image[self.x() + canvas.width() * self.y()] = format!("\n{}", self.colour().colour_string()); 
        image
    }
}
