use serde_json::json;

fn main() {
    let hanako = json!({
        "name": "Hanako",
        "age": 25,
        "favorites": {
            "food": ["Sushi", "Ramen"],
        }
    });
    println!("{:?}", hanako);
}
