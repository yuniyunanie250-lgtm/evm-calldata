/// Well-known 4-byte selectors, sorted by selector for binary search.
/// Every entry is verified in `tests/selectors.rs` against the first four
/// bytes of keccak-256(signature).
pub const KNOWN: &[(&str, &str)] = &[
    ("0x01ffc9a7", "supportsInterface(bytes4)"),
    ("0x06fdde03", "name()"),
    ("0x095ea7b3", "approve(address,uint256)"),
    (
        "0x150b7a02",
        "onERC721Received(address,address,uint256,bytes)",
    ),
    ("0x18160ddd", "totalSupply()"),
    ("0x23b872dd", "transferFrom(address,address,uint256)"),
    ("0x2e1a7d4d", "withdraw(uint256)"),
    ("0x313ce567", "decimals()"),
    ("0x3ccfd60b", "withdraw()"),
    ("0x42842e0e", "safeTransferFrom(address,address,uint256)"),
    ("0x6352211e", "ownerOf(uint256)"),
    ("0x70a08231", "balanceOf(address)"),
    ("0x8da5cb5b", "owner()"),
    ("0x95d89b41", "symbol()"),
    ("0xa22cb465", "setApprovalForAll(address,bool)"),
    ("0xa9059cbb", "transfer(address,uint256)"),
    ("0xd0e30db0", "deposit()"),
    ("0xdd62ed3e", "allowance(address,address)"),
    (
        "0xf23a6e61",
        "onERC1155Received(address,address,uint256,uint256,bytes)",
    ),
];

/// Look up a selector. Accepts with or without the `0x` prefix.
pub fn lookup(selector: &str) -> Option<&'static str> {
    let s = strip_0x(selector).to_ascii_lowercase();
    let needle = format!("0x{}", s);
    KNOWN
        .binary_search_by(|(k, _)| k.cmp(&needle.as_str()))
        .ok()
        .map(|i| KNOWN[i].1)
}

/// Strip a leading `0x`/`0X`, if present.
pub fn strip_0x(s: &str) -> &str {
    s.strip_prefix("0x")
        .or_else(|| s.strip_prefix("0X"))
        .unwrap_or(s)
}
