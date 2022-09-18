#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;
extern crate qrcodegen;

use qrcodegen::{QrCode as QrCode3, QrCodeEcc as QrCodeEcc2};
use napi::{Error, bindgen_prelude::Uint8ClampedArray};
use qrcode_png::{QrCode as QrCode2, QrCodeEcc, Color};

#[napi]
pub fn qrcode_image(text: String) -> Result<Uint8ClampedArray, Error> {
  if text.is_empty() {
    return Err(Error::from_reason("text length must be greater than 0".to_string()));
  }

  let mut qrcode = match QrCode2::new(text, QrCodeEcc::Medium) {
    Ok(v) => v,
    Err(err) => return Err(Error::from_reason(err.to_string()))
  };

  qrcode.margin(10);
  qrcode.zoom(10);

  let buf = match qrcode.generate(Color::Bitmap(false, true)) {
    Ok(v) => v,
    Err(err) => return Err(Error::from_reason(err.to_string()))
  };

  Ok(Uint8ClampedArray::new(buf))
}

#[napi]
pub fn qrcode_unicode(text: String) -> Result<String, Error> {
  if text.is_empty() {
    return Err(Error::from_reason("text length must be greater than 0".to_string()));
  }

  let qr = match QrCode3::encode_text(&text, QrCodeEcc2::Medium) {
    Ok(v) => v,
    Err(err) => return Err(Error::from_reason(err.to_string()))
  };

  let mut result = String::new();

  for y in 0..qr.size() {
    for x in 0..qr.size() {
      if qr.get_module(x, y) {
        result += "██";
      } else {
        result += "  ";
      }
    }
   
    result += "\n";
  }

  Ok(result.trim_end().to_string())
}

#[napi]
pub fn qrcode_svg(text: String) -> Result<String, Error> {
  if text.is_empty() {
    return Err(Error::from_reason("text length must be greater than 0".to_string()));
  }

  let qr = match QrCode3::encode_text(&text,QrCodeEcc2::Medium) {
    Ok(v) => v,
    Err(err) => return Err(Error::from_reason(err.to_string()))
  };
	let mut result = String::new();
  let dimension = qr.size();

	result += "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n";
	result += "<!DOCTYPE svg PUBLIC \"-//W3C//DTD SVG 1.1//EN\" \"http://www.w3.org/Graphics/SVG/1.1/DTD/svg11.dtd\">\n";
	result += &format!(
		"<svg xmlns=\"http://www.w3.org/2000/svg\" version=\"1.1\" viewBox=\"0 0 {0} {0}\" stroke=\"none\">\n", dimension * 2);
	result += "<rect width=\"100%\" height=\"100%\" fill=\"#FFFFFF\"/>\n";
	result += "<path d=\"";

	for y in 0..qr.size() {
		for x in 0..qr.size() {
			if qr.get_module(x, y) {
				if x != 0 || y != 0 {
					result += " ";
				}
				result += &format!("M{},{}h1v1h-1z", x, y);
			}
		}
	}

	result += "\" fill=\"#000000\"/>\n";
	result += "</svg>\n";

  Ok(result)
}