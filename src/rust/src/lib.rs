use extendr_api::prelude::*;
use polars::prelude::{Int32Chunked, NewChunkedArray, TakeRandom};

#[derive(Debug, Clone)]
struct AltInt32Chunked(Int32Chunked);

impl AltrepImpl for AltInt32Chunked {
    fn length(&self) -> usize {
        self.0.len()
    }
}

impl AltIntegerImpl for AltInt32Chunked {
    fn elt(&self, index: usize) -> i32 {
        if let Some(i) = self.0.get(index) {
            i
        } else {
            // TODO: can we return NA?
            unimplemented!()
        }
    }
}

/// @export
#[extendr(use_try_from = true)]
fn as_in32chunked(x: &[i32]) -> Robj {
    let x_chunked = Int32Chunked::new_from_slice("slice", x);
    let state = AltInt32Chunked(x_chunked);
    let class = Altrep::make_altinteger_class::<AltInt32Chunked>("int32_chunked", "polarsr");
    Altrep::from_state_and_class(state, class, false).into()
}

// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod polarsr;
    fn as_in32chunked;
}
