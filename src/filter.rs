pub fn test_filter_map() {
    let list = [Some(1), Some(2)];
    println!(
        "{:?}",
        list.iter()
            .filter_map(|num| num.map(|num| num))
            .collect::<Vec<_>>(),
    );
}
