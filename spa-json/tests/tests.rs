use proptest::{
    prelude::{Just, Strategy, any, prop, prop_assert_eq, proptest},
    prop_oneof,
};
use spa_json::{Map, Value, json};

#[test]
fn test() {
    expect_test::expect![[r#"
        {
          name = "Alice"
          age = 30
          is_student = false
          courses = [
            "Math"
            "Science"
            "Art"
          ]
          address = {
            street = "123 Main St"
            city = "Wonderland"
          }
        }"#]]
    .assert_eq(
        &spa_json::to_string_pretty(&json!({
            "name": "Alice",
            "age": 30,
            "is_student": false,
            "courses": ["Math", "Science", "Art"],
            "address": {
                "street": "123 Main St",
                "city": "Wonderland"
            }
        }))
        .unwrap(),
    );

    expect_test::expect![[r#"{name="Alice",age=30,is_student=false,courses=["Math" "Science" "Art"],address={street="123 Main St",city="Wonderland"}}"#]]
    .assert_eq(
        &spa_json::to_string(&json!({
            "name": "Alice",
            "age": 30,
            "is_student": false,
            "courses": ["Math", "Science", "Art"],
            "address": {
                "street": "123 Main St",
                "city": "Wonderland"
            }
        }))
        .unwrap(),
    );
}

proptest! {
    #[test]
    fn test_roundtrip_pretty(value in arb_v()) {
        let s = spa_json::to_string_pretty(&value).unwrap();
        let v2: Value = spa_json::from_str(&s).unwrap();
        prop_assert_eq!(&value, &v2);

        let s_pretty = spa_json::to_string_pretty(&value).unwrap();
        let v3: Value = spa_json::from_str(&s_pretty).unwrap();
        prop_assert_eq!(value, v3);
    }

    #[test]
    fn test_roundtrip_compact(value in arb_v()) {
        let s = spa_json::to_string(&value).unwrap();
        let v2: Value = spa_json::from_str(&s).unwrap();
        prop_assert_eq!(&value, &v2);

        let s_compact = spa_json::to_string(&value).unwrap();
        let v3: Value = spa_json::from_str(&s_compact).unwrap();
        prop_assert_eq!(value, v3);
    }
}

fn arb_v() -> impl Strategy<Value = Value> {
    let leaf = prop_oneof![
        Just(Value::Null),
        any::<bool>().prop_map(Value::Bool),
        any::<f64>().prop_map(Value::from),
        ".*".prop_map(Value::String),
    ];
    leaf.prop_recursive(
        8,   // 8 levels deep
        256, // maximum size of total nodes
        10,  // maximum size of each collection
        |inner| {
            prop_oneof![
                prop::collection::vec(inner.clone(), 0..10).prop_map(Value::Array),
                prop::collection::btree_map(".*", inner, 0..10)
                    .prop_map(Map::from_iter)
                    .prop_map(Value::Object),
            ]
        },
    )
}
