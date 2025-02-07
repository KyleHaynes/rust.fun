
rextendr::document()

geojson_file <- "C:/Users/kyleh/GitHub/rust.fun/meuse.geojson"  # Convert your shapefile to GeoJSON
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

# Example: Generate 10 random points
set.seed(42)  # For reproducibility
d = generate_lat_long(1E6)


# Assign points to polygons
system.time(x <- assign_points_to_polygons(geojson_file, d$lat, d$lon, property_name = "SA2_NAME21"), gcFirst = F)

# Print results
print(x)


# install.packages("sf")
# install.packages("geojsonsf")
install.packages("geojsonsf")
library(geojsonio)

# Load the exported GeoJSON
geojson <- geojson_read("output.geojson", what = "sp")

# Define custom CRS (if needed)
custom_crs <- list(
  "type" = "name",
  "properties" = list(
    "name" = "urn:ogc:def:crs:EPSG::7844"
  )
)

# Write GeoJSON with custom CRS
geojson_write(geojson, file = "output_with_crs.geojson", crs = custom_crs)

# Check the saved GeoJSON with custom CRS
print("GeoJSON with custom CRS saved.")



library(sf)
nc <- read_sf("C:\\SA2_2021_AUST_SHP_GDA2020\\SA2_2021_AUST_GDA2020.shp")
meuse_sf = st_as_sf(nc, coords = c("x", "y"), crs = 28992, agr = "constant")
st_write(nc, "meuse.geojson")


data(meuse)
coordinates(meuse) = c("x", "y")
class(meuse)
# [1] "SpatialPointsDataFrame"
writeOGR(meuse, "test_geojson", layer="meuse", driver="GeoJSON")

nc <- nc %>% st_transform(4326)

nc <- sf_geojson(nc, simplify = FALSE)

nc <- geojson_sf(nc)

df_hex = st_as_sf(nc)                                                                                                                 
st_write(nc, "df_hex.geojson") 
