pub fn greet_user(name: &str) -> String {
    format!("Hello {name}")
}

pub fn read_line() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Stdin not working");
    input.trim().to_string()
}

#[derive(PartialEq, Debug, Clone)]
pub enum LoginAction {
    Granted(LoginRole),
    Denied,
}

#[derive(PartialEq, Debug, Clone)]
pub enum LoginRole {
    Admin,
    User, 
}

pub struct User {
    pub username: String,
    pub password: String,
    pub role: LoginRole,
}

impl User {
    pub fn new(username: &str, password: &str, role: LoginRole) ->  User {
        Self {
             username: username.to_lowercase(), 
             password: password.to_string(), 
             role 
        }
    }
}

pub fn get_users() -> Vec<User> {
    vec![
        User::new( "admin", "password", LoginRole::Admin),
        User::new("bob", "password", LoginRole::User)
    ]

    // let mut users = Vec::new();
    // users.push(User::new("admin", "password", LoginRole::Admin))
}

#[warn(dead_code)]
fn get_admin_users() {
    let users: Vec<String> = get_users()
        .into_iter()
        .filter(|u | u.role == LoginRole::Admin)
        .map(|u| u.username)
        .collect();
}

pub fn login(username: &str, password: &str) -> Option<LoginAction> {

    let users = get_users();
    if let Some(user) = users.iter().find(|user: &&User| user.username == username) {
        if user.password == password {
            return Some(LoginAction::Granted(user.role.clone()))
        } else {
            return Some(LoginAction::Denied);
        }
    }
    None
}

#[cfg(test)] 
mod tests {
    use super::*;

    #[test]
    fn test_greet_user() {
        assert_eq!("Hello Robert", greet_user("Robert"));
    }

    #[test]
    fn test_login() {

        assert_eq!(login("admin",  "password"), Some(LoginAction::Granted(LoginRole::Admin)));
        assert_eq!(login("bob",  "password"), Some(LoginAction::Granted(LoginRole::User)));
        // assert_eq!(login("denied",  "password"), Some(LoginAction::Denied));

    }
    
}
