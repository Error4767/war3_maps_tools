use std::collections::HashMap;

use std::{ fs, str };

// 解析完的结构
pub type ParsedVariables = HashMap<String, String>;

// 解析 wts 内容
pub fn parse_to_hash_map<'a>(file_content: String) -> ParsedVariables {

    // let file_content = fs::read_to_string("./out/war3map.wts").unwrap();

    let mut parsed: ParsedVariables = HashMap::new();

    // 没有注释的主体内容
    let mut none_comment_content = String::from("");

    // 以行分割
    let lines: Vec<&str> = file_content.split("\n").collect();
    // 遍历所有行做动作
    for line in lines.iter() {
        // 去头尾空白
        let trimed_line = line.trim();

        // 转换为 Vec<char> 防止直接截取，特殊字符占多个位置的问题
        let line_chars = trimed_line.chars().collect::<Vec<char>>();
        // 如果不是注释, 就追加
        if !(line_chars.len() >= 2 && &line_chars[0..2] == ['/', '/']) {
            none_comment_content.push_str(&format!("{}\n", trimed_line));
        }
    }

    // 无注释部分以行分割, 根据 STRING 分割，每个 STRING 声明了一个变量
    let splitted: Vec<&str> = none_comment_content.split("STRING").collect::<Vec<&str>>();

    for splitted_str in splitted.iter() {
        // 如果分割的部分可以找到 { ,做动作
        if let Some(index) = splitted_str.find("{") {
            // 变量名
            let variable_name = &splitted_str[0..index].trim();
            // 如果找不到，截取 { 到最后
            let variable_value = &splitted_str[(index + 1)..splitted_str.rfind("}").unwrap_or_else(|| splitted_str.len())].trim();
            parsed.insert(variable_name.to_string(), variable_value.to_string());
        }
    }

    parsed
}