# pubkey

small crate that defines a `pubkey!` macro, used for compile-time parsing of public key strings into byte arrays for near 0-cost static public keys defined in your source code. The actual code is taken from [here](https://github.com/project-serum/anchor/commit/96036e149173603926074c6dba445c47bd6575aa).


# usage

```rust
#[cfg(test)]
mod test {
    use pubkey::pubkey;
    #[test]
    fn example() {
        let key = pubkey!("GjphYQcbP1m3FuDyCTUJf2mUMxKPE3j6feWU1rxvC7Ps");
        assert!(key.to_string() == "GjphYQcbP1m3FuDyCTUJf2mUMxKPE3j6feWU1rxvC7Ps");
    }
}
