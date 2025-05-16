use material_lib::Material;

pub fn search_by_keyword(materials: &[Material], keyword:&str){
    for item in materials.iter().filter(|m| m.spec.contains(keyword) || m.brand.contains(keyword)) {
        println!("型号: {:<20}, 数量: {}", item.model, item.quantity);
    }
}
