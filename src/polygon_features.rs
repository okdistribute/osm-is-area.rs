pub struct PolygonFeature {
    pub key: &'static str,
    pub polygon: &'static str,
    pub values: Vec<&'static str>
}

impl PolygonFeature {
    fn new (key: &'static str, polygon: &'static str, values: Option<Vec<&'static str>>) -> PolygonFeature {

        let mut parsed_values = vec![];
        if values.is_some() {
            parsed_values = values.unwrap()
        };

        return PolygonFeature {
            key: key,
            polygon: polygon,
            values: parsed_values
        };
    }
}

pub fn get_polygon_features () -> Vec<PolygonFeature> {
    return vec![
        PolygonFeature::new(
            "building",
            "all",
            None
        ),
        PolygonFeature::new(
            "highway",
            "whitelist",
            Some(vec![
                "services",
                "rest_area",
                "escape",
                "elevator"
            ])
        ),
        PolygonFeature::new(
            "natural",
            "blacklist",
            Some(vec![
                "coastline",
                "cliff",
                "ridge",
                "arete",
                "tree_row"
            ])
        ),
        PolygonFeature::new(
            "landuse",
            "all", None
        ),
        PolygonFeature::new(
            "waterway",
            "whitelist",
            Some(vec![
                "riverbank",
                "dock",
                "boatyard",
                "dam"
            ])
        ),
        PolygonFeature::new(
            "amenity",
            "all", None
        ),
        PolygonFeature::new(
            "leisure",
            "all", None
        ),
        PolygonFeature::new(
            "barrier",
            "whitelist",
            Some(vec![
                "city_wall",
                "ditch",
                "hedge",
                "retaining_wall",
                "wall",
                "spikes"
            ])
        ),
        PolygonFeature::new(
            "railway",
            "whitelist",
            Some(vec![
                "station",
                "turntable",
                "roundhouse",
                "platform"
            ])
        ),
        PolygonFeature::new(
            "area",
            "all", None
        ),
        PolygonFeature::new(
            "boundary",
            "all", None
        ),
        PolygonFeature::new(
            "man_made",
            "blacklist",
            Some(vec![
                "cutline",
                "embankment",
                "pipeline"
            ])
        ),
        PolygonFeature::new(
            "power",
            "whitelist",
            Some(vec![
                "plant",
                "substation",
                "generator",
                "transformer"
            ])
        ),
        PolygonFeature::new(
            "place",
            "all", None
        ),
        PolygonFeature::new(
            "shop",
            "all", None
        ),
        PolygonFeature::new(
            "aeroway",
            "blacklist",
            Some(vec![
                 "taxiway"
            ])
        ),
        PolygonFeature::new(
            "tourism",
            "all", None
        ),
        PolygonFeature::new(
            "historic",
            "all", None
        ),
        PolygonFeature::new(
            "public_transport",
            "all", None
        ),
        PolygonFeature::new(
            "office",
            "all", None
        ),
        PolygonFeature::new(
            "building:part",
            "all", None
        ),
        PolygonFeature::new(
            "military",
            "all", None
        ),
        PolygonFeature::new(
            "ruins",
            "all", None
        ),
        PolygonFeature::new(
            "area:highway",
            "all", None
        ),
        PolygonFeature::new(
            "craft",
            "all", None
        ),
        PolygonFeature::new(
            "golf",
            "all", None
        ),
        PolygonFeature::new(
            "indoor",
            "all", None
        )
    ];
}
