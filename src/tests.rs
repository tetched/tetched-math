#![allow(unused_imports)]
use crate::MathError::{ZeroInReserve, Overflow, InsufficientOutReserve};

#[test]
fn spot_price_should_work() {
    let cases = vec![
        (1000, 2000, 500, Ok(1000), "Easy case"),
        (1, 1, 1, Ok(1), "Easy case"),
        (0, 1, 1, Err(ZeroInReserve), "Zero sell_reserve"),
        (1, 0, 1, Ok(0), "Zero buy_reserve"),
        (1, 1, 0, Ok(0), "Zero amount"),
        (u128::MAX, u128::MAX-1, 1, Ok(0), "Truncated result"),
        (1, u128::MAX, u128::MAX, Err(Overflow), "Overflow weights"),
    ];

    for case in cases {
        assert_eq!(
            crate::amm::calculate_spot_price(case.0, case.1, case.2),
            case.3,
            "{}",
            case.4
        );
    }
}

#[test]
fn out_given_in_should_work() {
    let cases = vec![
        (1000, 2000, 500, Ok(667), "Easy case"),
        (0, u128::MAX, u128::MAX, Err(Overflow), "Zero sell reserve"),
        (0, 0, 0, Err(ZeroInReserve), "Zero reserves and weights"),
        (0, 1, 0, Err(ZeroInReserve), "Zero sell reserve and amount"),
        (1, 0, 0, Ok(1), "Zero buy reserve and amount"),
        (0, 0, u128::MAX, Ok(1), "Zero buy reserve and sell reserve"),
    ];

    for case in cases {
        assert_eq!(
            crate::amm::calculate_out_given_in(case.0, case.1, case.2),
            case.3,
            "{}",
            case.4
        );
    }
}

#[test]
fn in_given_out_should_work() {
    let cases = vec![
        (2000, 1000, 500, Ok(334), "Easy case"),
        (0, 0, 0, Err(ZeroInReserve), "Zero reserves and weights"),
        (0, 10, 1000, Err(InsufficientOutReserve), "amount cannot be > buy reserve"),
        (0, u128::MAX, u128::MAX, Err(InsufficientOutReserve), "div by zero"),
        (u128::MAX, u128::MAX, u128::MAX-1, Err(Overflow), "Overflow weights"),
    ];

    for case in cases {
        assert_eq!(
            crate::amm::calculate_in_given_out(case.0, case.1, case.2),
            case.3,
            "{}",
            case.4
        );
    }
}

#[test]
fn add_liquidity_should_work() {
    let cases = vec![
        (1000, 2000, 500, Ok(1000), "Easy case"),
        (100, 100, 0, Ok(0), "amount is zero"),
        (110, 0, 100, Ok(0), "asset b is zero"),
        (0, 110, 100, Err(ZeroInReserve), "asset a is zero"),
        (1, u128::MAX, u128::MAX, Err(Overflow), "asset b and amount are zero"),
    ];

    for case in cases {
        assert_eq!(
            crate::amm::calculate_liquidity_in(case.0, case.1, case.2),
            case.3,
            "{}",
            case.4
        );
    }
}

#[test]
fn remove_liquidity_should_work() {
    let cases = vec![
        (1000, 2000, 500, 2500, Ok((200, 400)), "Easy case"),
        (100, 100, 100, 0, Err(ZeroInReserve), "total liquidity is zero"),
        (0, 0, 0, 100, Ok((0,0)), "amount is zero"),
        (0, 110, 100, 100, Ok((0,110)), "remove amount a is zero"),
        (110, 0, 100, 100, Ok((110,0)), "remove amount b is zero"),
        (u128::MAX, 0, u128::MAX, 1, Err(Overflow), "Formula a overflow"),
        (0, u128::MAX, u128::MAX, 1, Err(Overflow), "Formula b overflow"),
    ];

    for case in cases {
        assert_eq!(
            crate::amm::calculate_liquidity_out(case.0, case.1, case.2, case.3),
            case.4,
            "{}",
            case.5
        );
    }
}
