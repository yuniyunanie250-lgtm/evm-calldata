//! Decode EVM calldata into a function selector and its 32-byte argument words.
//!
//! Useful when you have a raw transaction input (a block explorer copy-paste, a
//! mempool dump, a failing test vector) and want to know which function was
//! called and where the argument boundaries fall.
//!
//! ```no_run
//! let d = evm_calldata::decode("0xa9059cbb00000000000000000000000000..").unwrap();
//! assert_eq!(d.signature, Some("transfer(address,uint256)"));
//! ```
pub mod selectors;

pub use selectors::{lookup, strip_0x};

/// A decoded calldata payload.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Decoded {
    /// The 4-byte selector, lowercase, with `0x` prefix.
    pub selector: String,
    /// Known signature for the selector, when the table has one.
    pub signature: Option<&'static str>,
    /// Argument words after the selector.
    pub words: Vec<[u8; 32]>,
    /// Bytes left over that did not fill a whole 32-byte word.
    pub trailing: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DecodeError {
    Empty,
    BadHex(usize),
    TooShort(usize),
}

impl std::fmt::Display for DecodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DecodeError::Empty => write!(f, "empty input"),
            DecodeError::BadHex(i) => write!(f, "non-hex character at byte {}", i),
            DecodeError::TooShort(n) => {
                write!(f, "need at least 4 bytes for a selector, got {}", n)
            }
        }
    }
}

impl std::error::Error for DecodeError {}

fn unhex(c: u8) -> Option<u8> {
    match c {
        b'0'..=b'9' => Some(c - b'0'),
        b'a'..=b'f' => Some(c - b'a' + 10),
        b'A'..=b'F' => Some(c - b'A' + 10),
        _ => None,
    }
}

/// Parse a hex string (with or without `0x`) into bytes.
pub fn hex_to_bytes(input: &str) -> Result<Vec<u8>, DecodeError> {
    let s = strip_0x(input).trim();
    if s.is_empty() {
        return Err(DecodeError::Empty);
    }
    if s.len() % 2 != 0 {
        return Err(DecodeError::BadHex(s.len()));
    }
    let raw = s.as_bytes();
    let mut out = Vec::with_capacity(s.len() / 2);
    let mut i = 0;
    while i < raw.len() {
        let hi = unhex(raw[i]).ok_or(DecodeError::BadHex(i))?;
        let lo = unhex(raw[i + 1]).ok_or(DecodeError::BadHex(i + 1))?;
        out.push((hi << 4) | lo);
        i += 2;
    }
    Ok(out)
}

/// Decode calldata: selector, known signature, 32-byte words, trailing bytes.
pub fn decode(input: &str) -> Result<Decoded, DecodeError> {
    let bytes = hex_to_bytes(input)?;
    if bytes.len() < 4 {
        return Err(DecodeError::TooShort(bytes.len()));
    }
    let selector = format!(
        "0x{}",
        bytes[..4]
            .iter()
            .map(|b| format!("{:02x}", b))
            .collect::<String>()
    );
    let rest = &bytes[4..];
    let mut words = Vec::new();
    let mut chunks = rest.chunks_exact(32);
    for c in &mut chunks {
        let mut w = [0u8; 32];
        w.copy_from_slice(c);
        words.push(w);
    }
    let trailing = chunks.remainder().to_vec();
    Ok(Decoded {
        signature: lookup(&selector),
        selector,
        words,
        trailing,
    })
}

/// Render a 32-byte word as an unsigned integer, when it fits in a u128.
pub fn word_as_u128(w: &[u8; 32]) -> Option<u128> {
    if w[..16].iter().any(|b| *b != 0) {
        return None;
    }
    let mut v = 0u128;
    for b in &w[16..] {
        v = (v << 8) | *b as u128;
    }
    Some(v)
}
