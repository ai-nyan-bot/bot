// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

#[macro_export]
macro_rules! markdown {
    ($($arg:tt)*) => {{
        let text: String = format!("{}", format_args!($($arg)*));

        let mut result = String::with_capacity(text.len());
        let mut chars = text.chars().peekable();
        let mut skip_next = false;
        let mut current_line = String::new();

        while let Some(c) = chars.next() {
            if skip_next {
                current_line.push(c);
                skip_next = false;
                continue;
            }

            if c == ';' {
                skip_next = true;
                continue;
            }

            if c == '\n' {
                result.push_str(current_line.trim());
                result.push('\n');
                current_line.clear();
                continue;
            }

            match c {
                '[' | ']' | '(' | ')' | '~' | '`' | '"' | '\'' | '>' | '#' | '=' | '|' | '{'
                | '}' | '.' | '!' | '*' | '_' | '+' | '-' => {
                    current_line.push('\\');
                    current_line.push(c);
                }
                _ => current_line.push(c),
            }
        }

        if !current_line.is_empty() {
            result.push_str(current_line.trim());
        }

        result.trim().to_string()
    }};
}
mod tests {

    #[test]
    fn test_ok() {
        let symbol = "WSOL/USDT";
        let progress = 42.24;

        let result = markdown!(
            r#"
            ;*{symbol};*
            is ;* {progress} % ;* along the bonding curve and on its way to graduate to Raydium! 🔥🚀
        "#
        );
        assert_eq!(result, "*WSOL/USDT*\nis * 42\\.24 % * along the bonding curve and on its way to graduate to Raydium\\! 🔥🚀");
    }

    #[test]
    fn test_dot() {
        let result = markdown!(".");
        assert_eq!(result, "\\.");

        let result = markdown!(";.");
        assert_eq!(result, ".");
    }

    #[test]
    fn test_pipe() {
        let result = markdown!("|");
        assert_eq!(result, "\\|");

        let result = markdown!(";|");
        assert_eq!(result, "|");
    }

    #[test]
    fn test_exclamation_mark() {
        let result = markdown!("!");
        assert_eq!(result, "\\!");

        let result = markdown!(";!");
        assert_eq!(result, "!");
    }

    #[test]
    fn test_paren() {
        let result = markdown!("()");
        assert_eq!(result, "\\(\\)");

        let result = markdown!(";(;)");
        assert_eq!(result, "()");
    }

    #[test]
    fn test_plus() {
        let result = markdown!("+");
        assert_eq!(result, "\\+");

        let result = markdown!(";+");
        assert_eq!(result, "+");
    }

    #[test]
    fn test_minus() {
        let result = markdown!("-");
        assert_eq!(result, "\\-");

        let result = markdown!(";-");
        assert_eq!(result, "-");
    }

    #[test]
    fn test_asterisk() {
        let result = markdown!("*");
        assert_eq!(result, "\\*");

        let result = markdown!(";*");
        assert_eq!(result, "*");
    }

    #[test]
    fn test_apostrophes() {
        let result = markdown!(r#"'""#);
        assert_eq!(result, "\\'\\\"");

        let result = markdown!(r#";';""#);
        assert_eq!(result, r#"'""#);
    }
}
