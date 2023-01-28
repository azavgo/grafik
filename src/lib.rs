use std::fs::File; 
use std::io::{BufWriter, Write};

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


pub struct Canvas {
    width: usize, 
    height: usize,
    pub image: Vec<String>,
}

impl Canvas {
    pub fn new(width: usize, height: usize, colour: &Colour) -> Self {
        Self {
            width: width, 
            height: height,
            image: vec![format!("\n{}", colour.colour_string()); width * height],    
        }
    }

    pub fn width(self: &Self) -> usize {
        self.width
    }

    pub fn height(self: &Self) -> usize {
        self.height
    }

    pub fn file_ppm3(self: Self, file_name: &str) -> Result<(), GrafikError> {
        let file = File::create(file_name)?;
        let mut buffer = BufWriter::new(file);
        
        let mut data = format!("P3\n{} {}\n255", self.width(), self.height()); 
        let v = self.image;
        let s = v.into_iter().collect::<String>(); 
        data = format!("{}{}", data, s);
        
        buffer.write_all(data.as_bytes())?;        
        Ok(())
    }
}

pub trait CanvasAdd <'a>{
    fn add(self: &'a Self, canvas: Canvas) -> Canvas; 
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
}

impl<'a> CanvasAdd <'a> for Point <'a> {
    fn add(self: &Self, mut canvas: Canvas) -> Canvas {
        let c = self.x() + canvas.width() * self.y();
        canvas.image[c] = format!("\n{}", self.colour().colour_string()); 
        canvas
    }
}
 
#[derive(Debug)]
pub enum GrafikError { 
    IOError(std::io::Error),  
}

impl From<std::io::Error> for GrafikError {
    fn from(error: std::io::Error) -> Self {
        GrafikError::IOError(error)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_canvas_new_01() {
        let colour = Colour::new(22, 132, 89, 0.3);
        let canvas = Canvas::new(4, 2, &colour); 
        assert_eq!(vec!["\n22 132 89".to_string(); 8], canvas.image);
    }

    #[test]
    fn test_canvas_file_ppm3_01() -> Result<(), GrafikError> {
        let colour = Colour::new(145, 12, 56, 0.3);
        let canvas = Canvas::new(800, 600, &colour); 
        canvas.file_ppm3("test_canvas_file_ppm3_02")?;
        Ok(())
    }
}
