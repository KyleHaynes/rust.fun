
rextendr::document()

geojson_file <- "C:/Users/kyleh/GitHub/rust.fun/meuse.geojson"  # Convert your shapefile to GeoJSON
geojson_file <- "C:/Users/kyleh/GitHub/rust.fun/qld.geojson"  # Convert your shapefile to GeoJSON
# geojson_file <- "c:/temp/suburb-10-nsw.geojson"  # Convert your shapefile to GeoJSON
generate_lat_long <- function(n = 100) {
  # Define latitude and longitude bounds roughly covering Australia
  lat_min <- -44.0  # Southernmost point (Tasmania)
  lat_max <- -10.0  # Northernmost point (Top End)
  lon_min <- 112.0  # Westernmost point
  lon_max <- 154.0  # Easternmost point

  # Generate random latitudes and longitudes
  lat <- runif(n, lat_min, lat_max)
  lon <- runif(n, lon_min, lon_max)

  # Return as a data frame
  data.frame(lat = lat, lon = lon)
}

generate_qld_coords <- function(n) {
  set.seed(123) # For reproducibility
  
  # Approximate bounding box for Queensland
  min_lat <- -29.0  # Southernmost point
  max_lat <- -10.5  # Northernmost point
  min_lon <- 138.0  # Westernmost point
  max_lon <- 154.0  # Easternmost point

  # Generate n random latitudes and longitudes within the bounding box
  lat <- runif(n, min_lat, max_lat)
  lon <- runif(n, min_lon, max_lon)
  
  # Return as a data frame
  data.frame(latitude = lat, longitude = lon)
}

# Example usage: Generate 10 random coordinates in QLD
qld_coords <- generate_qld_coords(1E6)


# Example: Generate 10 random points
set.seed(42)  # For reproducibility
d = generate_lat_long(1E6)


# Assign points to polygons
system.time(x <- assign_points_to_polygons(geojson_file, d$lat, d$lon, property_name = "SA2_NAME21"), gcFirst = F)
# Australia wide
#     user   system  elapsed 
# 18808.50   108.07  1392.28
# QLD: ~65 seconds


# Assign points to polygons (that are more likely in qld)
system.time(x <- assign_points_to_polygons(geojson_file, qld_coords$latitude, qld_coords$longitude, property_name = "SA2_NAME21"), gcFirst = F)
# Extracted 1680 polygons from GeoJSON.
#    user  system elapsed 
#  907.71    0.89   58.24


# Print results
print(x)


# ---- Exports Shapefile as GeoJson ----

library(sf)
nc <- read_sf("C:\\SA2_2021_AUST_SHP_GDA2020\\SA2_2021_AUST_GDA2020.shp")
meuse_sf = st_as_sf(nc, coords = c("x", "y"), crs = 28992, agr = "constant")
st_write(nc, "meuse.geojson")

# and qld
require(data.table)
nc[nc$SA2_NAME21 %plike% "Beaudes", ]
nc <- nc[nc$SA4_CODE21 %plike% "^3", ]
meuse_sf = st_as_sf(nc, coords = c("x", "y"), crs = 28992, agr = "constant")
st_write(nc, "qld.geojson")


# ---- Sample random lat longs in a geojson file ----
qld <- generate_random_lat_longs(geojson_file, 3E4)
system.time(x <- assign_points_to_polygons(geojson_file, qld$lat, qld$lon, property_name = "SA2_NAME21"), gcFirst = F)
