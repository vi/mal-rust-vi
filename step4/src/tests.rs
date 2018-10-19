// ; -----------------------------------------------------
// ; Testing list functions
#[test]
fn test0() {
    super::test_it (
        &vec!["(list)"],
        Some("()"),
    );
}

#[test]
fn test1() {
    super::test_it (
        &vec!["(list? (list))"],
        Some("true"),
    );
}

#[test]
fn test2() {
    super::test_it (
        &vec!["(empty? (list))"],
        Some("true"),
    );
}

#[test]
fn test3() {
    super::test_it (
        &vec!["(empty? (list 1))"],
        Some("false"),
    );
}

#[test]
fn test4() {
    super::test_it (
        &vec!["(list 1 2 3)"],
        Some("(1 2 3)"),
    );
}

#[test]
fn test5() {
    super::test_it (
        &vec!["(count (list 1 2 3))"],
        Some("3"),
    );
}

#[test]
fn test6() {
    super::test_it (
        &vec!["(count (list))"],
        Some("0"),
    );
}

#[test]
fn test7() {
    super::test_it (
        &vec!["(count nil)"],
        Some("0"),
    );
}

#[test]
fn test8() {
    super::test_it (
        &vec!["(if (> (count (list 1 2 3)) 3) \"yes\" \"no\")"],
        Some("\"no\""),
    );
}

#[test]
fn test9() {
    super::test_it (
        &vec!["(if (>= (count (list 1 2 3)) 3) \"yes\" \"no\")"],
        Some("\"yes\""),
    );
}

// ; Testing if form
#[test]
fn test10() {
    super::test_it (
        &vec!["(if true 7 8)"],
        Some("7"),
    );
}

#[test]
fn test11() {
    super::test_it (
        &vec!["(if false 7 8)"],
        Some("8"),
    );
}

#[test]
fn test12() {
    super::test_it (
        &vec!["(if true (+ 1 7) (+ 1 8))"],
        Some("8"),
    );
}

#[test]
fn test13() {
    super::test_it (
        &vec!["(if false (+ 1 7) (+ 1 8))"],
        Some("9"),
    );
}

#[test]
fn test14() {
    super::test_it (
        &vec!["(if nil 7 8)"],
        Some("8"),
    );
}

#[test]
fn test15() {
    super::test_it (
        &vec!["(if 0 7 8)"],
        Some("7"),
    );
}

#[test]
fn test16() {
    super::test_it (
        &vec!["(if \"\" 7 8)"],
        Some("7"),
    );
}

#[test]
fn test17() {
    super::test_it (
        &vec!["(if (list) 7 8)"],
        Some("7"),
    );
}

#[test]
fn test18() {
    super::test_it (
        &vec!["(if (list 1 2 3) 7 8)"],
        Some("7"),
    );
}

#[test]
fn test19() {
    super::test_it (
        &vec!["(= (list) nil)"],
        Some("false"),
    );
}

// ; Testing 1-way if form
#[test]
fn test20() {
    super::test_it (
        &vec!["(if false (+ 1 7))"],
        Some("nil"),
    );
}

#[test]
fn test21() {
    super::test_it (
        &vec!["(if nil 8 7)"],
        Some("7"),
    );
}

#[test]
fn test22() {
    super::test_it (
        &vec!["(if true (+ 1 7))"],
        Some("8"),
    );
}

// ; Testing basic conditionals
#[test]
fn test23() {
    super::test_it (
        &vec!["(= 2 1)"],
        Some("false"),
    );
}

#[test]
fn test24() {
    super::test_it (
        &vec!["(= 1 1)"],
        Some("true"),
    );
}

#[test]
fn test25() {
    super::test_it (
        &vec!["(= 1 2)"],
        Some("false"),
    );
}

#[test]
fn test26() {
    super::test_it (
        &vec!["(= 1 (+ 1 1))"],
        Some("false"),
    );
}

#[test]
fn test27() {
    super::test_it (
        &vec!["(= 2 (+ 1 1))"],
        Some("true"),
    );
}

#[test]
fn test28() {
    super::test_it (
        &vec!["(= nil 1)"],
        Some("false"),
    );
}

#[test]
fn test29() {
    super::test_it (
        &vec!["(= nil nil)"],
        Some("true"),
    );
}

