pub struct Credential{
    username: String,
    password: String,
}

impl Credential{
    pub fn new(username:String, password:String) -> Credential {
        Credential { username: username, password: password }
    }
}