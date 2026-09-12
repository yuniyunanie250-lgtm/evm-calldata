use evm_calldata::selectors::KNOWN;

/// keccak256(signature)[..4] for every signature in KNOWN, computed offline.
const EXPECTED: &[(&str, &str)] = &[
    ("supportsInterface(bytes4)", "0x01ffc9a7"),
    ("name()", "0x06fdde03"),
    ("approve(address,uint256)", "0x095ea7b3"),
    (
        "onERC721Received(address,address,uint256,bytes)",
        "0x150b7a02",
    ),
    ("totalSupply()", "0x18160ddd"),
    ("transferFrom(address,address,uint256)", "0x23b872dd"),
    ("withdraw(uint256)", "0x2e1a7d4d"),
    ("decimals()", "0x313ce567"),
    ("withdraw()", "0x3ccfd60b"),
    ("safeTransferFrom(address,address,uint256)", "0x42842e0e"),
    ("ownerOf(uint256)", "0x6352211e"),
    ("balanceOf(address)", "0x70a08231"),
    ("owner()", "0x8da5cb5b"),
    ("symbol()", "0x95d89b41"),
    ("setApprovalForAll(address,bool)", "0xa22cb465"),
    ("transfer(address,uint256)", "0xa9059cbb"),
    ("deposit()", "0xd0e30db0"),
    ("allowance(address,address)", "0xdd62ed3e"),
    (
        "onERC1155Received(address,address,uint256,uint256,bytes)",
        "0xf23a6e61",
    ),
];

#[test]
fn table_matches_expected_selectors() {
    assert_eq!(KNOWN.len(), EXPECTED.len(), "table and vector list drifted");
    // KNOWN is (selector, signature); EXPECTED is (signature, selector)
    for ((sel, sig), (esig, esel)) in KNOWN.iter().zip(EXPECTED.iter()) {
        assert_eq!(sig, esig, "signature mismatch for {}", sel);
        assert_eq!(sel, esel, "selector mismatch for {}", sig);
    }
}

#[test]
fn table_is_sorted_for_binary_search() {
    assert!(KNOWN.windows(2).all(|w| w[0].0 < w[1].0));
}
