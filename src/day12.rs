use serde_json::Value;

fn main() {
    let input = include_str!("../data/day12.data");

    let json_object: serde_json::Value = serde_json::from_str(input).unwrap();

    let res_step1 = evaluate_node(&json_object, 1);
    println!("Part 1: {}", res_step1);
    let res_step2 = evaluate_node(&json_object, 2);
    println!("Part 2: {}", res_step2);
}

fn evaluate_node(value: &Value, step: usize) -> i64 {
    match value {
        Value::Number(n) => n.as_i64().unwrap(),
        Value::Array(items) => items.iter().map(|item| evaluate_node(item, step)).sum(),
        Value::Object(objects) => {
            if step == 2
                && objects.iter().any(|obj| {
                    if let Value::String(s) = obj.1 {
                        s == "red"
                    } else {
                        false
                    }
                })
            {
                0
            } else {
                objects
                    .iter()
                    .map(|(_, value)| evaluate_node(value, step))
                    .sum()
            }
        }
        _ => 0,
    }
}
