use crate::parser::*;

pub fn any_char() -> impl Parser<char> {
    move |s| {
        if let Some(c) = s.chars().next() {
            Some((c, s[1..].to_string()))
        } else {
            None
        }
    }
}

pub fn char1(c: char) -> impl Parser<char> {
    move |s| {
        let mut chars = s.chars();
        if chars.next() == Some(c) {
            Some((c, s[1..].to_string()))
        } else {
            None
        }
    }
}

pub fn skip_whitespace() -> impl Parser<()> {
    move |s| Some(((), s.trim_start().to_string()))
}

pub fn digit1() -> impl Parser<u32> {
    move |s| match s.chars().next() {
        Some(c) if c.is_ascii_digit() => Some((c.to_digit(10).unwrap(), s[1..].to_string())),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use crate::char::*;

    #[test]
    fn test_any_char() {
        let result = any_char()("abc".into());
        assert_eq!(result, Some(('a', "bc".into())));

        let result = any_char()("".into());
        assert_eq!(result, None);
    }

    #[test]
    fn test_char1() {
        let result = char1('A')("Abc".into());
        assert_eq!(result, Some(('A', "bc".into())));

        let result = char1('B')("Cde".into());
        assert_eq!(result, None);
    }

    #[test]
    fn test_skip_whitespace() {
        let result = skip_whitespace()("    abc".into());
        assert_eq!(result, Some(((), "abc".into())));
    }

    #[test]
    fn test_digit1() {
        let result = digit1()("123".into());
        assert_eq!(result, Some((1, "23".into())));

        let result = digit1()("abc".into());
        assert_eq!(result, None);
    }
}
