
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
#>   0.045   0.000   0.045
```

We can do the full csv to geojson process for a fair test as follows:

``` r
# run once
readr::write_csv(points_df, "points.csv")
```

``` r
if(file.exists("points.geojson")) file.remove("points.geojson")
#> [1] TRUE
system.time({
  csv_to_json_base_r(file_csv = "points.csv")
})
#> Writing output to points.geojson
#> Writing layer `points' to data source `points.geojson' using driver `GeoJSON'
#> Writing 100000 features with 0 fields and geometry type Point.
#>    user  system elapsed 
#>   0.661   0.020   0.681
file.exists("points.geojson")
#> [1] TRUE
```

``` r
if(file.exists("points_rust.geojson")) {
  file.remove("points_rust.geojson")
}
#> [1] TRUE
system.time({
  csv_to_geojson_rust()
})
#>    user  system elapsed 
#>   0.233   0.036   0.269
file.exists("points_rust.geojson")
#> [1] TRUE
```

You can also run the code from the system command line:

``` bash
cd src/rust
cargo test --release
```

Verify the time taken to run as follows:

``` bash
time cargo test --release
```

Check the output:

``` r
sf::read_sf("points.geojson")
#> Simple feature collection with 100000 features and 0 fields
#> Geometry type: POINT
#> Dimension:     XY
#> Bounding box:  xmin: -4.426115 ymin: -4.581759 xmax: 4.28053 ymax: 4.358714
#> Geodetic CRS:  WGS 84
#> # A tibble: 100,000 × 1
#>                   geometry
#>                <POINT [°]>
#>  1   (-0.183681 0.3248124)
#>  2    (-2.24645 -0.553099)
#>  3  (-0.5932318 0.3407814)
#>  4  (-0.2102984 0.6505756)
#>  5 (-0.9501803 0.08423856)
#>  6   (0.1042869 -1.845679)
#>  7   (-2.037787 -1.900653)
#>  8   (0.7674608 0.9088685)
#>  9  (-0.3537348 0.8850982)
#> 10   (0.1912675 -1.416043)
#> # … with 99,990 more rows
sf::read_sf("points_rust.geojson")
#> Simple feature collection with 100000 features and 0 fields
#> Geometry type: POINT
#> Dimension:     XY
#> Bounding box:  xmin: -4.426115 ymin: -4.581759 xmax: 4.28053 ymax: 4.358714
#> Geodetic CRS:  WGS 84
#> # A tibble: 100,000 × 1
#>                   geometry
#>                <POINT [°]>
#>  1   (-0.183681 0.3248124)
#>  2    (-2.24645 -0.553099)
#>  3  (-0.5932318 0.3407814)
#>  4  (-0.2102984 0.6505756)
#>  5 (-0.9501803 0.08423856)
#>  6   (0.1042869 -1.845679)
#>  7   (-2.037787 -1.900653)
#>  8   (0.7674608 0.9088685)
#>  9  (-0.3537348 0.8850982)
#> 10   (0.1912675 -1.416043)
#> # … with 99,990 more rows
```

``` r

system.time({
  points_georust = georustr::make_points(points_df)
})
```
