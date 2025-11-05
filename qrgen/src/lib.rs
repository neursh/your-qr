use qrcode::{ Color, EcLevel, QrCode };
use wasm_bindgen::prelude::*;

fn primitive_convert<D: AsRef<[u8]>>(data: D, correction_level: u8) -> Result<QrCode, String> {
  let code = QrCode::with_error_correction_level(data, match correction_level {
    0 => EcLevel::L,
    1 => EcLevel::M,
    2 => EcLevel::Q,
    3 => EcLevel::H,
    _ => {
      return Err("Invalid correction level.".to_owned());
    }
  });

  match code {
    Ok(code) => {
      return Ok(code);
    }
    Err(error) => {
      return Err(format!("Cannot construct thhe QR code. Error:{}", error));
    }
  }
}

fn convert_to_vec<D: AsRef<[u8]>>(data: D, correction_level: u8) -> Result<Vec<u8>, String> {
  let code = primitive_convert(data, correction_level)?;

  let bool_map: Vec<u8> = code
    .into_colors()
    .iter()
    .map(|value| {
      match value {
        qrcode::Color::Light => 0,
        qrcode::Color::Dark => 1,
      }
    })
    .collect();

  return Ok(bool_map);
}

fn convert_to_svg<D: AsRef<[u8]>>(data: D, correction_level: u8) -> Result<String, String> {
  let code = primitive_convert(data, correction_level)?;
  let module_count = code.width();
  let color_map = code.into_colors();

  let mut path_data = String::new();

  for y in 0..module_count {
    let mut x = 0;
    while x < module_count {
      let idx = y * module_count + x;
      if color_map[idx] == Color::Dark {
        let mut width = 1;
        while x + width < module_count && color_map[y * module_count + x + width] == Color::Dark {
          width += 1;
        }
        path_data.push_str(&format!("M{} {}h{}v{}H{}z", x, y, width, 1, x));
        x += width;
      } else {
        x += 1;
      }
    }
  }

  return Ok(path_data);
}

#[wasm_bindgen]
#[allow(non_snake_case)]
pub fn fromStringToQr(string: &str, correction_level: u8) -> Result<Vec<u8>, String> {
  return convert_to_vec(string, correction_level);
}

#[wasm_bindgen]
#[allow(non_snake_case)]
pub fn fromBytesToQr(data: Vec<u8>, correction_level: u8) -> Result<Vec<u8>, String> {
  return convert_to_vec(data, correction_level);
}

#[wasm_bindgen]
#[allow(non_snake_case)]
pub fn fromStringtoQrSvg(string: &str, correction_level: u8) -> Result<String, String> {
  return convert_to_svg(string, correction_level);
}

#[wasm_bindgen]
#[allow(non_snake_case)]
pub fn fromBytesToQrSvg(data: Vec<u8>, correction_level: u8) -> Result<String, String> {
  return convert_to_svg(data, correction_level);
}
