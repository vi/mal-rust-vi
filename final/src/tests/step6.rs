// ;; TODO: really a step5 test
// ;
// ; Testing that (do (do)) not broken by TCO
#[test]
fn test00() {
    crate::test_it (
        &vec!["(do (do 1 2))"],
        Some("2"),
    );
}

// ;
// ; Testing read-string, eval and slurp
#[test]
fn test01() {
    crate::test_it (
        &vec!["(read-string \"(1 2 (3 4) nil)\")"],
        Some("(1 2 (3 4) nil)"),
    );
}

#[test]
fn test02() {
    crate::test_it (
        &vec!["(read-string \"(+ 2 3)\")"],
        Some("(+ 2 3)"),
    );
}

#[test]
fn test03() {
    crate::test_it (
        &vec!["(read-string \"7 ;; comment\")"],
        Some("7"),
    );
}

#[test]
fn test03b() {
    crate::test_it (
        &vec!["(read-string \";; comment\")"],
        None,
    );
}

// ;; Differing output, but make sure no fatal error
#[test]
fn test04() {
    crate::test_it (
        &vec!["(eval (read-string \"(+ 2 3)\"))"],
        Some("5"),
    );
}

#[test]
fn test05() {
    crate::test_it (
        &vec!["(slurp \"src/test.txt\")"],
        Some("\"A line of text\\n\""),
    );
}

// ; Testing load-file
#[test]
fn test06() {
    crate::test_it (
        &vec![
            "(load-file \"src/inc.mal\")",
            "(inc1 7)",
        ],
        Some("8"),
    );
}

#[test]
fn test07() {
    crate::test_it (
        &vec![
            "(load-file \"src/inc.mal\")",
            "(inc2 7)",
        ],
        Some("9"),
    );
}

#[test]
fn test08() {
    crate::test_it (
        &vec![
            "(load-file \"src/inc.mal\")",
            "(inc3 9)",
        ],
        Some("12"),
    );
}

// ;
// ; Testing that *ARGV* exists and is an empty list
#[test]
fn test09() {
    crate::test_it (
        &vec!["(list? *ARGV*)"],
        Some("true"),
    );
}

#[test]
fn test10() {
    crate::test_it (
        &vec!["*ARGV*"],
        Some("()"),
    );
}

// ;
// ; Testing atoms
#[test]
fn test11() {
    crate::test_it (
        &vec!["(def! a (atom 2))"],
        Some("(atom 2)"),
    );
}

#[test]
fn test12() {
    crate::test_it (
        &vec!["(def! a (atom 2))  (atom? a)"],
        Some("true"),
    );
}

#[test]
fn test13() {
    crate::test_it (
        &vec!["(atom? 1)"],
        Some("false"),
    );
}

#[test]
fn test14() {
    crate::test_it (
        &vec!["(def! a (atom 2)) (atom? 1) (deref a)"],
        Some("2"),
    );
}

#[test]
fn test15() {
    crate::test_it (
        &vec!["(def! a (atom 2)) (atom? 1) (reset! a 3)"],
        Some("3"),
    );
}

#[test]
fn test16() {
    crate::test_it (
        &vec!["(def! a (atom 2)) (atom? 1) (reset! a 3) (deref a)"],
        Some("3"),
    );
}

#[test]
fn test17() {
    crate::test_it (
        &vec!["
        (def! a (atom 2))
        (atom? 1)
        (reset! a 3) 
        (def! inc3 (fn* (a) (+ 3 a)))
        (swap! a inc3)
        "],
        Some("6"),
    );
}

#[test]
fn test18() {
    crate::test_it (
        &vec!["
        (def! a (atom 2))
        (atom? 1)
        (reset! a 3) 
        (def! inc3 (fn* (a) (+ 3 a)))
        (swap! a inc3)
        (deref a)
        "],
        Some("6"),
    );
}

#[test]
fn test19() {
    crate::test_it (
        &vec!["
        (def! a (atom 2))
        (atom? 1)
        (reset! a 3) 
        (def! inc3 (fn* (a) (+ 3 a)))
        (swap! a inc3)
        (swap! a (fn* (a) a))
        "],
        Some("6"),
    );
}

#[test]
fn test20() {
    crate::test_it (
        &vec!["
        (def! a (atom 2))
        (atom? 1)
        (reset! a 3) 
        (def! inc3 (fn* (a) (+ 3 a)))
        (swap! a inc3)
        (swap! a (fn* (a) a))
        (swap! a (fn* (a) (* 2 a)))
        "],
        Some("12"),
    );
}

#[test]
fn test21() {
    crate::test_it (
        &vec!["
        (def! a (atom 2))
        (atom? 1)
        (reset! a 3) 
        (def! inc3 (fn* (a) (+ 3 a)))
        (swap! a inc3)
        (swap! a (fn* (a) a))
        (swap! a (fn* (a) (* 2 a)))
        (swap! a (fn* (a b) (* a b)) 10)
        "],
        Some("120"),
    );
}

#[test]
fn test22() {
    crate::test_it (
        &vec!["
        (def! a (atom 2))
        (atom? 1)
        (reset! a 3) 
        (def! inc3 (fn* (a) (+ 3 a)))
        (swap! a inc3)
        (swap! a (fn* (a) a))
        (swap! a (fn* (a) (* 2 a)))
        (swap! a (fn* (a b) (* a b)) 10)
        (swap! a + 3)
        "],
        Some("123"),
    );
}

// ; Testing swap!/closure interaction
#[test]
fn test23() {
    crate::test_it (
        &vec!["
        (def! inc-it (fn* (a) (+ 1 a)))
        (def! atm (atom 7))
        (def! f (fn* () (swap! atm inc-it)))
        (f)
        "],
        Some("8"),
    );
}

#[test]
fn test24() {
    crate::test_it (
        &vec!["
        (def! inc-it (fn* (a) (+ 1 a)))
        (def! atm (atom 7))
        (def! f (fn* () (swap! atm inc-it)))
        (f)
        (f)
        "],
        Some("9"),
    );
}

// >>> deferrable=True
// >>> optional=True
// ;
// ; -------- Deferrable/Optional Functionality --------
// ; Testing comments in a file
//  "incB.mal finished"
#[test]
fn test25() {
    crate::test_it (
        &vec!["(load-file \"src/incB.mal\")"],
        Some("\"incB.mal return string\""),
    );
}

#[test]
fn test26() {
    crate::test_it (
        &vec!["(load-file \"src/incB.mal\") (inc4 7)"],
        Some("11"),
    );
}

#[test]
fn test27() {
    crate::test_it (
        &vec!["(load-file \"src/incB.mal\") (inc5 7)"],
        Some("12"),
    );
}

// ; Testing map literal across multiple lines in a file
#[test]
fn test28() {
    crate::test_it (
        &vec!["(load-file \"src/incC.mal\") mymap"],
        Some("{\"a\" 1}"),
    );
}

// ; Testing `@` reader macro (short for `deref`)
#[test]
fn test29() {
    crate::test_it (
        &vec!["(def! atm (atom 9))  @atm"],
        Some("9"),
    );
}

// ;; TODO: really a step5 test
// ; Testing that vector params not broken by TCO
#[test]
fn test30() {
    crate::test_it (
        &vec!["(def! g (fn* [] 78)) (g)"],
        Some("78"),
    );
}

#[test]
fn test31() {
    crate::test_it (
        &vec!["(def! g (fn* [a] (+ a 78))) (g 3)"],
        Some("81"),
    );
}

