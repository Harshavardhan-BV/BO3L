pub(crate) fn list_topo_files(topodir: &str) -> Vec<String> {
    let mut files: Vec<String> = Vec::new();
    // Read the directory
    let paths = std::fs::read_dir(topodir).unwrap();
    for path in paths {
        // Get the file name
        let path = path.unwrap().file_name();
        let path_str = path.to_str().unwrap().to_string();
        // If the file is a .topo file, add it to the list
        if path_str.ends_with(".topo") {
            files.push(path_str);
        }
    }
    files
}