use extism_pdk::*;

#[plugin_fn]
pub fn run(Json(x): Json<Vec<i64>>) -> FnResult<Json<Vec<i64>>> {
    let mut values = Vec::new();

    for (idx, a) in x.iter().enumerate() {
        for b in x.iter() {
            if values.len() > idx {
                values[idx] += b;
            } else {
                values.push(a * b);
            }
        }
    }

    Ok(Json(values))
}
