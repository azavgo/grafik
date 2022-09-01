use ppm::*; 

#[derive(Debug)]
pub enum GrafikError { 
    PPMError(PPMError),  
}

impl From<PPMError> for GrafikError {
    fn from(error: PPMError) -> Self {
        GrafikError::PPMError(error)
    }
}

pub struct Point {
    x: usize, 
    y: usize, 
    colour: Colour, 
}

impl Point {
    pub fn new(x: usize, y: usize, colour: Colour) -> Self {
        Self {
            x: x, 
            y: y, 
            colour: colour, 
        }
    }

    pub fn x(self: &Self) -> &usize {
        &self.x
    }

    pub fn y(self: &Self) -> &usize {
        &self.y
    }

    pub fn colour(self: &Self) -> &Colour {
        &self.colour
    }
}

fn main() -> Result<(), GrafikError>{
    let background = Colour::new(255, 0, 0);
    let canvas = PPMP3::new(800, 600);  
    canvas.file_p3("test_01", background)?;         
    Ok(())
}