#[test]
fn test30() {
    super::test_it (
        &vec!["(> 2 1)"],
        Some("true"),
    );
}

#[test]
fn test31() {
    super::test_it (
        &vec!["(> 1 1)"],
        Some("false"),
    );
}

#[test]
fn test32() {
    super::test_it (
        &vec!["(> 1 2)"],
        Some("false"),
    );
}

#[test]
fn test33() {
    super::test_it (
        &vec!["(>= 2 1)"],
        Some("true"),
    );
}

#[test]
fn test34() {
    super::test_it (
        &vec!["(>= 1 1)"],
        Some("true"),
    );
}

#[test]
fn test35() {
    super::test_it (
        &vec!["(>= 1 2)"],
        Some("false"),
    );
}

#[test]
fn test36() {
    super::test_it (
        &vec!["(< 2 1)"],
        Some("false"),
    );
}

#[test]
fn test37() {
    super::test_it (
        &vec!["(< 1 1)"],
        Some("false"),
    );
}

#[test]
fn test38() {
    super::test_it (
        &vec!["(< 1 2)"],
        Some("true"),
    );
}

#[test]
fn test39() {
    super::test_it (
        &vec!["(<= 2 1)"],
        Some("false"),
    );
}

#[test]
fn test40() {
    super::test_it (
        &vec!["(<= 1 1)"],
        Some("true"),
    );
}

#[test]
fn test41() {
    super::test_it (
        &vec!["(<= 1 2)"],
        Some("true"),
    );
}

// ; Testing equality
#[test]
fn test42() {
    super::test_it (
        &vec!["(= 1 1)"],
        Some("true"),
    );
}

#[test]
fn test43() {
    super::test_it (
        &vec!["(= 0 0)"],
        Some("true"),
    );
}

#[test]
fn test44() {
    super::test_it (
        &vec!["(= 1 0)"],
        Some("false"),
    );
}

#[test]
fn test45() {
    super::test_it (
        &vec!["(= \"\" \"\")"],
        Some("true"),
    );
}

#[test]
fn test46() {
    super::test_it (
        &vec!["(= \"abc\" \"abc\")"],
        Some("true"),
    );
}

#[test]
fn test47() {
    super::test_it (
        &vec!["(= \"abc\" \"\")"],
        Some("false"),
    );
}

#[test]
fn test48() {
    super::test_it (
        &vec!["(= \"\" \"abc\")"],
        Some("false"),
    );
}

#[test]
fn test49() {
    super::test_it (
        &vec!["(= \"abc\" \"def\")"],
        Some("false"),
    );
}

#[test]
fn test50() {
    super::test_it (
        &vec!["(= \"abc\" \"ABC\")"],
        Some("false"),
    );
}

#[test]
fn test51() {
    super::test_it (
        &vec!["(= true true)"],
        Some("true"),
    );
}

#[test]
fn test52() {
    super::test_it (
        &vec!["(= false false)"],
        Some("true"),
    );
}

#[test]
fn test53() {
    super::test_it (
        &vec!["(= nil nil)"],
        Some("true"),
    );
}

#[test]
fn test54() {
    super::test_it (
        &vec!["(= (list) (list))"],
        Some("true"),
    );
}

#[test]
fn test55() {
    super::test_it (
        &vec!["(= (list 1 2) (list 1 2))"],
        Some("true"),
    );
}

#[test]
fn test56() {
    super::test_it (
        &vec!["(= (list 1) (list))"],
        Some("false"),
    );
}

#[test]
fn test57() {
    super::test_it (
        &vec!["(= (list) (list 1))"],
        Some("false"),
    );
}

#[test]
fn test58() {
    super::test_it (
        &vec!["(= 0 (list))"],
        Some("false"),
    );
}

#[test]
fn test59() {
    super::test_it (
        &vec!["(= (list) 0)"],
        Some("false"),
    );
}

#[test]
fn test60() {
    super::test_it (
        &vec!["(= (list) \"\")"],
        Some("false"),
    );
}

#[test]
fn test61() {
    super::test_it (
        &vec!["(= \"\" (list))"],
        Some("false"),
    );
}

// ; Testing builtin and user defined functions
#[test]
fn test62() {
    super::test_it (
        &vec!["(+ 1 2)"],
        Some("3"),
    );
}

