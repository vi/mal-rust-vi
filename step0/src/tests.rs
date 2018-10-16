//  Testing basic string
#[test]
fn test0() {
    super::test_it (
        "abcABC123",
        "abcABC123",
    );
}

//  Testing string containing spaces
#[test]
fn test1() {
    super::test_it (
        "hello mal world",
        "hello mal world",
    );
}

//  Testing string containing symbols
#[test]
fn test2() {
    super::test_it (
        "[]{}\"'* ;:()",
        "[]{}\"'* ;:()",
    );
}

//  Test long string
#[test]
fn test3() {
    super::test_it (
        "hello world abcdefghijklmnopqrstuvwxyz ABCDEFGHIJKLMNOPQRSTUVWXYZ 0123456789 (;:() []{}\"'* ;:() []{}\"'* ;:() []{}\"'*)",
        "hello world abcdefghijklmnopqrstuvwxyz ABCDEFGHIJKLMNOPQRSTUVWXYZ 0123456789 (;:() []{}\"'* ;:() []{}\"'* ;:() []{}\"'*)",
    );
}

