mod utils;
mod inits;
mod simulate;

fn main() {
    // inputs (TBD read from command line args)
    let topodir = "TOPO";
    let n_iter = 1000;
    let n_init = 100000;
    let mode = "Async";
    // Test case toggle switch 
    let n = 2;
    let adj_mat = vec![vec![0, -1], vec![-1, 0]];
    println!("Adjacency Matrix {:?}", adj_mat);
    let mut x = inits::rand_init(n);
    let conv:bool;
    println!("Initial {:?}", x);
    (x, conv) = simulate::bmodel_update(&adj_mat, &x, n_iter, simulate::async_update);
    println!("Final converged {:?} {:?}", x, conv);
}
