use crate::polygon_features;

/// Returns true if the given way is an area according to [Overpass turbo](https://wiki.openstreetmap.org/wiki/Overpass_turbo/Polygon_Features)
///
/// ```
/// use osm_is_area;
///
/// let tags = vec![
///  (r"waterway", r"riverbank")
/// ];
/// let refs = vec![1, 3, 2, 1];
///
/// let is_area = osm_is_area::way(&tags, &refs);
///
/// assert_eq!(true, is_area);
/// ```
///
pub fn way(tags: &[(&str, &str)], refs: &[i64]) -> bool {
    let features = &polygon_features::get_polygon_features();

    if refs.len() < 3 {
        return false;
    }

    if tags.len() == 0 {
        return false;
    }

    let mut refs_iter = refs.into_iter();
    let first = refs_iter.next();
    let last = refs_iter.last();

    // A way is considered an area if
    // 1. It forms a closed loop
    if first == last {
        // 2. It is not tagged `area=no`
        let opt = tags
            .into_iter()
            .find(|tag| tag.0 == "area" && tag.1 == "no");

        match opt {
            Some(_) => {
                return false;
            }
            None => {
                // 3. At least one of the following conditions is true
                let mut is_area = false;
                tags.into_iter().for_each(|tag| {
                    let key = tag.0;
                    let value = tag.1;

                    let mut iter = features
                        .into_iter()
                        .filter(|condition| condition.key == key);

                    let maybe_condition = iter.next();
                    if maybe_condition.is_some() {
                        let condition = maybe_condition.unwrap();
                        if condition.polygon == "all" && value != "no" {
                            is_area = true
                        }
                        if condition.polygon == "whitelist" {
                            let whitelist = &condition.values;
                            whitelist.into_iter().for_each(|val| {
                                if &value == val {
                                    is_area = true
                                }
                            });
                        }
                        if condition.polygon == "blacklist" {
                            is_area = true;
                            let blacklist = &condition.values;
                            blacklist.into_iter().for_each(|val| {
                                if &value == val {
                                    is_area = false
                                }
                            });
                        }
                    }
                });
                return is_area;
            }
        }
    }

    return false;
}
