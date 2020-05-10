use std::collections::HashMap;
use std::fmt::Error;

fn map_id(id: i32) -> Result<(i32, i32), String> {
    if id == 2 {
        Err(format!("Not found: {:?}", id))
    } else {
        Ok((id, id))
    }
}

pub fn closure_with_result() {
    let data = vec![1, 2];

    data.into_iter()
        .map(map_id)
        .collect::<Result<HashMap<i32, i32>, String>>();
}

pub fn closure_with_result_other() {
    let data = vec![1, 2];

    data.into_iter()
        .map(|id| -> Result<(i32, i32), String> {
            if id == 2 {
                Err(format!("Not found: {:?}", id))
            } else {
                Ok((id, id))
            }
        })
        .collect::<Result<HashMap<i32, i32>, String>>();
}
