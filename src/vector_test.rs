use crate::vector::Vector;

#[test]
#[allow(non_snake_case)]
fn add_good() {
    let vA: Vector = Vector::from(vec![0, 1, 2]);
    let vB: Vector = Vector::from(vec![1, 3, 4]);

    let vR: Vector = Vector::from(vec![1, 4, 6]);
    assert_eq!(vA + vB, vR);
}

#[test]
#[allow(non_snake_case)]
fn sub_good() {
    let vA: Vector = Vector::from(vec![0, 1, 2]);
    let vB: Vector = Vector::from(vec![1, 3, 4]);

    let vR: Vector = Vector::from(vec![-1, -2, -2]);
    assert_eq!(vA - vB, vR);
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn sub_bad() {
    let vA: Vector = Vector::new(2);
    let vB: Vector = Vector::new(3);

    let vR = vA - vB;
    dbg!(vR);
}

#[test]
#[allow(non_snake_case)]
fn dot_mul_good() {
    let vA: Vector = Vector::from(vec![0, 1, 2]);
    let vB: Vector = Vector::from(vec![1, 3, 4]);

    let vR: f64 = 3. + 2.*4.;
    assert_eq!(vA * vB, vR);
}

#[test]
#[should_panic]
#[allow(non_snake_case)]
fn dot_mul_bad() {
    let vA: Vector = Vector::new(2);
    let vB: Vector = Vector::new(3);

    let vR = vA * vB;
    dbg!(vR);
}


#[test]
#[allow(non_snake_case)]
fn scalar_div() {
    let vA: Vector = Vector::from(vec![0, 1, 2]);
    let B: f64 = 2.;

    let vR: Vector = Vector::from(vec![0., 0.5, 1.]);
    assert_eq!(vA / B, vR);
}
