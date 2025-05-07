mod naive_pyra;
pub use crate::naive_pyra::*;
mod dyn_pyra;
pub use crate::dyn_pyra::test_dyn_graph;

fn main() {

    if false {
        brute_force_search();
    }

    if false {
        test_graph_from_start_pos();
    }

    if false {
        test_full_graph();
    }

    test_dyn_graph();
}
