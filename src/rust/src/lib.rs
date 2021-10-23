use extendr_api::prelude::*;
use polars::prelude::{Int32Chunked, NewChunkedArray, TakeRandom};

// TODO: Debug should be implemented by ourselves to make the result of
// `.Internal(inspect(x))` nicer.
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
            // TODO: can we return NA more nicely?
            Rint::na().inner()
        }
    }
}

/// @export
#[extendr(use_try_from = true)]
fn as_in32chunked(x: Integers) -> Robj {
    let x_chunked = Int32Chunked::new_from_iter(
        "int32_chunked",
        x.iter().map(|i| {
            if i.is_na() {
                // TODO: handle NA properly
                unimplemented!()
            } else {
                i.inner()
            }
        }),
    );

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
