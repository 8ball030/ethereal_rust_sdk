//! Order prices snap to the tick without crossing in the taker direction, and quantities floor
//! onto the lot and never reach the venue as zero.

use ethereal_rust_sdk::models::OrderSide;
use ethereal_rust_sdk::utils::{floor_quantity_to_lot, round_price_to_tick};
use rust_decimal_macros::dec;

#[test]
fn a_buy_price_rounds_down_and_a_sell_price_rounds_up() {
    assert_eq!(
        round_price_to_tick(dec!(100.26), dec!(0.1), OrderSide::BUY).unwrap(),
        dec!(100.2)
    );
    assert_eq!(
        round_price_to_tick(dec!(100.21), dec!(0.1), OrderSide::SELL).unwrap(),
        dec!(100.3)
    );
    assert_eq!(
        round_price_to_tick(dec!(100.2), dec!(0.1), OrderSide::BUY).unwrap(),
        dec!(100.2),
        "on the tick stays"
    );
}

#[test]
fn adjacent_half_ticks_round_the_same_way_for_the_same_side() {
    // nearest-with-banker's rounding sent 100.15 to 100.2 and 100.25 to 100.2
    assert_eq!(
        round_price_to_tick(dec!(100.15), dec!(0.1), OrderSide::BUY).unwrap(),
        dec!(100.1)
    );
    assert_eq!(
        round_price_to_tick(dec!(100.25), dec!(0.1), OrderSide::BUY).unwrap(),
        dec!(100.2)
    );
    assert_eq!(
        round_price_to_tick(dec!(100.15), dec!(0.1), OrderSide::SELL).unwrap(),
        dec!(100.2)
    );
    assert_eq!(
        round_price_to_tick(dec!(100.25), dec!(0.1), OrderSide::SELL).unwrap(),
        dec!(100.3)
    );
}

#[test]
fn a_zero_tick_leaves_the_price_and_a_non_positive_price_is_an_error() {
    assert_eq!(
        round_price_to_tick(dec!(123.456), dec!(0), OrderSide::BUY).unwrap(),
        dec!(123.456)
    );
    assert!(round_price_to_tick(dec!(0), dec!(0.1), OrderSide::BUY).is_err());
    assert!(round_price_to_tick(dec!(-1), dec!(0.1), OrderSide::SELL).is_err());
}

#[test]
fn a_quantity_floors_onto_the_lot_and_never_rounds_up() {
    assert_eq!(
        floor_quantity_to_lot(dec!(0.25), dec!(0.1)).unwrap(),
        dec!(0.2)
    );
    assert_eq!(
        floor_quantity_to_lot(dec!(0.006), dec!(0.001)).unwrap(),
        dec!(0.006)
    );
    assert_eq!(
        floor_quantity_to_lot(dec!(1.6), dec!(0.1)).unwrap(),
        dec!(1.6)
    );
    assert_eq!(
        floor_quantity_to_lot(dec!(0.35), dec!(0.1)).unwrap(),
        dec!(0.3),
        "was 0.4 before"
    );
}

#[test]
fn a_quantity_below_one_lot_is_refused_instead_of_becoming_zero() {
    for q in [dec!(0.004), dec!(0.005), dec!(0.0099)] {
        let err = floor_quantity_to_lot(q, dec!(0.01))
            .unwrap_err()
            .to_string();
        assert!(err.contains("zero-quantity"), "{}: {}", q, err);
    }
    assert!(floor_quantity_to_lot(dec!(0), dec!(0.01)).is_err());
    assert_eq!(
        floor_quantity_to_lot(dec!(0.01), dec!(0.01)).unwrap(),
        dec!(0.01),
        "one lot is fine"
    );
}

#[test]
fn a_zero_lot_size_leaves_the_quantity() {
    assert_eq!(
        floor_quantity_to_lot(dec!(0.123), dec!(0)).unwrap(),
        dec!(0.123)
    );
}
