// ; Testing read of numbers
#[test]
fn test0() {
    super::test_it (
        "1",
        Some("1"),
    );
}

#[test]
fn test1() {
    super::test_it (
        "7",
        Some("7"),
    );
}

#[test]
fn test2() {
    super::test_it (
        "  7   ",
        Some("7"),
    );
}

#[test]
fn test3() {
    super::test_it (
        "-123",
        Some("-123"),
    );
}

// ; Testing read of symbols
#[test]
fn test4() {
    super::test_it (
        "+",
        Some("+"),
    );
}

#[test]
fn test5() {
    super::test_it (
        "abc",
        Some("abc"),
    );
}

#[test]
fn test6() {
    super::test_it (
        "   abc   ",
        Some("abc"),
    );
}

#[test]
fn test7() {
    super::test_it (
        "abc5",
        Some("abc5"),
    );
}

#[test]
fn test8() {
    super::test_it (
        "abc-def",
        Some("abc-def"),
    );
}

// ; Testing read of lists
#[test]
fn test9() {
    super::test_it (
        "(+ 1 2)",
        Some("(+ 1 2)"),
    );
}

#[test]
fn test10() {
    super::test_it (
        "()",
        Some("()"),
    );
}

#[test]
fn test11() {
    super::test_it (
        "(nil)",
        Some("(nil)"),
    );
}

#[test]
fn test12() {
    super::test_it (
        "((3 4))",
        Some("((3 4))"),
    );
}

#[test]
fn test13() {
    super::test_it (
        "(+ 1 (+ 2 3))",
        Some("(+ 1 (+ 2 3))"),
    );
}

#[test]
fn test14() {
    super::test_it (
        "  ( +   1   (+   2 3   )   )  ",
        Some("(+ 1 (+ 2 3))"),
    );
}

#[test]
fn test15() {
    super::test_it (
        "(* 1 2)",
        Some("(* 1 2)"),
    );
}

#[test]
fn test16() {
    super::test_it (
        "(** 1 2)",
        Some("(** 1 2)"),
    );
}

#[test]
fn test17() {
    super::test_it (
        "(* -3 6)",
        Some("(* -3 6)"),
    );
}

// ; Test commas as whitespace
#[test]
fn test18() {
    super::test_it (
        "(1 2, 3,,,,),,",
        Some("(1 2 3)"),
    );
}

// >>> deferrable=True
// ;
// ; -------- Deferrable Functionality --------
// ; Testing read of nil/true/false
#[test]
fn test19() {
    super::test_it (
        "nil",
        Some("nil"),
    );
}

#[test]
fn test20() {
    super::test_it (
        "true",
        Some("true"),
    );
}

#[test]
fn test21() {
    super::test_it (
        "false",
        Some("false"),
    );
}

// ; Testing read of strings
#[test]
fn test22() {
    super::test_it (
        "\"abc\"",
        Some("\"abc\""),
    );
}

#[test]
fn test23() {
    super::test_it (
        "   \"abc\"   ",
        Some("\"abc\""),
    );
}

#[test]
fn test24() {
    super::test_it (
        "\"abc (with parens)\"",
        Some("\"abc (with parens)\""),
    );
}

#[test]
fn test25() {
    super::test_it (
        "\"abc\\\"def\"",
        Some("\"abc\\\"def\""),
    );
}

// ;;"abc\ndef"
// ;;;=>"abc\ndef"
#[test]
fn test26() {
    super::test_it (
        "\"\"",
        Some("\"\""),
    );
}

// ; Testing reader errors

#[test]
fn testf1() {
    super::test_it(
        "(1 2",
        None,
    )
}

#[test]
fn testf2() {
    super::test_it(
        "\"abc",
        None,
    )
}

#[test]
fn testf3() {
    super::test_it(
        "[1 2",
        None,
    )
}

#[test]
fn testf4() {
    super::test_it(
        "(1 \"abc",
        None,
    )
}

// ; Testing read of quoting
#[test]
fn test27() {
    super::test_it (
        "'1",
        Some("(quote 1)"),
    );
}

#[test]
fn test28() {
    super::test_it (
        "'(1 2 3)",
        Some("(quote (1 2 3))"),
    );
}

#[test]
fn test29() {
    super::test_it (
        "`1",
        Some("(quasiquote 1)"),
    );
}

#[test]
fn test30() {
    super::test_it (
        "`(1 2 3)",
        Some("(quasiquote (1 2 3))"),
    );
}

#[test]
fn test31() {
    super::test_it (
        "~1",
        Some("(unquote 1)"),
    );
}

#[test]
fn test32() {
    super::test_it (
        "~(1 2 3)",
        Some("(unquote (1 2 3))"),
    );
}

#[test]
fn test33() {
    super::test_it (
        "`(1 ~a 3)",
        Some("(quasiquote (1 (unquote a) 3))"),
    );
}

#[test]
fn test34() {
    super::test_it (
        "~@(1 2 3)",
        Some("(splice-unquote (1 2 3))"),
    );
}

// >>> optional=True
// ;
// ; -------- Optional Functionality --------
// ; Testing keywords
#[test]
fn test35() {
    super::test_it (
        ":kw",
        Some(":kw"),
    );
}

#[test]
fn test36() {
    super::test_it (
        "(:kw1 :kw2 :kw3)",
        Some("(:kw1 :kw2 :kw3)"),
    );
}

// ; Testing read of vectors
#[test]
fn test37() {
    super::test_it (
        "[+ 1 2]",
        Some("[+ 1 2]"),
    );
}

#[test]
fn test38() {
    super::test_it (
        "[]",
        Some("[]"),
    );
}

#[test]
fn test39() {
    super::test_it (
        "[[3 4]]",
        Some("[[3 4]]"),
    );
}

#[test]
fn test40() {
    super::test_it (
        "[+ 1 [+ 2 3]]",
        Some("[+ 1 [+ 2 3]]"),
    );
}

#[test]
fn test41() {
    super::test_it (
        "  [ +   1   [+   2 3   ]   ]  ",
        Some("[+ 1 [+ 2 3]]"),
    );
}

// ; Testing read of hash maps
#[test]
fn test42() {
    super::test_it (
        "{\"abc\" 1}",
        Some("{\"abc\" 1}"),
    );
}

#[test]
fn test43() {
    super::test_it (
        "{\"a\" {\"b\" 2}}",
        Some("{\"a\" {\"b\" 2}}"),
    );
}

#[test]
fn test44() {
    super::test_it (
        "{\"a\" {\"b\" {\"c\" 3}}}",
        Some("{\"a\" {\"b\" {\"c\" 3}}}"),
    );
}

#[test]
fn test45() {
    super::test_it (
        "{  \"a\"  {\"b\"   {  \"cde\"     3   }  }}",
        Some("{\"a\" {\"b\" {\"cde\" 3}}}"),
    );
}

#[test]
fn test46() {
    super::test_it (
        "{  :a  {:b   {  :cde     3   }  }}",
        Some("{:a {:b {:cde 3}}}"),
    );
}

// ; Testing read of comments
#[test]
fn test47() {
    super::test_it (
        "1 ; comment after expression",
        Some("1"),
    );
}

#[test]
fn test48() {
    super::test_it (
        "1; comment after expression",
        Some("1"),
    );
}

// ; Testing read of ^/metadata
#[test]
fn test49() {
    super::test_it (
        "^{\"a\" 1} [1 2 3]",
        Some("(with-meta [1 2 3] {\"a\" 1})"),
    );
}

// ; Testing read of @/deref
#[test]
fn test50() {
    super::test_it (
        "@a",
        Some("(deref a)"),
    );
}

