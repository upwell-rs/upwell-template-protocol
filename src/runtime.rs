pub struct ExampleRuntime {
    application: String,
}

impl ExampleRuntime {
    pub(crate) fn new(application: String) -> Self {
        Self { application }
    }
}

impl upwell_app::ProtocolRuntime for ExampleRuntime {
    type Error = crate::Error;
}

{% if macro_crate %}
#[crate::protocol_component(by_value)]
#[derive(Clone)]
pub struct ProtocolState;
{% endif %}

impl upwell_app::Serve<()> for ExampleRuntime {
    async fn serve(
        self,
        _runtime: upwell_app::AppRuntime,
        _shutdown: upwell_app::ShutdownSignal,
        (): (),
    ) -> Result<(), Self::Error> {
        let _application = self.application;

        Ok(())
    }
}
