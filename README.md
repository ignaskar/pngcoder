# pngcoder
Encode/decode data in PNG files as chunks.

## Build from source
Clone this repository and run `cargo build --release`\
Requires Rust 1.64.0 or above.

## Usage
`pngcoder encode ./dice.png ruSt "This is a secret message!`

`pngcoder decode ./dice.png ruSt`

`pngcoder remove ./dice.png ruSt`

`pngcoder print ./dice.png`
