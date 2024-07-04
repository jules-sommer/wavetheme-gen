#![allow(internal_features)]
#![feature(generic_arg_infer)]
#![feature(let_chains)]
#![feature(fmt_internals)]

use bmp::Image;
use csv::ReaderBuilder;
use palette::{rgb::Rgb, xyz::Xyz, IntoColor, LinSrgb, Srgb};
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

struct Palette {
  base: (LinSrgb<f32>, Vec<LinSrgb<f32>>),
  accent: (LinSrgb<f32>, Vec<LinSrgb<f32>>),
  background: LinSrgb<f32>,
  foreground: LinSrgb<f32>,
}

struct Color {
  color: Srgb<u8>,
}

impl Color {
  fn as_linear(&self) -> LinSrgb<f32> {
    self.color.into()
  }
}
impl From<&str> for Color {
  fn from(str: &str) -> Self {
    let rgb_from_str: Srgb<u8> = str.parse().unwrap();
    Self {
      color: rgb_from_str,
    }
  }
}

impl Debug for Color {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let linear = self.as_linear();
    write!(
      f,
      "RGB({}, {}, {})\nRGB_VEC({}, {}, {})",
      self.color.red,
      self.color.green,
      self.color.blue,
      linear.red,
      linear.green,
      linear.blue
    )
  }
}

impl std::fmt::Display for Color {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(
      f,
      "RGB({}, {}, {})",
      self.color.red, self.color.green, self.color.blue
    )
  }
}

impl From<Srgb<u8>> for Color {
  fn from(rgb: Srgb<u8>) -> Self {
    Self { color: rgb }
  }
}

impl From<&Rgb> for Color {
  fn from(rgb: &Rgb) -> Self {
    println!("{:?}", rgb);

    Self {
      color: (*rgb).into(),
    }
  }
}

impl From<Srgb<f32>> for Color {
  fn from(rgb: Srgb<f32>) -> Self {
    Self { color: rgb.into() }
  }
}

type WaveChromaPair = (Wavelength, Xyz64);

#[derive(Debug, Deserialize, Serialize)]
struct Wavelength {
  wavelength: f32,
}

#[derive(Debug, Deserialize, Serialize)]
struct Xyz64 {
  x: f64,
  y: f64,
  z: f64,
}

/// use palette::{Srgb, LinSrgb};
///
/// let source: LinSrgb<f32> = todo!();
///
/// let u8_array: [u8; 3] = Srgb::from_linear(source).into();
/// let hex_string1 = format!("#{:x}", Srgb::<u8>::from_linear(source)); // The # is optional.
/// let u32_value: u32 = Srgb::from_linear(source).into();

const HEX: [&str; 16] = [
  "1A1B26", "16161E", "2F3549", "444B6A", "787C99", "A9B1D6", "CBCCD1",
  "D5D6DB", "C0CAF5", "A9B1D6", "0DB9D7", "9ECE6A", "B4F9F8", "2AC3DE",
  "BB9AF7", "F7768E",
];

fn random_srgb() -> Srgb<u8> {
  let mut rand = rand::thread_rng();
  let r = rand.gen_range(0..=255);
  let g = rand.gen_range(0..=255);
  let b = rand.gen_range(0..=255);

  Srgb::new(r, g, b)
}

fn xyz_to_chromaticity(xyz: Xyz) -> (f32, f32) {
  let (x, y, z) = xyz.into();
  let sum = x + y + z;
  let x = x / sum;
  let y = y / sum;

  (x, y)
}

fn main() {
  let w = 64 * HEX.len() as u32;
  let _bmp = Image::new(w, w);

  // Example sRGB color
  let color = Srgb::new(55u8, 32u8, 120u8);
  let random_color = random_srgb();

  let mixed = color.into_linear::<f32>() + random_color.into_linear::<f32>();
  println!("{:?}", mixed);

  let xyz: Xyz = color.into_format().into_color();
  let chroma = xyz_to_chromaticity(xyz);
  println!(
    "===============\nXYZ: {:#?}\nChroma: {:#?}\nColor: {:#?}\n===============",
    xyz, chroma, color
  );

  let cmf_data_csv = std::fs::File::open("cie_xyz-data.csv");
  match cmf_data_csv {
    Ok(file) => {
      let mut rdr = ReaderBuilder::new().has_headers(false).from_reader(file);
      for result in rdr.deserialize::<(Wavelength, Xyz64)>() {
        let (wavelength, xyz): WaveChromaPair = result.unwrap();
        println!("{{ wavelength: {:#?}, xyz: {:#?} }}", wavelength, xyz);
      }
    }
    Err(e) => println!("Error opening file: {}", e),
  }
}
