#[cfg(test)]
mod test {
    use pubkey::pubkey;
    #[test]
    fn test_pubkey() {
        let key = pubkey!("GjphYQcbP1m3FuDyCTUJf2mUMxKPE3j6feWU1rxvC7Ps");
        assert!(key.to_string() == "GjphYQcbP1m3FuDyCTUJf2mUMxKPE3j6feWU1rxvC7Ps");
    }
}