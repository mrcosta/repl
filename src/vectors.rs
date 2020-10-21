use itertools::Itertools;

pub fn test_sorted_vectors() {
    let test = vec!["1", "2", "3"];
    let test_2 = vec!["2", "1", "3"];

    // this fails
    // assert_eq!(test, test_2);

    // this works
    assert_eq!(
        test.iter().sorted().collect_vec(),
        test_2.iter().sorted().collect_vec()
    );
}
