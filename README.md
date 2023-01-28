## grafik - Graphics primitives library implementation in Rust

### Functionality: 
1. Creates a (width x height) image in memory with a background colour and an opacity; 
1. Creates a pixel (pixels) with certain coordinates, colour, and opacity within the image;   
1. Writes the image stored in memory to a hard drive as a ppm P3 image file. Opacity is not supported in this image format and will be dropped. Opacity will be supported when pam P7 image file format will be implemented

### How to use this library: 

1. Add to Cargo.toml file: 

```Toml
    [dependencies]
    grafik = {git = "https://github.com/azavgo/grafik", branch = "main"}
```
2. Create and print a complex number:  
```Rust
    use grafik::*;

    fn main() -> Result<(), GrafikError>{
    
        let canvas_background = Colour::new(23, 146, 59, 1.0);
        
        let canvas = Canvas::new(800, 600, &background); 

        canvas.file_ppm3("test_01")?;         
        Ok(())
    }
