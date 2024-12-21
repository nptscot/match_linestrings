library(tidyverse)
leeds_transport_network = osmactive::get_travel_network("Leeds")
# leeds_relations = osmextract::oe_get("Leeds", layer = "other_relations")
# summary(leeds_relations$osm_id %in% "6386965")

library(osmdata)
q = opq("Leeds") %>%
  add_osm_feature(key = "name", value = "Cycle Superhighway 1") %>%
  osmdata_sf()
lines = q$osm_lines
lines = lines[c("osm_id", "name", "highway")] |>
  sf::st_cast("LINESTRING")
plot(lines)
sf::write_sf(lines, "leeds_bradford_cycle_superhighway_linestrings.geojson", delete_dsn = TRUE)
system("gh release upload v0.1 leeds_bradford_cycle_superhighway_linestrings.geojson --clobber")
lines_100m_buffer = sf::st_buffer(sf::st_union(lines), 100)
plot(lines_100m_buffer)
leeds_transport_network_near_superhighway = leeds_transport_network |>
  sf::st_filter(lines_100m_buffer) |>
  filter(! osm_id %in% lines$osm_id) |>
  sf::st_cast("LINESTRING")
plot(leeds_transport_network_near_superhighway)
sf::write_sf(leeds_transport_network_near_superhighway, "leeds_transport_network_near_superhighway.geojson", delete_dsn = TRUE)
system("gh release upload v0.1 leeds_transport_network_near_superhighway.geojson --clobber")

#  The code above downloads the Leeds transport network and then filters it to only include the parts that are within 100m of the Leeds-Bradford Cycle Superhighway. 
#  The code below then calculates the shortest path between two points on the Leeds transport network near the Cycle Superhighway. 
 library(sf)