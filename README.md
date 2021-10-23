
<!-- README.md is generated from README.Rmd. Please edit that file -->

# An example R package to use [Polars](https://github.com/pola-rs/polars) via [extendr](https://extendr.github.io/)

**This repository is just for a quick proof of concept**

<!-- badges: start -->
<!-- badges: end -->

## Installation

You can install the development version of polarsr from
[GitHub](https://github.com/) with:

``` r
# install.packages("devtools")
devtools::install_github("yutannihilation/extendr-polars-example-R-package")
```

## Example

### `Series`

``` r
library(polarsr)

x <- as_int32chunked(1:10, "foo")
x
#>  [1]  1  2  3  4  5  6  7  8  9 10

.Internal(inspect(x))
#> @5638f35a5e38 13 INTSXP g0c0 [MARK,REF(65535)] AltInt32Chunked(shape: (10,)
#> ChunkedArray: 'foo' [Int32]
#> [
#>  1
#>  2
#>  3
#>  4
#>  5
#>  6
#>  7
#>  8
#>  9
#>  10
#> ])

# works
x[3]
#> [1] 3

# won't work yet
x[2:3]
#> NULL

# Do not execute this, this crashes the R session!
# x[3] <- 3
```

### `DataFrame`

``` r
pdf <- as_polar_dataframe(data.frame(x = 1:10, y = 11:20))
pdf
#> <pointer: 0x5638f165d350>
#> attr(,"class")
#> [1] "PolarsrDataFrame"

# This returns an Altrep
pdf$column("x")
#>  [1]  1  2  3  4  5  6  7  8  9 10
.Internal(inspect(pdf$column("x")))
#> @5638f14f53c8 13 INTSXP g0c0 [REF(65535)] AltInt32Chunked(shape: (10,)
#> ChunkedArray: 'x' [Int32]
#> [
#>  1
#>  2
#>  3
#>  4
#>  5
#>  6
#>  7
#>  8
#>  9
#>  10
#> ])

# non-existent column
pdf$column("foo")
#> NULL
```
