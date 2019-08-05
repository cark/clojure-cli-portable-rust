enum State{
    Start,
    Parsing,
    InDblQuotes,
    InQuotes,
    BackSlashing,
}

pub fn args_vec(args_string: &str) -> Vec<String> {
    let mut result = Vec::new();
    let mut state = State::Start;
    let mut curr_arg = String::new();
    let mut it = args_string.chars();
    while let Some(c) = it.next() {
        match state {
            State::Start => match c {
                ' ' => (),
                '"' => state = State::InDblQuotes,
                '\'' => state = State::InQuotes,
                c => {
                    state = State::Parsing;
                    curr_arg.push(c);
                }
            },
            State::Parsing => match c {
                ' ' => {
                    state = State::Start;
                    result.push(curr_arg);
                    curr_arg = String::new();
                }
                '\'' => state = State::InQuotes,
                '"' => state = State::InDblQuotes,
                c => curr_arg.push(c),
            },
            State::InDblQuotes => match c {
                '"' => state = State::Parsing,
                '\\' => state = State::BackSlashing,
                c => curr_arg.push(c),
            },
            State::BackSlashing => match c {
                '\\' => {
                    curr_arg.push('\\');
                    state = State::InDblQuotes;
                }
                '"' => {
                    curr_arg.push('"');
                    state = State::InDblQuotes;
                }
                c => {
                    curr_arg.push('\\');
                    curr_arg.push(c);
                    state = State::InDblQuotes;
                }
            },
            State::InQuotes => match c {
                '\'' => state = State::Parsing,
                c => curr_arg.push(c),
            },
        }
    }
    if !curr_arg.is_empty() {
        result.push(curr_arg)
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_args_vec() {
        assert_eq!(Vec::<String>::new(), args_vec(""));
        assert_eq!(vec!["blah"], args_vec("blah"));
        assert_eq!(vec!["blah"], args_vec(" blah"));
        assert_eq!(vec!["blah"], args_vec(" blah "));
        assert_eq!(vec!["foo", "bar", "baz"], args_vec("foo bar baz"));
        assert_eq!(
            vec!["foo", "bar", "baz"],
            args_vec("   foo bar baz")
        );
        assert_eq!(
            vec!["foo", "bar", "baz"],
            args_vec("   foo    bar   baz")
        );
        assert_eq!(
            vec!["foo", "bar", "baz"],
            args_vec("   foo bar    baz   ")
        );
        assert_eq!(vec!["foo"], args_vec("'foo'"));
        assert_eq!(vec!["foo"], args_vec("\"foo\""));
        assert_eq!(vec!["foobar"], args_vec("'foo'bar"));
        assert_eq!(vec!["foobar"], args_vec("foo'bar'"));
        assert_eq!(
            vec!["mamma", "blah\\foo", "tutu"],
            args_vec("mamma \"blah\\\\foo\"  tutu  ")
        );
        assert_eq!(vec!["-Sdeps", "{:deps {nrepl {:mvn/version \"0.6.0\"} refactor-nrepl {:mvn/version \"2.5.0-SNAPSHOT\"} cider/cider-nrepl {:mvn/version \"0.22.0-beta4\"}}}",
            "-m", "nrepl.cmdline", "--middleware", "[\"refactor-nrepl.middleware/wrap-refactor\", \"cider.nrepl/cider-middleware\"]"],
            args_vec("-Sdeps '{:deps {nrepl {:mvn/version \"0.6.0\"} refactor-nrepl {:mvn/version \"2.5.0-SNAPSHOT\"} cider/cider-nrepl {:mvn/version \"0.22.0-beta4\"}}}' -m nrepl.cmdline --middleware '[\"refactor-nrepl.middleware/wrap-refactor\", \"cider.nrepl/cider-middleware\"]'"));
        assert_eq!(vec!["-Sdeps", "{:aliases {:shadow-cljs-inject {:extra-deps {thheller/shadow-cljs {:mvn/version \"2.8.28\"}}}}}",
        "-A:dev:shadow-cljs-inject", "-m", "shadow.cljs.devtools.cli", "--npm", "watch", "test"],
            args_vec("-Sdeps \"{:aliases {:shadow-cljs-inject {:extra-deps {thheller/shadow-cljs {:mvn/version \\\"2.8.28\\\"}}}}}\" -A:dev:shadow-cljs-inject -m shadow.cljs.devtools.cli --npm watch test"));
    }
}
