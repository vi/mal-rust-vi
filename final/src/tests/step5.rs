// ; Testing recursive tail-call function
// ; TODO: test let*, and do for TCO
#[test]
fn test0() {
    crate::test_it (
        &vec!["
        (def! sum2 (fn* (n acc) (if (= n 0) acc (sum2 (- n 1) (+ n acc)))))
        (sum2 10 0)"],
        Some("55"),
    );
}

#[test]
fn test1() {
    crate::test_it (
        &vec!["(def! res2 nil)"],
        Some("nil"),
    );
}

#[test]
fn test2() {
    crate::test_it (
        &vec!["
        (def! sum2 (fn* (n acc) (if (= n 0) acc (sum2 (- n 1) (+ n acc)))))
        (def! res2 nil)
        (def! res2 (sum2 10000 0))
        res2
        "],
        Some("50005000"),
    );
}

// ; Test mutually recursive tail-call functions
#[test]
fn test3() {
    crate::test_it (
        &vec!["
        (def! foo (fn* (n) (if (= n 0) 0 (bar (- n 1)))))
        (def! bar (fn* (n) (if (= n 0) 0 (foo (- n 1)))))

        (foo 10000)
        "],
        Some("0"),
    );
}

