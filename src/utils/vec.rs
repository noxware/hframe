pub(crate) fn insert_or_replace<T, F, I>(
    v: &mut Vec<T>,
    elem: T,
    mut id_extractor: F,
) -> (&mut T, Option<T>)
where
    I: PartialEq,
    F: FnMut(&T) -> I,
{
    let pos = v
        .iter()
        .position(|x| id_extractor(x) == id_extractor(&elem));

    match pos {
        Some(pos) => {
            let old_elem = std::mem::replace(&mut v[pos], elem);
            (&mut v[pos], Some(old_elem))
        }
        None => {
            v.push(elem);
            let len = v.len();
            (&mut v[len - 1], None)
        }
    }
}
