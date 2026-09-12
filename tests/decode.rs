use evm_calldata::{decode, lookup, word_as_u128, DecodeError};

/// ERC-20 `transfer(address,uint256)` to vitalik.eth for 1e18 wei.
/// Built with concat! so no line-continuation or indentation can leak into
/// the hex string.
const TRANSFER: &str = concat!(
    "0xa9059cbb",
    "000000000000000000000000d8da6bf26964af9d7eed9e03e53415d37aa96045",
    "0000000000000000000000000000000000000000000000000de0b6b3a7640000"
);

#[test]
fn decodes_erc20_transfer() {
    let d = decode(TRANSFER).unwrap();
    assert_eq!(d.selector, "0xa9059cbb");
    assert_eq!(d.signature, Some("transfer(address,uint256)"));
    assert_eq!(d.words.len(), 2);
    assert!(d.trailing.is_empty());
    // the recipient is the low 20 bytes of word 0 (ABI left-pads addresses)
    let recipient: String = d.words[0][12..]
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect();
    assert_eq!(recipient, "d8da6bf26964af9d7eed9e03e53415d37aa96045");
    assert_eq!(word_as_u128(&d.words[1]), Some(1_000_000_000_000_000_000));
}

#[test]
fn rejects_short_and_bad_input() {
    assert_eq!(decode("0x"), Err(DecodeError::Empty));
    assert_eq!(decode("0xa905"), Err(DecodeError::TooShort(2)));
    assert!(matches!(decode("0xzzzzzzzz"), Err(DecodeError::BadHex(_))));
    // odd-length hex is rejected rather than silently truncated
    assert!(matches!(decode("0xa9059cb"), Err(DecodeError::BadHex(_))));
}

#[test]
fn reports_trailing_bytes() {
    // selector + one full word + 3 stray bytes
    let input = format!("0x70a08231{}{}", "00".repeat(32), "abcdef");
    let d = decode(&input).unwrap();
    assert_eq!(d.words.len(), 1);
    assert_eq!(d.trailing, vec![0xab, 0xcd, 0xef]);
}

#[test]
fn unknown_selector_has_no_signature() {
    let d = decode("0xdeadbeef").unwrap();
    assert_eq!(d.signature, None);
    assert!(d.words.is_empty());
}

#[test]
fn lookup_is_prefix_agnostic() {
    assert_eq!(lookup("a9059cbb"), lookup("0xA9059CBB"));
    assert_eq!(lookup("0xa9059cbb"), Some("transfer(address,uint256)"));
}

#[test]
fn word_as_u128_refuses_overflow() {
    let mut w = [0u8; 32];
    w[15] = 1; // set a byte in the high 16
    assert_eq!(word_as_u128(&w), None);
    let mut ok = [0u8; 32];
    ok[31] = 255;
    assert_eq!(word_as_u128(&ok), Some(255));
}
