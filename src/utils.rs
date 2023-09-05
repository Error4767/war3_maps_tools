use encoding::{ Encoding, DecoderTrap, all::UTF_8 };

extern crate mpq;

use std::str;
use mpq::Archive;

// u8 转化到 string
pub fn u8_to_string(buf: &[u8]) -> String {

    let mut content_string = String::new();

    // 解码
    UTF_8.decode_to(&buf, DecoderTrap::Replace, &mut content_string).unwrap();

    content_string
}

// 读取到 u8 vec
pub fn read_mpq_file_to_u8_vec(archive: &mut mpq::Archive, file_name: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let file = archive.open_file(file_name)?;
    // 创建一个指定长度的 buffer
    let mut buf: Vec<u8> = vec![0; file.size() as usize];
    // 读取到 u8 数据
    file.read(archive, &mut buf).unwrap();
    Ok(buf)
}

// 读取到 string
pub fn read_mpq_file_to_content_string(archive: &mut mpq::Archive, file_name: &str) -> Result<String, Box<dyn std::error::Error>> {
    let buf = read_mpq_file_to_u8_vec(archive, file_name)?;

    let mut content_string = String::new();

    // let content_string = buf.join("-");
    // // 输出16进制结果
    // for v in buf.iter() {
    //     content_string.push_str(
    //         &(if v < &16 {
    //             format!(" 0{:x}", v)
    //         } else {
    //             format!(" {:x}", v)
    //         })
    //     );
    // }
    // 解码
    UTF_8.decode_to(&buf, DecoderTrap::Replace, &mut content_string).unwrap();

    Ok(content_string)
}

pub fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}
