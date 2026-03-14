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
