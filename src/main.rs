const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");
const TARGET: &str = env!("BUILD_TARGET");

fn version() -> String {
    format!("{NAME} {VERSION} ({TARGET})")
}

fn output(arg: Option<&str>) -> String {
    match arg {
        Some("--help" | "-h") => {
            format!(
                "Usage: {NAME} [--version | -V] [--help | -h]\nPrints its version and build target."
            )
        }
        _ => version(),
    }
}

fn main() {
    println!("{}", output(std::env::args().nth(1).as_deref()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_has_version_and_target() {
        let out = version();
        assert!(out.contains(env!("CARGO_PKG_VERSION")));
        assert!(out.contains(env!("BUILD_TARGET")));
        assert!(env!("BUILD_TARGET").contains('-'));
    }

    #[test]
    fn flags() {
        assert_eq!(output(Some("-V")), version());
        assert!(output(Some("--help")).starts_with("Usage:"));
    }
}
