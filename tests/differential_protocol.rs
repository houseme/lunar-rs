use std::collections::HashSet;

use lunar_rs::Solar;
use lunar_rs::differential_support::{PROTOCOL_VERSION, SOLAR_SNAPSHOT_KEYS, solar_snapshot};

#[test]
fn solar_snapshot_protocol_is_stable() {
    let solar = Solar::from_ymd_hms(2024, 4, 22, 23, 30, 0).unwrap();
    let snapshot = solar_snapshot(solar);

    assert_eq!(snapshot.len(), SOLAR_SNAPSHOT_KEYS.len());

    let keys: Vec<&str> = snapshot.iter().map(|(key, _)| *key).collect();
    assert_eq!(keys, SOLAR_SNAPSHOT_KEYS);

    let unique_keys: HashSet<&str> = keys.iter().copied().collect();
    assert_eq!(unique_keys.len(), keys.len());

    let values = snapshot.into_iter().collect::<std::collections::HashMap<_, _>>();
    assert_eq!(values.get("protocol_version").map(String::as_str), Some(PROTOCOL_VERSION));
    assert_eq!(values.get("calendar").map(String::as_str), Some("solar"));
    assert_eq!(values.get("solar").map(String::as_str), Some("2024-04-22 23:30:00"));
    assert_eq!(values.get("jieqi").map(String::as_str), Some(""));
    assert_eq!(values.get("lunar").map(String::as_str), Some("二〇二四年三月十四"));
    assert_eq!(values.get("year_ganzhi").map(String::as_str), Some("甲辰"));
}
