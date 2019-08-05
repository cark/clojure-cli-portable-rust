use std::iter::FromIterator;
//use std::ops::Add;

enum ASState {
    Start,
    Parsing,
    InDblQuotes,
    IgnorableSpace,
}

pub fn args_string(command_line: &str) -> String {
    let mut it = command_line.chars().peekable();
    let mut state = ASState::Start;
    while let Some(c) = it.peek() {
        match state {
            ASState::Start => match c {
                ' ' => (),
                '"' => state = ASState::InDblQuotes,
                _c => state = ASState::Parsing,
            },
            ASState::Parsing => match c {
                '"' => state = ASState::InDblQuotes,
                ' ' => state = ASState::IgnorableSpace,
                _c => (),
            },
            ASState::InDblQuotes => match c {
                '"' => state = ASState::IgnorableSpace,
                _c => (),
            },
            ASState::IgnorableSpace => match c {
                ' ' => (),
                _c => break,
            },
        }
        let _ = it.next();
    }
    String::from_iter(it)
}

enum ASVState {
    Start,
    Parsing,
    InDblQuotes,
    InQuotes,
    BackSlashing,
}

pub fn args_string_to_vec(args_string: &str) -> Vec<String> {
    let mut result = Vec::new();
    let mut state = ASVState::Start;
    let mut curr_arg = String::new();
    let mut it = args_string.chars().peekable();
    while let Some(c) = it.peek() {
        match state {
            ASVState::Start => match c {
                ' ' => (),
                '"' => state = ASVState::InDblQuotes,
                '\'' => state = ASVState::InQuotes,
                c => {
                    state = ASVState::Parsing;
                    curr_arg.push(*c);
                }
            },
            ASVState::Parsing => match c {
                ' ' => {
                    state = ASVState::Start;
                    result.push(curr_arg);
                    curr_arg = String::new();
                }
                '\'' => state = ASVState::InQuotes,
                '"' => state = ASVState::InDblQuotes,
                c => curr_arg.push(*c),
            },
            ASVState::InDblQuotes => match c {
                '"' => state = ASVState::Parsing,
                '\\' => state = ASVState::BackSlashing,
                c => curr_arg.push(*c),
            },
            ASVState::BackSlashing => match c {
                '\\' => {
                    curr_arg.push('\\');
                    state = ASVState::InDblQuotes;
                }
                '"' => {
                    curr_arg.push('"');
                    state = ASVState::InDblQuotes;
                }
                c => {
                    curr_arg.push('\\');
                    curr_arg.push(*c);
                    state = ASVState::InDblQuotes;
                }
            },
            ASVState::InQuotes => match c {
                '\'' => state = ASVState::Parsing,
                c => curr_arg.push(*c),
            },
        }
        let _ = it.next();
    }
    if !curr_arg.is_empty() {
        result.push(curr_arg);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_args_string() {
        assert_eq!("", args_string(""));
        assert_eq!("", args_string("blah"));
        assert_eq!("blah", args_string("blah blah"));
        assert_eq!("arg1 arg2", args_string("clojure.exe arg1 arg2"));
        assert_eq!("", args_string("\"clojure.exe\""));
        assert_eq!("arg1 arg2", args_string("\"clojure.exe\" arg1 arg2"));
        assert_eq!("arg1 arg2", args_string("   \"clojure.exe\" arg1 arg2"));
        assert_eq!(
            "arg1 arg2",
            args_string("   \"clojure.exe\"      arg1 arg2")
        );
        assert_eq!(
            "\"arg1 arg2\"",
            args_string("   \"clojure.exe\"      \"arg1 arg2\"")
        );
    }

    #[test]
    fn test_args_string_to_vec() {
        assert_eq!(Vec::<String>::new(), args_string_to_vec(""));
        assert_eq!(vec!["blah"], args_string_to_vec("blah"));
        assert_eq!(vec!["blah"], args_string_to_vec(" blah"));
        assert_eq!(vec!["blah"], args_string_to_vec(" blah "));
        assert_eq!(vec!["foo", "bar", "baz"], args_string_to_vec("foo bar baz"));
        assert_eq!(
            vec!["foo", "bar", "baz"],
            args_string_to_vec("   foo bar baz")
        );
        assert_eq!(
            vec!["foo", "bar", "baz"],
            args_string_to_vec("   foo    bar   baz")
        );
        assert_eq!(
            vec!["foo", "bar", "baz"],
            args_string_to_vec("   foo bar    baz   ")
        );
        assert_eq!(vec!["foo"], args_string_to_vec("'foo'"));
        assert_eq!(vec!["foo"], args_string_to_vec("\"foo\""));
        assert_eq!(vec!["foobar"], args_string_to_vec("'foo'bar"));
        assert_eq!(vec!["foobar"], args_string_to_vec("foo'bar'"));
        assert_eq!(
            vec!["mamma", "blah\\foo", "tutu"],
            args_string_to_vec("mamma \"blah\\\\foo\"  tutu  ")
        );
        assert_eq!(vec!["-Sdeps", "{:deps {nrepl {:mvn/version \"0.6.0\"} refactor-nrepl {:mvn/version \"2.5.0-SNAPSHOT\"} cider/cider-nrepl {:mvn/version \"0.22.0-beta4\"}}}",
            "-m", "nrepl.cmdline", "--middleware", "[\"refactor-nrepl.middleware/wrap-refactor\", \"cider.nrepl/cider-middleware\"]"],
            args_string_to_vec("-Sdeps '{:deps {nrepl {:mvn/version \"0.6.0\"} refactor-nrepl {:mvn/version \"2.5.0-SNAPSHOT\"} cider/cider-nrepl {:mvn/version \"0.22.0-beta4\"}}}' -m nrepl.cmdline --middleware '[\"refactor-nrepl.middleware/wrap-refactor\", \"cider.nrepl/cider-middleware\"]'"));
        assert_eq!(vec!["-Sdeps", "{:aliases {:shadow-cljs-inject {:extra-deps {thheller/shadow-cljs {:mvn/version \"2.8.28\"}}}}}",
        "-A:dev:shadow-cljs-inject", "-m", "shadow.cljs.devtools.cli", "--npm", "watch", "test"],
            args_string_to_vec("-Sdeps \"{:aliases {:shadow-cljs-inject {:extra-deps {thheller/shadow-cljs {:mvn/version \\\"2.8.28\\\"}}}}}\" -A:dev:shadow-cljs-inject -m shadow.cljs.devtools.cli --npm watch test"));
    }
}
