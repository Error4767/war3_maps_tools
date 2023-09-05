use std::collections::HashMap;
use std::fs::{ self, File, read_to_string };
use std::io::Write;

extern crate mpq;

use mpq::Archive;
mod utils;
use crate::utils::{read_mpq_file_to_u8_vec, read_mpq_file_to_content_string};
mod datasets;
mod extract_info;
use extract_info::{ get_map_basic_info, MapInfo };
mod parse_wts_file_content;
use parse_wts_file_content::parse_to_hash_map;
mod extra;
use extra::{ get_mini_maps, save_to_path };

// 所有地图信息
type MapInfos = HashMap<String, Option<MapInfo>>;

const OUT_FILES_SAVE_DIRECTORY: &str = "./analyze/";
// 图片文件目录
const IMAGE_FILES_DIRECTORY: &str =  "./analyze/images/";
// 小地图文件扩展文件名
const MINI_MAP_FILES_EXTENSIONS_FILE_NAME: &str = "-war3mapMap.png";
// 自定义小地图文件扩展文件名
const CUSTOM_MINI_MAP_FILES_EXTENSIONS_FILE_NAME: &str = "-war3mapPreview.png";
// 预览图文件扩展文件名
const PREVIEW_MAP_FILES_EXTENSIONS_FILE_NAME: &str = "-war3mapPreview.png";

// 生成一个友好的可读的 html 文件
fn generate_html_file(map_infos: MapInfos) {
    let mut content = String::from("");

    for (map_file_name, map_info) in map_infos.into_iter() {
        content.push_str(
            &format!(
                r#"
                <h4>{}</h4><pre>{}</pre>
                <img class="mini-map" src="./images/{}{}" alt="">
                <img class="mini-map" src="./images/{}{}" alt="">
                <img class="mini-map" src="./images/{}{}" alt="">
                <hr>
                "#, 
                map_file_name, 
                // 去掉外面一层
                map_info.map_or_else(|| "".to_string(), |v| format!("{:#?}", v)), 
                map_file_name, 
                MINI_MAP_FILES_EXTENSIONS_FILE_NAME,
                map_file_name, 
                CUSTOM_MINI_MAP_FILES_EXTENSIONS_FILE_NAME,
                map_file_name,
                PREVIEW_MAP_FILES_EXTENSIONS_FILE_NAME,
            )
        );
    }

    let file_content = format!(r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta http-equiv="X-UA-Compatible" content="IE=edge">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Document</title>
    <style>
        .mini-map {{
            display: inline-block;
            width: 300px;
            height: 300px;
        }}
    </style>
</head>
<body>
    <div style="margin: 50px;">{}</div>
</body>
</html>
    "#, content);
    let mut out_file = File::create(format!("{}/index.html", OUT_FILES_SAVE_DIRECTORY)).unwrap();
    // 输出到文件
    write!(out_file, "{}", file_content);
}

fn main() {
    let directory = "./";
    let mut map_infos: MapInfos = HashMap::new();

    // 删除原有的文件
    fs::remove_dir_all(OUT_FILES_SAVE_DIRECTORY);

    // 创建一些存储的路径
    fs::create_dir_all(OUT_FILES_SAVE_DIRECTORY);
    fs::create_dir_all(IMAGE_FILES_DIRECTORY);

    for entry in fs::read_dir(directory).unwrap() {
        let entry = entry.unwrap().path();
        // 只有这些格式才会尝试解析
        if entry.extension().is_some() && ["w3x"].contains(&entry.extension().unwrap().to_str().unwrap()) {
            let file_name = entry.file_name().unwrap().to_str().unwrap();
            // 无效mpq文件给None
            match Archive::open(format!("{}{}", directory, file_name)) {
                Ok(mut archive)=> {
                    let map_basic_info = get_map_basic_info(&mut archive);

                    let (mini_map, custom_mini_map, preview_map) = get_mini_maps(&mut archive);
                    // 处理小地图，在成功的时候保存
                    mini_map.map(|image| save_to_path(image, &format!("{}{}{}", IMAGE_FILES_DIRECTORY, file_name, MINI_MAP_FILES_EXTENSIONS_FILE_NAME)));

                    custom_mini_map.map(|image| save_to_path(image, &format!("{}{}{}", IMAGE_FILES_DIRECTORY, file_name, CUSTOM_MINI_MAP_FILES_EXTENSIONS_FILE_NAME)));

                    preview_map.map(|image| save_to_path(image, &format!("{}{}{}", IMAGE_FILES_DIRECTORY, file_name, PREVIEW_MAP_FILES_EXTENSIONS_FILE_NAME)));
                    
                    map_infos.insert(file_name.to_string(), get_map_basic_info(&mut archive))
                },
                Err(_)=> map_infos.insert(file_name.to_string(), None),
            };
        }
    }
    // 生成基本信息文件
    let mut out_file = File::create(format!("{}/map_info_analyze.txt", OUT_FILES_SAVE_DIRECTORY)).unwrap();
    // 格式化输出存到文件
    write!(out_file, "{:#?}", &map_infos);

    // 生成友好的可查看 html 文档
    generate_html_file(map_infos);
}