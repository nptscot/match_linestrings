import * as fs from "fs";

let gj = JSON.parse(fs.readFileSync(process.argv[2], { encoding: "utf8" }));
for (let [idx, f] of gj.features.entries()) {
  f.id = idx;
  //f.properties = {};
  f.properties = {
    matching_sources: f.properties.matching_sources,
    reviewed: f.properties.reviewed,
  };
}
fs.writeFileSync("out", JSON.stringify(gj));
