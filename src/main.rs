mod utils;

fn main() {
    // List all .topo files in the topodir
    let files = utils::list_topo_files("TOPO");
    // Print the list of .topo files
    for file in files {
        println!("{}", file);
    }
}