#[test]
fn test63() {
    super::test_it (
        &vec!["( (fn* (a b) (+ b a)) 3 4)"],
        Some("7"),
    );
}

#[test]
fn test64() {
    super::test_it (
        &vec!["( (fn* () 4) )"],
        Some("4"),
    );
}

#[test]
fn test65() {
    super::test_it (
        &vec!["( (fn* (f x) (f x)) (fn* (a) (+ 1 a)) 7)"],
        Some("8"),
    );
}

// ; Testing closures
#[test]
fn test66() {
    super::test_it (
        &vec!["( ( (fn* (a) (fn* (b) (+ a b))) 5) 7)"],
        Some("12"),
    );
}

#[test]
fn test67() {
    super::test_it (
        &vec![
            "(def! gen-plus5 (fn* () (fn* (b) (+ 5 b))))",
            "(def! plus5 (gen-plus5))",
            "(plus5 7)",
        ],
        Some("12"),
    );
}

#[test]
fn test68() {
    super::test_it (
        &vec![
            "(def! gen-plus5 (fn* () (fn* (b) (+ 5 b))))",
            "(def! plus5 (gen-plus5))",
            "(def! gen-plusX (fn* (x) (fn* (b) (+ x b))))",
            "(def! plus7 (gen-plusX 7))",
            "(plus7 8)",
        ],
        Some("15"),
    );
}

// ; Testing do form
//  "prn output1"
#[test]
fn test69() {
    super::test_it (
        &vec!["(do (prn \"prn output1\"))"],
        Some("nil"),
    );
}

//  "prn output2"
#[test]
fn test70() {
    super::test_it (
        &vec!["(do (prn \"prn output2\") 7)"],
        Some("7"),
    );
}

//  "prn output1"
//  "prn output2"
#[test]
fn test71() {
    super::test_it (
        &vec!["(do (prn \"prn output1\") (prn \"prn output2\") (+ 1 2))"],
        Some("3"),
    );
}

#[test]
fn test72() {
    super::test_it (
        &vec!["(do (def! a 6) 7 (+ a 8))"],
        Some("14"),
    );
}

#[test]
fn test73() {
    super::test_it (
        &vec!["(do (def! a 6) 7 (+ a 8))","a"],
        Some("6"),
    );
}

// ; Testing special form case-sensitivity
#[test]
fn test74() {
    super::test_it (
        &vec!["(def! DO (fn* (a) 7))","(DO 3)"],
        Some("7"),
    );
}

// ; Testing recursive sumdown function
#[test]
fn test75() {
    super::test_it (
        &vec![
            "(def! sumdown (fn* (N) (if (> N 0) (+ N (sumdown  (- N 1))) 0)))",
            "(sumdown 1)",
        ],
        Some("1"),
    );
}

#[test]
fn test76() {
    super::test_it (
        &vec![
            "(def! sumdown (fn* (N) (if (> N 0) (+ N (sumdown  (- N 1))) 0)))",
            "(sumdown 2)",
        ],
        Some("3"),
    );
}

#[test]
fn test77() {
    super::test_it (
        &vec![
            "(def! sumdown (fn* (N) (if (> N 0) (+ N (sumdown  (- N 1))) 0)))",
            "(sumdown 6)",
        ],
        Some("21"),
    );
}

// ; Testing recursive fibonacci function
#[test]
fn test78() {
    super::test_it (
        &vec![
            "(def! fib (fn* (N) (if (= N 0) 1 (if (= N 1) 1 (+ (fib (- N 1)) (fib (- N 2)))))))",
            "(fib 1)",
        ],
        Some("1"),
    );
}

#[test]
fn test79() {
    super::test_it (
        &vec![
            "(def! fib (fn* (N) (if (= N 0) 1 (if (= N 1) 1 (+ (fib (- N 1)) (fib (- N 2)))))))",
            "(fib 2)",
        ],
        Some("2"),
    );
}

#[test]
fn test80() {
    super::test_it (
        &vec![
            "(def! fib (fn* (N) (if (= N 0) 1 (if (= N 1) 1 (+ (fib (- N 1)) (fib (- N 2)))))))",
            "(fib 4)",
        ],
        Some("5"),
    );
}

