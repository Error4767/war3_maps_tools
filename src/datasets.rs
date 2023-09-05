use std::collections::HashMap;
use lazy_static::lazy_static;

lazy_static! {
    // 地形
    static ref TILESET_LIST: HashMap<&'static str, [&'static str; 2]> = HashMap::from([
        ("A", ["白杨谷", "Ashenvale"]),
        ("B", ["荒芜之地", "Barrens"]),
        ("C", ["费尔伍德", "Felwood"]),
        ("D", ["地牢", "Dungeon"]),
        ("F", ["洛丹伦(秋)", "Lordaeron Fall"]),
        ("G", ["地下城", "Underground"]),
        ("I", ["冰封王座", "Icecrown"]),
        ("J", ["达拉然的废墟", "Dalaran Ruins"]),
        ("K", ["黑色城堡", "Black Citadel"]),
        ("L", ["洛丹伦(夏)", "Lordaeron Summer"]),
        ("N", ["诺森德", "Northrend"]),
        ("O", ["边缘之地", "Outland"]),
        ("Q", ["村庄(秋)", "Village Fall"]),
        ("v", ["村庄", "Village"]),
        ("W", ["洛丹伦(冬)", "Lordaeron Winter"]),
        ("X", ["达拉然", "Dalaran"]),
        ("Y", ["城邦", "Cityscape"]),
        ("Z", ["沉沦的废墟", "Sunken Ruins"]),
    ]);
    // 尺寸描述
    // Tiny - from 45 to 7500
    // 极小，从45到7500

    // Small - from 7501 to 13500
    // 小，从7501到13500

    // Medium - from 13501 to 22000
    // 中等-从13501年到2000年

    // Large - from 22001 to 32500
    // 大型-由2001年至32500年

    // Huge (TFT) - from 32501 to 45000
    // 巨大(TFT)-从32501到45000

    // Epic (TFT) - from 45001 to 62997
    // 史诗(TFT)-从45001到62997
    static ref SIZE_DESCRIPTION: [(i32, [&'static str; 2]); 7] = [
        (45, ["极小的", "Tiny"]),
        (7501, ["小的", "Small"]),
        (13501, ["中等的", "Medium"]),
        (22001, ["大的", "Large"]),
        (32501, ["庞大的", "Huge"]),
        (45001, ["壮丽的", "Epic"]),
        // 这个是实测出的，大于 316 * 316 可用是未知, 所以是 99856 + 1
        (99857, ["未知的", "Unknown"]),
    ];
    static ref SIZE_DESCRIPTION_MORE_THAN_THE_1_29_VERSION: [(i32, [&'static str; 2]); 9] = [
        (45, ["极小的", "Tiny"]),
        (6001, ["特小的", "Extra Small"]),
        (12801, ["小的", "Small"]),
        (21001, ["中等的", "Medium"]),
        (31001, ["大的", "Large"]),
        (43501, ["特大的", "Extra Large"]),
        // 这个是实测出的，大于 316 * 316 可用是未知, 所以是 99856 + 1
        (74001, ["庞大的", "Huge"]),
        (135001, ["壮丽的", "Epic"]),
        (215001, ["传奇的", "Legendary"]),
    ];
}

pub fn get_tileset_text(tileset: &str) -> &'static str {
    // 如果没有就给空
    TILESET_LIST.get(tileset).unwrap_or_else(|| &["", ""])[0]
}

// 获取尺寸描述
pub fn get_size_description(size: i32) -> &'static str {
    let mut description: &'static str = "";
    SIZE_DESCRIPTION.iter().find(|(size_standard, [description_standard, _])| {
        if size_standard > &size {
            true
        } else {
            description = description_standard;
            false
        }
    });
    description
}

// 获取尺寸描述 >= 1.29
pub fn get_size_description_more_than_1_29_version(size: i32) -> &'static str {
    let mut description: &'static str = "";
    SIZE_DESCRIPTION_MORE_THAN_THE_1_29_VERSION.iter().find(|(size_standard, [description_standard, _])| {
        if size_standard > &size {
            true
        } else {
            description = description_standard;
            false
        }
    });
    description
}