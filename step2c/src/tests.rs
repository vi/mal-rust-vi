// ; Testing evaluation of arithmetic operations
#[test]
fn test0() {
    super::test_it (
        "(+ 1 2)",
        Some("3"),
    );
}

#[test]
fn test1() {
    super::test_it (
        "(+ 5 (* 2 3))",
        Some("11"),
    );
}

#[test]
fn test2() {
    super::test_it (
        "(- (+ 5 (* 2 3)) 3)",
        Some("8"),
    );
}

#[test]
fn test3() {
    super::test_it (
        "(/ (- (+ 5 (* 2 3)) 3) 4)",
        Some("2"),
    );
}

#[test]
fn test4() {
    super::test_it (
        "(/ (- (+ 515 (* 87 311)) 302) 27)",
        Some("1010"),
    );
}

#[test]
fn test5() {
    super::test_it (
        "(* -3 6)",
        Some("-18"),
    );
}

#[test]
fn test6() {
    super::test_it (
        "(/ (- (+ 515 (* -87 311)) 296) 27)",
        Some("-994"),
    );
}

//; .*\'abc\' not found.*
#[test]
fn testf1() {
    super::test_it (
        "(abc 1 2 3)",
        None,
    );
}



// ; Testing empty list
#[test]
fn test7() {
    super::test_it (
        "()",
        Some("()"),
    );
}

// >>> deferrable=True
// >>> optional=True
// ;
// ; -------- Deferrable/Optional Functionality --------
// ; Testing evaluation within collection literals
#[test]
fn test8() {
    super::test_it (
        "[1 2 (+ 1 2)]",
        Some("[1 2 3]"),
    );
}

#[test]
fn test9() {
    super::test_it (
        "{\"a\" (+ 7 8)}",
        Some("{\"a\" 15}"),
    );
}

#[test]
fn test10() {
    super::test_it (
        "{:a (+ 7 8)}",
        Some("{:a 15}"),
    );
}

