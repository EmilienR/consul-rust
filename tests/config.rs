extern crate consul;

use consul::ConfigBuilder;
use std::time::Duration;

#[test]
fn cfg_test_builder_simple() {
    let config = ConfigBuilder::new()
        .with_hostname("127.0.0.2")
        .with_port(8666)
        .with_token("XX-TOKEN-XX")
        .with_http_timeout(Duration::from_secs(5))
        .build()
        .unwrap();

    assert_eq!(&config.address, "http://127.0.0.2:8666");
    assert_eq!(config.datacenter, None);
    assert_eq!(config.token, Some("XX-TOKEN-XX".to_string()));
    assert_eq!(config.namespace, None);
}

#[test]
fn cfg_test_builder_full_address() {
    let config_builder = ConfigBuilder::from_full_address("https://127.0.0.2:7555").unwrap();
    assert_eq!(config_builder.scheme, Some("https".to_string()));
    assert_eq!(config_builder.hostname, Some("127.0.0.2".to_string()));
    assert_eq!(config_builder.port, Some(7555));

    let config = config_builder.build().unwrap();

    assert_eq!(&config.address, "https://127.0.0.2:7555");
    assert_eq!(config.datacenter, None);
    assert_eq!(config.token, None);
    assert_eq!(config.namespace, None);
}

#[test]
fn cfg_test_builder_full_address_invalid_scheme() {
    let config_builder = ConfigBuilder::from_full_address("tcp://127.0.0.2:7555");
    assert!(config_builder.is_err());
}

#[test]
fn cfg_test_builder_full_address_no_port() {
    let config_builder = ConfigBuilder::from_full_address("http://127.0.0.2").unwrap();
    assert_eq!(config_builder.scheme, Some("http".to_string()));
    assert_eq!(config_builder.hostname, Some("127.0.0.2".to_string()));
    assert_eq!(config_builder.port, None);
}

#[test]
fn cfg_test_builder_token_only_default_address() {
    let config = ConfigBuilder::new()
        .with_token("XX-TOKEN-XX")
        .build()
        .unwrap();

    assert_eq!(&config.address, "http://localhost:8500");
    assert_eq!(config.datacenter, None);
    assert_eq!(config.token, Some("XX-TOKEN-XX".to_string()));
    assert_eq!(config.namespace, None);
}

#[test]
fn cfg_test_builder_ns() {
    let config = ConfigBuilder::new()
        .with_namespace("NSTEST")
        .build()
        .unwrap();

    assert_eq!(&config.address, "http://localhost:8500");
    assert_eq!(config.datacenter, None);
    assert_eq!(config.token, None);
    assert_eq!(config.namespace, Some("NSTEST".to_string()));
}

#[test]
fn cfg_test_builder_dc() {
    let config = ConfigBuilder::new()
        .with_datacenter("DCTEST")
        .build()
        .unwrap();

    assert_eq!(&config.address, "http://localhost:8500");
    assert_eq!(config.datacenter, Some("DCTEST".to_string()));
    assert_eq!(config.token, None);
    assert_eq!(config.namespace, None);
}
