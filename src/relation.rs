/// Returns true if the given relation is an area.
///
/// A relation is an area when it has a tag "type" with value "multipolygon".
/// ```
/// use osm_is_area;
///
/// let tags = vec![
///     (r"type", r"multipolygon")
/// ];
/// let members = vec![1, 3, 2, 1];
///
/// let is_area = osm_is_area::relation(&tags, &members);
/// assert_eq!(true, is_area);
/// ```
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

