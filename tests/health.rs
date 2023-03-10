extern crate consul;
use consul::{Client, Config};

#[test]
fn health_test() {
    use consul::health::Health;
    let config = Config::new().unwrap();
    let client = Client::new(config);
    // An existing service for a agent in dev mode
    let r = client.service("consul", None, true, None).unwrap();
    let (snodes, meta) = (r.0, r.1);
    {
        assert!(!snodes.is_empty(), "should have at least one Service Node");
        assert!(meta.last_index.unwrap() > 0, "index must be positive");
        if let Some(consul_srvc_entry) = snodes.get(0) {
            if let Some(meta) = &consul_srvc_entry.Service.Meta {
                assert!(
                    meta.get("grpc_port").is_some(),
                    "consul service meta grpc_port should exists"
                );
                assert_eq!(
                    meta.get("grpc_port").unwrap(),
                    "8502",
                    "consul service meta grpc_port should be 8502"
                );
            } else {
                panic!("consul service entry should have Meta values");
            }
            assert_eq!(
                consul_srvc_entry.Service.Weights.get("Passing").unwrap(),
                &1,
                "consul service passing Weight must be 1"
            );
            assert_eq!(
                consul_srvc_entry.Service.Weights.get("Warning").unwrap(),
                &1,
                "consul service warning Weight must be 1"
            );
            assert_eq!(
                consul_srvc_entry.Service.Namespace, "",
                "namespace should be empty for consul service"
            );
        } else {
            panic!("No consul service entry found")
        }
    }
    // A non existing, should be empty
    let r = client
        .service("non-existing-service", None, true, None)
        .unwrap();
    let (snodes, meta) = (r.0, r.1);
    {
        assert_eq!(snodes.len(), 0);
        assert!(meta.last_index.unwrap() > 0, "index must be positive");
    }
}
#[test]
fn health_node_test() {
    use consul::health::Health;
    let config = Config::new().unwrap();
    let client = Client::new(config);
    let system_hostname = hostname::get().unwrap().into_string().unwrap();
    // An existing service for a agent in dev mode
    let r = client
        .node(&system_hostname, Some("serfHealth"), None, None, None)
        .unwrap();
    let (services, meta) = (r.0, r.1);
    {
        assert!(
            !services.is_empty(),
            "should have at least one Service Node"
        );
        assert!(meta.last_index.unwrap() > 0, "index must be positive");
    }
    // A non existing node, should be empty
    let r = client
        .node("non-existing-node", Some("serfHealth"), None, None, None)
        .unwrap();
    let (services, meta) = (r.0, r.1);
    {
        assert_eq!(services.len(), 0);
        assert!(meta.last_index.unwrap() > 0, "index must be positive");
    }
    // A non existing check, should be empty
    let r = client
        .node(
            &system_hostname,
            Some("non-existing-check"),
            None,
            None,
            None,
        )
        .unwrap();
    let (services, meta) = (r.0, r.1);
    {
        assert_eq!(services.len(), 0);
        assert!(meta.last_index.unwrap() > 0, "index must be positive");
    }
    // A non existing service, should be empty
    let r = client
        .node(
            &system_hostname,
            None,
            Some("non-existing-service"),
            None,
            None,
        )
        .unwrap();
    let (services, meta) = (r.0, r.1);
    {
        assert_eq!(services.len(), 0);
        assert!(meta.last_index.unwrap() > 0, "index must be positive");
    }
}
