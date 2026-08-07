pub struct Session;

impl upwell_app::StaticScope for Session {
    const ID: upwell_app::ScopeId =
        upwell_app::namespaced_id!(upwell_app::ScopeId, "{{ crate_name }}/session");
    const RANK: u8 = 128;
    const NAME: &'static str = "Session";
}
