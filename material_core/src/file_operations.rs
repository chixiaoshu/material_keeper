use std::fs;
use std::io::{self, BufReader, BufWriter};
use std::path::Path;
use material_lib::Material;

pub fn read_material_from_file(path: &Path) -> io::Result<Vec<Material>> {
    if path.exists() {
        let file = fs::File::open(path)?;
        let reader = BufReader::new(file);
        let materials: Vec<Material> = serde_json::from_reader(reader)?;
        Ok(materials)
    }
    else {
        Ok(Vec::new())
    }
}

pub fn write_material_to_file(path: &Path, materials: &[Material]) -> io::Result<()> {
    let file = fs::File::create(path)?;
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, materials)?;
    Ok(())
}
