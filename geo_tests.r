

geojson_file <- "C:/Users/kyleh/GitHub/rust.fun/df_hex.geojson"  # Convert your shapefile to GeoJSON
latitudes <- c(-33.8688, -33.8788, -27.4698)  # Example latitudes
longitudes <- c(151.2093, 151.2022, 153.0251) # Example longitudes

# Assign points to polygons
polygon_indices <- assign_points_to_polygons(geojson_file, latitudes, longitudes)

# Print results
print(polygon_indices)


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
library(geojsonsf)

nc <- read_sf("C:\\SA2_2021_AUST_SHP_GDA2020\\SA2_2021_AUST_GDA2020.shp")

nc <- nc %>% st_transform(4326)

nc <- sf_geojson(nc, simplify = FALSE)

nc <- geojson_sf(nc)

df_hex = st_as_sf(nc)                                                                                                                 
st_write(nc, "df_hex.geojson") 
