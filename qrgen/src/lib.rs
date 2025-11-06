use qrcode::{ EcLevel, QrCode };
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
pub fn fromQrToSvgPath(qr: Vec<u8>) -> String {
  let module_count = (qr.len() as f64).sqrt() as usize;

  let mut path_data = String::new();

  for y in 0..module_count {
    let mut x = 0;
    while x < module_count {
      let idx = y * module_count + x;
      if qr[idx] == 1 {
        let mut width = 1;
        while x + width < module_count && qr[y * module_count + x + width] == 1 {
          width += 1;
        }
        path_data.push_str(&format!("M{} {}h{}v{}H{}z", x, y, width, 1, x));
        x += width;
      } else {
        x += 1;
      }
    }
  }

  return path_data;
}
