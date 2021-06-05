const SPANISH: &str = "Spanish";
const FRENCH: &str = "French";
const ENGLISH_HELLO_PREFIX: &str = "Hello,";
const SPANISH_HELLO_PREFIX: &str = "Hola,";
const FRENCH_HELLO_PREFIX: &str = "Bonjour,";

fn hello(name: &str, lang: &str) -> String {
    let name = if name == "" {
        "World"
    } else { name };
    return get_prefix(lang).to_owned() + name;
}

fn get_prefix(lang: &str) -> &str {
    match lang {
        SPANISH => SPANISH_HELLO_PREFIX,
        FRENCH => FRENCH_HELLO_PREFIX,
        _ => ENGLISH_HELLO_PREFIX
    }
}


#[cfg(test)]
mod tests {
    use crate::hello::hello;

    #[test]
    fn test_hello_out_put() {
        use crate::hello::SPANISH;
        use crate::hello::FRENCH;
        assert_eq!(hello("Chris", ""), "Hello,Chris");
        assert_eq!(hello("", ""), "Hello,World");
        assert_eq!(hello("Elodie", SPANISH), "Hola,Elodie");
        assert_eq!(hello("Lauren", FRENCH), "Bonjour,Lauren");
    }
}
