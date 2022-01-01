
<!-- README.md is generated from README.Rmd. Please edit that file -->

# georustr

<!-- badges: start -->
<!-- badges: end -->

This repo currently experimental code and a minimal R package to test
calling Rust code from R.

The code examples are all documented in this README to keep things
simple. To reproduce these examples (and to begin hacking R/Rust code!)
you will need to have installed:

1.  [Install the Rust
    toolchain](https://www.rust-lang.org/tools/install), e.g. with

``` bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

1.  [Install R](https://cran.r-project.org/)
2.  [Install the R package
    `sf`](https://rtask.thinkr.fr/installation-of-r-4-0-on-ubuntu-20-04-lts-and-tips-for-spatial-packages/)
3.  Install the R package `rextendr`, the development version of which
    can be installed with the following command:

``` r
remotes::install_github("extendr/rextendr")
#> Using github PAT from envvar GITHUB_PAT
#> Skipping install of 'rextendr' from a github remote, the SHA1 (bb6b9f1f) has not changed since last install.
#>   Use `force = TRUE` to force installation
```

After you have installed these things you can clone the repo and open
the director in an editor of your choice, e.g. with

``` bash
rstudio georustr/georustr.Rproj # open it in RStudio
# or...
code -r georustr/ # open it in VS Code
code -r georustr/src/rust # open the rust crate in VS Code
```

Rebuild it from the root directory with the following command from the R
command line:

``` r
rextendr::document()
#> ℹ Generating extendr wrapper functions for package: georustr.
#> ℹ 'R/extendr-wrappers.R' is up-to-date. Skip generating wrapper functions.
#> ℹ Updating georustr documentation
#> ℹ Loading georustr
#> Warning: [/mnt/57982e2a-2874-4246-a6fe-115c199bc6bd/orgs/robinlovelace/georustr/
#> R/csv_to_geojson.R:9] @examples requires a value
#> Writing NAMESPACE
#> Writing NAMESPACE
```

Load it with:

``` r
devtools::load_all()
#> ℹ Loading georustr
```

You can download the test data with the following command:

``` r
u = "https://github.com/Robinlovelace/georustr/releases/download/v0.0.0.9000/points.csv"
f = basename(u)
if(!file.exists(f)) {
  download.file(url = u, destfile = f)
}
```

After that the following should work:

``` r
n = 1e5
points_df = data.frame(x = rnorm(n = n), y = rnorm(n))
system.time({
  points_sf = sf::st_as_sf(points_df, coords = c("x", "y"), crs = 4326)
})
#>    user  system elapsed 
#>   0.037   0.008   0.046
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
#>   0.861   0.016   0.880
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
#>   0.281   0.048   0.330
file.exists("points_rust.geojson")
#> [1] TRUE
```

The results show that, for this simple test, **Rust is more than 2 times
faster than equivalent R/GDAL code**. Depending on your application,
much greater speed-ups should be possible but calling Rust code from R.

You can also run the code from the system command line:

``` bash
cd src/rust
cargo test --release
```

Verify the time taken to run as follows:

``` bash
time cargo test --release
```

Check the outputs are the same (they are):

``` r
sf::read_sf("points.geojson")
sf::read_sf("points_rust.geojson")
```
