use rand::Rng; 

pub(crate) fn async_update(adj_mat: &Vec<Vec<isize>>, x: &mut Vec<usize>) {
    // Select node to update
    let mut rng = rand::thread_rng();
    let n = adj_mat.len();
    let node = rng.gen_range(0..n);
    // Find the iteration sum
    let mut sum:isize = 0;
    for i in 0..n {
        sum += adj_mat[node][i] * x[i] as isize;
    }
    // Update rule: change to 1 if sum positive, 0 if sum negative, leave unchanged if sum is 0
    if sum > 0 {
        x[node] = 1;
    } else if sum < 0 {
        x[node] = 0;
    }
}

pub(crate) fn bmodel_update(adj_mat: &Vec<Vec<isize>>, x: &Vec<usize>, n_iter: usize, update_fn: impl Fn(&Vec<Vec<isize>>, &mut Vec<usize>)) -> (Vec<usize>, bool) { 
    // Copy x 
    let mut x = x.clone();
    let mut conv = false;
    // Iterate n_iter times
    for _ in 0..n_iter {
        // Update x
        update_fn(adj_mat, &mut x);
        // Convergence check to be added
    }
    return (x, conv);
}