#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;
extern crate qrcodegen;

use napi::bindgen_prelude::{AbortSignal, AsyncTask, Uint8ClampedArray};
use napi::{Env, Error, Task};
use qrcode_png::{Color, QrCode as QrCode2, QrCodeEcc};
use qrcodegen::{QrCode as QrCode3, QrCodeEcc as QrCodeEcc2};

pub struct QrCodeTask {
  input: String,
  output_type: String,
}

#[napi]
impl Task for QrCodeTask {
  type Output = Uint8ClampedArray;
  type JsValue = Uint8ClampedArray;

  fn compute(&mut self) -> Result<Self::Output, Error> {
    match self.output_type.as_str() {
      "Image" => qrcode_image(self.input.to_string()),
      "SVG" => qrcode_svg(self.input.to_string()),
      "Unicode" => qrcode_unicode(self.input.to_string()),
      _ => Err(Error::from_reason("unknown type")),
    }
  }

  fn resolve(&mut self, _env: Env, output: Self::Output) -> Result<Self::Output, Error> {
    Ok(output)
  }

  fn reject(&mut self, _env: Env, err: Error) -> napi::Result<Self::JsValue> {
    Err(err)
  }
}

#[napi]
pub struct QrCode {
  pub(crate) input: String,
}

#[napi]
impl QrCode {
  #[napi(constructor)]
  pub fn new(input: String) -> QrCode {
    Self { input }
  }

  #[napi]
  pub fn async_image(&mut self, signal: Option<AbortSignal>) -> AsyncTask<QrCodeTask> {
    AsyncTask::with_optional_signal(
      QrCodeTask {
        input: self.input.clone(),
        output_type: "Image".to_string(),
      },
      signal,
    )
  }

  #[napi]
  pub fn async_svg(&mut self, signal: Option<AbortSignal>) -> AsyncTask<QrCodeTask> {
    AsyncTask::with_optional_signal(
      QrCodeTask {
        input: self.input.clone(),
        output_type: "SVG".to_string(),
      },
      signal,
    )
  }

  #[napi]
  pub fn async_unicode(&mut self, signal: Option<AbortSignal>) -> AsyncTask<QrCodeTask> {
    AsyncTask::with_optional_signal(
      QrCodeTask {
        input: self.input.clone(),
        output_type: "Unicode".to_string(),
      },
      signal,
    )
  }
}

#[napi]
pub fn qrcode_image(text: String) -> Result<Uint8ClampedArray, Error> {
  if text.is_empty() {
    return Err(Error::from_reason(
      "text length must be greater than 0".to_string(),
    ));
  }

  let mut qrcode = match QrCode2::new(text, QrCodeEcc::Medium) {
    Ok(v) => v,
    Err(err) => return Err(Error::from_reason(err.to_string())),
  };

  qrcode.margin(10);
  qrcode.zoom(10);

  let buf = match qrcode.generate(Color::Bitmap(false, true)) {
    Ok(v) => v,
    Err(err) => return Err(Error::from_reason(err.to_string())),
  };

  Ok(Uint8ClampedArray::new(buf))
}

#[napi]
pub fn qrcode_unicode(text: String) -> Result<Uint8ClampedArray, Error> {
  if text.is_empty() {
    return Err(Error::from_reason(
      "text length must be greater than 0".to_string(),
    ));
  }

  let qr = match QrCode3::encode_text(&text, QrCodeEcc2::Medium) {
    Ok(v) => v,
    Err(err) => return Err(Error::from_reason(err.to_string())),
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

  Ok(Uint8ClampedArray::new(Vec::from(
    result.trim_end().as_bytes(),
  )))
}

#[napi]
pub fn qrcode_svg(text: String) -> Result<Uint8ClampedArray, Error> {
  if text.is_empty() {
    return Err(Error::from_reason(
      "text length must be greater than 0".to_string(),
    ));
  }

  let qr = match QrCode3::encode_text(&text, QrCodeEcc2::Medium) {
    Ok(v) => v,
    Err(err) => return Err(Error::from_reason(err.to_string())),
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

  Ok(Uint8ClampedArray::new(Vec::from(result.as_bytes())))
}
