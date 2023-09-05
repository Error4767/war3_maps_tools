use mpq::Archive;
use crate::utils::{ read_mpq_file_to_u8_vec, print_type_of };

use std::io::Cursor;

use image::io::Reader as ImageReader;
pub use image::DynamicImage;
use image::ImageFormat;

use image_blp::types::image::BlpImage;
use image_blp::{convert::blp_to_image, parser::parse_blp};

// 获取小地图
pub fn get_mini_map(archive: &mut mpq::Archive) -> Option<DynamicImage> {

  let buf = match read_mpq_file_to_u8_vec(archive, "war3mapMap.blp") {
    Ok(v)=> v,
    Err(_)=> {
      return None;
    },
  };

  let blp_file = match parse_blp(&buf) {
    Ok(v)=> v.1,
    Err(_)=> {
      return None;
    }
  };

  match blp_to_image(&blp_file, 0) {
    Ok(v)=> Some(v),
    Err(_)=> {
      return None;
    }
  }
}

// // tga 格式小地图
// pub fn get_mini_map_tga() {
//   return ImageReader::new(Cursor::new(buf)).with_guessed_format()
//     .map_or_else(|err| Err(err), |v| Ok(v.decode()))
//     .map_or_else(|_| None, |v| v.ok());
//   }
// }

// 获取自定义小地图
pub fn get_custom_mini_map(archive: &mut mpq::Archive)-> Option<DynamicImage> {
  read_mpq_file_to_u8_vec(archive, "War3MapMap.tga")
    .map_or_else(|_| None, |buf| ImageReader::with_format(Cursor::new(buf), ImageFormat::Tga).decode().ok())
}

// 获取预览图
pub fn get_preview_map(archive: &mut mpq::Archive)-> Option<DynamicImage> {
  read_mpq_file_to_u8_vec(archive, "War3MapPreview.tga")
    .map_or_else(|_| None, |buf| ImageReader::with_format(Cursor::new(buf), ImageFormat::Tga).decode().ok())
}

// 获取一些小地图
pub fn get_mini_maps(archive: &mut mpq::Archive) -> (Option<DynamicImage>, Option<DynamicImage>, Option<DynamicImage>) {
  (get_mini_map(archive), get_custom_mini_map(archive), get_preview_map(archive))
}

pub fn save_to_path(image: DynamicImage, path: &str) -> Result<(), Box<dyn std::error::Error>> {
  // 如果有错误就显示
  image.save(path)?;
  Ok(())
}