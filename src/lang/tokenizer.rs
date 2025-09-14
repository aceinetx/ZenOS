pub struct Tokenizer<'a> {
    code: &'a str,
}

impl Tokenizer<'_> {
    pub fn new(code: &str) -> Tokenizer<'_> {
        return Tokenizer { code: code };
    }
}
