#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    /// Protocol preparation or runtime construction failed in Upwell.
    #[error(transparent)]
    Application(#[from] upwell_app::Error),
}

/// Protocol result type.
pub type Result<T, E = Error> = core::result::Result<T, E>;
