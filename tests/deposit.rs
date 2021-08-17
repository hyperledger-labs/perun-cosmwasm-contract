mod common;
use common::setup::*;
use cosmwasm_std::coin;
use perun_cosmwasm::types::NativeBalance;
use std::ops::Add;

#[test]
fn init() {
    do_init();
}

#[test]
fn deposit_some() {
    let (s, mut deps) = do_init();
    do_deposit(deps.as_mut(), &s.fids[0], &s.bals, ALICE.into()).unwrap();

    let deposited = query_deposit(deps.as_mut(), s.fids[0].clone());
    assert_eq!(deposited, s.bals);
}

#[test]
fn deposit_twice() {
    let (s, mut deps) = do_init();
    let fid = &s.fids[0];

    do_deposit(deps.as_mut(), &s.fids[0], &s.bals, ALICE.into()).unwrap();
    do_deposit(deps.as_mut(), &s.fids[0], &s.bals, BOB.into()).unwrap();

    let deposited = query_deposit(deps.as_mut(), fid.clone());
    let double_bals = s.bals.clone().add(&s.bals);
    assert_eq!(deposited, double_bals);
}

#[test]
#[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
fn deposit_overflow() {
    let (s, mut deps) = do_init();

    let bals: NativeBalance = vec![coin(std::u128::MAX - 10, DENOMS[0])].into();
    // Normal
    do_deposit(deps.as_mut(), &s.fids[0], &bals, ALICE.into()).unwrap();
    // Overflow
    do_deposit(deps.as_mut(), &s.fids[0], &bals, ALICE.into()).expect_err("Should overflow");
}
