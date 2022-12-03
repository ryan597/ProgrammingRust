use num::Complex;
use std::str::FromStr;
use image::ColorType;
use image::png::PNGEncoder;
use std::fs::File;
use std::env;

/*
// Complex is defined as
struct Complex<T>
{
    re: T,
    im: T,
} */

/*
// Option is an enum
enum Option<T>
{
    None,
    Some(T),
}
*/

/// Parse the string s as a coordinate pair
fn parse_pair<T: FromStr>(s: &str, seperator: char) -> Option<(T, T)>
{
    match s.find(seperator)
    {
        None => None,  // if find returns None, return None
        Some(index) => {  // if find returns some index, use the index as location of the seperator
            match (T::from_str(&s[..index]), T::from_str(&s[index + 1..])) {  // get type T from substring
                (Ok(l), Ok(r)) => Some((l, r)),  // if both l and r are ok, return Some (l, r) tuple
                _ => None  // Else the parsing failed, _ is a wildcard pattern, return None
            }
        }
    }
}

#[test]
fn test_parse_pair()
{
    assert_eq!(parse_pair::<i32>("", ','), None);
    assert_eq!(parse_pair::<i32>("10,", ','), None);
    assert_eq!(parse_pair::<i32>(",10", ','), None);
    assert_eq!(parse_pair::<i32>("10,20", ','), Some((10, 20)));
    assert_eq!(parse_pair::<i32>("10,20xy", ','), None);
    assert_eq!(parse_pafn complex_square_add_loop(c: Complex<f64>)
    {
        let mut z = Complex{re: 0.0, im: 0.0};
        loop
        {
            z = z * z + c;
        }
    }ir::<i32>("0.5x", 'x'), None);
    assert_eq!(parse_pair::<i32>("0.5x1.5", 'x'), Some((0.5, 1.5)));
}


/// Parse a pair of floating point numbers, seperated by a comma as a complex number
fn parse_complex(s: &str) -> Option<Complex<f64>>
{
    match parse_pair(s, ',')
    {
        Some((re, im)) => Some(Complex{re, im}),
        None => None
    }
}

#[test]
fn test_parse_complex()
{
    assert_eq!(parse_complex("1.25,-0.0625"), Some(Complex{re: 1.25, im: -0.0625}));
    assert_eq!(parse_complex(", -0.0625"), None);
}


/// Given row and column of a pixel, return the corresponding point on the complex plane.
///
/// `bounds` is a pair giving width and height of the image
/// `pixel` is a (column, row) pair of the coordinates
/// `upper_left` and `lower_right` indicate the bounds of the complex plane we cover
fn pixel_to_point(bounds: (usize, usize),
                  pixel: (usize, usize),
                  upper_left: Complex<f64>,
                  lower_right: Complex<f64>) -> Complex<f64>
{
    let (width, height) = (lower_right.re - upper_left.re, upper_left.im - lower_right.im);

    Complex{re: upper_left.re + (pixel.0 as f64) * width / (bounds.0 as f64),
            im: upper_left.im - (pixel.1 as f64) * height / (bounds.1 as f64)
        // pixel.1 increases as we go down, im component increase as we go up thus subtraction
        }
}

#[test]
fn test_pixel_to_point()
{
    assert_eq!(pixel_to_point((100, 200), (25, 175),
                              Complex{re: -1.0, im: 1.0},
                              Complex{re: 1.0, im: -1.0}),
               Complex{re: -0.5, im: -0.75});
}

/// Try determine if c is in the mandelbrot set
/// If not in the set, then function returns the iteration when it left (out to infinity)
/// If c is in the set, the function returns None
fn escape_time(c: Complex<f64>, limit: usize) -> Option<usize>
{
    let mut z = Complex{re: 0.0, im: 0.0};
    for i in 0..limit
    {
        if z.norm_sqr() > 4.0
        {
            return Some(i);
        }
        z = z * z + c;
    }
    None
}


/// Render a rectangle of the Mandelbrot set into a buffer of pixels
///
/// `bounds` gives the width and height of the buffer `pixels` which holds
/// one grayscale pixel per byte. The `upper_left` and `lower_right` args specify
/// points on the complex plane corresponding to the corners of the pixel buffer.
fn render(pixels: &mut [u8],
          bounds: (usize, usize),
          upper_left: Complex<f64>,
          lower_right: Complex<f64>)
{
    assert!(pixels.len() == bounds.0 * bounds.1);

    for row in 0..bounds.1
    {
        for column in 0..bounds.0
        {
            let point = pixel_to_point(bounds, (column, row), upper_left, lower_right);
            pixels[row * bounds.0 + column] = match escape_time(point, 255)
            {
                None => 0,
                Some(count) => 255 - (count as u8)
            };
        }
    }
}


/// Write teh bufffer `pixels`, whose dimensions are given by `bounds` to the file `filename`
fn write_image(filename: &str, pixels: &[u8], bounds: (usize, usize)) -> Result<(), std::io::Error>
{
    /*
    let output = match File::create(filename)
    {
        Ok(f) => f,
        Err(e) => {return Err(e);}
    };
    */
    // Equivalent to
    let output = File::create(filename)?;

    let encoder = PNGEncoder::new(output);
    encoder.encode(&pixels, bounds.0 as u32, bounds.1 as u32, ColorType::Gray(8))?;

    Ok(())
}


fn main()
{
    let args: Vec<String> = env::args().collect();

    if args.len() != 5
    {
        eprintln!("Usage: {} FILE PIXELS UPPERLEFT LOWERRIGHT", args[0]);
        eprintln!("Example: {} mandel.png 1000x750 -1.20,0.35 -1,0.20", args[0]);
        std::process::exit(1);
    }

    let bounds = parse_pair(&args[2], 'x').expect("error parsing image dimensions");
    let upper_left = parse_complex(&args[3]).expect("error parsing upper left corner point");
    let lower_right = parse_complex(&args[4]).expect("error parsing lower right corner point");

    let mut pixels = vec![0; bounds.0 * bounds.1];

    // single thread
    //render(&mut pixels, bounds, upper_left, lower_right);
    // concurrent threads
    let threads = 8;
    let rows_per_band = bounds.1 / threads + 1;
    {
        // divide the pixel buffer into bands. Chunks returns iterator producing mutable nonoverlapping slices of the buffer
        // collect returns a vector of the iterable chunks
        let bands: Vec<&mut [u8]> = pixels.chunks_mut(rows_per_band * bounds.0).collect();
        crossbeam::scope(|spawner|  // scope ensures all processes finish before moving on (synchronise)
        {
            for (i, band) in bands.into_iter().enumerate()
            {
                let top = rows_per_band * i;
                let height = band.len() / bounds.0;
                let band_bounds = (bounds.0, height);
                let band_upper_left = pixel_to_point(bounds, (0, top), upper_left, lower_right);
                let band_lower_right = pixel_to_point(bounds, (bounds.0, top + height), upper_left, lower_right);

                spawner.spawn(move |_|
                {
                    render(band, band_bounds, band_upper_left, band_lower_right);
                });
            }
        }).unwrap();  // unwrap if any errors occur
    }
    // concurrent threads end

    write_image(&args[1], &pixels, bounds).expect("error writing PNG file");
}
