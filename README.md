
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
n = 1e6
points_df = data.frame(x = rnorm(n = n), y = rnorm(n))
system.time({
  points_sf = sf::st_as_sf(points_df, coords = c("x", "y"), crs = 4326)
})
#>    user  system elapsed 
#>   0.694   0.044   0.738
```

We can do the full csv to geojson process for a fair test as follows:

``` r
write.csv(points_df, "points.csv")
system.time({
  csv_to_json_base_r(file_csv = "points.csv")
})
#> Writing output to points.geojson
#> Writing layer `points' to data source `points.geojson' using driver `GeoJSON'
#> Writing 1000000 features with 1 fields and geometry type Point.
#>    user  system elapsed 
#>  10.042   0.160  10.217
```

``` r
system.time({
  csv_to_geojson()
})
```

``` r
system.time({
  points_georust = georustr::make_points(points_df)
})
```
