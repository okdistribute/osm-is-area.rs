pub mod osmisarea;

#[cfg(test)]
mod tests {

    #[test]
    fn way_circular_refs () {
        let end = 1252234;
        let refs = vec![end, 23452234, 28373423, end];
        let tags = vec![
            (r"waterway", r"riverbank")
        ];
        assert_eq!(true, crate::osmisarea::way(&tags, &refs).unwrap());
    }

    #[test]
    fn way_area_no_tag () {
        let end = 1252234;
        let refs = vec![end, 23452234, 28373423, end];
        let tags = vec![
            (r"waterway", r"riverbank"),
            (r"area", r"no")
        ];
        assert_eq!(false, crate::osmisarea::way(&tags, &refs).unwrap());
    }

    #[test]
    fn way_no_circular_refs () {
        let end = 1252234;
        let refs = vec![end, 23452234, end, 28373423];
        let tags = vec![
            (r"waterway", r"riverbank"),
            (r"area", r"yes")
        ];
        assert_eq!(false, crate::osmisarea::way(&tags, &refs).unwrap());
    }

    #[test]
    fn relation () {
        let tags = vec![
            (r"waterway", r"riverbank"),
            (r"type", r"multipolygon"),
        ];
        let end = 1252234;
        let members = vec![end, 23452234, 1252234, end, 52354653];
        assert_eq!(true, crate::osmisarea::relation(&tags, &members).unwrap());
    }

    #[test]
    fn relation_fail () {
        let tags = vec![
            (r"waterway", r"riverbank"),
            (r"type", r"area")
        ];
        let end = 1252234;
        let members = vec![end, 23452234, 1252234, end, 52354653];
        assert_eq!(false, crate::osmisarea::relation(&tags, &members).unwrap());
    }
}

