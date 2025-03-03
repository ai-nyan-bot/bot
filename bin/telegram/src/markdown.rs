// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

#[macro_export]
macro_rules! markdown {
    ($($arg:tt)*) => {{
        let res = format!("{}",format_args!($($arg)*));
        res.replace(".", "\\.")
           .replace("|", "\\|")
           .replace("!", "\\!")
           .replace("(", "\\(")
           .replace(")", "\\)")
           .replace("+", "\\+")
    }};
}

mod tests {

    #[test]
    fn test_ok() {
        let symbol = "WSOL/USDT";
        let progress = 42.24;

        let result = markdown!(r#"
️*{symbol}*
is * {progress} % * along the bonding curve and on its way to graduate to Raydium! 🔥🚀
"#
        );
        assert_eq!(result, "\n\u{fe0f}*WSOL/USDT*\nis * 42\\.24 % * along the bonding curve and on its way to graduate to Raydium\\! 🔥🚀\n");
    }

    #[test]
    fn test_dot() {
        let result = markdown!(".");
        assert_eq!(result, "\\.");
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
}
