//! systemd unit printed by `picomon --print-service`.

const TEMPLATE: &str = include_str!("../contrib/picomon.service");

/// Binary path used in contrib/picomon.service, replaced with the real one.
const TEMPLATE_EXE: &str = "/usr/local/bin/picomon";

/// Returns the unit with `ExecStart` pointing at `exe`.
pub fn unit(exe: &str) -> String {
    TEMPLATE.replacen(TEMPLATE_EXE, &quote(exe), 1)
}

/// Quotes a path for use as the first word of `ExecStart`. `%` starts a unit
/// specifier, so it is doubled; whitespace, quotes and backslashes need quoting.
fn quote(path: &str) -> String {
    let path = path.replace('%', "%%");
    if path
        .chars()
        .any(|c| c.is_whitespace() || matches!(c, '"' | '\'' | '\\'))
    {
        format!("\"{}\"", path.replace('\\', "\\\\").replace('"', "\\\""))
    } else {
        path
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn template_contains_exe_once() {
        assert_eq!(TEMPLATE.matches(TEMPLATE_EXE).count(), 1);
    }

    #[test]
    fn substitutes_exec_start() {
        let u = unit("/opt/picomon/bin/picomon");
        assert!(u.contains("ExecStart=/opt/picomon/bin/picomon --config"));
        assert!(!u.contains(TEMPLATE_EXE));
    }

    #[test]
    fn quotes_awkward_paths() {
        assert_eq!(quote("/usr/bin/picomon"), "/usr/bin/picomon");
        assert_eq!(quote("/opt/my apps/picomon"), "\"/opt/my apps/picomon\"");
        assert_eq!(quote("/opt/100%/picomon"), "/opt/100%%/picomon");
    }
}
