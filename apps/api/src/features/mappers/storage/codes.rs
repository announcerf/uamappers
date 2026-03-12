pub fn kind_code(value: &str) -> i16 {
    match value {
        "graveyard" => 1,
        "guest" => 2,
        "loved" => 3,
        "nominated" => 4,
        "pending" => 5,
        "ranked" => 6,
        _ => 0,
    }
}

pub fn kind_str(code: i16) -> &'static str {
    match code {
        1 => "graveyard",
        2 => "guest",
        3 => "loved",
        4 => "nominated",
        5 => "pending",
        6 => "ranked",
        _ => "unknown",
    }
}

pub fn mode_code(value: &str) -> i16 {
    match value {
        "osu" => 1,
        "taiko" => 2,
        "catch" => 3,
        "mania" => 4,
        _ => 0,
    }
}

pub fn mode_str(code: i16) -> &'static str {
    match code {
        1 => "osu",
        2 => "taiko",
        3 => "catch",
        4 => "mania",
        _ => "unknown",
    }
}

pub fn status_code(value: &str) -> i16 {
    match value {
        "graveyard" => 1,
        "wip" => 2,
        "pending" => 3,
        "ranked" => 4,
        "approved" => 5,
        "qualified" => 6,
        "loved" => 7,
        _ => 0,
    }
}

pub fn status_str(code: i16) -> &'static str {
    match code {
        1 => "graveyard",
        2 => "wip",
        3 => "pending",
        4 => "ranked",
        5 => "approved",
        6 => "qualified",
        7 => "loved",
        _ => "unknown",
    }
}

pub fn genre_code(value: &str) -> i16 {
    match value {
        "any" => 1,
        "unspecified" => 2,
        "video_game" => 3,
        "anime" => 4,
        "rock" => 5,
        "pop" => 6,
        "other" => 7,
        "novelty" => 8,
        "hip_hop" => 9,
        "electronic" => 10,
        "metal" => 11,
        "classical" => 12,
        "folk" => 13,
        "jazz" => 14,
        _ => 0,
    }
}

pub fn genre_str(code: i16) -> Option<&'static str> {
    match code {
        1 => Some("any"),
        2 => Some("unspecified"),
        3 => Some("video_game"),
        4 => Some("anime"),
        5 => Some("rock"),
        6 => Some("pop"),
        7 => Some("other"),
        8 => Some("novelty"),
        9 => Some("hip_hop"),
        10 => Some("electronic"),
        11 => Some("metal"),
        12 => Some("classical"),
        13 => Some("folk"),
        14 => Some("jazz"),
        _ => None,
    }
}

pub fn language_code(value: &str) -> i16 {
    match value {
        "any" => 1,
        "other" => 2,
        "english" => 3,
        "japanese" => 4,
        "chinese" => 5,
        "instrumental" => 6,
        "korean" => 7,
        "french" => 8,
        "german" => 9,
        "swedish" => 10,
        "spanish" => 11,
        "italian" => 12,
        "russian" => 13,
        "polish" => 14,
        "unspecified" => 15,
        _ => 0,
    }
}

pub fn language_str(code: i16) -> Option<&'static str> {
    match code {
        1 => Some("any"),
        2 => Some("other"),
        3 => Some("english"),
        4 => Some("japanese"),
        5 => Some("chinese"),
        6 => Some("instrumental"),
        7 => Some("korean"),
        8 => Some("french"),
        9 => Some("german"),
        10 => Some("swedish"),
        11 => Some("spanish"),
        12 => Some("italian"),
        13 => Some("russian"),
        14 => Some("polish"),
        15 => Some("unspecified"),
        _ => None,
    }
}
