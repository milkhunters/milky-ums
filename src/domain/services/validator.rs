


pub struct ValidatorService {
    firstname_length: usize,
    firstname_regex: regex::Regex,
    lastname_length: usize,
    lastname_regex: regex::Regex,
    username_length: usize,
    username_regex: regex::Regex,
    password_length: usize,
    email_length: usize,
    email_regex: regex::Regex,
}

impl ValidatorService {

    pub fn new() -> Self {
        // First name
        let firstname_length = 64;
        let firstname_regex = regex::Regex::new(r"^[a-zA-Zа-яА-Я]*$").unwrap();

        // Last name
        let lastname_length = 64;
        let lastname_regex = regex::Regex::new(r"^[a-zA-Zа-яА-Я]*$").unwrap();

        // Username
        let username_length = 32;
        let username_regex = regex::Regex::new(r"^[a-zA-Z0-9._]*$").unwrap();

        // Password
        let password_length = 32;

        // Email RFC2822
        let email_length = 255;
        let email_regex = regex::Regex::new(r"^([^\x00-\x20\x22\x28\x29\x2c\x2e\x3a-\x3c\x3e\x40\x5b-\x5d\x7f-\xff]+|\x22([^\x0d\x22\x5c\x80-\xff]|\x5c[\x00-\x7f])*\x22)(\x2e([^\x00-\x20\x22\x28\x29\x2c\x2e\x3a-\x3c\x3e\x40\x5b-\x5d\x7f-\xff]+|\x22([^\x0d\x22\x5c\x80-\xff]|\x5c[\x00-\x7f])*\x22))*\x40([^\x00-\x20\x22\x28\x29\x2c\x2e\x3a-\x3c\x3e\x40\x5b-\x5d\x7f-\xff]+|\x5b([^\x0d\x5b-\x5d\x80-\xff]|\x5c[\x00-\x7f])*\x5d)(\x2e([^\x00-\x20\x22\x28\x29\x2c\x2e\x3a-\x3c\x3e\x40\x5b-\x5d\x7f-\xff]+|\x5b([^\x0d\x5b-\x5d\x80-\xff]|\x5c[\x00-\x7f])*\x5d))*$").unwrap();

        ValidatorService {
            firstname_length,
            firstname_regex,
            lastname_length,
            lastname_regex,
            username_length,
            username_regex,
            password_length,
            email_length,
            email_regex,
        }
    }


    pub fn validate_username(&self, username: &str) -> Result<(), String> {

        if username.len() < 4 || username.len() > self.username_length {
            return Err(
                format!("Имя пользователя должно содержать от 4 до {} символов", self.username_length)
            );
        }

        if !self.username_regex.is_match(username) {
            return Err(
                "Имя пользователя может содержать только буквы, \
                цифры, точки и символы подчеркивания".to_string()
            );
        }
        Ok(())
    }

    pub fn validate_email(&self, email: &str) -> Result<(), String> {

        if email.len() > self.email_length {
            return Err(format!("Email должен содержать максимум {} символов", self.email_length));
        }

        if !self.email_regex.is_match(email) {
            return Err("Неверный формат email".to_string());
        }
        Ok(())
    }

    pub fn validate_password(&self, password: &str) -> Result<(), String> {

        if password.len() < 8 || password.len() > self.password_length {
            return Err(
                format!("Пароль должен содержать от 8 до {} символов", self.password_length)
            );
        }

        if !password.chars().any(char::is_numeric) {
            return Err("Пароль должен содержать хотя бы одну цифру".to_string());
        }

        if !password.chars().any(char::is_alphabetic) {
            return Err("Пароль должен содержать хотя бы одну букву".to_string());
        }

        if password.chars().any(char::is_whitespace) {
            return Err("Пароль не должен содержать пробелов".to_string());
        }

        Ok(())
    }

    pub fn validate_last_name(&self, last_name: &str) -> Result<(), String> {
        if last_name.len() > self.lastname_length {
            return Err(format!("Фамилия должна состоять максимум из {} символов", self.lastname_length));
        }

        if !self.lastname_regex.is_match(last_name) {
            return Err("Фамилия должна состоять из латинских или кириллических букв".to_string());
        }

        Ok(())
    }

    pub fn validate_first_name(&self, first_name: &str) -> Result<(), String> {
        if first_name.len() > self.firstname_length {
            return Err(format!("Имя должно состоять максимум из {} символов", self.firstname_length));
        }

        if !self.firstname_regex.is_match(first_name) {
            return Err("Имя должно состоять из латинских или кириллических букв".to_string());
        }

        Ok(())
    }

}
