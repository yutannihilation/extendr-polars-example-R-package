use extendr_api::prelude::*;
use polars::prelude::{DataFrame, Int32Chunked, NewChunkedArray, TakeRandom};

// TODO: Debug should be implemented by ourselves to make the result of
// `.Internal(inspect(x))` nicer.
#[derive(Debug, Clone)]
struct AltInt32Chunked(Int32Chunked);

impl AltInt32Chunked {
    fn new(x: Int32Chunked) -> Self {
        Self(x)
    }

    fn make_altrep(self) -> Altrep {
        let class = Altrep::make_altinteger_class::<AltInt32Chunked>("int32_chunked", "polarsr");
        Altrep::from_state_and_class(self, class, false)
    }
}

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

struct PolarsrDataFrame(DataFrame);

#[extendr]
impl PolarsrDataFrame {
    fn column(&self, name: &str) -> Robj {
        if let Ok(series) = self.0.column(name) {
            match series.dtype() {
                polars::prelude::DataType::Int32 => {
                    // TODO: Altrep seems to require static lifetime and I
                    // couldn't figure out how to take the reference instead of
                    // cloning here.
                    let int32_chunked = series.i32().unwrap().clone();
                    AltInt32Chunked::new(int32_chunked).make_altrep().into()
                }
                polars::prelude::DataType::Null => NULL.into(),
                _ => todo!(),
            }
        } else {
            NULL.into()
        }
    }
}

fn as_int32chunked_inner(x: Integers, name: &str) -> Int32Chunked {
    Int32Chunked::new_from_iter(
        name,
        x.iter().map(|i| {
            if i.is_na() {
                // TODO: handle NA properly
                unimplemented!()
            } else {
                i.inner()
            }
        }),
    )
}

/// @export
#[extendr(use_try_from = true)]
fn as_int32chunked(x: Integers, name: &str) -> Robj {
    let x_chunked = as_int32chunked_inner(x, name);

    AltInt32Chunked::new(x_chunked).make_altrep().into()
}

/// @export
#[extendr(use_try_from = true)]
fn as_polar_dataframe(x: List) -> PolarsrDataFrame {
    // If x is not a data.frame, return NULL
    if !x.is_frame() {
        panic!("Not a data.frame!");
    }

    let df: DataFrame = x
        .iter()
        .map(|(nm, col)| {
            if let Some(i) = col.as_integers() {
                as_int32chunked_inner(i, nm).into()
            } else {
                unimplemented!()
            }
        })
        .collect();

    PolarsrDataFrame(df)
}

// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod polarsr;
    impl PolarsrDataFrame;
    fn as_int32chunked;
    fn as_polar_dataframe;
}
