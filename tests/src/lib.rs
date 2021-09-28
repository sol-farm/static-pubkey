#[cfg(test)]
mod test {
    use static_pubkey::static_pubkey;
    #[test]
    fn test_pubkey() {
        let key = static_pubkey!("GjphYQcbP1m3FuDyCTUJf2mUMxKPE3j6feWU1rxvC7Ps");
        assert!(key.to_string() == "GjphYQcbP1m3FuDyCTUJf2mUMxKPE3j6feWU1rxvC7Ps");
    }
}