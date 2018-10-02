#![deny(warnings)]
extern crate hyper;

extern crate reqwest;
use std::io;

#[test]
fn test_nem_heartbeat() {
    let url = "http://bigalice2.nem.ninja:7890/heartbeat";
    let mut res = reqwest::get(url).unwrap();
    res.copy_to(&mut io::stdout()).unwrap();
}

#[test]
fn test_nem_status() {
    let url = "http://bigalice2.nem.ninja:7890/status";
    let mut res = reqwest::get(url).unwrap();
    res.copy_to(&mut io::stdout()).unwrap();
}

#[test]
fn test_nem_account_generate() {
    let url = "http://bigalice2.nem.ninja:7890/account/generate";
    let mut res = reqwest::get(url).unwrap();
    res.copy_to(&mut io::stdout()).unwrap();
}

#[test]
fn test_nem_chain_height() {
    let url = "http://bigalice2.nem.ninja:7890/chain/height";
    let mut res = reqwest::get(url).unwrap();
    res.copy_to(&mut io::stdout()).unwrap();
}

#[test]
fn test_nem_chain_score() {
    let url = "http://bigalice2.nem.ninja:7890/chain/score";
    let mut res = reqwest::get(url).unwrap();
    res.copy_to(&mut io::stdout()).unwrap();
}

#[test]
fn test_nem_chain_lastblock() {
    let url = "http://bigalice2.nem.ninja:7890/chain/last-block";
    let mut res = reqwest::get(url).unwrap();
    res.copy_to(&mut io::stdout()).unwrap();
}

#[test]
fn test_nem_node_info() {
    let url = "http://bigalice2.nem.ninja:7890/node/info";
    let mut res = reqwest::get(url).unwrap();
    res.copy_to(&mut io::stdout()).unwrap();
}

#[test]
fn test_nem_node_extendedinfo() {
    let url = "http://bigalice2.nem.ninja:7890/node/extended-info";
    let mut res = reqwest::get(url).unwrap();
    res.copy_to(&mut io::stdout()).unwrap();
}

#[test]
fn test_nem_node_peerlist_all() {
    let url = "http://bigalice2.nem.ninja:7890/node/peer-list/all";
    let mut res = reqwest::get(url).unwrap();
    res.copy_to(&mut io::stdout()).unwrap();
}

#[test]
fn test_nem_node_peerlist_reachable() {
    let url = "http://bigalice2.nem.ninja:7890/node/peer-list/reachable";
    let mut res = reqwest::get(url).unwrap();
    res.copy_to(&mut io::stdout()).unwrap();
}

#[test]
fn test_nem_node_peerlist_active() {
    let url = "http://bigalice2.nem.ninja:7890/node/peer-list/active";
    let mut res = reqwest::get(url).unwrap();
    res.copy_to(&mut io::stdout()).unwrap();
}

#[test]
fn test_nem_node_activepeers_maxchainheight() {
    let url = "http://bigalice2.nem.ninja:7890/node/active-peers/max-chain-height";
    let mut res = reqwest::get(url).unwrap();
    res.copy_to(&mut io::stdout()).unwrap();
}

#[test]
fn test_nem_node_experiences() {
    let url = "http://bigalice2.nem.ninja:7890/node/experiences";
    let mut res = reqwest::get(url).unwrap();
    res.copy_to(&mut io::stdout()).unwrap();
}

#[test]
fn test_nem_debug_timesynchronization() {
    let url = "http://bigalice2.nem.ninja:7890/debug/time-synchronization";
    let mut res = reqwest::get(url).unwrap();
    res.copy_to(&mut io::stdout()).unwrap();
}

#[test]
fn test_nem_debug_connections_incoming() {
    let url = "http://bigalice2.nem.ninja:7890/debug/connections/incoming";
    let mut res = reqwest::get(url).unwrap();
    res.copy_to(&mut io::stdout()).unwrap();
}

#[test]
fn test_nem_debug_connections_outgoing() {
    let url = "http://bigalice2.nem.ninja:7890/debug/connections/outgoing";
    let mut res = reqwest::get(url).unwrap();
    res.copy_to(&mut io::stdout()).unwrap();
}

#[test]
fn test_nem_debug_timers() {
    let url = "http://bigalice2.nem.ninja:7890/debug/timers";
    let mut res = reqwest::get(url).unwrap();
    res.copy_to(&mut io::stdout()).unwrap();
}