// ;; Too slow for bash, erlang, make and miniMAL
// ;;(fib 10)
// ;;;=>89
// >>> deferrable=True
// ;
// ; -------- Deferrable Functionality --------
// ; Testing variable length arguments
#[test]
fn test81() {
    super::test_it (
        &vec!["( (fn* (& more) (count more)) 1 2 3)"],
        Some("3"),
    );
}

#[test]
fn test82() {
    super::test_it (
        &vec!["( (fn* (& more) (list? more)) 1 2 3)"],
        Some("true"),
    );
}

#[test]
fn test83() {
    super::test_it (
        &vec!["( (fn* (& more) (count more)) 1)"],
        Some("1"),
    );
}

#[test]
fn test84() {
    super::test_it (
        &vec!["( (fn* (& more) (count more)) )"],
        Some("0"),
    );
}

#[test]
fn test85() {
    super::test_it (
        &vec!["( (fn* (& more) (list? more)) )"],
        Some("true"),
    );
}

#[test]
fn test86() {
    super::test_it (
        &vec!["( (fn* (a & more) (count more)) 1 2 3)"],
        Some("2"),
    );
}

#[test]
fn test87() {
    super::test_it (
        &vec!["( (fn* (a & more) (count more)) 1)"],
        Some("0"),
    );
}

#[test]
fn test88() {
    super::test_it (
        &vec!["( (fn* (a & more) (list? more)) 1)"],
        Some("true"),
    );
}

// ; Testing language defined not function
#[test]
fn test89() {
    super::test_it (
        &vec!["(not false)"],
        Some("true"),
    );
}

#[test]
fn test90() {
    super::test_it (
        &vec!["(not nil)"],
        Some("true"),
    );
}

#[test]
fn test91() {
    super::test_it (
        &vec!["(not true)"],
        Some("false"),
    );
}

#[test]
fn test92() {
    super::test_it (
        &vec!["(not \"a\")"],
        Some("false"),
    );
}

#[test]
fn test93() {
    super::test_it (
        &vec!["(not 0)"],
        Some("false"),
    );
}

// ; -----------------------------------------------------
// ; Testing string quoting
#[test]
fn test94() {
    super::test_it (
        &vec!["\"\""],
        Some("\"\""),
    );
}

#[test]
fn test95() {
    super::test_it (
        &vec!["\"abc\""],
        Some("\"abc\""),
    );
}

#[test]
fn test96() {
    super::test_it (
        &vec!["\"abc  def\""],
        Some("\"abc  def\""),
    );
}

#[test]
fn test97() {
    super::test_it (
        &vec!["\"\\\"\""],
        Some("\"\\\"\""),
    );
}

#[test]
fn test98() {
    super::test_it (
        &vec!["\"abc\\ndef\\nghi\""],
        Some("\"abc\\ndef\\nghi\""),
    );
}

#[test]
fn test99() {
    super::test_it (
        &vec!["\"abc\\\\def\\\\ghi\""],
        Some("\"abc\\\\def\\\\ghi\""),
    );
}

#[test]
fn test100() {
    super::test_it (
        &vec!["\"\\\\n\""],
        Some("\"\\\\n\""),
    );
}

// ; Testing pr-str
#[test]
fn test101() {
    super::test_it (
        &vec!["(pr-str)"],
        Some("\"\""),
    );
}

#[test]
fn test102() {
    super::test_it (
        &vec!["(pr-str \"\")"],
        Some("\"\\\"\\\"\""),
    );
}

#[test]
fn test103() {
    super::test_it (
        &vec!["(pr-str \"abc\")"],
        Some("\"\\\"abc\\\"\""),
    );
}

#[test]
fn test104() {
    super::test_it (
        &vec!["(pr-str \"abc  def\" \"ghi jkl\")"],
        Some("\"\\\"abc  def\\\" \\\"ghi jkl\\\"\""),
    );
}

#[test]
fn test105() {
    super::test_it (
        &vec!["(pr-str \"\\\"\")"],
        Some("\"\\\"\\\\\\\"\\\"\""),
    );
}

#[test]
fn test106() {
    super::test_it (
        &vec!["(pr-str (list 1 2 \"abc\" \"\\\"\") \"def\")"],
        Some("\"(1 2 \\\"abc\\\" \\\"\\\\\\\"\\\") \\\"def\\\"\""),
    );
}

