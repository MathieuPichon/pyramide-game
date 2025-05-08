mod naive_pyra;

pub use crate::naive_pyra::*;
mod dyn_pyra;
use dyn_pyra::PyramideRules;
use dyn_pyra::test_dyn_graph;

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

    test_dyn_graph(PyramideRules{lines:4, diag_allowed: true});
}
