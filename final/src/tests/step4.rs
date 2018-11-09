// ; -----------------------------------------------------
// ; Testing list functions
#[test]
fn test000() {
    crate::test_it (
        &vec!["(list)"],
        Some("()"),
    );
}

#[test]
fn test001() {
    crate::test_it (
        &vec!["(list? (list))"],
        Some("true"),
    );
}

#[test]
fn test002() {
    crate::test_it (
        &vec!["(empty? (list))"],
        Some("true"),
    );
}

#[test]
fn test003() {
    crate::test_it (
        &vec!["(empty? (list 1))"],
        Some("false"),
    );
}

#[test]
fn test004() {
    crate::test_it (
        &vec!["(list 1 2 3)"],
        Some("(1 2 3)"),
    );
}

#[test]
fn test005() {
    crate::test_it (
        &vec!["(count (list 1 2 3))"],
        Some("3"),
    );
}

#[test]
fn test006() {
    crate::test_it (
        &vec!["(count (list))"],
        Some("0"),
    );
}

#[test]
fn test7() {
    crate::test_it (
        &vec!["(count nil)"],
        Some("0"),
    );
}

#[test]
fn test008() {
    crate::test_it (
        &vec!["(if (> (count (list 1 2 3)) 3) \"yes\" \"no\")"],
        Some("\"no\""),
    );
}

#[test]
fn test009() {
    crate::test_it (
        &vec!["(if (>= (count (list 1 2 3)) 3) \"yes\" \"no\")"],
        Some("\"yes\""),
    );
}

// ; Testing if form
#[test]
fn test010() {
    crate::test_it (
        &vec!["(if true 7 8)"],
        Some("7"),
    );
}

#[test]
fn test011() {
    crate::test_it (
        &vec!["(if false 7 8)"],
        Some("8"),
    );
}

#[test]
fn test012() {
    crate::test_it (
        &vec!["(if true (+ 1 7) (+ 1 8))"],
        Some("8"),
    );
}

#[test]
fn test013() {
    crate::test_it (
        &vec!["(if false (+ 1 7) (+ 1 8))"],
        Some("9"),
    );
}

#[test]
fn test014() {
    crate::test_it (
        &vec!["(if nil 7 8)"],
        Some("8"),
    );
}

#[test]
fn test015() {
    crate::test_it (
        &vec!["(if 0 7 8)"],
        Some("7"),
    );
}

#[test]
fn test016() {
    crate::test_it (
        &vec!["(if \"\" 7 8)"],
        Some("7"),
    );
}

#[test]
fn test017() {
    crate::test_it (
        &vec!["(if (list) 7 8)"],
        Some("7"),
    );
}

#[test]
fn test018() {
    crate::test_it (
        &vec!["(if (list 1 2 3) 7 8)"],
        Some("7"),
    );
}

#[test]
fn test019() {
    crate::test_it (
        &vec!["(= (list) nil)"],
        Some("false"),
    );
}

// ; Testing 1-way if form
#[test]
fn test020() {
    crate::test_it (
        &vec!["(if false (+ 1 7))"],
        Some("nil"),
    );
}

#[test]
fn test021() {
    crate::test_it (
        &vec!["(if nil 8 7)"],
        Some("7"),
    );
}

#[test]
fn test022() {
    crate::test_it (
        &vec!["(if true (+ 1 7))"],
        Some("8"),
    );
}

// ; Testing basic conditionals
#[test]
fn test023() {
    crate::test_it (
        &vec!["(= 2 1)"],
        Some("false"),
    );
}

#[test]
fn test024() {
    crate::test_it (
        &vec!["(= 1 1)"],
        Some("true"),
    );
}

#[test]
fn test025() {
    crate::test_it (
        &vec!["(= 1 2)"],
        Some("false"),
    );
}

#[test]
fn test026() {
    crate::test_it (
        &vec!["(= 1 (+ 1 1))"],
        Some("false"),
    );
}

#[test]
fn test027() {
    crate::test_it (
        &vec!["(= 2 (+ 1 1))"],
        Some("true"),
    );
}

