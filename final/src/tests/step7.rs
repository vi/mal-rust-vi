// ; Testing cons function
#[test]
fn test00() {
    crate::test_it (
        &vec!["
            (cons 1 (list))
        "],
        Some("(1)"),
    );
}

#[test]
fn test01() {
    crate::test_it (
        &vec!["
            (cons 1 (list 2))
        "],
        Some("(1 2)"),
    );
}

#[test]
fn test02() {
    crate::test_it (
        &vec!["
            (cons 1 (list 2 3))
        "],
        Some("(1 2 3)"),
    );
}

#[test]
fn test03() {
    crate::test_it (
        &vec!["
            (cons (list 1) (list 2 3))
        "],
        Some("((1) 2 3)"),
    );
}

#[test]
fn test04() {
    crate::test_it (
        &vec!["
            (def! a (list 2 3))
            (cons 1 a)
        "],
        Some("(1 2 3)"),
    );
}

#[test]
fn test05() {
    crate::test_it (
        &vec!["
            (def! a (list 2 3))
            a
        "],
        Some("(2 3)"),
    );
}

// ; Testing concat function
#[test]
fn test06() {
    crate::test_it (
        &vec!["
            (concat)
        "],
        Some("()"),
    );
}

#[test]
fn test07() {
    crate::test_it (
        &vec!["
            (concat (list 1 2))
        "],
        Some("(1 2)"),
    );
}

#[test]
fn test08() {
    crate::test_it (
        &vec!["
            (concat (list 1 2) (list 3 4))
        "],
        Some("(1 2 3 4)"),
    );
}

#[test]
fn test09() {
    crate::test_it (
        &vec!["
            (concat (list 1 2) (list 3 4) (list 5 6))
        "],
        Some("(1 2 3 4 5 6)"),
    );
}

#[test]
fn test10() {
    crate::test_it (
        &vec!["
            (concat (concat))
        "],
        Some("()"),
    );
}

#[test]
fn test11() {
    crate::test_it (
        &vec!["
            (concat (list) (list))
        "],
        Some("()"),
    );
}

#[test]
fn test12() {
    crate::test_it (
        &vec!["
            (def! a (list 1 2))
            (def! b (list 3 4))
            (concat a b (list 5 6))
        "],
        Some("(1 2 3 4 5 6)"),
    );
}

#[test]
fn test13() {
    crate::test_it (
        &vec!["
            (def! a (list 1 2))
            (def! b (list 3 4))
            a
        "],
        Some("(1 2)"),
    );
}

#[test]
fn test14() {
    crate::test_it (
        &vec!["
            (def! a (list 1 2))
            (def! b (list 3 4))
            b
        "],
        Some("(3 4)"),
    );
}

// ; Testing regular quote
#[test]
fn test15() {
    crate::test_it (
        &vec!["
            (quote 7)
        "],
        Some("7"),
    );
}

#[test]
fn test16() {
    crate::test_it (
        &vec!["
            (quote (1 2 3))
        "],
        Some("(1 2 3)"),
    );
}

#[test]
fn test17() {
    crate::test_it (
        &vec!["
            (quote (1 2 (3 4)))
        "],
        Some("(1 2 (3 4))"),
    );
}

// ; Testing simple quasiquote
#[test]
fn test18() {
    crate::test_it (
        &vec!["
            (quasiquote 7)
        "],
        Some("7"),
    );
}

#[test]
fn test19() {
    crate::test_it (
        &vec!["
            (quasiquote (1 2 3))
        "],
        Some("(1 2 3)"),
    );
}

#[test]
fn test20() {
    crate::test_it (
        &vec!["
            (quasiquote (1 2 (3 4)))
        "],
        Some("(1 2 (3 4))"),
    );
}

#[test]
fn test21() {
    crate::test_it (
        &vec!["
            (quasiquote (nil))
        "],
        Some("(nil)"),
    );
}

// ; Testing unquote
#[test]
fn test22() {
    crate::test_it (
        &vec!["
            (quasiquote (unquote 7))
        "],
        Some("7"),
    );
}

#[test]
fn test23() {
    crate::test_it (
        &vec!["
            (def! a 8)
        "],
        Some("8"),
    );
}

#[test]
fn test24() {
    crate::test_it (
        &vec!["
            (def! a 8)
            (quasiquote a)
        "],
        Some("a"),
    );
}

#[test]
fn test25() {
    crate::test_it (
        &vec!["
            (def! a 8)
            (quasiquote (unquote a))
        "],
        Some("8"),
    );
}

#[test]
fn test26() {
    crate::test_it (
        &vec!["
            (def! a 8)
            (quasiquote (1 a 3))
        "],
        Some("(1 a 3)"),
    );
}

#[test]
fn test27() {
    crate::test_it (
        &vec!["
            (def! a 8)
            (quasiquote (1 (unquote a) 3))
        "],
        Some("(1 8 3)"),
    );
}

#[test]
fn test28() {
    crate::test_it (
        &vec!["
            (def! b (quote (1 \"b\" \"d\")))
        "],
        Some("(1 \"b\" \"d\")"),
    );
}

#[test]
fn test29() {
    crate::test_it (
        &vec!["
            (def! b (quote (1 \"b\" \"d\")))
            (quasiquote (1 b 3))
        "],
        Some("(1 b 3)"),
    );
}

#[test]
fn test30() {
    crate::test_it (
        &vec!["
            (def! b (quote (1 \"b\" \"d\")))
            (quasiquote (1 (unquote b) 3))
        "],
        Some("(1 (1 \"b\" \"d\") 3)"),
    );
}

#[test]
fn test31() {
    crate::test_it (
        &vec!["
            (quasiquote ((unquote 1) (unquote 2)))
        "],
        Some("(1 2)"),
    );
}

// ; Testing splice-unquote
#[test]
fn test32() {
    crate::test_it (
        &vec!["
            (def! c (quote (1 \"b\" \"d\")))
        "],
        Some("(1 \"b\" \"d\")"),
    );
}

#[test]
fn test33() {
    crate::test_it (
        &vec!["
            (def! c (quote (1 \"b\" \"d\")))
            (quasiquote (1 c 3))
        "],
        Some("(1 c 3)"),
    );
}

#[test]
fn test34() {
    crate::test_it (
        &vec!["
            (def! c (quote (1 \"b\" \"d\")))
            (quasiquote (1 (splice-unquote c) 3))
        "],
        Some("(1 1 \"b\" \"d\" 3)"),
    );
}

// ; Testing symbol equality
#[test]
fn test35() {
    crate::test_it (
        &vec!["
            (= (quote abc) (quote abc))
        "],
        Some("true"),
    );
}

#[test]
fn test36() {
    crate::test_it (
        &vec!["
            (= (quote abc) (quote abcd))
        "],
        Some("false"),
    );
}

#[test]
fn test37() {
    crate::test_it (
        &vec!["
            (= (quote abc) \"abc\")
        "],
        Some("false"),
    );
}

#[test]
fn test38() {
    crate::test_it (
        &vec!["
            (= \"abc\" (quote abc))
        "],
        Some("false"),
    );
}

#[test]
fn test39() {
    crate::test_it (
        &vec!["
            (= \"abc\" (str (quote abc)))
        "],
        Some("true"),
    );
}

#[test]
fn test40() {
    crate::test_it (
        &vec!["
            (= (quote abc) nil)
        "],
        Some("false"),
    );
}

#[test]
fn test41() {
    crate::test_it (
        &vec!["
            (= nil (quote abc))
        "],
        Some("false"),
    );
}

// ;;;; Test quine

#[test]
fn test41_q() {
    crate::test_it (
        &vec!["
            (
                (fn* [q] 
                    (quasiquote 
                        ((unquote q) (quote (unquote q)))
                    )
                )
                (quote 
                    (fn* [q] 
                        (quasiquote 
                            ((unquote q) (quote (unquote q)))
                        )
                    )
                )
            )
        "],
        Some("((fn* [q] (quasiquote ((unquote q) (quote (unquote q))))) (quote (fn* [q] (quasiquote ((unquote q) (quote (unquote q)))))))"),
    );
}

// >>> deferrable=True
// ;
// ; -------- Deferrable Functionality --------
// ; Testing ' (quote) reader macro
#[test]
fn test42() {
    crate::test_it (
        &vec!["
            '7
        "],
        Some("7"),
    );
}

#[test]
fn test43() {
    crate::test_it (
        &vec!["
            '(1 2 3)
        "],
        Some("(1 2 3)"),
    );
}

#[test]
fn test44() {
    crate::test_it (
        &vec!["
            '(1 2 (3 4))
        "],
        Some("(1 2 (3 4))"),
    );
}

// ; Testing ` (quasiquote) reader macro
#[test]
fn test45() {
    crate::test_it (
        &vec!["
            `7
        "],
        Some("7"),
    );
}

#[test]
fn test46() {
    crate::test_it (
        &vec!["
            `(1 2 3)
        "],
        Some("(1 2 3)"),
    );
}

#[test]
fn test47() {
    crate::test_it (
        &vec!["
            `(1 2 (3 4))
        "],
        Some("(1 2 (3 4))"),
    );
}

#[test]
fn test48() {
    crate::test_it (
        &vec!["
            `(nil)
        "],
        Some("(nil)"),
    );
}

// ; Testing ~ (unquote) reader macro
#[test]
fn test49() {
    crate::test_it (
        &vec!["
            `~7
        "],
        Some("7"),
    );
}

#[test]
fn test50() {
    crate::test_it (
        &vec!["
            (def! a 8)
        "],
        Some("8"),
    );
}

#[test]
fn test51() {
    crate::test_it (
        &vec!["
            (def! a 8)
            `(1 ~a 3)
        "],
        Some("(1 8 3)"),
    );
}

#[test]
fn test52() {
    crate::test_it (
        &vec!["
            (def! b '(1 \"b\" \"d\"))
        "],
        Some("(1 \"b\" \"d\")"),
    );
}

#[test]
fn test53() {
    crate::test_it (
        &vec!["
            (def! b '(1 \"b\" \"d\"))
            `(1 b 3)
        "],
        Some("(1 b 3)"),
    );
}

#[test]
fn test54() {
    crate::test_it (
        &vec!["
            (def! b '(1 \"b\" \"d\"))
            `(1 ~b 3)
        "],
        Some("(1 (1 \"b\" \"d\") 3)"),
    );
}

// ; Testing ~@ (splice-unquote) reader macro
#[test]
fn test55() {
    crate::test_it (
        &vec!["
            (def! c '(1 \"b\" \"d\"))
        "],
        Some("(1 \"b\" \"d\")"),
    );
}

#[test]
fn test56() {
    crate::test_it (
        &vec!["
            (def! c '(1 \"b\" \"d\"))
            `(1 c 3)
        "],
        Some("(1 c 3)"),
    );
}

#[test]
fn test57() {
    crate::test_it (
        &vec!["
            (def! c '(1 \"b\" \"d\"))
            `(1 ~@c 3)
        "],
        Some("(1 1 \"b\" \"d\" 3)"),
    );
}

// >>> optional=True
// ;
// ; -------- Optional Functionality --------
// ; Testing cons, concat, first, rest with vectors
#[test]
fn test58() {
    crate::test_it (
        &vec!["
            (cons [1] [2 3])
        "],
        Some("([1] 2 3)"),
    );
}

#[test]
fn test59() {
    crate::test_it (
        &vec!["
            (cons 1 [2 3])
        "],
        Some("(1 2 3)"),
    );
}

#[test]
fn test60() {
    crate::test_it (
        &vec!["
            (concat [1 2] (list 3 4) [5 6])
        "],
        Some("(1 2 3 4 5 6)"),
    );
}

// ; Testing unquote with vectors
#[test]
fn test61() {
    crate::test_it (
        &vec!["
            (def! a 8)
        "],
        Some("8"),
    );
}

#[test]
fn test62() {
    crate::test_it (
        &vec!["
            (def! a 8)
            `[1 a 3]
        "],
        Some("[1 a 3]"),
    );
}

// ; Testing splice-unquote with vectors
#[test]
fn test63() {
    crate::test_it (
        &vec!["
            (def! c '(1 \"b\" \"d\"))
        "],
        Some("(1 \"b\" \"d\")"),
    );
}

#[test]
fn test64() {
    crate::test_it (
        &vec!["
            (def! c '(1 \"b\" \"d\"))
            `[1 ~@c 3]
        "],
        Some("[1 1 \"b\" \"d\" 3]"),
    );
}

