use failure::Error;

pub fn way (
    tags: &Vec<(&str, &str)>,
    refs: &Vec<i64>
) -> Result<bool, Error> {
    if refs.len() < 3 {
        return Ok(false)
    }

    if tags.len() == 0 {
        return Ok(false);
    }
    let mut refs_iter = refs.into_iter();
    let first = refs_iter.next();
    let last = refs_iter.last();

    if first == last {
        let opt = tags.into_iter()
            .find(|tag| tag.0 == "area" && tag.1 == "no");

        match opt {
            Some(_) => {
                return Ok(false)
            },
            None => {
                // TODO: Check polygon_features.json
                return Ok(true)
            }
        }
    }

    return Ok(false);
}

pub fn relation (
    tags: &Vec<(&str, &str)>,
    members: &Vec<i64>
) -> Result<bool, Error> {
    if members.len() == 0 {
        return Ok(false)
    }
    let opt = tags.into_iter()
        .find(|tag| tag.0 == "type" && tag.1 == "multipolygon");
    match opt {
        Some(_) => return Ok(true),
        None => return Ok(false)
    }
}