#[test]
fn test028() {
    crate::test_it (
        &vec!["(= nil 1)"],
        Some("false"),
    );
}

#[test]
fn test029() {
    crate::test_it (
        &vec!["(= nil nil)"],
        Some("true"),
    );
}

#[test]
fn test030() {
    crate::test_it (
        &vec!["(> 2 1)"],
        Some("true"),
    );
}

#[test]
fn test031() {
    crate::test_it (
        &vec!["(> 1 1)"],
        Some("false"),
    );
}

#[test]
fn test032() {
    crate::test_it (
        &vec!["(> 1 2)"],
        Some("false"),
    );
}

#[test]
fn test033() {
    crate::test_it (
        &vec!["(>= 2 1)"],
        Some("true"),
    );
}

#[test]
fn test034() {
    crate::test_it (
        &vec!["(>= 1 1)"],
        Some("true"),
    );
}

#[test]
fn test035() {
    crate::test_it (
        &vec!["(>= 1 2)"],
        Some("false"),
    );
}

#[test]
fn test036() {
    crate::test_it (
        &vec!["(< 2 1)"],
        Some("false"),
    );
}

#[test]
fn test037() {
    crate::test_it (
        &vec!["(< 1 1)"],
        Some("false"),
    );
}

#[test]
fn test038() {
    crate::test_it (
        &vec!["(< 1 2)"],
        Some("true"),
    );
}

#[test]
fn test039() {
    crate::test_it (
        &vec!["(<= 2 1)"],
        Some("false"),
    );
}

#[test]
fn test040() {
    crate::test_it (
        &vec!["(<= 1 1)"],
        Some("true"),
    );
}

#[test]
fn test041() {
    crate::test_it (
        &vec!["(<= 1 2)"],
        Some("true"),
    );
}

// ; Testing equality
#[test]
fn test042() {
    crate::test_it (
        &vec!["(= 1 1)"],
        Some("true"),
    );
}

#[test]
fn test043() {
    crate::test_it (
        &vec!["(= 0 0)"],
        Some("true"),
    );
}

#[test]
fn test044() {
    crate::test_it (
        &vec!["(= 1 0)"],
        Some("false"),
    );
}

#[test]
fn test045() {
    crate::test_it (
        &vec!["(= \"\" \"\")"],
        Some("true"),
    );
}

#[test]
fn test046() {
    crate::test_it (
        &vec!["(= \"abc\" \"abc\")"],
        Some("true"),
    );
}

#[test]
fn test047() {
    crate::test_it (
        &vec!["(= \"abc\" \"\")"],
        Some("false"),
    );
}

#[test]
fn test048() {
    crate::test_it (
        &vec!["(= \"\" \"abc\")"],
        Some("false"),
    );
}

#[test]
fn test049() {
    crate::test_it (
        &vec!["(= \"abc\" \"def\")"],
        Some("false"),
    );
}

#[test]
fn test050() {
    crate::test_it (
        &vec!["(= \"abc\" \"ABC\")"],
        Some("false"),
    );
}

#[test]
fn test051() {
    crate::test_it (
        &vec!["(= true true)"],
        Some("true"),
    );
}

#[test]
fn test052() {
    crate::test_it (
        &vec!["(= false false)"],
        Some("true"),
    );
}

#[test]
fn test053() {
    crate::test_it (
        &vec!["(= nil nil)"],
        Some("true"),
    );
}

#[test]
fn test054() {
    crate::test_it (
        &vec!["(= (list) (list))"],
        Some("true"),
    );
}

#[test]
fn test055() {
    crate::test_it (
        &vec!["(= (list 1 2) (list 1 2))"],
        Some("true"),
    );
}

#[test]
fn test056() {
    crate::test_it (
        &vec!["(= (list 1) (list))"],
        Some("false"),
    );
}

#[test]
fn test057() {
    crate::test_it (
        &vec!["(= (list) (list 1))"],
        Some("false"),
    );
}

#[test]
fn test058() {
    crate::test_it (
        &vec!["(= 0 (list))"],
        Some("false"),
    );
}

#[test]
fn test059() {
    crate::test_it (
        &vec!["(= (list) 0)"],
        Some("false"),
    );
}

