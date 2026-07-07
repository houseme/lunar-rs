use lunar_rs::holiday_util;

#[test]
fn holiday_override_api_accepts_current_snapshot() {
    let names = holiday_util::holiday_names();
    let data = holiday_util::holiday_data();

    holiday_util::set_holiday_data(data.clone()).unwrap();
    holiday_util::replace_holiday_data(data.clone()).unwrap();
    holiday_util::set_holidays(names.clone(), data.clone()).unwrap();
    holiday_util::replace_holidays(names.clone(), data.clone()).unwrap();
    holiday_util::load_holiday_data(data.clone()).unwrap();
    holiday_util::load_holidays(names.clone(), data.clone()).unwrap();

    assert_eq!(holiday_util::holiday_names(), names);
    assert_eq!(holiday_util::holiday_data(), data);

    holiday_util::reset_holidays();
}

#[test]
fn holiday_override_api_rejects_invalid_payload() {
    let names_before = holiday_util::holiday_names();
    let data_before = holiday_util::holiday_data();

    let err = holiday_util::set_holiday_data("bad-data").unwrap_err();
    assert!(err.to_string().contains("multiple"));

    let err = holiday_util::set_holidays(vec!["元旦".to_string()], "202401012020240101").unwrap_err();
    assert!(err.to_string().contains("name index"));

    let err = holiday_util::replace_holiday_data("bad-data").unwrap_err();
    assert!(err.to_string().contains("multiple"));

    let err = holiday_util::load_holidays(vec!["元旦".to_string()], "202401012020240101").unwrap_err();
    assert!(err.to_string().contains("name index"));

    assert_eq!(holiday_util::holiday_names(), names_before);
    assert_eq!(holiday_util::holiday_data(), data_before);
}
