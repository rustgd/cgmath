use sign::*;

#[test]
fn test_abs() {
    assert 0.abs()      == 0;
    assert 2.abs()      == 2;
    assert (-2).abs()   == 2;
    assert abs(&0)      == 0;
    assert abs(&2)      == 2;
    assert abs(&-2)     == 2;
    
    assert 0.0.abs()    == 0.0;
    assert 2.5.abs()    == 2.5;
    assert (-2.5).abs() == 2.5;
    assert abs(&0.0)    == 0.0;
    assert abs(&2.5)    == 2.5;
    assert abs(&-2.5)   == 2.5;
}

#[test]
fn test_sign() {
    assert 0.sign()     == 0;
    assert 2.sign()     == 1;
    assert (-2).sign()  == -1;
    assert sign(&0)     == 0;
    assert sign(&2)     == 1;
    assert sign(&-2)    == -1;
    
    assert 0.0.sign()   == 0.0;
    assert 2.5.sign()   == 1.0;
    assert (-2.5).sign()== -1.0;
    assert sign(&0.0)   == 0.0;
    assert sign(&2.5)   == 1.0;
    assert sign(&-2.5)  == -1.0;
}