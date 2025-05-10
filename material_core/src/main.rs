use std::fs;
use std::path::Path;
use material_lib::Material;

fn main() {
    let new_item = Material {
        model: "STM32L031G6U6".to_string(),
        brand: "ST".to_string(),
        package: "UFQFPN-28".to_string(),
        spec: "ARM Cortex-M0+ MCU 32KB FLASH".to_string(),
        code: "C96514".to_string(),
        quantity: 1,
    };

    let path = Path::new("material_core/data/material.json");
    let mut materials: Vec<Material>;

    if path.exists() {
        let metadata = fs::metadata(path).unwrap();
        if metadata.len() == 0 {
            // 空文件
            materials = Vec::new();
        } else {
            // 正常解析
            let file = fs::File::open(path).unwrap();
            let reader = std::io::BufReader::new(file);
            materials = serde_json::from_reader(reader).unwrap();
        }
    } else {
        materials = Vec::new();
    }

    materials.push(new_item);

    let file = fs::File::create(path).unwrap();
    let writer = std::io::BufWriter::new(file);
    serde_json::to_writer_pretty(writer, &materials).unwrap();

    println!("成功！");
    
    let key_word = "MCU";
    
    let path = Path::new("material_core/data/material.json");
    let file = fs::File::open(path).expect("Could not open file");
    let reader = std::io::BufReader::new(file);
    let materials: Vec<Material> = serde_json::from_reader(reader).expect("Could not parse JSON");

    println!("包含关键字\"{}\"的列表", key_word);
    for item in materials.iter().filter(|m| m.spec.contains(key_word)){
        println!("型号: {:<20}, 数量: {}", item.model, item.quantity);
    }
}
