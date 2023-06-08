use once_cell::sync::Lazy;
use regex::Regex;

pub enum Actions {
    Player(String),
    User(u32),
    Top(usize, String),
    Tags(Vec<String>),
    Quit
}

const PLAYER_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"^player\s*(.+)$").unwrap());
const USER_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"^user\s*(\d+)$").unwrap());
const TOP_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"^top(\d+)\s*'([A-Z]+)'$").unwrap());
const TAGS_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"^tags\s*'(.+)'$").unwrap());

pub fn parse(input: &str) -> Option<Actions> {
    let parts = input.trim();

    if let Some(captures) = PLAYER_RE.captures(parts) {
        Some(Actions::Player(captures[1].to_string()))
    } else if let Some(captures) = USER_RE.captures(parts) {
        if let Ok(id) = captures[1].parse() {
            Some(Actions::User(id))
        } else {
            None
        }
    } else if let Some(captures) = TOP_RE.captures(parts) {
        if let Ok(amount) = captures[1].parse() {
            Some(Actions::Top(amount, captures[2].to_string()))
        } else {
            None
        }
    } else if let Some(captures) = TAGS_RE.captures(parts) {
        let tags = Vec::from_iter(captures[1].split("' '").map(|st| st.to_string()));
        Some(Actions::Tags(tags))
    } else if input.eq("q") {
        Some(Actions::Quit)
    }
    else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn player() {
        let input = String::from("player Fer");
        let Actions::Player(name) = parse(&input).unwrap() else {panic!()};

        assert_eq!(&name, "Fer");
    }

    #[test]
    fn user() {
        let input = String::from("user 69");
        let Actions::User(id) = parse(&input).unwrap() else {panic!()};

        assert_eq!(id, 69);
    }

    #[test]
    fn top() {
        let input = String::from("top10 'ST'");
        let Actions::Top(amount, pos) = parse(&input).unwrap() else {panic!()};

        assert_eq!(amount, 10);
        assert_eq!(&pos, "ST");
    }

    #[test]
    fn tags() {
        let real_tags = vec![
            String::from("Brazil"),
            String::from("Sertanejo Universitário"),
        ];
        let input = String::from("tags 'Brazil' 'Sertanejo Universitário'");
        let Actions::Tags(tags) = parse(&input).unwrap() else {panic!()};

        assert_eq!(tags, real_tags);
    }
}
