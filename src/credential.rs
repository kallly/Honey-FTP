#[allow(dead_code)]
pub struct Credential{
    username: String,
    password: String,
}

impl Credential{
    #[allow(dead_code)]
    pub fn new(username:String, password:String) -> Credential {
        Credential { username: username, password: password }
    }
    #[allow(dead_code)]
    pub fn compare(&self, username:&String, password:&String) -> bool {
        self.username.eq(username) && self.password.eq(password)
    }
}