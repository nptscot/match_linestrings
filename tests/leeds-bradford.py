# Note: I failed to get this working so switched to R

import requests
import osmtogeojson

# Define the relation ID
relation_id = 6386965

# Overpass API query
query = f"""
[out:json];
relation({relation_id});
(._;>;);
out geom;
"""

# Fetch data from Overpass API
response = requests.get(
    "https://overpass-api.de/api/interpreter",
    params={'data': query}
)

if response.status_code == 200:
    # Convert the OSM data to GeoJSON
    osm_data = response.json()
    geojson = osmtogeojson.process_osm_json(osm_data)

    # Save GeoJSON to a file
    output_file = "relation_6386965.geojson"
    with open(output_file, 'w', encoding='utf-8') as f:
        json.dump(geojson, f, ensure_ascii=False, indent=4)

    print(f"GeoJSON file saved as '{output_file}'")
else:
    print(f"Failed to fetch data from Overpass API. Status code: {response.status_code}")
