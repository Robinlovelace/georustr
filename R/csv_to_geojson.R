#' Convert csv file to geojson file
#'
#' @param file_csv 
#' @param file_geojson 
#'
#' @return
#' @export
#'
#' @examples
csv_to_json_base_r = function(file_csv, file_geojson = NULL) {
  if(is.null(file_geojson)) {
    file_geojson = paste0(gsub(pattern = ".csv", replacement = ".geojson", x = file_csv))
  }
  d = read.csv(file_csv)
  d_sf = sf::st_as_sf(d, coords = c("x", "y"), crs = 4326)
  message("Writing output to ", file_geojson)
  sf::st_write(d_sf, file_geojson)
}