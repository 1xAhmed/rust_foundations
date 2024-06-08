use authentication::{get_users, save_users, LoginRole, User};
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command()]
struct  Args {
    #[command(subcommand)]
    command: Option<Commands>,

}

#[derive(Subcommand)]
enum Commands {
    /// List all users.
    List,
    ///Add a user.
    Add {
        /// The user's login name
        username: String,
        /// The user's password (plaintext)
        password: String,
        ///Optional - mark as admin
        admin: Option<bool>
    },
     /// Delete a user
     Delete {
        /// User to delete
        username: String,
     }
}

fn list_users() {
    println!("{:<20}{:<20?}", "Username", "Role");
    println!("{:-<40}", "");
    let  users = get_users();
    users
        .iter()
        .for_each(|(_, user) | {
            println!("{:<20}{:<20?}", user.username, user.role);
        });
}

fn add_user(username: String, password: String, admin: bool ) {
    let mut users = get_users();
    let role = if admin {
        LoginRole::Admin
    } else {
        LoginRole::User
    };

    let user = User::new(&username, &password, role);
    users.insert(username, user);
    save_users(users);
}

fn delete_user(username: String) {
    let mut users = get_users();
    if users.contains_key(&username) {
        users.remove(&username);
        save_users(users);
    } else {
        println!("{username} does not exist");
    }
}

fn main() {
    let cli = Args::parse();
    match cli.command {
        Some(Commands::List) => {
            list_users()
        }
        Some(Commands::Add { username, password, admin }) => {
            add_user(username, password, admin.unwrap_or(false));
        }
        Some(Commands::Delete {username}) => {
            delete_user(username)
        }
        None => {
            println!("Run with --help to see instructions");
        }
    }
}
