// ;
// ; Testing try*/catch*
#[test]
fn test000() {
    crate::test_it (
        &vec!["
            (try* 123 (catch* e 456))
        "],
        Some("123"),
    );
}

//  "exc is:" "'abc' not found"
#[test]
fn test001() {
    crate::test_it (
        &vec!["
            (try* (abc 1 2) (catch* exc (prn \"exc is:\" exc)))
        "],
        Some("nil"),
    );
}

//  "exc:" "my exception"
#[test]
fn test002() {
    crate::test_it (
        &vec!["
            (try* (throw \"my exception\") (catch* exc (do (prn \"exc:\" exc) 7)))
        "],
        Some("7"),
    );
}

// ;; Test that throw is a function:
#[test]
fn test003() {
    crate::test_it (
        &vec!["
            (try* (map throw (list \"my err\")) (catch* exc exc))
        "],
        Some("\"my err\""),
    );
}

// ;
// ; Testing builtin functions
#[test]
fn test004() {
    crate::test_it (
        &vec!["
            (symbol? 'abc)
        "],
        Some("true"),
    );
}

#[test]
fn test005() {
    crate::test_it (
        &vec!["
            (symbol? \"abc\")
        "],
        Some("false"),
    );
}

#[test]
fn test006() {
    crate::test_it (
        &vec!["
            (nil? nil)
        "],
        Some("true"),
    );
}

#[test]
fn test007() {
    crate::test_it (
        &vec!["
            (nil? true)
        "],
        Some("false"),
    );
}

#[test]
fn test008() {
    crate::test_it (
        &vec!["
            (true? true)
        "],
        Some("true"),
    );
}

#[test]
fn test009() {
    crate::test_it (
        &vec!["
            (true? false)
        "],
        Some("false"),
    );
}

#[test]
fn test010() {
    crate::test_it (
        &vec!["
            (true? true?)
        "],
        Some("false"),
    );
}

#[test]
fn test011() {
    crate::test_it (
        &vec!["
            (false? false)
        "],
        Some("true"),
    );
}

#[test]
fn test012() {
    crate::test_it (
        &vec!["
            (false? true)
        "],
        Some("false"),
    );
}

// ; Testing apply function with core functions
#[test]
fn test013() {
    crate::test_it (
        &vec!["
            (apply + (list 2 3))
        "],
        Some("5"),
    );
}

#[test]
fn test014() {
    crate::test_it (
        &vec!["
            (apply + 4 (list 5))
        "],
        Some("9"),
    );
}

//  1 2 "3" ()
#[test]
fn test015() {
    crate::test_it (
        &vec!["
            (apply prn (list 1 2 \"3\" (list)))
        "],
        Some("nil"),
    );
}

//  1 2 "3" ()
#[test]
fn test016() {
    crate::test_it (
        &vec!["
            (apply prn 1 2 (list \"3\" (list)))
        "],
        Some("nil"),
    );
}

#[test]
fn test017() {
    crate::test_it (
        &vec!["
            (apply list (list))
        "],
        Some("()"),
    );
}

#[test]
fn test018() {
    crate::test_it (
        &vec!["
            (apply symbol? (list (quote two)))
        "],
        Some("true"),
    );
}

// ; Testing apply function with user functions
#[test]
fn test019() {
    crate::test_it (
        &vec!["
            (apply (fn* (a b) (+ a b)) (list 2 3))
        "],
        Some("5"),
    );
}

#[test]
fn test020() {
    crate::test_it (
        &vec!["
            (apply (fn* (a b) (+ a b)) 4 (list 5))
        "],
        Some("9"),
    );
}

// ; Testing map function
#[test]
fn test021() {
    crate::test_it (
        &vec!["
            (def! nums (list 1 2 3))
            (def! double (fn* (a) (* 2 a)))
            (double 3)
        "],
        Some("6"),
    );
}

#[test]
fn test022() {
    crate::test_it (
        &vec!["
            (def! nums (list 1 2 3))
            (def! double (fn* (a) (* 2 a)))
            (map double nums) 
        "],
        Some("(2 4 6)"),
    );
}

#[test]
fn test023() {
    crate::test_it (
        &vec!["
            (map (fn* (x) (symbol? x)) (list 1 (quote two) \"three\"))
        "],
        Some("(false true false)"),
    );
}

// >>> deferrable=True
// ;
// ; ------- Deferrable Functionality ----------
// ; ------- (Needed for self-hosting) -------
// ; Testing symbol and keyword functions
#[test]
fn test024() {
    crate::test_it (
        &vec!["
            (symbol? :abc)
        "],
        Some("false"),
    );
}

#[test]
fn test025() {
    crate::test_it (
        &vec!["
            (symbol? 'abc)
        "],
        Some("true"),
    );
}

#[test]
fn test026() {
    crate::test_it (
        &vec!["
            (symbol? \"abc\")
        "],
        Some("false"),
    );
}

#[test]
fn test027() {
    crate::test_it (
        &vec!["
            (symbol? (symbol \"abc\"))
        "],
        Some("true"),
    );
}

#[test]
fn test028() {
    crate::test_it (
        &vec!["
            (keyword? :abc)
        "],
        Some("true"),
    );
}

#[test]
fn test029() {
    crate::test_it (
        &vec!["
            (keyword? 'abc)
        "],
        Some("false"),
    );
}

#[test]
fn test030() {
    crate::test_it (
        &vec!["
            (keyword? \"abc\")
        "],
        Some("false"),
    );
}

#[test]
fn test031() {
    crate::test_it (
        &vec!["
            (keyword? \"\")
        "],
        Some("false"),
    );
}

#[test]
fn test032() {
    crate::test_it (
        &vec!["
            (keyword? (keyword \"abc\"))
        "],
        Some("true"),
    );
}

#[test]
fn test033() {
    crate::test_it (
        &vec!["
            (symbol \"abc\")
        "],
        Some("abc"),
    );
}
#[test]
fn test033b() {
    crate::test_it (
        &vec!["
            (keyword :abc)
        "],
        Some(":abc"),
    );
}

#[test]
fn test034() {
    crate::test_it (
        &vec!["
            (keyword \"abc\")
        "],
        Some(":abc"),
    );
}

// ; Testing sequential? function
#[test]
fn test035() {
    crate::test_it (
        &vec!["
            (sequential? (list 1 2 3))
        "],
        Some("true"),
    );
}

#[test]
fn test036() {
    crate::test_it (
        &vec!["
            (sequential? [15])
        "],
        Some("true"),
    );
}

#[test]
fn test037() {
    crate::test_it (
        &vec!["
            (sequential? sequential?)
        "],
        Some("false"),
    );
}

#[test]
fn test038() {
    crate::test_it (
        &vec!["
            (sequential? nil)
        "],
        Some("false"),
    );
}

#[test]
fn test039() {
    crate::test_it (
        &vec!["
            (sequential? \"abc\")
        "],
        Some("false"),
    );
}

// ; Testing apply function with core functions and arguments in vector
#[test]
fn test040() {
    crate::test_it (
        &vec!["
            (apply + 4 [5])
        "],
        Some("9"),
    );
}

//  1 2 "3" 4
#[test]
fn test041() {
    crate::test_it (
        &vec!["
            (apply prn 1 2 [\"3\" 4])
        "],
        Some("nil"),
    );
}

#[test]
fn test042() {
    crate::test_it (
        &vec!["
            (apply list [])
        "],
        Some("()"),
    );
}

// ; Testing apply function with user functions and arguments in vector
#[test]
fn test043() {
    crate::test_it (
        &vec!["
            (apply (fn* (a b) (+ a b)) [2 3])
        "],
        Some("5"),
    );
}

#[test]
fn test044() {
    crate::test_it (
        &vec!["
            (apply (fn* (a b) (+ a b)) 4 [5])
        "],
        Some("9"),
    );
}

// ; Testing map function with vectors
#[test]
fn test045() {
    crate::test_it (
        &vec!["
            (map (fn* (a) (* 2 a)) [1 2 3])
        "],
        Some("(2 4 6)"),
    );
}

#[test]
fn test046() {
    crate::test_it (
        &vec!["
            (map (fn* [& args] (list? args)) [1 2])
        "],
        Some("(true true)"),
    );
}

// ; Testing vector functions
#[test]
fn test047() {
    crate::test_it (
        &vec!["
            (vector? [10 11])
        "],
        Some("true"),
    );
}

#[test]
fn test048() {
    crate::test_it (
        &vec!["
            (vector? '(12 13))
        "],
        Some("false"),
    );
}

#[test]
fn test049() {
    crate::test_it (
        &vec!["
            (vector 3 4 5)
        "],
        Some("[3 4 5]"),
    );
}

#[test]
fn test050() {
    crate::test_it (
        &vec!["
            (map? {})
        "],
        Some("true"),
    );
}

#[test]
fn test051() {
    crate::test_it (
        &vec!["
            (map? '())
        "],
        Some("false"),
    );
}

#[test]
fn test052() {
    crate::test_it (
        &vec!["
            (map? [])
        "],
        Some("false"),
    );
}

#[test]
fn test053() {
    crate::test_it (
        &vec!["
            (map? 'abc)
        "],
        Some("false"),
    );
}

#[test]
fn test054() {
    crate::test_it (
        &vec!["
            (map? :abc)
        "],
        Some("false"),
    );
}

// ;
// ; Testing hash-maps
#[test]
fn test055() {
    crate::test_it (
        &vec!["
            (hash-map \"a\" 1)
        "],
        Some("{\"a\" 1}"),
    );
}

#[test]
fn test056() {
    crate::test_it (
        &vec!["
            {\"a\" 1}
        "],
        Some("{\"a\" 1}"),
    );
}

#[test]
fn test057() {
    crate::test_it (
        &vec!["
            (assoc {} \"a\" 1)
        "],
        Some("{\"a\" 1}"),
    );
}

#[test]
fn test058() {
    crate::test_it (
        &vec!["
            (get (assoc (assoc {\"a\" 1 } \"b\" 2) \"c\" 3) \"a\")
        "],
        Some("1"),
    );
}

#[test]
fn test059() {
    crate::test_it (
        &vec!["
            (def! hm1 (hash-map))
        "],
        Some("{}"),
    );
}

#[test]
fn test060() {
    crate::test_it (
        &vec!["
            (def! hm1 (hash-map))
            (map? hm1)
        "],
        Some("true"),
    );
}

#[test]
fn test061() {
    crate::test_it (
        &vec!["
            (map? 1)
        "],
        Some("false"),
    );
}

#[test]
fn test062() {
    crate::test_it (
        &vec!["
            (map? \"abc\")
        "],
        Some("false"),
    );
}

#[test]
fn test063() {
    crate::test_it (
        &vec!["
            (get nil \"a\")
        "],
        Some("nil"),
    );
}

#[test]
fn test064() {
    crate::test_it (
        &vec!["
            (def! hm1 (hash-map))
            (get hm1 \"a\")
        "],
        Some("nil"),
    );
}

#[test]
fn test065() {
    crate::test_it (
        &vec!["
            (def! hm1 (hash-map))
            (contains? hm1 \"a\")
        "],
        Some("false"),
    );
}

#[test]
fn test066() {
    crate::test_it (
        &vec!["
            (def! hm1 (hash-map))
            (def! hm2 (assoc hm1 \"a\" 1))
        "],
        Some("{\"a\" 1}"),
    );
}

#[test]
fn test067() {
    crate::test_it (
        &vec!["
            (def! hm1 (hash-map))
            (def! hm2 (assoc hm1 \"a\" 1))
            (get hm1 \"a\")
        "],
        Some("nil"),
    );
}

#[test]
fn test068() {
    crate::test_it (
        &vec!["
            (def! hm1 (hash-map))
            (def! hm2 (assoc hm1 \"a\" 1))
            (contains? hm1 \"a\")
        "],
        Some("false"),
    );
}

#[test]
fn test069() {
    crate::test_it (
        &vec!["
            (def! hm1 (hash-map))
            (def! hm2 (assoc hm1 \"a\" 1))
            (get hm2 \"a\")
        "],
        Some("1"),
    );
}

#[test]
fn test070() {
    crate::test_it (
        &vec!["
            (def! hm1 (hash-map))
            (def! hm2 (assoc hm1 \"a\" 1))
            (contains? hm2 \"a\")
        "],
        Some("true"),
    );
}

// ;; TODO: fix. Clojure returns nil but this breaks mal impl
#[test]
fn test071() {
    crate::test_it (
        &vec!["
            (def! hm1 (hash-map))
            (def! hm2 (assoc hm1 \"a\" 1))
            (keys hm1)
        "],
        Some("()"),
    );
}

#[test]
fn test072() {
    crate::test_it (
        &vec!["
            (def! hm1 (hash-map))
            (def! hm2 (assoc hm1 \"a\" 1))
            (keys hm2)
        "],
        Some("(\"a\")"),
    );
}

// ;; TODO: fix. Clojure returns nil but this breaks mal impl
#[test]
fn test073() {
    crate::test_it (
        &vec!["
            (def! hm1 (hash-map))
            (def! hm2 (assoc hm1 \"a\" 1))
            (vals hm1)
        "],
        Some("()"),
    );
}

#[test]
fn test074() {
    crate::test_it (
        &vec!["
            (def! hm1 (hash-map))
            (def! hm2 (assoc hm1 \"a\" 1))
            (vals hm2)
        "],
        Some("(1)"),
    );
}

#[test]
fn test075() {
    crate::test_it (
        &vec!["
            (def! hm1 (hash-map))
            (def! hm2 (assoc hm1 \"a\" 1))
            (count (keys (assoc hm2 \"b\" 2 \"c\" 3)))
        "],
        Some("3"),
    );
}

// ; Testing keywords as hash-map keys
#[test]
fn test076() {
    crate::test_it (
        &vec!["
            (get {:abc 123} :abc)
        "],
        Some("123"),
    );
}

#[test]
fn test077() {
    crate::test_it (
        &vec!["
            (contains? {:abc 123} :abc)
        "],
        Some("true"),
    );
}

#[test]
fn test078() {
    crate::test_it (
        &vec!["
            (contains? {:abcd 123} :abc)
        "],
        Some("false"),
    );
}

#[test]
fn test079() {
    crate::test_it (
        &vec!["
            (assoc {} :bcd 234)
        "],
        Some("{:bcd 234}"),
    );
}

#[test]
fn test080() {
    crate::test_it (
        &vec!["
            (keyword? (nth (keys {:abc 123 :def 456}) 0))
        "],
        Some("true"),
    );
}

#[test]
fn test080b() {
    crate::test_it (
        &vec!["
            (keyword? (nth (keys {\":abc\" 123 \":def\" 456}) 0))
        "],
        Some("false"),
    );
}

#[test]
fn test081() {
    crate::test_it (
        &vec!["
            (keyword? (nth (vals {\"a\" :abc \"b\" :def}) 0))
        "],
        Some("true"),
    );
}

// ; Testing whether assoc updates properly
#[test]
fn test082() {
    crate::test_it (
        &vec!["
            (def! hm4 (assoc {:a 1 :b 2} :a 3 :c 1))
            (get hm4 :a)
        "],
        Some("3"),
    );
}

#[test]
fn test083() {
    crate::test_it (
        &vec!["
            (def! hm4 (assoc {:a 1 :b 2} :a 3 :c 1))
            (get hm4 :b)
        "],
        Some("2"),
    );
}

#[test]
fn test084() {
    crate::test_it (
        &vec!["
            (def! hm4 (assoc {:a 1 :b 2} :a 3 :c 1))
            (get hm4 :c)
        "],
        Some("1"),
    );
}

// ; Testing nil as hash-map values
#[test]
fn test085() {
    crate::test_it (
        &vec!["
            (contains? {:abc nil} :abc)
        "],
        Some("true"),
    );
}

#[test]
fn test086() {
    crate::test_it (
        &vec!["
            (assoc {} :bcd nil)
        "],
        Some("{:bcd nil}"),
    );
}

// ;
// ; Additional str and pr-str tests
#[test]
fn test087() {
    crate::test_it (
        &vec!["
            (str \"A\" {:abc \"val\"} \"Z\")
        "],
        Some("\"A{:abc val}Z\""),
    );
}

#[test]
fn test088() {
    crate::test_it (
        &vec!["
            (str true \".\" false \".\" nil \".\" :keyw \".\" 'symb)
        "],
        Some("\"true.false.nil.:keyw.symb\""),
    );
}

#[test]
fn test089() {
    crate::test_it (
        &vec!["
            (pr-str \"A\" {:abc \"val\"} \"Z\")
        "],
        Some("\"\\\"A\\\" {:abc \\\"val\\\"} \\\"Z\\\"\""),
    );
}

#[test]
fn test090() {
    crate::test_it (
        &vec!["
            (pr-str true \".\" false \".\" nil \".\" :keyw \".\" 'symb)
        "],
        Some("\"true \\\".\\\" false \\\".\\\" nil \\\".\\\" :keyw \\\".\\\" symb\""),
    );
}

#[test]
fn test091() {
    crate::test_it (
        &vec!["
            (def! s (str {:abc \"val1\" :def \"val2\"}))
            (or (= s \"{:abc val1, :def val2}\") (= s \"{:def val2, :abc val1}\"))
        "],
        Some("true"),
    );
}

#[test]
fn test092() {
    crate::test_it (
        &vec!["
            (def! p (pr-str {:abc \"val1\" :def \"val2\"}))
            (or (= p \"{:abc \\\"val1\\\", :def \\\"val2\\\"}\") (= p \"{:def \\\"val2\\\", :abc \\\"val1\\\"}\"))
        "],
        Some("true"),
    );
}

// ;
// ; Test extra function arguments as Mal List (bypassing TCO with apply)
#[test]
fn test093() {
    crate::test_it (
        &vec!["
            (apply (fn* (& more) (list? more)) [1 2 3])
        "],
        Some("true"),
    );
}

#[test]
fn test094() {
    crate::test_it (
        &vec!["
            (apply (fn* (& more) (list? more)) [])
        "],
        Some("true"),
    );
}

#[test]
fn test095() {
    crate::test_it (
        &vec!["
            (apply (fn* (a & more) (list? more)) [1])
        "],
        Some("true"),
    );
}

// >>> soft=True
// >>> optional=True
// ;
// ; ------- Optional Functionality --------------
// ; ------- (Not needed for self-hosting) -------
// ;;TODO: fix so long lines don't trigger ANSI escape codes ;;;(try*
// ;;(try* (throw ["data" "foo"]) (catch* exc (do (prn "exc is:" exc) 7))) ;;;;
// ;;; "exc is:" ["data" "foo"] ;;;;=>7
// ;;;=>7
// ;
// ; Testing throwing non-strings
//  "err:" (1 2 3)
#[test]
fn test096() {
    crate::test_it (
        &vec!["
            (try* (throw (list 1 2 3)) (catch* exc (do (prn \"err:\" exc) 7)))
        "],
        Some("7"),
    );
}

// ;
// ; Testing dissoc
#[test]
fn test097() {
    crate::test_it (
        &vec!["
            (def! hm1 (hash-map))
            (def! hm2 (assoc hm1 \"a\" 1))
            (def! hm3 (assoc hm2 \"b\" 2))
            (count (keys hm3))
        "],
        Some("2"),
    );
}

#[test]
fn test098() {
    crate::test_it (
        &vec!["
            (def! hm1 (hash-map))
            (def! hm2 (assoc hm1 \"a\" 1))
            (def! hm3 (assoc hm2 \"b\" 2))
            (count (vals hm3))
        "],
        Some("2"),
    );
}

#[test]
fn test099() {
    crate::test_it (
        &vec!["
            (def! hm1 (hash-map))
            (def! hm2 (assoc hm1 \"a\" 1))
            (def! hm3 (assoc hm2 \"b\" 2))
            (dissoc hm3 \"a\")
        "],
        Some("{\"b\" 2}"),
    );
}

#[test]
fn test100() {
    crate::test_it (
        &vec!["
            (def! hm1 (hash-map))
            (def! hm2 (assoc hm1 \"a\" 1))
            (def! hm3 (assoc hm2 \"b\" 2))
            (dissoc hm3 \"a\" \"b\")
        "],
        Some("{}"),
    );
}

#[test]
fn test101() {
    crate::test_it (
        &vec!["
            (def! hm1 (hash-map))
            (def! hm2 (assoc hm1 \"a\" 1))
            (def! hm3 (assoc hm2 \"b\" 2))
            (dissoc hm3 \"a\" \"b\" \"c\")
        "],
        Some("{}"),
    );
}

#[test]
fn test102() {
    crate::test_it (
        &vec!["
            (def! hm1 (hash-map))
            (def! hm2 (assoc hm1 \"a\" 1))
            (def! hm3 (assoc hm2 \"b\" 2))
            (dissoc hm3 \"a\" \"b\")
            (dissoc hm3 \"a\" \"b\" \"c\")
            (count (keys hm3))
        "],
        Some("2"),
    );
}

#[test]
fn test103() {
    crate::test_it (
        &vec!["
            (dissoc {:cde 345 :fgh 456} :cde)
        "],
        Some("{:fgh 456}"),
    );
}

#[test]
fn test104() {
    crate::test_it (
        &vec!["
            (dissoc {:cde nil :fgh 456} :cde)
        "],
        Some("{:fgh 456}"),
    );
}

// ;
// ; Testing equality of hash-maps
#[test]
fn test105() {
    crate::test_it (
        &vec!["
            (= {} {})
        "],
        Some("true"),
    );
}

#[test]
fn test106() {
    crate::test_it (
        &vec!["
            (= {:a 11 :b 22} (hash-map :b 22 :a 11))
        "],
        Some("true"),
    );
}

#[test]
fn test107() {
    crate::test_it (
        &vec!["
            (= {:a 11 :b [22 33]} (hash-map :b [22 33] :a 11))
        "],
        Some("true"),
    );
}

#[test]
fn test108() {
    crate::test_it (
        &vec!["
            (= {:a 11 :b {:c 33}} (hash-map :b {:c 33} :a 11))
        "],
        Some("true"),
    );
}

#[test]
fn test109() {
    crate::test_it (
        &vec!["
            (= {:a 11 :b 22} (hash-map :b 23 :a 11))
        "],
        Some("false"),
    );
}

#[test]
fn test110() {
    crate::test_it (
        &vec!["
            (= {:a 11 :b 22} (hash-map :a 11))
        "],
        Some("false"),
    );
}

#[test]
fn test111() {
    crate::test_it (
        &vec!["
            (= {:a [11 22]} {:a (list 11 22)})
        "],
        Some("true"),
    );
}

#[test]
fn test112() {
    crate::test_it (
        &vec!["
            (= {:a 11 :b 22} (list :a 11 :b 22))
        "],
        Some("false"),
    );
}

#[test]
fn test113() {
    crate::test_it (
        &vec!["
            (= {} [])
        "],
        Some("false"),
    );
}

#[test]
fn test114() {
    crate::test_it (
        &vec!["
            (= [] {})
        "],
        Some("false"),
    );
}

