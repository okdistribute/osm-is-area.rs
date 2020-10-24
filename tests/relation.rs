use osm_is_area;

#[test]
fn relation () {
    let tags = vec![
        (r"waterway", r"riverbank"),
        (r"type", r"multipolygon"),
    ];
    let end = 1252234;
    let members = vec![end, 23452234, 1252234, end, 52354653];
    assert_eq!(true, osm_is_area::relation(&tags, &members));
}

#[test]
fn relation_fail () {
    let tags = vec![
        (r"waterway", r"riverbank"),
        (r"type", r"area")
    ];
    let end = 1252234;
    let members = vec![end, 23452234, 1252234, end, 52354653];
    assert_eq!(false, osm_is_area::relation(&tags, &members));
}
