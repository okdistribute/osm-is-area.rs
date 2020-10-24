/// Returns true if the given relation is an area.
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

