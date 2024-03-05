use rand::Rng; 

pub(crate) fn rand_init(n: usize) -> Vec<usize> {
    // Generate a random initial state of n nodes
    let mut rng = rand::thread_rng();
    let mut vec: Vec<usize> = Vec::new();
    for _ in 0..n {
        vec.push(rng.gen_range(0..2));
    }
    return vec
}

pub(crate) fn read_init(file: &str) -> Vec<usize> {
    // Read the initial state from a file
    let mut vec: Vec<usize> = Vec::new();
    let contents = std::fs::read_to_string(file).unwrap();
    for line in contents.lines() {
        vec.push(line.parse().unwrap());
    }
    return vec
}