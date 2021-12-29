
<!-- README.md is generated from README.Rmd. Please edit that file -->

# georustr

<!-- badges: start -->
<!-- badges: end -->

This is a minimal R package to test calling Rust code from R.

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
#>   0.045   0.000   0.045
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
#>   0.793   0.008   0.815
```

``` r
system.time({
  csv_to_geojson()
})
#>    user  system elapsed 
#>   0.862   5.762   6.656
```

Running that from the system shell resulted in:

    cargo test  2.69s user 7.47s system 99% cpu 10.177 total

``` r
system.time({
  points_georust = georustr::make_points(points_df)
})
```
