use mpq;
use std::str;
use lazy_static;
use crate::utils::{ read_mpq_file_to_u8_vec, u8_to_string, read_mpq_file_to_content_string };
use crate::datasets::{ get_tileset_text, get_size_description, get_size_description_more_than_1_29_version };
use crate::parse_wts_file_content::{ parse_to_hash_map, ParsedVariables };

#[derive(Debug)]
pub struct MapSize {
    playable: String,
    full: String,
    description: String,
    description_more_than_the_1_29_version: String,
}

// 地图信息
#[derive(Debug)]
pub struct MapInfo {
    name: String,
    author: String,
    description: String,
    tileset: String,
    recommend_number_of_players: String,
    maxium_number_of_players: i32,
    map_size: MapSize,
}

// 根据信息真实的值，如果是 TRIGSTR_xxx 就进行变量读取
pub fn get_info_real_value(parsed_variables: &ParsedVariables, name: &str) -> String {
    // 如果以这个开头，则去 wts 文件里找对应的
    if name.starts_with("TRIGSTR_") {
        // 转化为数字，则将其再次转为字符串获取其值， wts 文件中 String 1 对应 w3i 文件中 TRIGSTR_001，去多余的0处理， 不可转化为数字的话，直接返回原来的
        match &name[8..].parse::<i32>() {
            Ok(v)=> String::from(parsed_variables.get(&v.to_string()).unwrap_or(&String::from(name))),
            Err(_)=> name.to_string()
        }
    }else {
        name.to_string()
    }
}

// 获取基础字符串信息, 从 0xC 开始，以 byte 0 分割，前四个有效字符串
pub fn get_map_basic_string_info(buf: &[u8]) -> Vec<String> {
    // 基础信息数组，对应 名称，作者，描述，推荐玩家数
    let mut basic_infos: Vec<String> = vec![];

    // 当前项的 byte 数据
    let mut current_item_bytes: Vec<u8> = vec![];
    // 分割字节
    let split_byte: u8 = 0;
    // 从 0xC 开始遍历
    for byte in &buf[0xC..] {
        // 长度超过4，不再解析
        if basic_infos.len() == 4 {
            break;
        }
        // 如果遇到分割位，存储已经解析的字节
        if byte == &split_byte {
            // 如果有内容，存入
            if current_item_bytes.len() > 0 {
                basic_infos.push(u8_to_string(&current_item_bytes));
                // 清空，等待下次收集
                current_item_bytes.clear();
            }
        } else {
            current_item_bytes.push(*byte);
        }
    }
    basic_infos
}

// 获取地图尺寸
fn get_map_size(buf: &[u8], offset_bytes_length: i32) -> MapSize {
    // 根据偏移量获取真实位置, 并且转为 usize
    let get_real_index = |index: u8| (index as i32 + offset_bytes_length) as usize;

    // 获取宽度 0x6C 1 = 1，0x6D 1 = 256

    // 长度第二位值
    let length_two_byte_value = buf[get_real_index(0x6D)];
    // 实际长度值
    let length = buf[get_real_index(0x6C)] as i32 + (if length_two_byte_value as i32 > 0 { 256 * length_two_byte_value as i32 } else { 0 });
    // 宽度第二位值
    let width_two_byte_value = buf[get_real_index(0x71)];
    // 实际宽度值
    let width = buf[get_real_index(0x70)] as i32 + (if width_two_byte_value as i32 > 0 { 256 * width_two_byte_value as i32 } else { 0 });
    
    MapSize {
        playable: format!("{}x{}", length, width),
        full: format!("{}x{}", buf[get_real_index(0x5C)] as i32 + buf[get_real_index(0x60)] as i32 + length, buf[get_real_index(0x64)] as i32 + buf[get_real_index(0x68)] as i32 + width),
        description: get_size_description(length * width).to_string(),
        description_more_than_the_1_29_version: get_size_description_more_than_1_29_version(length * width).to_string(),
    }
}

pub fn get_map_basic_info(archive: &mut mpq::Archive) -> Option<MapInfo> {
    let buf = match read_mpq_file_to_u8_vec(archive, "war3map.w3i") {
        Ok(v)=>  v,
        Err(_)=> {
            return None;
        }
    };

    let basic_info = get_map_basic_string_info(&buf);

    // 偏移字节长度(44 是一个标准值，当几个基础属性都是变量，如 TRIGSTR_001 是11个字节长度，四个字符串字段)
    let mut offset_bytes_length: i32 = -44;
    for basic_info_value in basic_info.iter() {
        offset_bytes_length += basic_info_value.len() as i32;
    }

    // 根据偏移量获取真实位置, 并且转为 usize
    let get_real_index = |index: u8| (index as i32 + offset_bytes_length) as usize;
    
    // 地图基本信息及引用变量名称
    let mut map_info = MapInfo {
        name: basic_info[0].to_string(),
        author: basic_info[1].to_string(),
        description: basic_info[2].to_string(),
        recommend_number_of_players: basic_info[3].to_string(),
        tileset: u8_to_string(&buf[get_real_index(0x78)..(get_real_index(0x78) + 1)]),
        maxium_number_of_players: buf[get_real_index(0xA7)] as i32,
        map_size: get_map_size(&buf, offset_bytes_length),
    };

    // 如果可以读取到 wts 文件
    if let Ok(content_string) = read_mpq_file_to_content_string(archive, "war3map.wts") {
        let parsed_variables = parse_to_hash_map(content_string);
        map_info.name = get_info_real_value(&parsed_variables, &map_info.name);
        map_info.author = get_info_real_value(&parsed_variables, &map_info.author);
        map_info.description = get_info_real_value(&parsed_variables, &map_info.description);
        map_info.recommend_number_of_players = get_info_real_value(&parsed_variables, &map_info.recommend_number_of_players);
    }

    map_info.tileset = get_tileset_text(&map_info.tileset).to_string();

    Some(map_info)
}