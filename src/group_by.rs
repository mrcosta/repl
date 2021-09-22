use itertools::Itertools;
use std::collections::HashMap;

#[derive(Debug)]
struct Test {
    id: i32,
}

// IT NEEDS TO BE ORDERED TO WORK
pub fn group_by_example() {
    let data = vec![
        (1, vec![Test { id: 0 }]),
        (1, vec![Test { id: 3 }]),
        (0, vec![Test { id: 1 }]),
    ];

    data.into_iter()
        .group_by(|test| test.0)
        .into_iter()
        .for_each(|(_x, y)| {
            // println!("{:?}", (x, y.map(|test|.collect())
            println!("{:?}", y.collect::<Vec<(i32, Vec<Test>)>>())
        });
    // println!(
    //     "{:?}",
    //     data.into_iter()
    //         .group_by(|test| test.0)
    //         .into_iter()
    //         .map(|(x, y)| {
    //             let ids_together = y
    //                 .collect::<Vec<(i32, Vec<Test>)>>()
    //                 .into_iter()
    //                 .flat_map(|(_id, values)| values)
    //                 .collect();
    //
    //             (x, ids_together)
    //         })
    //         .collect::<HashMap<i32, Vec<Test>>>()
    // );
}

pub fn group_by_second_example() {
    let data = vec![Test { id: 0 }, Test { id: 1 }, Test { id: 2 }];

    data.into_iter()
        .group_by(|test| test.id)
        .into_iter()
        .for_each(|(x, y)| {
            println!("{:?}", (x, y.collect::<Vec<_>>()));
            // println!("{:?}", y.collect::<Vec<(i32, Vec<Test>)>>())
        });
    // .collect::<HashMap<i32, Vec<Test>>>();

    // let grouped_by = data
    //     .into_iter()
    //     .group_by(|test| test.id)
    //     .into_iter()
    //     .map(|(x, y)| {
    //         println!("{:?}", (x, y.collect::<Vec<_>>()));
    //         // println!("{:?}", y.collect::<Vec<(i32, Vec<Test>)>>())
    //         (x, y.collect())
    //     })
    //     .collect::<HashMap<i32, Vec<Test>>>();
    // println!("{:?}", grouped_by);
}
