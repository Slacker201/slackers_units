#[test]
fn correct_addition() {
    let zero_point_five = unit!(0.5m);
    assert_eq!(zero_point_five + zero_point_five, unit!(1.0m));
}
#[test]
fn correct_subtraction() {
    let two = unit!(2s);
    let one = unit!(1s);

    assert_eq!(two - one, one);
}
#[test]
fn correct_multiplication() {
    let four = unit!(4m);
    let two = unit!(2m);

    assert_eq!(four * two, unit!(8m^2));
}
#[test]
fn correct_division() {
    let four = unit!(4m);
    let two = unit!(2m);

    assert_eq!(four / two, unit!(2));
}