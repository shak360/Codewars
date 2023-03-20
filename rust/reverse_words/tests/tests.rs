#[cfg(test)]
extern crate quickcheck;
#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;
#[cfg(test)]
mod tests {

    use reverse_words::reverse_words;
    use reverse_words::reverse_words2;

    #[test]
    fn test1() {
        let fn_arr: Vec<fn(&str) -> String> = vec![reverse_words, reverse_words];

        for f in fn_arr.iter() {
            println!("{:?}", "\u{80}");
            println!("{:?}", reverse_words("\u{80}"));
            assert_eq!(f("\u{80}"), "\u{80}");
            assert_eq!(f("apple"), "elppa");
            assert_eq!(
                f("The quick brown fox jumps over the lazy dog."),
                "ehT kciuq nworb xof spmuj revo eht yzal .god"
            );
            assert_eq!(f("a b c d"), "a b c d");
            assert_eq!(f("double  spaced  words"), "elbuod  decaps  sdrow");
        }
    }

    macro_rules! assert_reverse_eq {
        ($left:expr, $right:expr) => {
            assert_eq!(reverse_words($left), $right);
            assert_eq!(reverse_words2($left), $right);
        };
    }

    #[test]
    fn both_with_macros() {
        assert_reverse_eq!("\u{80}", "\u{80}")
        assert_reverse_eq!("apple", "elppa");
        assert_reverse_eq!(
            "The quick brown fox jumps over the lazy dog.",
            "ehT kciuq nworb xof spmuj revo eht yzal .god"
        );
        assert_reverse_eq!("a b c d", "a b c d");
        assert_reverse_eq!("double  spaced  words", "elbuod  decaps  sdrow");
    }

    #[quickcheck]
    fn double_reverse_words_is_identity(str: String) -> bool {
        reverse_words(&str) == reverse_words2(&str)
    }
}
