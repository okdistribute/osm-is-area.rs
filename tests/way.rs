use osm_is_area;

#[test]
fn way_circular_refs_no_way_tag () {
    let end = 1252234;
    let refs = vec![end, 23452234, 28373423, end];
    let tags = vec![
        (r"waterway", r"custom")
    ];
    assert_eq!(false, osm_is_area::way(&tags, &refs));
}

#[test]
fn way_area_no_tag () {
    let end = 1252234;
    let refs = vec![end, 23452234, 28373423, end];
    let tags = vec![
        (r"waterway", r"riverbank"),
        (r"area", r"no")
    ];
    assert_eq!(false, osm_is_area::way(&tags, &refs));
}

#[test]
fn way_no_circular_refs () {
    let end = 1252234;
    let refs = vec![end, 23452234, end, 28373423];
    let tags = vec![
        (r"waterway", r"riverbank"),
        (r"area", r"yes")
    ];
    assert_eq!(false, osm_is_area::way(&tags, &refs));
}

#[test]
fn way_polygon_whitelist () {
    let end = 1252234;
    let refs = vec![end, 23452234, 28373423, end];
    let tags = vec![
        (r"waterway", r"riverbank"),
    ];
    assert_eq!(true, osm_is_area::way(&tags, &refs));
}

#[test]
fn way_polygon_blacklist () {
    let end = 1252234;
    let refs = vec![end, 23452234, 28373423, end];
    let tags = vec![
        (r"natural", r"cliff"),
    ];
    assert_eq!(false, osm_is_area::way(&tags, &refs));
}