#[test]
fn test060() {
    crate::test_it (
        &vec!["(= (list) \"\")"],
        Some("false"),
    );
}

#[test]
fn test061() {
    crate::test_it (
        &vec!["(= \"\" (list))"],
        Some("false"),
    );
}

// ; Testing builtin and user defined functions
#[test]
fn test062() {
    crate::test_it (
        &vec!["(+ 1 2)"],
        Some("3"),
    );
}

#[test]
fn test063() {
    crate::test_it (
        &vec!["( (fn* (a b) (+ b a)) 3 4)"],
        Some("7"),
    );
}

#[test]
fn test064() {
    crate::test_it (
        &vec!["( (fn* () 4) )"],
        Some("4"),
    );
}

#[test]
fn test065() {
    crate::test_it (
        &vec!["( (fn* (f x) (f x)) (fn* (a) (+ 1 a)) 7)"],
        Some("8"),
    );
}

// ; Testing closures
#[test]
fn test066() {
    crate::test_it (
        &vec!["( ( (fn* (a) (fn* (b) (+ a b))) 5) 7)"],
        Some("12"),
    );
}

#[test]
fn test067() {
    crate::test_it (
        &vec![
            "(def! gen-plus5 (fn* () (fn* (b) (+ 5 b))))",
            "(def! plus5 (gen-plus5))",
            "(plus5 7)",
        ],
        Some("12"),
    );
}

