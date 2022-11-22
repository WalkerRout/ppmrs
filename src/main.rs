
mod ppmimage;

use ppmimage::PPMImage;

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let mut image = PPMImage::new(680, 480);
  image.fill(0xFF0000FF);
  image.save("testrust.ppm")?;

  Ok(())
}
