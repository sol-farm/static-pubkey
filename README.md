# static-pubkey

![docs.rs](https://img.shields.io/docsrs/static-pubkey)


The `static-pubkey` crate provides a macro `static_pubkey!`, used for compile-time parsing of strings into a static public key. This prvodies an efficient way of declaring public keys in source code while incurring almost no runtime cost in solana programs, without having to declare the byte array yourself.  The actual code is taken from [here](https://github.com/project-serum/anchor/commit/96036e149173603926074c6dba445c47bd6575aa).


# usage

```rust
#[cfg(test)]
mod test {
    use static_pubkey::static_pubkey;
    #[test]
    fn example() {
        let key = static_pubkey!("GjphYQcbP1m3FuDyCTUJf2mUMxKPE3j6feWU1rxvC7Ps");
        assert!(key.to_string() == "GjphYQcbP1m3FuDyCTUJf2mUMxKPE3j6feWU1rxvC7Ps");
    }
}
```

# links

* [crates.io](https://crates.io/crates/static-pubkey)
* [docs.rs](https://docs.rs/crate/static-pubkey)
