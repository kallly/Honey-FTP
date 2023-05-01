#[derive(Clone)]
pub struct Credential{
    username: String,
    password: String,
}

impl Credential{
    pub fn new(username:String, password:String) -> Credential {
        Credential { username: username, password: password }
    }
    pub fn compare(&self, username:&String, password:&String) -> bool {
        self.username.eq(username) && self.password.eq(password)
    }
}