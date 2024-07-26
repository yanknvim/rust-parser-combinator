use crate::parser::*;

pub fn many<T>(parser: impl Parser<T>) -> impl Parser<Vec<T>> {
    move |s| {
        let mut result: Vec<T> = Vec::new();

        let mut last_s = s;
        while let Some((c, s)) = parser(last_s.clone()) {
            result.push(c);
            last_s = s;
        }

        Some((result, last_s))
    }
}

pub fn many1<T>(parser: impl Parser<T>) -> impl Parser<Vec<T>> {
    move |s| {
        let mut result: Vec<T> = Vec::new();

        let mut last_s = s;
        if let Some((c, s)) = parser(last_s.clone()) {
            result.push(c);
            last_s = s;
        } else {
            return None
        }

        while let Some((c, s)) = parser(last_s.clone()) {
            result.push(c);
            last_s = s;
        }

        Some((result, last_s))
    }
}

pub fn choice<T>(a: impl Parser<T>, b: impl Parser<T>) -> impl Parser<T> {
    move |s| {
        if let Some((c, s)) = a(s.clone()) {
            Some((c, s))
        } else if let Some((c, s)) = b(s) {
            Some((c, s))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{char::*, combinator::*};

    #[test]
    fn test_many() {
        let result = many(digit1())("123abc".into());
        assert_eq!(result, Some((vec![1, 2, 3], "abc".into())));
    }

    #[test]
    fn test_many1() {
        let result = many1(digit1())("123abc".into());
        assert_eq!(result, Some((vec![1, 2, 3], "abc".into())));

        let result = many1(digit1())("abcxyz".into());
        assert_eq!(result, None);
    }

    #[test]
    fn test_choice() {
        let result = choice(char1('a'), char1('x'))("abc".into());
        assert_eq!(result, Some(('a', "bc".into())));

        let result = choice(char1('a'), char1('x'))("xyz".into());
        assert_eq!(result, Some(('x', "yz".into())));

        let result = choice(char1('a'), char1('x'))("pqr".into());
        assert_eq!(result, None);
    }
} 
