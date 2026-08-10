# evm-calldata

Decode raw EVM calldata into a function selector, the matching function
signature, and its 32-byte argument words.

When you are staring at a raw transaction input from a block explorer, a mempool
dump, or a failing test vector, this tells you which function was called and
where the argument boundaries fall.

## Install

```bash
cargo install --git https://github.com/yuniyunanie250-lgtm/evm-calldata
```

## Usage

```bash
$ evm-calldata 0xa9059cbb000000000000000000000000d8da6bf26964af9d7eed9e03e53415d37aa960450000000000000000000000000000000000000000000000000de0b6b3a7640000
selector   0xa9059cbb
signature  transfer(address,uint256)
arg words  2
  [0] 0x000000000000000000000000d8da6bf26964af9d7eed9e03e53415d37aa96045  (> u128)
  [1] 0x0000000000000000000000000000000000000000000000000de0b6b3a7640000  (1000000000000000000)
```

Reads from stdin when no argument is given:

```bash
cast tx <hash> --raw | evm-calldata
```

## What it does and does not do

- Splits calldata into the 4-byte selector and 32-byte words. No ABI needed.
- Resolves the signature for 19 well-known ERC-20/721/1155 selectors.
- Does **not** decode dynamic types (strings, bytes, arrays) -- those need the
  full ABI, because their layout depends on the parameter list. Use `cast` or
  `abitype` for that.
- Does **not** verify a selector against a signature; it only looks it up. The
  table is asserted in `tests/selectors.rs` against the first four bytes of
  keccak-256 of each signature.

## Development

```bash
cargo test
```

## License

MIT
