use super::crypto::random_account;
use cosmwasm_std::{
    coin, from_binary,
    testing::{mock_dependencies, mock_env, mock_info, MockApi, MockQuerier, MockStorage},
    Coin, DepsMut, Env, OwnedDeps, Response,
};
use perun_cosmwasm::{
    contract::{execute, instantiate, query},
    crypto::Sig,
    error::ContractError,
    msg::*,
    types::*,
};

type Deps = OwnedDeps<MockStorage, MockApi, MockQuerier>;

use k256::ecdsa::SigningKey;
use rand_core::{OsRng, RngCore};

pub const DENOMS: [&str; 2] = ["COSM", "ATOM"];
pub const ALICE: &str = "alice";
pub const BOB: &str = "bob";

pub struct Setup {
    pub keys: Vec<SigningKey>,
    pub params: Params,
    pub cid: ChannelId,
    pub fids: Vec<FundingId>,
    pub final_state: State,
    pub nfinal_state: State,
    pub bals: NativeBalance,
    pub alloc: Vec<NativeBalance>,
}

pub fn random_nonce() -> Nonce {
    let mut ret: Nonce = [0; NONCE_LEN];
    OsRng.fill_bytes(&mut ret);
    ret
}

pub fn new_setup() -> Setup {
    let mut rng = rand::thread_rng();
    let (alice_off, bob_off) = (random_account(&mut rng), random_account(&mut rng));
    let params = Params {
        nonce: random_nonce(),
        participants: vec![alice_off.1.clone(), bob_off.1.clone()],
        dispute_duration: 60,
    };
    let cid = params.channel_id().unwrap();
    let bals = vec![coin(2, DENOMS[0]), coin(20, DENOMS[1])];
    let alloc = vec![
        NativeBalance::from(bals.clone()),
        NativeBalance::from(bals.clone()),
    ];
    Setup {
        keys: vec![alice_off.0, bob_off.0],
        params,
        cid: cid.clone(),
        fids: vec![
            calc_funding_id(&cid, &alice_off.1).unwrap(),
            calc_funding_id(&cid, &bob_off.1).unwrap(),
        ],
        final_state: State {
            channel_id: cid.clone(),
            version: 123,
            balances: alloc.clone(),
            finalized: true,
        },
        nfinal_state: State {
            channel_id: cid.clone(),
            version: 123,
            balances: alloc.clone(),
            finalized: false,
        },
        alloc,
        bals: bals.into(),
    }
}

pub fn do_init() -> (Setup, Deps) {
    let mut deps = mock_dependencies(&[]);

    // Instantiate
    let msg = InitMsg {};
    let info = mock_info("creator_key", &[]);
    instantiate(deps.as_mut(), mock_env(), info, msg).expect("Init failed");
    (new_setup(), deps)
}

pub fn do_deposit(
    deps: DepsMut,
    fid: &FundingId,
    bals: &NativeBalance,
    who: String,
) -> Result<Response, ContractError> {
    let msg = ExecuteMsg::Deposit(fid.clone());
    let info = mock_info(who.as_ref(), Vec::<Coin>::from(bals.clone()).as_slice());
    execute(deps, mock_env(), info, msg)
}

pub fn do_conclude(
    deps: DepsMut,
    params: &Params,
    state: &State,
    sigs: &[Sig],
) -> Result<Response, ContractError> {
    let msg = ExecuteMsg::Conclude(params.clone(), state.clone(), sigs.into());
    let info = mock_info(ALICE, &[]);
    execute(deps, mock_env(), info, msg)
}

pub fn do_dispute(
    deps: DepsMut,
    params: &Params,
    state: &State,
    sigs: &Vec<Sig>,
) -> Result<Response, ContractError> {
    let msg = ExecuteMsg::Dispute(params.clone(), state.clone(), sigs.clone());
    let info = mock_info(ALICE, &[]);
    execute(deps, mock_env(), info, msg)
}

pub fn do_conclude_dispute(deps: DepsMut, params: &Params) -> Result<Response, ContractError> {
    let msg = ExecuteMsg::ConcludeDispute(params.clone());
    let info = mock_info(ALICE, &[]);
    execute(deps, mock_env(), info, msg)
}

pub fn do_withdraw(
    deps: DepsMut,
    withdrawal: &Withdrawal,
    sig: &Sig,
) -> Result<Response, ContractError> {
    let msg = ExecuteMsg::Withdraw(withdrawal.clone(), sig.clone());
    let info = mock_info(ALICE, &[]);
    execute(deps, mock_env(), info, msg)
}

pub fn query_deposit(deps: DepsMut, fid: FundingId) -> NativeBalance {
    let _deposited = query(deps.as_ref(), mock_env(), QueryMsg::Deposit(fid)).unwrap();
    from_binary::<NativeBalance>(&_deposited).unwrap()
}

pub fn advance_time(mut env: Env, by: Seconds) -> Env {
    env.block.time = env.block.time.plus_seconds(by);
    env
}
