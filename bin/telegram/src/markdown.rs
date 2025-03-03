// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

#[macro_export]
macro_rules! markdown {
    ($($arg:tt)*) => {{
        let text: String = format!("{}",format_args!($($arg)*));

        let mut result = String::with_capacity(text.len());
        let mut chars = text.chars().peekable();
        let mut skip_next = false;

        while let Some(c) = chars.next() {
            if skip_next {
                result.push(c);
                skip_next = false;
                continue;
            }
    
            if c == ';' {
                skip_next = true;
                continue;
            }
    
            match c {
                '[' | ']' | '(' | ')' | '~' | '`' | '’' | '"' | '\'' | '>' | '#' | '=' | '|' | '{'
                | '}' | '.' | '!' | '*' | '_' | '+' | '-' => {
                    result.push('\\');
                    result.push(c);
                }
                _ => result.push(c),
            }
        }
    
        result
    }};
}

mod tests {

    #[test]
    fn test_ok() {
        let symbol = "WSOL/USDT";
        let progress = 42.24;

        let result = markdown!(
            r#"
️;*{symbol};*
is ;* {progress} % ;* along the bonding curve and on its way to graduate to Raydium! 🔥🚀
"#
        );
        assert_eq!(result, "\n\u{fe0f}*WSOL/USDT*\nis * 42\\.24 % * along the bonding curve and on its way to graduate to Raydium\\! 🔥🚀\n");
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
    }

    #[test]
    fn test_exclamation_mark() {
        let result = markdown!("!");
        assert_eq!(result, "\\!");
    }

    #[test]
    fn test_paren() {
        let result = markdown!("()");
        assert_eq!(result, "\\(\\)");
    }

    #[test]
    fn test_plus() {
        let result = markdown!("+");
        assert_eq!(result, "\\+");
    }

    #[test]
    fn test_minus() {
        let result = markdown!("-");
        assert_eq!(result, "\\-");
    }

    #[test]
    fn test_apostrophes() {
        let result = markdown!(r#"’'""#);
        assert_eq!(result, "\\’\\'\\\"");
    }
}
