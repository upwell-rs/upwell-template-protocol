//! First-class protocol definition, runtime, scopes, and extension points.

mod error;
mod protocol;
mod runtime;
mod scope;

pub use error::{Error, Result};
pub use protocol::ExampleProtocol;
pub use runtime::ExampleRuntime;
pub use scope::Session;
{% if macro_crate %}
pub use {{ crate_name }}_macros::protocol_component;
{% endif %}

pub mod prelude {
    //! Common imports for protocol implementors and consumers.

    pub use crate::{Error, ExampleProtocol, ExampleRuntime, Result, Session};
    {% if macro_crate %}
    pub use crate::protocol_component;
    {% endif %}
    pub use upwell_app::{PreparedProtocol, ProtocolDefinition, ProtocolRuntime, Serve};
}
