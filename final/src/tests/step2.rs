// ; Testing evaluation of arithmetic operations
#[test]
fn test0() {
    crate::test_it (
        &vec!["(+ 1 2)"],
        Some("3"),
    );
}

#[test]
fn test1() {
    crate::test_it (
        &vec!["(+ 5 (* 2 3))"],
        Some("11"),
    );
}

#[test]
fn test2() {
    crate::test_it (
        &vec!["(- (+ 5 (* 2 3)) 3)"],
        Some("8"),
    );
}

#[test]
fn test3() {
    crate::test_it (
        &vec!["(/ (- (+ 5 (* 2 3)) 3) 4)"],
        Some("2"),
    );
}

#[test]
fn test4() {
    crate::test_it (
        &vec!["(/ (- (+ 515 (* 87 311)) 302) 27)"],
        Some("1010"),
    );
}

#[test]
fn test5() {
    crate::test_it (
        &vec!["(* -3 6)"],
        Some("-18"),
    );
}

#[test]
fn test6() {
    crate::test_it (
        &vec!["(/ (- (+ 515 (* -87 311)) 296) 27)"],
        Some("-994"),
    );
}

//; .*\'abc\' not found.*
#[test]
fn testf1() {
    crate::test_it (
        &vec!["(abc 1 2 3)"],
        None,
    );
}



// ; Testing empty list
#[test]
fn test7() {
    crate::test_it (
        &vec!["()"],
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
    crate::test_it (
        &vec!["[1 2 (+ 1 2)]"],
        Some("[1 2 3]"),
    );
}

#[test]
fn test9() {
    crate::test_it (
        &vec!["{\"a\" (+ 7 8)}"],
        Some("{\"a\" 15}"),
    );
}

#[test]
fn test10() {
    crate::test_it (
        &vec!["{:a (+ 7 8)}"],
        Some("{:a 15}"),
    );
}

