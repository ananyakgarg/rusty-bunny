#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use rocket::response::Redirect;
mod utils;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/search?<cmd>")]
fn search(cmd: String) -> Redirect {
    println!("You typed in: {}", cmd);
    let command = utils::get_command_from_query_string(&cmd);

    let redirect_url = match command {
        "tw" => String::from("https://twitter.com"),
        "fb" => String::from("https://facebook.com"),
        "gh" => String::from("https://github.com"),
        "li" => utils::linkedin::construct_linkedin_url(&cmd),
        "chat" => String::from("https://chat.openai.com"),
        "disc" => String::from("https://discord.com/invite/5tw9uWwN"),
        "canv" => String::from("https://canvas.unc.edu"),
        "gm" => String::from("https://mail.google.com"),
        "cal" => String::from("https://calendar.google.com"),
        _ => utils::google::construct_google_search_url(&cmd)
    };
    
    Redirect::to(redirect_url)
}

fn main() {
    rocket::ignite()
    .mount("/", routes![index, search])
    .launch();
}

fn get_command_from_query_string(query_string: &str) -> &str {
    if query_string.contains(' ') {
        let index_of_space = query_string.find(' ').unwrap_or(0);
        return &query_string[..index_of_space];
    }
    &query_string
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_command_from_query_string_no_whitespace() {
        // Test with command only
        let actual = get_command_from_query_string("tw");
        let expected = "tw";
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_get_command_from_query_string_with_whitespace() {
        let actual = get_command_from_query_string("tw @fbOpenSource");
        let expected = "tw";
        assert_eq!(actual, expected);
    }
}
