use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BeatmapsetsCursor {
    pub osu_user_id: i64,
    pub kind_index: usize,
    pub offset: usize,
}

impl BeatmapsetsCursor {
    pub fn start() -> Self {
        Self {
            osu_user_id: 0,
            kind_index: 0,
            offset: 0,
        }
    }
}

pub fn parse_last_id_cursor(cursor: Option<String>) -> i64 {
    let Some(cursor) = cursor else {
        return 0;
    };

    let Some(value) = cursor.strip_prefix("last_id:") else {
        return 0;
    };

    value.parse::<i64>().unwrap_or(0)
}

pub fn format_last_id_cursor(last_id: i64) -> String {
    format!("last_id:{last_id}")
}

pub fn parse_beatmapsets_cursor(cursor: Option<String>) -> BeatmapsetsCursor {
    let Some(cursor) = cursor else {
        return BeatmapsetsCursor::start();
    };

    serde_json::from_str::<BeatmapsetsCursor>(&cursor)
        .unwrap_or_else(|_| BeatmapsetsCursor::start())
}

pub fn format_beatmapsets_cursor(cursor: &BeatmapsetsCursor) -> String {
    serde_json::to_string(cursor)
        .unwrap_or_else(|_| "{\"osu_user_id\":0,\"kind_index\":0,\"offset\":0}".to_string())
}

pub fn next_beatmapsets_cursor(
    cursor: &BeatmapsetsCursor,
    returned: usize,
    page_size: usize,
) -> BeatmapsetsCursor {
    if returned < page_size {
        BeatmapsetsCursor {
            osu_user_id: cursor.osu_user_id,
            kind_index: cursor.kind_index + 1,
            offset: 0,
        }
    } else {
        BeatmapsetsCursor {
            osu_user_id: cursor.osu_user_id,
            kind_index: cursor.kind_index,
            offset: cursor.offset + page_size,
        }
    }
}
