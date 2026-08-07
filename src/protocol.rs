const BOUNDARIES: [upwell_app::ScopeBoundary; 1] = [upwell_app::ScopeBoundary::new(
    &crate::Session,
    upwell_app::ScopeParent::Root,
)];

#[derive(Default)]
pub struct ExampleProtocol;

pub struct PreparedExampleProtocol {
    application: String,
}

impl upwell_app::ProtocolDefinition for ExampleProtocol {
    type Prepared = PreparedExampleProtocol;
    type Error = crate::Error;

    const ID: upwell_app::ProtocolId =
        upwell_app::namespaced_id!(upwell_app::ProtocolId, "{{ crate_name }}/protocol");
    const SCOPE_TOPOLOGY: upwell_app::ScopeTopology =
        upwell_app::ScopeTopology::new(&BOUNDARIES);

    fn register(&self, _registry: &mut upwell_app::AppRegistry) {}

    fn prepare(
        self,
        context: &upwell_app::ValidationContext<'_>,
    ) -> Result<Self::Prepared, Self::Error> {
        Ok(PreparedExampleProtocol {
            application: context.name().to_owned(),
        })
    }
}

impl upwell_app::PreparedProtocol for PreparedExampleProtocol {
    type Runtime = crate::ExampleRuntime;
    type Error = crate::Error;

    fn build(
        self,
        _runtime: &upwell_app::AppRuntime,
    ) -> Result<Self::Runtime, Self::Error> {
        Ok(crate::ExampleRuntime::new(self.application))
    }

    #[cfg(feature = "tooling")]
    fn tooling(&self, contributions: &mut upwell_app::ToolingContributions) {
        contributions.display(upwell_app::ResourceDisplay {
            label: Some(String::from("Example protocol")),
            summary: Some(format!("Protocol for {}", self.application)),
            ..Default::default()
        });
    }
}

#[cfg(test)]
mod tests;
