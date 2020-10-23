// Decides if a way is an area or not. Returns a boolean
//
// # Examples
//
// ```
// let tags = vec![
//   (r"waterway", r"riverbank")
// ];
// let refs = vec![1, 3, 2, 1];
//
// let is_area = osmisarea::way(tags, refs);
//
// assert_eq!(true, is_area);
// ```
pub fn way (
    tags: &Vec<(&str, &str)>,
    refs: &Vec<i64>
) -> bool {
    if refs.len() < 3 {
        return false;
    }

    if tags.len() == 0 {
        return false;
    }
    let mut refs_iter = refs.into_iter();
    let first = refs_iter.next();
    let last = refs_iter.last();

    if first == last {
        let opt = tags.into_iter()
            .find(|tag| tag.0 == "area" && tag.1 == "no");

        match opt {
            Some(_) => {
                return false;
            },
            None => {
                // TODO: Check polygon_features.json
                return true;
            }
        }
    }

    return false;
}

// Decides if a relation is an area or not. Returns a boolean
// A relation is an area when it has a tag "type" with value "multipolygon".
//
// # Examples
//
// ```
// let tags = vec![
//   (r"waterway", r"riverbank")
// ];
// let members = vec![1, 3, 2, 1];
//
// let is_area = osmisarea::relation(tags, refs);
//
// assert_eq!(false, is_area);
// ```

pub fn relation (
    tags: &Vec<(&str, &str)>,
    members: &Vec<i64>
) -> bool {
    if members.len() == 0 {
        return false;
    }
    let opt = tags.into_iter()
        .find(|tag| tag.0 == "type" && tag.1 == "multipolygon");
    match opt {
        Some(_) => return true,
        None => return false
    }
}

