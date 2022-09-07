mod client;

use std::env;

const TOKEN_NAME: &str = "AOC_TOKEN";

fn get_token() -> String {
    env::var(TOKEN_NAME).unwrap();
}

#[cfg(test)]
mod tests {
    #[test]
    fn get_token_test() {
        let value = "abc";
        env::set_var(TOKEN_NAME, value);

        assert_eq!(value, get_token());
    }
}
