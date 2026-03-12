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
