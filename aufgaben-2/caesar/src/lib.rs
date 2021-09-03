// === Caesar-Verschlüsselung


fn caesar(text: &str, key: u8, encode: bool) -> String {
    todo!()
}


// === Bonus: Vigenere-Verschlüsselung


fn vigenere(text: &str, key: &str, encode: bool) -> String {
    todo!()
}


// === Tests


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_caesar() {
        let plain = "STUDENTISCHE INFORMATIKTAGE!";
        let cipher3 = "VWXGHQWLVFKH LQIRUPDWLNWDJH!";
        let cipher11 = "DEFOPYETDNSP TYQZCXLETVELRP!";

        assert_eq!(caesar(plain, 3, true), cipher3);
        assert_eq!(caesar(plain, 11, true), cipher11);
        assert_eq!(caesar(plain, 26, true), plain);
        assert_eq!(caesar(plain, 28, true), caesar(plain, 2, true));

        assert_eq!(caesar(cipher3, 3, false), plain);
        assert_eq!(caesar(cipher11, 11, false), plain);
    }

    #[test]
    fn test_vigenere() {
        let plain = "STUDENTISCHE INFORMATIKTAGE!";
        let cipher_sit = "KBNVMGLQLUPX AVYGZFSBBCBTYM!";

        assert_eq!(vigenere(plain, "SIT", true), cipher_sit);
        assert_eq!(vigenere(cipher_sit, "SIT", false), plain);
    }
}