#[test]
fn test107() {
    super::test_it (
        &vec!["(pr-str \"abc\\ndef\\nghi\")"],
        Some("\"\\\"abc\\\\ndef\\\\nghi\\\"\""),
    );
}

#[test]
fn test108() {
    super::test_it (
        &vec!["(pr-str \"abc\\\\def\\\\ghi\")"],
        Some("\"\\\"abc\\\\\\\\def\\\\\\\\ghi\\\"\""),
    );
}

#[test]
fn test109() {
    super::test_it (
        &vec!["(pr-str (list))"],
        Some("\"()\""),
    );
}

// ; Testing str
#[test]
fn test110() {
    super::test_it (
        &vec!["(str)"],
        Some("\"\""),
    );
}

#[test]
fn test111() {
    super::test_it (
        &vec!["(str \"\")"],
        Some("\"\""),
    );
}

#[test]
fn test112() {
    super::test_it (
        &vec!["(str \"abc\")"],
        Some("\"abc\""),
    );
}

#[test]
fn test113() {
    super::test_it (
        &vec!["(str \"\\\"\")"],
        Some("\"\\\"\""),
    );
}

#[test]
fn test114() {
    super::test_it (
        &vec!["(str 1 \"abc\" 3)"],
        Some("\"1abc3\""),
    );
}

#[test]
fn test115() {
    super::test_it (
        &vec!["(str \"abc  def\" \"ghi jkl\")"],
        Some("\"abc  defghi jkl\""),
    );
}

#[test]
fn test116() {
    super::test_it (
        &vec!["(str \"abc\\ndef\\nghi\")"],
        Some("\"abc\\ndef\\nghi\""),
    );
}

#[test]
fn test117() {
    super::test_it (
        &vec!["(str \"abc\\\\def\\\\ghi\")"],
        Some("\"abc\\\\def\\\\ghi\""),
    );
}

#[test]
fn test118() {
    super::test_it (
        &vec!["(str (list 1 2 \"abc\" \"\\\"\") \"def\")"],
        Some("\"(1 2 abc \\\")def\""),
    );
}

#[test]
fn test119() {
    super::test_it (
        &vec!["(str (list))"],
        Some("\"()\""),
    );
}

// ; Testing prn
//  
#[test]
fn test120() {
    super::test_it (
        &vec!["(prn)"],
        Some("nil"),
    );
}

//  ""
#[test]
fn test121() {
    super::test_it (
        &vec!["(prn \"\")"],
        Some("nil"),
    );
}

//  "abc"
#[test]
fn test122() {
    super::test_it (
        &vec!["(prn \"abc\")"],
        Some("nil"),
    );
}

//  "abc  def" "ghi jkl"
//  "\""
#[test]
fn test123() {
    super::test_it (
        &vec!["(prn \"\\\"\")"],
        Some("nil"),
    );
}

//  "abc\ndef\nghi"
#[test]
fn test124() {
    super::test_it (
        &vec!["(prn \"abc\\ndef\\nghi\")"],
        Some("nil"),
    );
}

//  "abc\\def\\ghi"
//  (1 2 "abc" "\"") "def"
#[test]
fn test125() {
    super::test_it (
        &vec!["(prn (list 1 2 \"abc\" \"\\\"\") \"def\")"],
        Some("nil"),
    );
}

// ; Testing println
//  
#[test]
fn test126() {
    super::test_it (
        &vec!["(println)"],
        Some("nil"),
    );
}

//  
#[test]
fn test127() {
    super::test_it (
        &vec!["(println \"\")"],
        Some("nil"),
    );
}

//  abc
#[test]
fn test128() {
    super::test_it (
        &vec!["(println \"abc\")"],
        Some("nil"),
    );
}

//  abc  def ghi jkl
//  "
#[test]
fn test129() {
    super::test_it (
        &vec!["(println \"\\\"\")"],
        Some("nil"),
    );
}

//  abc
//  def
//  ghi
#[test]
fn test130() {
    super::test_it (
        &vec!["(println \"abc\\ndef\\nghi\")"],
        Some("nil"),
    );
}

