use once_cell::sync::OnceCell;
use tracing_subscriber::{fmt, EnvFilter};

static LOG_INIT: OnceCell<()> = OnceCell::new();

pub fn init_logging() {
    LOG_INIT.get_or_init(|| {
        let filter = EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| EnvFilter::new("info"));

        fmt()
            .with_env_filter(filter)
            .with_target(false)
            .init();
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init_logging_twice() {
        init_logging();
        init_logging(); // should not panic
    }
}