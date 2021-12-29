
<!-- README.md is generated from README.Rmd. Please edit that file -->

# georustr

<!-- badges: start -->
<!-- badges: end -->

This is a minimal R package to test calling Rust code from R.

Rebuild it with:

``` r
rextendr::document()
#> ℹ Generating extendr wrapper functions for package: georustr.
#> ℹ 'R/extendr-wrappers.R' is up-to-date. Skip generating wrapper functions.
#> ℹ Updating georustr documentation
#> ℹ Loading georustr
#> Warning: [/home/robin/learning/rust/georustr/R/csv_to_geojson.R:9] @examples
#> requires a value
#> Writing NAMESPACE
#> Writing NAMESPACE
```

Load it with:

``` r
devtools::load_all()
#> ℹ Loading georustr
```

When it is complete, the following should work:

``` r
n = 1e5
points_df = data.frame(x = rnorm(n = n), y = rnorm(n))
system.time({
  points_sf = sf::st_as_sf(points_df, coords = c("x", "y"), crs = 4326)
})
#>    user  system elapsed 
#>   0.034   0.000   0.035
```

We can do the full csv to geojson process for a fair test as follows:

``` r
readr::write_csv(points_df, "points.csv")
if(file.exists("points.geojson")) file.remove("points.geojson")
#> [1] TRUE
system.time({
  csv_to_json_base_r(file_csv = "points.csv")
})
#> Writing output to points.geojson
#> Writing layer `points' to data source `points.geojson' using driver `GeoJSON'
#> Writing 100000 features with 0 fields and geometry type Point.
#>    user  system elapsed 
#>   0.645   0.048   0.693
```

``` r
system.time({
  csv_to_geojson()
})
#>    user  system elapsed 
#>   1.558   9.666  11.245
```

Running that from the system shell resulted in:

    cargo test  2.69s user 7.47s system 99% cpu 10.177 total

``` r
system.time({
  points_georust = georustr::make_points(points_df)
})
```
