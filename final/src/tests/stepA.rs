// ;;
// ;; See IMPL/tests/stepA_mal.mal for implementation specific
// ;; interop tests.
// ;;
// ;
// ; Testing readline
/*
#[test]
fn test00() {
    crate::test_it (
        &vec!["
            \"hello\"
        "],
        Some("\"\\\"hello\\\"\""),
    );
}
*/

// ;
// ; Testing *host-language*
// ;; each impl is different, but this should return false
// ;; rather than throwing an exception
#[test]
fn test01() {
    crate::test_it (
        &vec!["
            (= \"something bogus\" *host-language*)
        "],
        Some("false"),
    );
}

// >>> deferrable=True
// ;
// ; ------- Deferrable Functionality ----------
// ; ------- (Needed for self-hosting) -------
// ;
// ; Testing metadata on functions
// ;
// ; Testing metadata on mal functions
#[test]
fn test02() {
    crate::test_it (
        &vec!["
            (meta (fn* (a) a))
        "],
        Some("nil"),
    );
}

#[test]
fn test03() {
    crate::test_it (
        &vec!["
            (meta (with-meta (fn* (a) a) {\"b\" 1}))
        "],
        Some("{\"b\" 1}"),
    );
}

#[test]
fn test04() {
    crate::test_it (
        &vec!["
            (meta (with-meta (fn* (a) a) \"abc\"))
        "],
        Some("\"abc\""),
    );
}

#[test]
fn test05() {
    crate::test_it (
        &vec!["
            (def! l-wm (with-meta (fn* (a) a) {\"b\" 2}))
            (meta l-wm)
        "],
        Some("{\"b\" 2}"),
    );
}

#[test]
fn test06() {
    crate::test_it (
        &vec!["
            (def! l-wm (with-meta (fn* (a) a) {\"b\" 2}))
            (meta (with-meta l-wm {\"new_meta\" 123}))
        "],
        Some("{\"new_meta\" 123}"),
    );
}

#[test]
fn test07() {
    crate::test_it (
        &vec!["
            (def! l-wm (with-meta (fn* (a) a) {\"b\" 2}))
            (meta (with-meta l-wm {\"new_meta\" 123}))
            (meta l-wm)
        "],
        Some("{\"b\" 2}"),
    );
}

#[test]
fn test08() {
    crate::test_it (
        &vec!["
            (def! f-wm (with-meta (fn* [a] (+ 1 a)) {\"abc\" 1}))
            (meta f-wm)
        "],
        Some("{\"abc\" 1}"),
    );
}

#[test]
fn test09() {
    crate::test_it (
        &vec!["
            (def! f-wm (with-meta (fn* [a] (+ 1 a)) {\"abc\" 1}))
            (meta (with-meta f-wm {\"new_meta\" 123}))
        "],
        Some("{\"new_meta\" 123}"),
    );
}

#[test]
fn test10() {
    crate::test_it (
        &vec!["
            (def! f-wm (with-meta (fn* [a] (+ 1 a)) {\"abc\" 1}))
            (meta (with-meta f-wm {\"new_meta\" 123}))
            (meta f-wm)
        "],
        Some("{\"abc\" 1}"),
    );
}

#[test]
fn test11() {
    crate::test_it (
        &vec!["
            (def! f-wm2 ^{\"abc\" 1} (fn* [a] (+ 1 a)))
            (meta f-wm2)
        "],
        Some("{\"abc\" 1}"),
    );
}

// ; Meta of native functions should return nil (not fail)
#[test]
fn test12() {
    crate::test_it (
        &vec!["
            (meta +)
        "],
        Some("nil"),
    );
}

// ;
// ; Make sure closures and metadata co-exist
#[test]
fn test13() {
    crate::test_it (
        &vec!["
            (def! gen-plusX (fn* (x) (with-meta (fn* (b) (+ x b)) {\"meta\" 1})))
            (def! plus7 (gen-plusX 7))
            (def! plus8 (gen-plusX 8))
            (plus7 8)
        "],
        Some("15"),
    );
}

#[test]
fn test14() {
    crate::test_it (
        &vec!["
            (def! gen-plusX (fn* (x) (with-meta (fn* (b) (+ x b)) {\"meta\" 1})))
            (def! plus7 (gen-plusX 7))
            (def! plus8 (gen-plusX 8))
            (meta plus7)
        "],
        Some("{\"meta\" 1}"),
    );
}

#[test]
fn test15() {
    crate::test_it (
        &vec!["
            (def! gen-plusX (fn* (x) (with-meta (fn* (b) (+ x b)) {\"meta\" 1})))
            (def! plus7 (gen-plusX 7))
            (def! plus8 (gen-plusX 8))
            (meta plus8)
        "],
        Some("{\"meta\" 1}"),
    );
}

#[test]
fn test16() {
    crate::test_it (
        &vec!["
            (def! gen-plusX (fn* (x) (with-meta (fn* (b) (+ x b)) {\"meta\" 1})))
            (def! plus7 (gen-plusX 7))
            (def! plus8 (gen-plusX 8))
            (meta (with-meta plus7 {\"meta\" 2}))
        "],
        Some("{\"meta\" 2}"),
    );
}

#[test]
fn test17() {
    crate::test_it (
        &vec!["
            (def! gen-plusX (fn* (x) (with-meta (fn* (b) (+ x b)) {\"meta\" 1})))
            (def! plus7 (gen-plusX 7))
            (def! plus8 (gen-plusX 8))
            (meta (with-meta plus7 {\"meta\" 2}))
            (meta plus8)
        "],
        Some("{\"meta\" 1}"),
    );
}

// ;
// ; Testing hash-map evaluation and atoms (i.e. an env)
#[test]
fn test18() {
    crate::test_it (
        &vec!["
            (def! e (atom {\"+\" +}))
            (swap! e assoc \"-\" -)
            ( (get @e \"+\") 7 8)
        "],
        Some("15"),
    );
}

#[test]
fn test19() {
    crate::test_it (
        &vec!["
            (def! e (atom {\"+\" +}))
            (swap! e assoc \"-\" -)
            ( (get @e \"-\") 11 8)
        "],
        Some("3"),
    );
}

#[test]
fn test20() {
    crate::test_it (
        &vec!["
            (def! e (atom {\"+\" +}))
            (swap! e assoc \"-\" -)
            (swap! e assoc \"foo\" (list))
            (get @e \"foo\")
        "],
        Some("()"),
    );
}

#[test]
fn test21() {
    crate::test_it (
        &vec!["
            (def! e (atom {\"+\" +}))
            (swap! e assoc \"-\" -)
            (swap! e assoc \"foo\" (list))
            (swap! e assoc \"bar\" '(1 2 3))
            (get @e \"bar\")
        "],
        Some("(1 2 3)"),
    );
}

// ; ------------------------------------------------------------------
// >>> soft=True
// >>> optional=True
// ;
// ; ------- Optional Functionality --------------
// ; ------- (Not needed for self-hosting) -------
// ;
// ; Testing string? function
#[test]
fn test22() {
    crate::test_it (
        &vec!["
            (string? \"\")
        "],
        Some("true"),
    );
}

#[test]
fn test23() {
    crate::test_it (
        &vec!["
            (string? 'abc)
        "],
        Some("false"),
    );
}

#[test]
fn test24() {
    crate::test_it (
        &vec!["
            (string? \"abc\")
        "],
        Some("true"),
    );
}

#[test]
fn test25() {
    crate::test_it (
        &vec!["
            (string? :abc)
        "],
        Some("false"),
    );
}

#[test]
fn test26() {
    crate::test_it (
        &vec!["
            (string? (keyword \"abc\"))
        "],
        Some("false"),
    );
}

#[test]
fn test27() {
    crate::test_it (
        &vec!["
            (string? 234)
        "],
        Some("false"),
    );
}

#[test]
fn test28() {
    crate::test_it (
        &vec!["
            (string? nil)
        "],
        Some("false"),
    );
}

// ; Testing number? function
#[test]
fn test29() {
    crate::test_it (
        &vec!["
            (number? 123)
        "],
        Some("true"),
    );
}

#[test]
fn test30() {
    crate::test_it (
        &vec!["
            (number? -1)
        "],
        Some("true"),
    );
}

#[test]
fn test31() {
    crate::test_it (
        &vec!["
            (number? nil)
        "],
        Some("false"),
    );
}

#[test]
fn test32() {
    crate::test_it (
        &vec!["
            (number? false)
        "],
        Some("false"),
    );
}

#[test]
fn test33() {
    crate::test_it (
        &vec!["
            (number? \"123\")
        "],
        Some("false"),
    );
}

// ; Testing fn? function
#[test]
fn test34() {
    crate::test_it (
        &vec!["
            (fn? +)
        "],
        Some("true"),
    );
}

#[test]
fn test35() {
    crate::test_it (
        &vec!["
            (def! add1 (fn* (x) (+ x 1)))
            (fn? add1)
        "],
        Some("true"),
    );
}

#[test]
fn test36() {
    crate::test_it (
        &vec!["
            (fn? cond)
        "],
        Some("false"),
    );
}

#[test]
fn test37() {
    crate::test_it (
        &vec!["
            (fn? \"+\")
        "],
        Some("false"),
    );
}

#[test]
fn test38() {
    crate::test_it (
        &vec!["
            (fn? :+)
        "],
        Some("false"),
    );
}

// ; Testing macro? function
#[test]
fn test39() {
    crate::test_it (
        &vec!["
            (macro? cond)
        "],
        Some("true"),
    );
}

#[test]
fn test40() {
    crate::test_it (
        &vec!["
            (macro? +)
        "],
        Some("false"),
    );
}

#[test]
fn test41() {
    crate::test_it (
        &vec!["
            (def! add1 (fn* (x) (+ x 1)))
            (macro? add1)
        "],
        Some("false"),
    );
}

#[test]
fn test42() {
    crate::test_it (
        &vec!["
            (macro? \"+\")
        "],
        Some("false"),
    );
}

#[test]
fn test43() {
    crate::test_it (
        &vec!["
            (macro? :+)
        "],
        Some("false"),
    );
}

// ;
// ; Testing conj function
#[test]
fn test44() {
    crate::test_it (
        &vec!["
            (conj (list) 1)
        "],
        Some("(1)"),
    );
}

#[test]
fn test45() {
    crate::test_it (
        &vec!["
            (conj (list 1) 2)
        "],
        Some("(2 1)"),
    );
}

#[test]
fn test46() {
    crate::test_it (
        &vec!["
            (conj (list 2 3) 4)
        "],
        Some("(4 2 3)"),
    );
}

#[test]
fn test47() {
    crate::test_it (
        &vec!["
            (conj (list 2 3) 4 5 6)
        "],
        Some("(6 5 4 2 3)"),
    );
}

#[test]
fn test48() {
    crate::test_it (
        &vec!["
            (conj (list 1) (list 2 3))
        "],
        Some("((2 3) 1)"),
    );
}

#[test]
fn test49() {
    crate::test_it (
        &vec!["
            (conj [] 1)
        "],
        Some("[1]"),
    );
}

#[test]
fn test50() {
    crate::test_it (
        &vec!["
            (conj [1] 2)
        "],
        Some("[1 2]"),
    );
}

#[test]
fn test51() {
    crate::test_it (
        &vec!["
            (conj [2 3] 4)
        "],
        Some("[2 3 4]"),
    );
}

#[test]
fn test52() {
    crate::test_it (
        &vec!["
            (conj [2 3] 4 5 6)
        "],
        Some("[2 3 4 5 6]"),
    );
}

#[test]
fn test53() {
    crate::test_it (
        &vec!["
            (conj [1] [2 3])
        "],
        Some("[1 [2 3]]"),
    );
}

// ;
// ; Testing seq function
#[test]
fn test54() {
    crate::test_it (
        &vec!["
            (seq \"abc\")
        "],
        Some("(\"a\" \"b\" \"c\")"),
    );
}

#[test]
fn test55() {
    crate::test_it (
        &vec!["
            (apply str (seq \"this is a test\"))
        "],
        Some("\"this is a test\""),
    );
}

#[test]
fn test56() {
    crate::test_it (
        &vec!["
            (seq '(2 3 4))
        "],
        Some("(2 3 4)"),
    );
}

#[test]
fn test57() {
    crate::test_it (
        &vec!["
            (seq [2 3 4])
        "],
        Some("(2 3 4)"),
    );
}

#[test]
fn test58() {
    crate::test_it (
        &vec!["
            (seq \"\")
        "],
        Some("()"),
    );
}

#[test]
fn test59() {
    crate::test_it (
        &vec!["
            (seq '())
        "],
        Some("()"),
    );
}

#[test]
fn test60() {
    crate::test_it (
        &vec!["
            (seq [])
        "],
        Some("()"),
    );
}

#[test]
fn test61() {
    crate::test_it (
        &vec!["
            (seq nil)
        "],
        Some("()"),
    );
}

// ;
// ; Testing metadata on collections
#[test]
fn test62() {
    crate::test_it (
        &vec!["
            (meta [1 2 3])
        "],
        Some("nil"),
    );
}

#[test]
fn test63() {
    crate::test_it (
        &vec!["
            (with-meta [1 2 3] {\"a\" 1})
        "],
        Some("[1 2 3]"),
    );
}

#[test]
fn test64() {
    crate::test_it (
        &vec!["
            (meta (with-meta [1 2 3] {\"a\" 1}))
        "],
        Some("{\"a\" 1}"),
    );
}

#[test]
fn test65() {
    crate::test_it (
        &vec!["
            (vector? (with-meta [1 2 3] {\"a\" 1}))
        "],
        Some("true"),
    );
}

#[test]
fn test66() {
    crate::test_it (
        &vec!["
            (meta (with-meta [1 2 3] \"abc\"))
        "],
        Some("\"abc\""),
    );
}

#[test]
fn test67() {
    crate::test_it (
        &vec!["
            (meta (with-meta (list 1 2 3) {\"a\" 1}))
        "],
        Some("{\"a\" 1}"),
    );
}

#[test]
fn test68() {
    crate::test_it (
        &vec!["
            (list? (with-meta (list 1 2 3) {\"a\" 1}))
        "],
        Some("true"),
    );
}

#[test]
fn test69() {
    crate::test_it (
        &vec!["
            (meta (with-meta {\"abc\" 123} {\"a\" 1}))
        "],
        Some("{\"a\" 1}"),
    );
}

#[test]
fn test70() {
    crate::test_it (
        &vec!["
            (map? (with-meta {\"abc\" 123} {\"a\" 1}))
        "],
        Some("true"),
    );
}

// ;; Not actually supported by Clojure
// ;;(meta (with-meta (atom 7) {"a" 1}))
// ;;;=>{"a" 1}
#[test]
fn test71() {
    crate::test_it (
        &vec!["
            (def! l-wm (with-meta [4 5 6] {\"b\" 2}))
        "],
        Some("[4 5 6]"),
    );
}

#[test]
fn test72() {
    crate::test_it (
        &vec!["
            (def! l-wm (with-meta [4 5 6] {\"b\" 2}))
            (meta l-wm)
        "],
        Some("{\"b\" 2}"),
    );
}

#[test]
fn test73() {
    crate::test_it (
        &vec!["
            (def! l-wm (with-meta [4 5 6] {\"b\" 2}))
            (meta (with-meta l-wm {\"new_meta\" 123}))
        "],
        Some("{\"new_meta\" 123}"),
    );
}

#[test]
fn test74() {
    crate::test_it (
        &vec!["
            (def! l-wm (with-meta [4 5 6] {\"b\" 2}))
            (meta l-wm)
        "],
        Some("{\"b\" 2}"),
    );
}

// ;
// ; Testing metadata on builtin functions
#[test]
fn test75() {
    crate::test_it (
        &vec!["
            (meta +)
        "],
        Some("nil"),
    );
}

#[test]
fn test76() {
    crate::test_it (
        &vec!["
            (def! f-wm3 ^{\"def\" 2} +)
            (meta f-wm3)
        "],
        Some("{\"def\" 2}"),
    );
}

#[test]
fn test77() {
    crate::test_it (
        &vec!["
            (meta +)
        "],
        Some("nil"),
    );
}

// ;
// ; Testing gensym and clean or macro
#[test]
fn test78() {
    crate::test_it (
        &vec!["
            (= (gensym) (gensym))
        "],
        Some("false"),
    );
}

#[test]
fn test79() {
    crate::test_it (
        &vec!["
            (let* [or_FIXME 23] (or false (+ or_FIXME 100)))
        "],
        Some("123"),
    );
}

// ;
// ; Testing time-ms function
#[test]
fn test80() {
    crate::test_it (
        &vec!["
            (def! start-time (time-ms))
            (= start-time 0)
        "],
        Some("false"),
    );
}

#[test]
fn test81() {
    crate::test_it (
        &vec!["
            (def! start-time (time-ms))
            (let* [sumdown (fn* (N) (if (> N 0) (+ N (sumdown (- N 1))) 0))] (sumdown 10)) ; Waste some time
        "],
        Some("55"),
    );
}

#[test]
fn test82() {
    crate::test_it (
        &vec!["
            (def! start-time (time-ms))
            (let* [sumdown (fn* (N) (if (> N 0) (+ N (sumdown (- N 1))) 0))] (sumdown 10)) ; Waste some time
            (> (time-ms) start-time)
        "],
        Some("true"),
    );
}

