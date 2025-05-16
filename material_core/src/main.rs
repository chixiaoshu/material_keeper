mod file_operations;
mod material_db;

use std::fs;
use std::path::Path;
use material_lib::Material;
use crate::file_operations::{read_material_from_file, write_material_to_file};
use crate::material_db::search_by_keyword;

fn main() {
    let new_item = Material {
        model: "STM32F407VET6".to_string(),
        brand: "ST".to_string(),
        package: "LQFP-100".to_string(),
        code: "C2".to_string(),
        spec: "ARM Cortex-M4 MCU 512KB FLASH".to_string(),
        quantity: 1,
    };

    let path = Path::new("material_core/data/material.json");
    let mut materials = read_material_from_file(path).unwrap_or_else(|_| Vec::new());
    let mut found = false;
    // 物料的名称或编号若有其一相同则只增加数量
    for item in &mut materials {
                if item.model == new_item.model 
                || item.code == new_item.code
                {
                    found = true;
                    item.quantity += new_item.quantity;
                    break;
                }
            }
            
            // 如果没有找到已有物料
            if !found {
                materials.push(new_item);
            }

    write_material_to_file(path, &materials).unwrap();

    println!("成功！");
    
    let key_word = "ST";
    
    let path = Path::new("material_core/data/material.json");
    let file = fs::File::open(path).expect("Could not open file");
    let reader = std::io::BufReader::new(file);
    let materials: Vec<Material> = serde_json::from_reader(reader).expect("Could not parse JSON");

    println!("包含关键字\"{}\"的列表", key_word);
    search_by_keyword(&materials, key_word);
}