//  abc\def\ghi
#[test]
fn test131() {
    super::test_it (
        &vec!["(println \"abc\\\\def\\\\ghi\")"],
        Some("nil"),
    );
}

//  (1 2 abc ") def
#[test]
fn test132() {
    super::test_it (
        &vec!["(println (list 1 2 \"abc\" \"\\\"\") \"def\")"],
        Some("nil"),
    );
}

// >>> optional=True
// ;
// ; -------- Optional Functionality --------
// ; Testing keywords
#[test]
fn test133() {
    super::test_it (
        &vec!["(= :abc :abc)"],
        Some("true"),
    );
}

#[test]
fn test134() {
    super::test_it (
        &vec!["(= :abc :def)"],
        Some("false"),
    );
}

#[test]
fn test135() {
    super::test_it (
        &vec!["(= :abc \":abc\")"],
        Some("false"),
    );
}

// ; Testing vector truthiness
#[test]
fn test136() {
    super::test_it (
        &vec!["(if [] 7 8)"],
        Some("7"),
    );
}

// ; Testing vector printing
#[test]
fn test137() {
    super::test_it (
        &vec!["(pr-str [1 2 \"abc\" \"\\\"\"] \"def\")"],
        Some("\"[1 2 \\\"abc\\\" \\\"\\\\\\\"\\\"] \\\"def\\\"\""),
    );
}

#[test]
fn test138() {
    super::test_it (
        &vec!["(pr-str [])"],
        Some("\"[]\""),
    );
}

#[test]
fn test139() {
    super::test_it (
        &vec!["(str [1 2 \"abc\" \"\\\"\"] \"def\")"],
        Some("\"[1 2 abc \\\"]def\""),
    );
}

#[test]
fn test140() {
    super::test_it (
        &vec!["(str [])"],
        Some("\"[]\""),
    );
}

// ; Testing vector functions
#[test]
fn test141() {
    super::test_it (
        &vec!["(count [1 2 3])"],
        Some("3"),
    );
}

#[test]
fn test142() {
    super::test_it (
        &vec!["(empty? [1 2 3])"],
        Some("false"),
    );
}

#[test]
fn test143() {
    super::test_it (
        &vec!["(empty? [])"],
        Some("true"),
    );
}

#[test]
fn test144() {
    super::test_it (
        &vec!["(list? [4 5 6])"],
        Some("false"),
    );
}

// ; Testing vector equality
#[test]
fn test145() {
    super::test_it (
        &vec!["(= [] (list))"],
        Some("true"),
    );
}

#[test]
fn test146() {
    super::test_it (
        &vec!["(= [7 8] [7 8])"],
        Some("true"),
    );
}

#[test]
fn test147() {
    super::test_it (
        &vec!["(= (list 1 2) [1 2])"],
        Some("true"),
    );
}

#[test]
fn test148() {
    super::test_it (
        &vec!["(= (list 1) [])"],
        Some("false"),
    );
}

#[test]
fn test149() {
    super::test_it (
        &vec!["(= [] [1])"],
        Some("false"),
    );
}

#[test]
fn test150() {
    super::test_it (
        &vec!["(= 0 [])"],
        Some("false"),
    );
}

#[test]
fn test151() {
    super::test_it (
        &vec!["(= [] 0)"],
        Some("false"),
    );
}

#[test]
fn test152() {
    super::test_it (
        &vec!["(= [] \"\")"],
        Some("false"),
    );
}

#[test]
fn test153() {
    super::test_it (
        &vec!["(= \"\" [])"],
        Some("false"),
    );
}

// ; Testing vector parameter lists
#[test]
fn test154() {
    super::test_it (
        &vec!["( (fn* [] 4) )"],
        Some("4"),
    );
}

#[test]
fn test155() {
    super::test_it (
        &vec!["( (fn* [f x] (f x)) (fn* [a] (+ 1 a)) 7)"],
        Some("8"),
    );
}

// ; Nested vector/list equality
#[test]
fn test156() {
    super::test_it (
        &vec!["(= [(list)] (list []))"],
        Some("true"),
    );
}

#[test]
fn test157() {
    super::test_it (
        &vec!["(= [1 2 (list 3 4 [5 6])] (list 1 2 [3 4 (list 5 6)]))"],
        Some("true"),
    );
}

