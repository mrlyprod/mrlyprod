use mrlycore::json;
use mrlycore::json::{parse, Json, Map};

#[test]
fn floats_are_rejected() {
    for text in [
        "1.5",
        "-0.5",
        "1e3",
        "2E5",
        "[3.14]",
        r#"{"a": 0.1}"#,
        "1.0",
        "1e-2",
    ] {
        assert!(parse(text).is_err(), "{text} should not parse");
    }
}

#[test]
fn integers_round_trip() {
    for text in [
        "0",
        "-0",
        "7",
        "-7",
        "9223372036854775807",
        "-9223372036854775808",
    ] {
        let value = parse(text).unwrap();
        assert!(value.is_number());
    }
    assert_eq!(parse("9223372036854775807").unwrap(), i64::MAX);
    assert_eq!(parse("-9223372036854775808").unwrap(), i64::MIN);
    assert_eq!(
        parse("-9223372036854775808").unwrap().to_string(),
        "-9223372036854775808"
    );
    assert!(parse("9223372036854775808").is_err());
    assert!(parse("-9223372036854775809").is_err());
}

#[test]
fn malformed_input_is_rejected() {
    for text in [
        "",
        "01",
        "-01",
        "1 x",
        "tru",
        "[1,]",
        "{\"a\":}",
        "{\"a\" 1}",
        "[1 2]",
        "\"open",
        "nul",
        "+1",
        "-",
    ] {
        assert!(parse(text).is_err(), "{text} should not parse");
    }
}

#[test]
fn deep_nesting_is_bounded() {
    let deep_ok = format!("{}0{}", "[".repeat(100), "]".repeat(100));
    assert!(parse(&deep_ok).is_ok());
    let too_deep = format!("{}0{}", "[".repeat(200), "]".repeat(200));
    assert!(parse(&too_deep).is_err());
}

#[test]
fn string_escapes() {
    assert_eq!(parse(r#""😀""#).unwrap(), "😀");
    assert_eq!(parse(r#""A\t""#).unwrap(), "A\t");
    assert!(parse(r#""\ud83d""#).is_err());
    assert!(parse(r#""\udc00""#).is_err());
    assert!(parse(r#""\x41""#).is_err());
    assert!(parse("\"a\u{0001}b\"").is_err());
    let value = Json::Str("quote \" back \\ ctrl \u{0007} tab \t 😀".to_string());
    assert_eq!(parse(&value.to_string()).unwrap(), value);
}

#[test]
fn duplicate_keys_keep_first_position_last_value() {
    let value = parse(r#"{"a": 1, "b": 2, "a": 3}"#).unwrap();
    assert_eq!(value.to_string(), r#"{"a":3,"b":2}"#);
}

#[test]
fn macro_shapes() {
    assert_eq!(json!(null), Json::Null);
    assert_eq!(json!(true), Json::Bool(true));
    assert_eq!(json!([]), Json::Arr(vec![]));
    assert_eq!(json!({}), Json::Obj(Map::new()));
    assert_eq!(json!([1, 2, 3,]).to_string(), "[1,2,3]");
    assert_eq!(
        json!({"a": 1, "b": [true, null],}).to_string(),
        r#"{"a":1,"b":[true,null]}"#
    );
}

#[test]
fn macro_interpolation() {
    let n = 5i32;
    let name = String::from("mrly");
    let items = vec![1i64, 2];
    let inner = json!({"deep": 1});
    let value = json!({
        "n": n,
        "name": name,
        "slice": items,
        "some": Some(9),
        "none": None::<i64>,
        "inner": inner,
        (format!("k{}", 1)): 2,
    });
    assert_eq!(
        value.to_string(),
        r#"{"n":5,"name":"mrly","slice":[1,2],"some":9,"none":null,"inner":{"deep":1},"k1":2}"#
    );
}

#[test]
fn indexing_and_access() {
    let mut value = json!({"a": {"b": [10, 20]}});
    assert_eq!(value["a"]["b"][1], 20i64);
    assert!(value["missing"].is_null());
    assert!(value["a"]["b"][9].is_null());
    value["new"] = json!(5);
    assert_eq!(value["new"], 5i64);
    assert_eq!(
        value
            .get("a")
            .and_then(|a| a.get("b"))
            .and_then(|b| b.get(0))
            .and_then(Json::as_i64),
        Some(10)
    );
    assert_eq!(value["a"]["b"].take().to_string(), "[10,20]");
    assert!(value["a"]["b"].is_null());
}

#[test]
fn map_behavior() {
    let mut map = Map::new();
    map.insert("x".to_string(), json!(1));
    map.insert("y".to_string(), json!(2));
    assert_eq!(map.insert("x".to_string(), json!(3)), Some(json!(1)));
    assert_eq!(Json::Obj(map.clone()).to_string(), r#"{"x":3,"y":2}"#);
    assert_eq!(map.remove("x"), Some(json!(3)));
    assert_eq!(Json::Obj(map.clone()).to_string(), r#"{"y":2}"#);
    *map.entry("z").or_insert(json!(0)) = json!(7);
    assert_eq!(map.get("z"), Some(&json!(7)));
    assert_eq!(map.entry("z").or_insert(json!(0)).clone(), json!(7));
    assert_eq!(map.keys().collect::<Vec<_>>(), ["y", "z"]);
}

#[test]
fn number_accessors() {
    assert_eq!(json!(-1).as_u64(), None);
    assert_eq!(json!(7).as_u64(), Some(7));
    assert_eq!(json!(7).as_i64(), Some(7));
    assert_eq!(json!("7").as_i64(), None);
}
