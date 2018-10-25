// ; Testing cons function
#[test]
fn test00() {
    super::test_it (
        &vec!["
            (cons 1 (list))
        "],
        Some("(1)"),
    );
}

#[test]
fn test01() {
    super::test_it (
        &vec!["
            (cons 1 (list 2))
        "],
        Some("(1 2)"),
    );
}

#[test]
fn test02() {
    super::test_it (
        &vec!["
            (cons 1 (list 2 3))
        "],
        Some("(1 2 3)"),
    );
}

#[test]
fn test03() {
    super::test_it (
        &vec!["
            (cons (list 1) (list 2 3))
        "],
        Some("((1) 2 3)"),
    );
}

#[test]
fn test04() {
    super::test_it (
        &vec!["
            (def! a (list 2 3))
            (cons 1 a)
        "],
        Some("(1 2 3)"),
    );
}

#[test]
fn test05() {
    super::test_it (
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
    super::test_it (
        &vec!["
            (concat)
        "],
        Some("()"),
    );
}

#[test]
fn test07() {
    super::test_it (
        &vec!["
            (concat (list 1 2))
        "],
        Some("(1 2)"),
    );
}

#[test]
fn test08() {
    super::test_it (
        &vec!["
            (concat (list 1 2) (list 3 4))
        "],
        Some("(1 2 3 4)"),
    );
}

#[test]
fn test09() {
    super::test_it (
        &vec!["
            (concat (list 1 2) (list 3 4) (list 5 6))
        "],
        Some("(1 2 3 4 5 6)"),
    );
}

#[test]
fn test10() {
    super::test_it (
        &vec!["
            (concat (concat))
        "],
        Some("()"),
    );
}

#[test]
fn test11() {
    super::test_it (
        &vec!["
            (concat (list) (list))
        "],
        Some("()"),
    );
}

#[test]
fn test12() {
    super::test_it (
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
    super::test_it (
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
    super::test_it (
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
    super::test_it (
        &vec!["
            (quote 7)
        "],
        Some("7"),
    );
}

#[test]
fn test16() {
    super::test_it (
        &vec!["
            (quote (1 2 3))
        "],
        Some("(1 2 3)"),
    );
}

#[test]
fn test17() {
    super::test_it (
        &vec!["
            (quote (1 2 (3 4)))
        "],
        Some("(1 2 (3 4))"),
    );
}

// ; Testing simple quasiquote
#[test]
fn test18() {
    super::test_it (
        &vec!["
            (quasiquote 7)
        "],
        Some("7"),
    );
}

#[test]
fn test19() {
    super::test_it (
        &vec!["
            (quasiquote (1 2 3))
        "],
        Some("(1 2 3)"),
    );
}

#[test]
fn test20() {
    super::test_it (
        &vec!["
            (quasiquote (1 2 (3 4)))
        "],
        Some("(1 2 (3 4))"),
    );
}

#[test]
fn test21() {
    super::test_it (
        &vec!["
            (quasiquote (nil))
        "],
        Some("(nil)"),
    );
}

// ; Testing unquote
#[test]
fn test22() {
    super::test_it (
        &vec!["
            (quasiquote (unquote 7))
        "],
        Some("7"),
    );
}

#[test]
fn test23() {
    super::test_it (
        &vec!["
            (def! a 8)
        "],
        Some("8"),
    );
}

#[test]
fn test24() {
    super::test_it (
        &vec!["
            (def! a 8)
            (quasiquote a)
        "],
        Some("a"),
    );
}

#[test]
fn test25() {
    super::test_it (
        &vec!["
            (def! a 8)
            (quasiquote (unquote a))
        "],
        Some("8"),
    );
}

#[test]
fn test26() {
    super::test_it (
        &vec!["
            (def! a 8)
            (quasiquote (1 a 3))
        "],
        Some("(1 a 3)"),
    );
}

#[test]
fn test27() {
    super::test_it (
        &vec!["
            (def! a 8)
            (quasiquote (1 (unquote a) 3))
        "],
        Some("(1 8 3)"),
    );
}

#[test]
fn test28() {
    super::test_it (
        &vec!["
            (def! b (quote (1 \"b\" \"d\")))
        "],
        Some("(1 \"b\" \"d\")"),
    );
}

#[test]
fn test29() {
    super::test_it (
        &vec!["
            (def! b (quote (1 \"b\" \"d\")))
            (quasiquote (1 b 3))
        "],
        Some("(1 b 3)"),
    );
}

#[test]
fn test30() {
    super::test_it (
        &vec!["
            (def! b (quote (1 \"b\" \"d\")))
            (quasiquote (1 (unquote b) 3))
        "],
        Some("(1 (1 \"b\" \"d\") 3)"),
    );
}

#[test]
fn test31() {
    super::test_it (
        &vec!["
            (quasiquote ((unquote 1) (unquote 2)))
        "],
        Some("(1 2)"),
    );
}

// ; Testing splice-unquote
#[test]
fn test32() {
    super::test_it (
        &vec!["
            (def! c (quote (1 \"b\" \"d\")))
        "],
        Some("(1 \"b\" \"d\")"),
    );
}

#[test]
fn test33() {
    super::test_it (
        &vec!["
            (def! c (quote (1 \"b\" \"d\")))
            (quasiquote (1 c 3))
        "],
        Some("(1 c 3)"),
    );
}

#[test]
fn test34() {
    super::test_it (
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
    super::test_it (
        &vec!["
            (= (quote abc) (quote abc))
        "],
        Some("true"),
    );
}

#[test]
fn test36() {
    super::test_it (
        &vec!["
            (= (quote abc) (quote abcd))
        "],
        Some("false"),
    );
}

#[test]
fn test37() {
    super::test_it (
        &vec!["
            (= (quote abc) \"abc\")
        "],
        Some("false"),
    );
}

#[test]
fn test38() {
    super::test_it (
        &vec!["
            (= \"abc\" (quote abc))
        "],
        Some("false"),
    );
}

#[test]
fn test39() {
    super::test_it (
        &vec!["
            (= \"abc\" (str (quote abc)))
        "],
        Some("true"),
    );
}

#[test]
fn test40() {
    super::test_it (
        &vec!["
            (= (quote abc) nil)
        "],
        Some("false"),
    );
}

#[test]
fn test41() {
    super::test_it (
        &vec!["
            (= nil (quote abc))
        "],
        Some("false"),
    );
}

// ;;;; Test quine

#[test]
fn test41_q() {
    super::test_it (
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
    super::test_it (
        &vec!["
            '7
        "],
        Some("7"),
    );
}

#[test]
fn test43() {
    super::test_it (
        &vec!["
            '(1 2 3)
        "],
        Some("(1 2 3)"),
    );
}

#[test]
fn test44() {
    super::test_it (
        &vec!["
            '(1 2 (3 4))
        "],
        Some("(1 2 (3 4))"),
    );
}

// ; Testing ` (quasiquote) reader macro
#[test]
fn test45() {
    super::test_it (
        &vec!["
            `7
        "],
        Some("7"),
    );
}

#[test]
fn test46() {
    super::test_it (
        &vec!["
            `(1 2 3)
        "],
        Some("(1 2 3)"),
    );
}

#[test]
fn test47() {
    super::test_it (
        &vec!["
            `(1 2 (3 4))
        "],
        Some("(1 2 (3 4))"),
    );
}

#[test]
fn test48() {
    super::test_it (
        &vec!["
            `(nil)
        "],
        Some("(nil)"),
    );
}

// ; Testing ~ (unquote) reader macro
#[test]
fn test49() {
    super::test_it (
        &vec!["
            `~7
        "],
        Some("7"),
    );
}

#[test]
fn test50() {
    super::test_it (
        &vec!["
            (def! a 8)
        "],
        Some("8"),
    );
}

#[test]
fn test51() {
    super::test_it (
        &vec!["
            (def! a 8)
            `(1 ~a 3)
        "],
        Some("(1 8 3)"),
    );
}

#[test]
fn test52() {
    super::test_it (
        &vec!["
            (def! b '(1 \"b\" \"d\"))
        "],
        Some("(1 \"b\" \"d\")"),
    );
}

#[test]
fn test53() {
    super::test_it (
        &vec!["
            (def! b '(1 \"b\" \"d\"))
            `(1 b 3)
        "],
        Some("(1 b 3)"),
    );
}

#[test]
fn test54() {
    super::test_it (
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
    super::test_it (
        &vec!["
            (def! c '(1 \"b\" \"d\"))
        "],
        Some("(1 \"b\" \"d\")"),
    );
}

#[test]
fn test56() {
    super::test_it (
        &vec!["
            (def! c '(1 \"b\" \"d\"))
            `(1 c 3)
        "],
        Some("(1 c 3)"),
    );
}

#[test]
fn test57() {
    super::test_it (
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
    super::test_it (
        &vec!["
            (cons [1] [2 3])
        "],
        Some("([1] 2 3)"),
    );
}

#[test]
fn test59() {
    super::test_it (
        &vec!["
            (cons 1 [2 3])
        "],
        Some("(1 2 3)"),
    );
}

#[test]
fn test60() {
    super::test_it (
        &vec!["
            (concat [1 2] (list 3 4) [5 6])
        "],
        Some("(1 2 3 4 5 6)"),
    );
}

// ; Testing unquote with vectors
#[test]
fn test61() {
    super::test_it (
        &vec!["
            (def! a 8)
        "],
        Some("8"),
    );
}

#[test]
fn test62() {
    super::test_it (
        &vec!["
            (def! a 8)
            `[1 a 3]
        "],
        Some("(1 a 3)"),
    );
}

// ;; TODO: fix this
// ;;;=>[1 a 3]
// ; Testing splice-unquote with vectors
#[test]
fn test63() {
    super::test_it (
        &vec!["
            (def! c '(1 \"b\" \"d\"))
        "],
        Some("(1 \"b\" \"d\")"),
    );
}

#[test]
fn test64() {
    super::test_it (
        &vec!["
            (def! c '(1 \"b\" \"d\"))
            `[1 ~@c 3]
        "],
        Some("(1 1 \"b\" \"d\" 3)"),
    );
}

// ;; TODO: fix this
// ;;;=>[1 1 "b" "d" 3]
