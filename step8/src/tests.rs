// ; Testing trivial macros
#[test]
fn test00() {
    super::test_it (
        &vec!["
            (defmacro! one (fn* () 1))
            (one)
        "],
        Some("1"),
    );
}

#[test]
fn test01() {
    super::test_it (
        &vec!["
            (defmacro! two (fn* () 2))
            (two)
        "],
        Some("2"),
    );
}

// ; Testing unless macros
#[test]
fn test02() {
    super::test_it (
        &vec!["
            (defmacro! unless (fn* (pred a b) `(if ~pred ~b ~a)))
            (unless false 7 8)
        "],
        Some("7"),
    );
}

#[test]
fn test03() {
    super::test_it (
        &vec!["
            (defmacro! unless (fn* (pred a b) `(if ~pred ~b ~a)))
            (unless true 7 8)
        "],
        Some("8"),
    );
}

#[test]
fn test04() {
    super::test_it (
        &vec!["
            (defmacro! unless2 (fn* (pred a b) `(if (not ~pred) ~a ~b)))
            (unless2 false 7 8)
        "],
        Some("7"),
    );
}

#[test]
fn test05() {
    super::test_it (
        &vec!["
            (defmacro! unless2 (fn* (pred a b) `(if (not ~pred) ~a ~b)))
            (unless2 true 7 8)
        "],
        Some("8"),
    );
}

// ; Testing macroexpand
#[test]
fn test06() {
    super::test_it (
        &vec!["
            (defmacro! unless2 (fn* (pred a b) `(if (not ~pred) ~a ~b)))
            (macroexpand (unless2 2 3 4))
        "],
        Some("(if (not 2) 3 4)"),
    );
}

// ; Testing evaluation of macro result
#[test]
fn test07() {
    super::test_it (
        &vec!["
            (defmacro! identity (fn* (x) x))
            (let* (a 123) (identity a))
        "],
        Some("123"),
    );
}

// >>> deferrable=True
// ;
// ; -------- Deferrable Functionality --------
// ; Testing non-macro function
#[test]
fn test08() {
    super::test_it (
        &vec!["
            (not (= 1 1))
        "],
        Some("false"),
    );
}

// ;; This should fail if it is a macro
#[test]
fn test09() {
    super::test_it (
        &vec!["
            (not (= 1 2))
        "],
        Some("true"),
    );
}

// ; Testing nth, first and rest functions
#[test]
fn test10() {
    super::test_it (
        &vec!["
            (nth (list 1) 0)
        "],
        Some("1"),
    );
}

#[test]
fn test11() {
    super::test_it (
        &vec!["
            (nth (list 1 2) 1)
        "],
        Some("2"),
    );
}

#[test]
fn test12() {
    super::test_it (
        &vec![
            "(def! x \"x\")",
            "(def! x (nth (list 1 2) 2))",
            "x",
        ],
        Some("\"x\""),
    );
}

#[test]
fn test13() {
    super::test_it (
        &vec!["
            (first (list))
        "],
        Some("nil"),
    );
}

#[test]
fn test14() {
    super::test_it (
        &vec!["
            (first (list 6))
        "],
        Some("6"),
    );
}

#[test]
fn test15() {
    super::test_it (
        &vec!["
            (first (list 7 8 9))
        "],
        Some("7"),
    );
}

#[test]
fn test16() {
    super::test_it (
        &vec!["
            (rest (list))
        "],
        Some("()"),
    );
}

#[test]
fn test17() {
    super::test_it (
        &vec!["
            (rest (list 6))
        "],
        Some("()"),
    );
}

#[test]
fn test18() {
    super::test_it (
        &vec!["
            (rest (list 7 8 9))
        "],
        Some("(8 9)"),
    );
}

// ; Testing or macro
#[test]
fn test19() {
    super::test_it (
        &vec!["
            (or)
        "],
        Some("nil"),
    );
}

#[test]
fn test20() {
    super::test_it (
        &vec!["
            (or 1)
        "],
        Some("1"),
    );
}

#[test]
fn test21() {
    super::test_it (
        &vec!["
            (or 1 2 3 4)
        "],
        Some("1"),
    );
}

#[test]
fn test22() {
    super::test_it (
        &vec!["
            (or false 2)
        "],
        Some("2"),
    );
}

#[test]
fn test23() {
    super::test_it (
        &vec!["
            (or false nil 3)
        "],
        Some("3"),
    );
}

#[test]
fn test24() {
    super::test_it (
        &vec!["
            (or false nil false false nil 4)
        "],
        Some("4"),
    );
}

#[test]
fn test25() {
    super::test_it (
        &vec!["
            (or false nil 3 false nil 4)
        "],
        Some("3"),
    );
}

#[test]
fn test26() {
    super::test_it (
        &vec!["
            (or (or false 4))
        "],
        Some("4"),
    );
}

// ; Testing cond macro
#[test]
fn test27() {
    super::test_it (
        &vec!["
            (cond)
        "],
        Some("nil"),
    );
}

#[test]
fn test28() {
    super::test_it (
        &vec!["
            (cond true 7)
        "],
        Some("7"),
    );
}

#[test]
fn test29() {
    super::test_it (
        &vec!["
            (cond true 7 true 8)
        "],
        Some("7"),
    );
}

#[test]
fn test30() {
    super::test_it (
        &vec!["
            (cond false 7 true 8)
        "],
        Some("8"),
    );
}

#[test]
fn test31() {
    super::test_it (
        &vec!["
            (cond false 7 false 8 \"else\" 9)
        "],
        Some("9"),
    );
}

#[test]
fn test32() {
    super::test_it (
        &vec!["
            (cond false 7 (= 2 2) 8 \"else\" 9)
        "],
        Some("8"),
    );
}

#[test]
fn test33() {
    super::test_it (
        &vec!["
            (cond false 7 false 8 false 9)
        "],
        Some("nil"),
    );
}

// ; Testing EVAL in let*
#[test]
fn test34() {
    super::test_it (
        &vec!["
            (let* (x (or nil \"yes\")) x)
        "],
        Some("\"yes\""),
    );
}

// >>> optional=True
// ;
// ; -------- Optional Functionality --------
// ; Testing nth, first, rest with vectors
#[test]
fn test35() {
    super::test_it (
        &vec!["
            (nth [1] 0)
        "],
        Some("1"),
    );
}

#[test]
fn test36() {
    super::test_it (
        &vec!["
            (nth [1 2] 1)
        "],
        Some("2"),
    );
}

#[test]
fn test37() {
    super::test_it (
        &vec![
            "(def! x \"x\")",
            "(def! x (nth [1 2] 2))",
            "x"
        ],
        Some("\"x\""),
    );
}

#[test]
fn test38() {
    super::test_it (
        &vec!["
            (first [])
        "],
        Some("nil"),
    );
}

#[test]
fn test39() {
    super::test_it (
        &vec!["
            (first nil)
        "],
        Some("nil"),
    );
}

#[test]
fn test40() {
    super::test_it (
        &vec!["
            (first [10])
        "],
        Some("10"),
    );
}

#[test]
fn test41() {
    super::test_it (
        &vec!["
            (first [10 11 12])
        "],
        Some("10"),
    );
}

#[test]
fn test42() {
    super::test_it (
        &vec!["
            (rest [])
        "],
        Some("()"),
    );
}

#[test]
fn test43() {
    super::test_it (
        &vec!["
            (rest nil)
        "],
        Some("()"),
    );
}

#[test]
fn test44() {
    super::test_it (
        &vec!["
            (rest [10])
        "],
        Some("()"),
    );
}

#[test]
fn test45() {
    super::test_it (
        &vec!["
            (rest [10 11 12])
        "],
        Some("(11 12)"),
    );
}

// ; Testing EVAL in vector let*
#[test]
fn test46() {
    super::test_it (
        &vec!["
            (let* [x (or nil \"yes\")] x)
        "],
        Some("\"yes\""),
    );
}

// ;
// ; Loading core.mal
// ; Testing -> macro
#[test]
fn test47() {
    super::test_it (
        &vec!["
            (-> 7)
        "],
        Some("7"),
    );
}

#[test]
fn test48() {
    super::test_it (
        &vec!["
            (-> (list 7 8 9) first)
        "],
        Some("7"),
    );
}

#[test]
fn test49() {
    super::test_it (
        &vec!["
            (-> (list 7 8 9) (first))
        "],
        Some("7"),
    );
}

#[test]
fn test50() {
    super::test_it (
        &vec!["
            (-> (list 7 8 9) first (+ 7))
        "],
        Some("14"),
    );
}

#[test]
fn test51() {
    super::test_it (
        &vec!["
            (-> (list 7 8 9) rest (rest) first (+ 7))
        "],
        Some("16"),
    );
}

// ; Testing ->> macro
#[test]
fn test52() {
    super::test_it (
        &vec!["
            (->> \"L\")
        "],
        Some("\"L\""),
    );
}

#[test]
fn test53() {
    super::test_it (
        &vec!["
            (->> \"L\" (str \"A\") (str \"M\"))
        "],
        Some("\"MAL\""),
    );
}

#[test]
fn test54() {
    super::test_it (
        &vec!["
            (->> [4] (concat [3]) (concat [2]) rest (concat [1]))
        "],
        Some("(1 3 4)"),
    );
}

