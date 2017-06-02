fn binary_search(v: vec<i32>, target: i32) -> option<i32> {
    let mut min: usize = 0;
    let mut max: usize = v.len() - 1;
    let mut found: Option<i32> = None;

    while min <= max {
        let mid: usize = min + (max - min) / 2;

        if target == v[mid] {
            found = Some(target);
            break;
        }

        if target > v[mid] {
            min = mid + 1;
        }

        if target < v[mid] {
            max = mid - 1;
        }
    }

    found
}
