// ; Testing REPL_ENV
#[test]
fn test0() {
    super::test_it (
        &vec!["(+ 1 2)"],
        Some("3"),
    );
}

#[test]
fn test1() {
    super::test_it (
        &vec!["(/ (- (+ 5 (* 2 3)) 3) 4)"],
        Some("2"),
    );
}

// ; Testing def!
#[test]
fn test2() {
    super::test_it (
        &vec!["(def! x 3)"],
        Some("3"),
    );
}

#[test]
fn test3() {
    super::test_it (
        &vec!["(def! x 3)", "x"],
        Some("3"),
    );
}

#[test]
fn test4() {
    super::test_it (
        &vec!["(def! x 3)","(def! x 4)"],
        Some("4"),
    );
}

#[test]
fn test5() {
    super::test_it (
        &vec!["(def! x 3)","(def! x 4)","x"],
        Some("4"),
    );
}

#[test]
fn test6() {
    super::test_it (
        &vec!["(def! y (+ 1 7))"],
        Some("8"),
    );
}

#[test]
fn test7() {
    super::test_it (
        &vec!["(def! y (+ 1 7))", "y"],
        Some("8"),
    );
}

// ; Verifying symbols are case-sensitive
#[test]
fn test10() {
    super::test_it (
        &vec!["(def! mynum 111)","(def! MYNUM 222)","mynum"],
        Some("111"),
    );
}

#[test]
fn test11() {
    super::test_it (
        &vec!["(def! mynum 111)","(def! MYNUM 222)","MYNUM"],
        Some("222"),
    );
}

// ; Check env lookup non-fatal error
#[test]
fn testf1() {
    super::test_it (
        &vec!["(abc 1 2 3)"],
        None,
    );
}

// ; Check that error aborts def!
#[test]
fn test12() {
    super::test_it (
        &vec!["(def! w 123)", "(def! w (abc))", "w"],
        Some("123"),
    );
}

// ; Testing let*
#[test]
fn test13() {
    super::test_it (
        &vec!["(let* (z 9) z)"],
        Some("9"),
    );
}

#[test]
fn test14() {
    super::test_it (
        &vec!["(let* (x 9) x)"],
        Some("9"),
    );
}

#[test]
fn test15() {
    super::test_it (
        &vec!["(def! x 4)", "(let* (x 9) x)", "x"],
        Some("4"),
    );
}

#[test]
fn test16() {
    super::test_it (
        &vec!["(let* (z (+ 2 3)) (+ 1 z))"],
        Some("6"),
    );
}

#[test]
fn test17() {
    super::test_it (
        &vec!["(let* (p (+ 2 3) q (+ 2 p)) (+ p q))"],
        Some("12"),
    );
}

#[test]
fn test18() {
    super::test_it (
        &vec!["(def! y (let* (z 7) z))","y"],
        Some("7"),
    );
}

// ; Testing outer environment
#[test]
fn test19() {
    super::test_it (
        &vec!["(def! a 4)"],
        Some("4"),
    );
}

#[test]
fn test20() {
    super::test_it (
        &vec!["(def! a 4)","(let* (q 9) q)"],
        Some("9"),
    );
}

#[test]
fn test21() {
    super::test_it (
        &vec!["(def! a 4)","(let* (q 9) a)"],
        Some("4"),
    );
}

#[test]
fn test22() {
    super::test_it (
        &vec!["(def! a 4)","(let* (z 2) (let* (q 9) a))"],
        Some("4"),
    );
}

#[test]
fn test23() {
    super::test_it (
        &vec!["(def! a 4)","(let* (x 4) (def! a 5))"],
        Some("5"),
    );
}

#[test]
fn test24() {
    super::test_it (
        &vec!["(def! a 4)", "(let* (x 4) (def! a 5))", "a"],
        Some("4"),
    );
}

// >>> deferrable=True
// >>> optional=True
// ;
// ; -------- Deferrable/Optional Functionality --------
// ; Testing let* with vector bindings
#[test]
fn test25() {
    super::test_it (
        &vec!["(let* [z 9] z)"],
        Some("9"),
    );
}

#[test]
fn test26() {
    super::test_it (
        &vec!["(let* [p (+ 2 3) q (+ 2 p)] (+ p q))"],
        Some("12"),
    );
}

// ; Testing vector evaluation
#[test]
fn test27() {
    super::test_it (
        &vec!["(let* (a 5 b 6) [3 4 a [b 7] 8])"],
        Some("[3 4 5 [6 7] 8]"),
    );
}

