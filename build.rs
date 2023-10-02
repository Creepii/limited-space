fn main() {
    let mut files = Vec::new();
    if let Ok(directory) = std::fs::read_dir("assets/tilemap") {
        for file in directory {
            if let Ok(file) = file {
                if file.path().to_string_lossy().ends_with(".png") {
                    files.push(file.path().to_str().unwrap()[7..].to_string());
                }
            }
        }
    }
    let contents = files.join("\n");
    std::fs::write("src/tilemap_assets.txt", contents).unwrap();
}
