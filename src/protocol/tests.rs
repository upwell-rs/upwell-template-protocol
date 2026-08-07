use upwell_app::{ProtocolDefinition as _, StaticScope as _};

#[test]
fn protocol_declares_stable_identity_and_scope_topology() {
    assert_eq!(
        crate::ExampleProtocol::ID.as_str(),
        "{{ crate_name }}/protocol"
    );
    assert_eq!(crate::ExampleProtocol::SCOPE_TOPOLOGY.boundaries().len(), 1);
    assert_eq!(crate::Session::ID.as_str(), "{{ crate_name }}/session");
}

#[tokio::test]
async fn protocol_prepares_builds_and_serves() {
    let application = upwell_app::App::<crate::ExampleProtocol>::builder("protocol-test")
        .build()
        .await
        .expect("protocol builds");

    application
        .serve(())
        .await
        .expect("protocol serves its endpoint");
}