#[test]
fn test068() {
    crate::test_it (
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
fn test069() {
    crate::test_it (
        &vec!["(do (prn \"prn output1\"))"],
        Some("nil"),
    );
}

//  "prn output2"
#[test]
fn test070() {
    crate::test_it (
        &vec!["(do (prn \"prn output2\") 7)"],
        Some("7"),
    );
}

//  "prn output1"
//  "prn output2"
#[test]
fn test071() {
    crate::test_it (
        &vec!["(do (prn \"prn output1\") (prn \"prn output2\") (+ 1 2))"],
        Some("3"),
    );
}

#[test]
fn test072() {
    crate::test_it (
        &vec!["(do (def! a 6) 7 (+ a 8))"],
        Some("14"),
    );
}

#[test]
fn test073() {
    crate::test_it (
        &vec!["(do (def! a 6) 7 (+ a 8))","a"],
        Some("6"),
    );
}

// ; Testing special form case-sensitivity
#[test]
fn test074() {
    crate::test_it (
        &vec!["(def! DO (fn* (a) 7))","(DO 3)"],
        Some("7"),
    );
}

// ; Testing recursive sumdown function
#[test]
fn test075() {
    crate::test_it (
        &vec![
            "(def! sumdown (fn* (N) (if (> N 0) (+ N (sumdown  (- N 1))) 0)))",
            "(sumdown 1)",
        ],
        Some("1"),
    );
}

#[test]
fn test076() {
    crate::test_it (
        &vec![
            "(def! sumdown (fn* (N) (if (> N 0) (+ N (sumdown  (- N 1))) 0)))",
            "(sumdown 2)",
        ],
        Some("3"),
    );
}

#[test]
fn test077() {
    crate::test_it (
        &vec![
            "(def! sumdown (fn* (N) (if (> N 0) (+ N (sumdown  (- N 1))) 0)))",
            "(sumdown 6)",
        ],
        Some("21"),
    );
}

// ; Testing recursive fibonacci function
#[test]
fn test078() {
    crate::test_it (
        &vec![
            "(def! fib (fn* (N) (if (= N 0) 1 (if (= N 1) 1 (+ (fib (- N 1)) (fib (- N 2)))))))",
            "(fib 1)",
        ],
        Some("1"),
    );
}

#[test]
fn test079() {
    crate::test_it (
        &vec![
            "(def! fib (fn* (N) (if (= N 0) 1 (if (= N 1) 1 (+ (fib (- N 1)) (fib (- N 2)))))))",
            "(fib 2)",
        ],
        Some("2"),
    );
}

#[test]
fn test080() {
    crate::test_it (
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
fn test081() {
    crate::test_it (
        &vec!["( (fn* (& more) (count more)) 1 2 3)"],
        Some("3"),
    );
}

#[test]
fn test082() {
    crate::test_it (
        &vec!["( (fn* (& more) (list? more)) 1 2 3)"],
        Some("true"),
    );
}

#[test]
fn test083() {
    crate::test_it (
        &vec!["( (fn* (& more) (count more)) 1)"],
        Some("1"),
    );
}

#[test]
fn test084() {
    crate::test_it (
        &vec!["( (fn* (& more) (count more)) )"],
        Some("0"),
    );
}

#[test]
fn test085() {
    crate::test_it (
        &vec!["( (fn* (& more) (list? more)) )"],
        Some("true"),
    );
}

#[test]
fn test086() {
    crate::test_it (
        &vec!["( (fn* (a & more) (count more)) 1 2 3)"],
        Some("2"),
    );
}

#[test]
fn test087() {
    crate::test_it (
        &vec!["( (fn* (a & more) (count more)) 1)"],
        Some("0"),
    );
}

#[test]
fn test088() {
    crate::test_it (
        &vec!["( (fn* (a & more) (list? more)) 1)"],
        Some("true"),
    );
}

// ; Testing language defined not function
#[test]
fn test089() {
    crate::test_it (
        &vec!["(not false)"],
        Some("true"),
    );
}

#[test]
fn test090() {
    crate::test_it (
        &vec!["(not nil)"],
        Some("true"),
    );
}

#[test]
fn test091() {
    crate::test_it (
        &vec!["(not true)"],
        Some("false"),
    );
}

#[test]
fn test092() {
    crate::test_it (
        &vec!["(not \"a\")"],
        Some("false"),
    );
}

#[test]
fn test093() {
    crate::test_it (
        &vec!["(not 0)"],
        Some("false"),
    );
}

// ; -----------------------------------------------------
// ; Testing string quoting
#[test]
fn test094() {
    crate::test_it (
        &vec!["\"\""],
        Some("\"\""),
    );
}

#[test]
fn test095() {
    crate::test_it (
        &vec!["\"abc\""],
        Some("\"abc\""),
    );
}

#[test]
fn test096() {
    crate::test_it (
        &vec!["\"abc  def\""],
        Some("\"abc  def\""),
    );
}

#[test]
fn test097() {
    crate::test_it (
        &vec!["\"\\\"\""],
        Some("\"\\\"\""),
    );
}

#[test]
fn test098() {
    crate::test_it (
        &vec!["\"abc\\ndef\\nghi\""],
        Some("\"abc\\ndef\\nghi\""),
    );
}

#[test]
fn test099() {
    crate::test_it (
        &vec!["\"abc\\\\def\\\\ghi\""],
        Some("\"abc\\\\def\\\\ghi\""),
    );
}

#[test]
fn test100() {
    crate::test_it (
        &vec!["\"\\\\n\""],
        Some("\"\\\\n\""),
    );
}

// ; Testing pr-str
#[test]
fn test101() {
    crate::test_it (
        &vec!["(pr-str)"],
        Some("\"\""),
    );
}

#[test]
fn test102() {
    crate::test_it (
        &vec!["(pr-str \"\")"],
        Some("\"\\\"\\\"\""),
    );
}

#[test]
fn test103() {
    crate::test_it (
        &vec!["(pr-str \"abc\")"],
        Some("\"\\\"abc\\\"\""),
    );
}

#[test]
fn test104() {
    crate::test_it (
        &vec!["(pr-str \"abc  def\" \"ghi jkl\")"],
        Some("\"\\\"abc  def\\\" \\\"ghi jkl\\\"\""),
    );
}

#[test]
fn test105() {
    crate::test_it (
        &vec!["(pr-str \"\\\"\")"],
        Some("\"\\\"\\\\\\\"\\\"\""),
    );
}

#[test]
fn test106() {
    crate::test_it (
        &vec!["(pr-str (list 1 2 \"abc\" \"\\\"\") \"def\")"],
        Some("\"(1 2 \\\"abc\\\" \\\"\\\\\\\"\\\") \\\"def\\\"\""),
    );
}

#[test]
fn test107() {
    crate::test_it (
        &vec!["(pr-str \"abc\\ndef\\nghi\")"],
        Some("\"\\\"abc\\\\ndef\\\\nghi\\\"\""),
    );
}

#[test]
fn test108() {
    crate::test_it (
        &vec!["(pr-str \"abc\\\\def\\\\ghi\")"],
        Some("\"\\\"abc\\\\\\\\def\\\\\\\\ghi\\\"\""),
    );
}

#[test]
fn test109() {
    crate::test_it (
        &vec!["(pr-str (list))"],
        Some("\"()\""),
    );
}

// ; Testing str
#[test]
fn test110() {
    crate::test_it (
        &vec!["(str)"],
        Some("\"\""),
    );
}

#[test]
fn test111() {
    crate::test_it (
        &vec!["(str \"\")"],
        Some("\"\""),
    );
}

#[test]
fn test112() {
    crate::test_it (
        &vec!["(str \"abc\")"],
        Some("\"abc\""),
    );
}

#[test]
fn test113() {
    crate::test_it (
        &vec!["(str \"\\\"\")"],
        Some("\"\\\"\""),
    );
}

#[test]
fn test114() {
    crate::test_it (
        &vec!["(str 1 \"abc\" 3)"],
        Some("\"1abc3\""),
    );
}

#[test]
fn test115() {
    crate::test_it (
        &vec!["(str \"abc  def\" \"ghi jkl\")"],
        Some("\"abc  defghi jkl\""),
    );
}

#[test]
fn test116() {
    crate::test_it (
        &vec!["(str \"abc\\ndef\\nghi\")"],
        Some("\"abc\\ndef\\nghi\""),
    );
}

#[test]
fn test117() {
    crate::test_it (
        &vec!["(str \"abc\\\\def\\\\ghi\")"],
        Some("\"abc\\\\def\\\\ghi\""),
    );
}

#[test]
fn test118() {
    crate::test_it (
        &vec!["(str (list 1 2 \"abc\" \"\\\"\") \"def\")"],
        Some("\"(1 2 abc \\\")def\""),
    );
}

#[test]
fn test119() {
    crate::test_it (
        &vec!["(str (list))"],
        Some("\"()\""),
    );
}

// ; Testing prn
//  
#[test]
fn test120() {
    crate::test_it (
        &vec!["(prn)"],
        Some("nil"),
    );
}

//  ""
#[test]
fn test121() {
    crate::test_it (
        &vec!["(prn \"\")"],
        Some("nil"),
    );
}

//  "abc"
#[test]
fn test122() {
    crate::test_it (
        &vec!["(prn \"abc\")"],
        Some("nil"),
    );
}

//  "abc  def" "ghi jkl"
//  "\""
#[test]
fn test123() {
    crate::test_it (
        &vec!["(prn \"\\\"\")"],
        Some("nil"),
    );
}

//  "abc\ndef\nghi"
#[test]
fn test124() {
    crate::test_it (
        &vec!["(prn \"abc\\ndef\\nghi\")"],
        Some("nil"),
    );
}

//  "abc\\def\\ghi"
//  (1 2 "abc" "\"") "def"
#[test]
fn test125() {
    crate::test_it (
        &vec!["(prn (list 1 2 \"abc\" \"\\\"\") \"def\")"],
        Some("nil"),
    );
}

// ; Testing println
//  
#[test]
fn test126() {
    crate::test_it (
        &vec!["(println)"],
        Some("nil"),
    );
}

//  
#[test]
fn test127() {
    crate::test_it (
        &vec!["(println \"\")"],
        Some("nil"),
    );
}

//  abc
#[test]
fn test128() {
    crate::test_it (
        &vec!["(println \"abc\")"],
        Some("nil"),
    );
}

//  abc  def ghi jkl
//  "
#[test]
fn test129() {
    crate::test_it (
        &vec!["(println \"\\\"\")"],
        Some("nil"),
    );
}

//  abc
//  def
//  ghi
#[test]
fn test130() {
    crate::test_it (
        &vec!["(println \"abc\\ndef\\nghi\")"],
        Some("nil"),
    );
}

//  abc\def\ghi
#[test]
fn test131() {
    crate::test_it (
        &vec!["(println \"abc\\\\def\\\\ghi\")"],
        Some("nil"),
    );
}

//  (1 2 abc ") def
#[test]
fn test132() {
    crate::test_it (
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
    crate::test_it (
        &vec!["(= :abc :abc)"],
        Some("true"),
    );
}

#[test]
fn test134() {
    crate::test_it (
        &vec!["(= :abc :def)"],
        Some("false"),
    );
}

#[test]
fn test135() {
    crate::test_it (
        &vec!["(= :abc \":abc\")"],
        Some("false"),
    );
}

// ; Testing vector truthiness
#[test]
fn test136() {
    crate::test_it (
        &vec!["(if [] 7 8)"],
        Some("7"),
    );
}

// ; Testing vector printing
#[test]
fn test137() {
    crate::test_it (
        &vec!["(pr-str [1 2 \"abc\" \"\\\"\"] \"def\")"],
        Some("\"[1 2 \\\"abc\\\" \\\"\\\\\\\"\\\"] \\\"def\\\"\""),
    );
}

#[test]
fn test138() {
    crate::test_it (
        &vec!["(pr-str [])"],
        Some("\"[]\""),
    );
}

#[test]
fn test139() {
    crate::test_it (
        &vec!["(str [1 2 \"abc\" \"\\\"\"] \"def\")"],
        Some("\"[1 2 abc \\\"]def\""),
    );
}

#[test]
fn test140() {
    crate::test_it (
        &vec!["(str [])"],
        Some("\"[]\""),
    );
}

// ; Testing vector functions
#[test]
fn test141() {
    crate::test_it (
        &vec!["(count [1 2 3])"],
        Some("3"),
    );
}

#[test]
fn test142() {
    crate::test_it (
        &vec!["(empty? [1 2 3])"],
        Some("false"),
    );
}

#[test]
fn test143() {
    crate::test_it (
        &vec!["(empty? [])"],
        Some("true"),
    );
}

#[test]
fn test144() {
    crate::test_it (
        &vec!["(list? [4 5 6])"],
        Some("false"),
    );
}

// ; Testing vector equality
#[test]
fn test145() {
    crate::test_it (
        &vec!["(= [] (list))"],
        Some("true"),
    );
}

#[test]
fn test146() {
    crate::test_it (
        &vec!["(= [7 8] [7 8])"],
        Some("true"),
    );
}

#[test]
fn test147() {
    crate::test_it (
        &vec!["(= (list 1 2) [1 2])"],
        Some("true"),
    );
}

#[test]
fn test148() {
    crate::test_it (
        &vec!["(= (list 1) [])"],
        Some("false"),
    );
}

#[test]
fn test149() {
    crate::test_it (
        &vec!["(= [] [1])"],
        Some("false"),
    );
}

#[test]
fn test150() {
    crate::test_it (
        &vec!["(= 0 [])"],
        Some("false"),
    );
}

#[test]
fn test151() {
    crate::test_it (
        &vec!["(= [] 0)"],
        Some("false"),
    );
}

#[test]
fn test152() {
    crate::test_it (
        &vec!["(= [] \"\")"],
        Some("false"),
    );
}

#[test]
fn test153() {
    crate::test_it (
        &vec!["(= \"\" [])"],
        Some("false"),
    );
}

// ; Testing vector parameter lists
#[test]
fn test154() {
    crate::test_it (
        &vec!["( (fn* [] 4) )"],
        Some("4"),
    );
}

#[test]
fn test155() {
    crate::test_it (
        &vec!["( (fn* [f x] (f x)) (fn* [a] (+ 1 a)) 7)"],
        Some("8"),
    );
}

// ; Nested vector/list equality
#[test]
fn test156() {
    crate::test_it (
        &vec!["(= [(list)] (list []))"],
        Some("true"),
    );
}

#[test]
fn test157() {
    crate::test_it (
        &vec!["(= [1 2 (list 3 4 [5 6])] (list 1 2 [3 4 (list 5 6)]))"],
        Some("true"),
    );
}

