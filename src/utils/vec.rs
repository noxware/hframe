pub(crate) fn insert_or_replace<T, F, I>(
    v: &mut Vec<T>,
    elem: T,
    mut id_extractor: F,
) -> (&mut T, Option<T>)
where
    I: PartialEq,
    F: FnMut(&T) -> I,
{
    // First, determine if and where a matching element exists
    let pos = v
        .iter()
        .position(|x| id_extractor(x) == id_extractor(&elem));

    match pos {
        Some(pos) => {
            // If found, replace the element at that position
            let old_elem = std::mem::replace(&mut v[pos], elem);
            (&mut v[pos], Some(old_elem))
        }
        None => {
            // If not found, push the new element to the end of the vector
            v.push(elem);
            let len = v.len();
            (&mut v[len - 1], None)
        }
    }
}
