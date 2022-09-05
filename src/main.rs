mod grafik_error;
use grafik_error::GrafikError;

mod grafik_lib;
use grafik_lib::*;

fn main() -> Result<(), GrafikError>{
    let background = Colour::new(255, 255, 255, 1.0);
    let point_colour = Colour::new(0, 0, 0, 1.0); 
    let canvas = Canvas::new(50, 50, &background); 
    let mut image = canvas.image_init();  
    let mut point = Point::new(0, 0, &point_colour); 
/*
    for _j in canvas.height()/2 .. *canvas.height() {
        for _i in canvas.width()/2 .. *canvas.width() {
            point = Point::new(_i, _j, &point_colour);
            image = point.add(&canvas, image);
        }
    }
 */
    canvas.file_ppm3("test_01", image)?;         
    Ok(())
}
