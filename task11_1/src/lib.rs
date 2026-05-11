pub fn is_strong_password(password: &str) -> bool{
    if password.chars().count()>8 && password.contains("!"){
        true
    }
    else{false}
}

#[cfg(test)]
mod tests {
    use super::*;

#[test]
fn test_is_strong_password(){
let str1="Daia!12222222";
assert!(is_strong_password(&str1) == true);
}



#[test]
fn test_password_too_short() {
        let short_password = "Daia!";
        assert!(is_strong_password(short_password),
            "Пароль '{}' слишком короткий (должен быть длиннее 8 символов)",
            short_password);
    }


    #[test]
    fn test_missing_bang(){
        let password_without_bang = "DariaLongPassword";
        assert!(
            is_strong_password(password_without_bang),
            "Пароль '{}' должен содержать восклицательный знак!",
            password_without_bang
        );
    }
}