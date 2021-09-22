use itertools::{EitherOrBoth, Itertools};
use std::collections::HashMap;

// IT NEEDS TO BE ORDERED TO WORK
pub fn merge_join_by_example() {
    /* will print
    i: 1, j: 4
    i: 2, j: 4
    i: 3, j: 4
    {1: "1", 3: "3", 4: "5", 6: "6", 2: "2", 7: "7"}
    It means that didn't find any assignee_id matching reviewer_id to compare
    */
    // let assignees = vec![(1, "1"), (2, "2"), (3, "3")];
    // let reviewers = vec![(4, "5"), (6, "6"), (7, "7")];

    /* will print
    i: 1, j: 4
    i: 2, j: 4
    i: 3, j: 4
    {3: "3", 7: "7", 1: "1", 8: "8", 2: "2", 4: "5", 6: "6"}
    It means that number of iterations is made based on the first iterator
    */
    // let assignees = vec![(1, "1"), (2, "2"), (3, "3")];
    // let reviewers = vec![(4, "5"), (6, "6"), (7, "7"), (8, "8")];

    /* will print
    i: 1, j: 1
    i: 2, j: 2
    i: 3, j: 3
    {3: "37", 1: "15", 8: "8", 2: "26"}
    It means that will pair the ones that are equal
     */
    // let assignees = vec![(1, "1"), (2, "2"), (3, "3"), (8, "8")];
    // let reviewers = vec![(1, "5"), (2, "6"), (3, "7")];

    /* will print
    i: 1, j: 1 // merge
    i: 2, j: 3 // remove 2
    i: 3, j: 3 // remove both
    i: 8, j: 2 // remove 2
    i: 8, j: 9 // remove 8
    {2: "7", 3: "36", 8: "8", 9: "9", 1: "15"}
     */
    let assignees = vec![(1, "1"), (2, "2"), (3, "3"), (8, "8")];
    let reviewers = vec![(1, "5"), (3, "6"), (2, "7"), (9, "9")];

    /* - Emit `EitherOrBoth::Left(i)` when `i < j`,
    ///   and remove `i` from its source iterator
    /// - Emit `EitherOrBoth::Right(j)` when `i > j`,
    ///   and remove `j` from its source iterator
    /// - Emit `EitherOrBoth::Both(i, j)` when  `i == j`,
    and remove both `i` and `j` from their respective source iterators */
    let assignees_and_reviewers: HashMap<i32, String> = assignees
        .into_iter()
        .merge_join_by(reviewers.into_iter(), |(i, _), (j, _)| {
            println!("i: {}, j: {}", i, j);
            i.cmp(j)
        })
        .map(|either| match either {
            EitherOrBoth::Left((assignee_id, value)) => (assignee_id, value.to_string()),
            EitherOrBoth::Right((reviewer_id, value)) => (reviewer_id, value.to_string()),
            EitherOrBoth::Both((assignee_id, assignee_value), (_reviewer_id, reviewer_value)) => (
                assignee_id,
                (assignee_value.to_owned() + reviewer_value).to_string(),
            ),
        })
        .collect();

    println!("{:?}", assignees_and_reviewers);
}

pub fn merge_join_by_example_with_sorted() {
    let assignees = vec![(2, "2"), (3, "3"), (8, "8"), (1, "1")];
    let reviewers = vec![(1, "5"), (3, "6"), (2, "7"), (9, "9"), (1, "5")];

    let assignees_and_reviewers: HashMap<i32, String> = assignees
        .into_iter()
        .sorted()
        .merge_join_by(reviewers.into_iter(), |(i, _), (j, _)| {
            println!("i: {}, j: {}", i, j);
            i.cmp(j)
        })
        .map(|either| match either {
            EitherOrBoth::Left((assignee_id, value)) => (assignee_id, value.to_string()),
            EitherOrBoth::Right((reviewer_id, value)) => (reviewer_id, value.to_string()),
            EitherOrBoth::Both((assignee_id, assignee_value), (_reviewer_id, reviewer_value)) => (
                assignee_id,
                (assignee_value.to_owned() + reviewer_value).to_string(),
            ),
        })
        .collect();

    println!("{:?}", assignees_and_reviewers);
}
