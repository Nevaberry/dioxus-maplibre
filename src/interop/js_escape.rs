//! Small JS-string escaping helpers for bridge code generation.

/// Escape text for use inside a single-quoted JavaScript string literal.
pub(crate) fn js_single_quoted(value: &str) -> String {
    let escaped = value
        .replace('\\', "\\\\")
        .replace('\'', "\\'")
        .replace('\n', "\\n")
        .replace('\r', "\\r");
    format!("'{escaped}'")
}

/// Escape text for use inside a JavaScript template literal body.
pub(crate) fn js_template_literal(value: &str) -> String {
    value
        .replace('\\', "\\\\")
        .replace('`', "\\`")
        .replace("${", "\\${")
}

#[cfg(test)]
mod tests {
    use super::{js_single_quoted, js_template_literal};

    #[test]
    fn single_quoted_escapes_quotes_and_newlines() {
        let value = js_single_quoted("a'b\nc");
        assert_eq!(value, "'a\\'b\\nc'");
    }

    #[test]
    fn template_literal_escapes_backticks_and_interpolation() {
        let value = js_template_literal("`hello ${name}`");
        assert_eq!(value, "\\`hello \\${name}\\`");
    }
}
