pub mod error;
pub mod logging;
pub mod math;
pub mod prelude;
pub mod plugin;
pub mod ids;

pub type Result<T> = anyhow::Result<T>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_builds_and_logs() -> Result<()> {
        logging::init_logging();
        tracing::info!("foundation ok");
        let a = math::Vec3::X + math::Vec3::Y;
        assert_eq!(a, math::Vec3::new(1.0, 1.0, 0.0));
        Ok(())
    }
}