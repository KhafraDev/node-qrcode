#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use napi::{Error, bindgen_prelude::Uint8ClampedArray};
use qrcode::{QrCode, render::{svg, unicode}};
use qrcode_png::{QrCode as QrCode2, QrCodeEcc, Color};

#[napi(object)]
pub struct SvgOptions {
  pub min_width: Option<u32>,
  pub min_height: Option<u32>,
  pub dark_color: Option<&'static str>,
  pub light_color: Option<&'static str>
}

#[napi]
pub fn qrcode_image(text: String) -> Result<Uint8ClampedArray, Error> {
  if text.is_empty() {
    return Err(Error::from_reason("text length must be greater than 0".to_string()));
  }

  let mut qrcode = QrCode2::new(text, QrCodeEcc::Medium).unwrap();

  qrcode.margin(10);
  qrcode.zoom(10);

  let buf = qrcode.generate(Color::Bitmap(false, true))
    .expect("unable to create a qrcode");
  
  Ok(Uint8ClampedArray::new(buf))
}

#[napi]
pub fn qrcode_svg(text: String, options: Option<SvgOptions>) -> Result<String, Error> {
  if text.is_empty() {
    return Err(Error::from_reason("text length must be greater than 0".to_string()));
  }

  let code = QrCode::new(text).expect("unable to create a qrcode");

  if options.is_none() {
    let image = code.render::<svg::Color>()
      .min_dimensions(200, 200)
      .dark_color(svg::Color("#000"))
      .light_color(svg::Color("#fff"))
      .quiet_zone(false)
      .build();

    return Ok(image)
  }

  let svg_options = options.unwrap();
  let min_width = svg_options.min_width.unwrap_or(200);
  let min_height = svg_options.min_height.unwrap_or(200);
  let dark_color = svg_options.dark_color.unwrap_or("#000");
  let light_color = svg_options.light_color.unwrap_or("#fff");

  let image = code.render::<svg::Color>()
    .min_dimensions(min_width, min_height)
    .dark_color(svg::Color(dark_color))
    .light_color(svg::Color(light_color))
    .quiet_zone(false)
    .build();

  Ok(image)
}

#[napi]
pub fn qrcode_unicode(text: String) -> Result<String, Error> {
  if text.is_empty() {
    return Err(Error::from_reason("text length must be greater than 0".to_string()))
  }

  let code = QrCode::new(text).expect("unable to create a qrcode");
  let image = code.render::<unicode::Dense1x2>()
    .dark_color(unicode::Dense1x2::Dark)
    .light_color(unicode::Dense1x2::Light)
    .quiet_zone(false)
    .build();

  Ok(image)
}