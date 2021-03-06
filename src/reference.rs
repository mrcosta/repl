#[derive(Debug)]
struct RefTest {
    id: i32,
    value: String,
}

pub fn reference_test() {
    let mut test = Vec::new();
    test.push(RefTest {
        id: 1,
        value: "a".to_string(),
    });

    let reference = get_reference(&test);
    println!("{:?}", reference);
}

fn get_reference(existing: &Vec<RefTest>) -> Vec<&RefTest> {
    let mut return_test = Vec::new();
    return_test.push(existing.get(0).unwrap());

    return return_test;
}
