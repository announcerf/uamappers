use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared(
                r#"
                ALTER TABLE beatmapset_profiles
                    ALTER COLUMN genre TYPE smallint USING CASE genre
                        WHEN 'any' THEN 1
                        WHEN 'unspecified' THEN 2
                        WHEN 'video_game' THEN 3
                        WHEN 'anime' THEN 4
                        WHEN 'rock' THEN 5
                        WHEN 'pop' THEN 6
                        WHEN 'other' THEN 7
                        WHEN 'novelty' THEN 8
                        WHEN 'hip_hop' THEN 9
                        WHEN 'electronic' THEN 10
                        WHEN 'metal' THEN 11
                        WHEN 'classical' THEN 12
                        WHEN 'folk' THEN 13
                        WHEN 'jazz' THEN 14
                        ELSE NULL
                    END,
                    ALTER COLUMN language TYPE smallint USING CASE language
                        WHEN 'any' THEN 1
                        WHEN 'other' THEN 2
                        WHEN 'english' THEN 3
                        WHEN 'japanese' THEN 4
                        WHEN 'chinese' THEN 5
                        WHEN 'instrumental' THEN 6
                        WHEN 'korean' THEN 7
                        WHEN 'french' THEN 8
                        WHEN 'german' THEN 9
                        WHEN 'swedish' THEN 10
                        WHEN 'spanish' THEN 11
                        WHEN 'italian' THEN 12
                        WHEN 'russian' THEN 13
                        WHEN 'polish' THEN 14
                        WHEN 'unspecified' THEN 15
                        ELSE NULL
                    END,
                    ALTER COLUMN status TYPE smallint USING CASE status
                        WHEN 'graveyard' THEN 1
                        WHEN 'wip' THEN 2
                        WHEN 'pending' THEN 3
                        WHEN 'ranked' THEN 4
                        WHEN 'approved' THEN 5
                        WHEN 'qualified' THEN 6
                        WHEN 'loved' THEN 7
                        ELSE 0
                    END;

                ALTER TABLE beatmap_profiles
                    ALTER COLUMN mode TYPE smallint USING CASE mode
                        WHEN 'osu' THEN 1
                        WHEN 'taiko' THEN 2
                        WHEN 'catch' THEN 3
                        WHEN 'mania' THEN 4
                        ELSE 0
                    END,
                    ALTER COLUMN status TYPE smallint USING CASE status
                        WHEN 'graveyard' THEN 1
                        WHEN 'wip' THEN 2
                        WHEN 'pending' THEN 3
                        WHEN 'ranked' THEN 4
                        WHEN 'approved' THEN 5
                        WHEN 'qualified' THEN 6
                        WHEN 'loved' THEN 7
                        ELSE 0
                    END;

                ALTER TABLE mapper_stats_current
                    ALTER COLUMN main_mode TYPE smallint USING CASE main_mode
                        WHEN 'osu' THEN 1
                        WHEN 'taiko' THEN 2
                        WHEN 'catch' THEN 3
                        WHEN 'mania' THEN 4
                        ELSE 0
                    END;

                ALTER TABLE mapper_aggregate_snapshots_weekly
                    ALTER COLUMN main_mode TYPE smallint USING CASE main_mode
                        WHEN 'osu' THEN 1
                        WHEN 'taiko' THEN 2
                        WHEN 'catch' THEN 3
                        WHEN 'mania' THEN 4
                        ELSE 0
                    END;

                ALTER TABLE beatmapset_snapshots_weekly
                    ALTER COLUMN status TYPE smallint USING CASE status
                        WHEN 'graveyard' THEN 1
                        WHEN 'wip' THEN 2
                        WHEN 'pending' THEN 3
                        WHEN 'ranked' THEN 4
                        WHEN 'approved' THEN 5
                        WHEN 'qualified' THEN 6
                        WHEN 'loved' THEN 7
                        ELSE 0
                    END;

                ALTER TABLE osu_user_beatmapsets
                    ALTER COLUMN kind TYPE smallint USING CASE kind
                        WHEN 'graveyard' THEN 1
                        WHEN 'guest' THEN 2
                        WHEN 'loved' THEN 3
                        WHEN 'nominated' THEN 4
                        WHEN 'pending' THEN 5
                        WHEN 'ranked' THEN 6
                        ELSE 0
                    END;
                "#,
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared(
                r#"
                ALTER TABLE beatmapset_profiles
                    ALTER COLUMN genre TYPE text USING CASE genre
                        WHEN 1 THEN 'any'
                        WHEN 2 THEN 'unspecified'
                        WHEN 3 THEN 'video_game'
                        WHEN 4 THEN 'anime'
                        WHEN 5 THEN 'rock'
                        WHEN 6 THEN 'pop'
                        WHEN 7 THEN 'other'
                        WHEN 8 THEN 'novelty'
                        WHEN 9 THEN 'hip_hop'
                        WHEN 10 THEN 'electronic'
                        WHEN 11 THEN 'metal'
                        WHEN 12 THEN 'classical'
                        WHEN 13 THEN 'folk'
                        WHEN 14 THEN 'jazz'
                        ELSE NULL
                    END,
                    ALTER COLUMN language TYPE text USING CASE language
                        WHEN 1 THEN 'any'
                        WHEN 2 THEN 'other'
                        WHEN 3 THEN 'english'
                        WHEN 4 THEN 'japanese'
                        WHEN 5 THEN 'chinese'
                        WHEN 6 THEN 'instrumental'
                        WHEN 7 THEN 'korean'
                        WHEN 8 THEN 'french'
                        WHEN 9 THEN 'german'
                        WHEN 10 THEN 'swedish'
                        WHEN 11 THEN 'spanish'
                        WHEN 12 THEN 'italian'
                        WHEN 13 THEN 'russian'
                        WHEN 14 THEN 'polish'
                        WHEN 15 THEN 'unspecified'
                        ELSE NULL
                    END,
                    ALTER COLUMN status TYPE text USING CASE status
                        WHEN 1 THEN 'graveyard'
                        WHEN 2 THEN 'wip'
                        WHEN 3 THEN 'pending'
                        WHEN 4 THEN 'ranked'
                        WHEN 5 THEN 'approved'
                        WHEN 6 THEN 'qualified'
                        WHEN 7 THEN 'loved'
                        ELSE 'unknown'
                    END;

                ALTER TABLE beatmap_profiles
                    ALTER COLUMN mode TYPE text USING CASE mode
                        WHEN 1 THEN 'osu'
                        WHEN 2 THEN 'taiko'
                        WHEN 3 THEN 'catch'
                        WHEN 4 THEN 'mania'
                        ELSE 'unknown'
                    END,
                    ALTER COLUMN status TYPE text USING CASE status
                        WHEN 1 THEN 'graveyard'
                        WHEN 2 THEN 'wip'
                        WHEN 3 THEN 'pending'
                        WHEN 4 THEN 'ranked'
                        WHEN 5 THEN 'approved'
                        WHEN 6 THEN 'qualified'
                        WHEN 7 THEN 'loved'
                        ELSE 'unknown'
                    END;

                ALTER TABLE mapper_stats_current
                    ALTER COLUMN main_mode TYPE text USING CASE main_mode
                        WHEN 1 THEN 'osu'
                        WHEN 2 THEN 'taiko'
                        WHEN 3 THEN 'catch'
                        WHEN 4 THEN 'mania'
                        ELSE 'unknown'
                    END;

                ALTER TABLE mapper_aggregate_snapshots_weekly
                    ALTER COLUMN main_mode TYPE text USING CASE main_mode
                        WHEN 1 THEN 'osu'
                        WHEN 2 THEN 'taiko'
                        WHEN 3 THEN 'catch'
                        WHEN 4 THEN 'mania'
                        ELSE 'unknown'
                    END;

                ALTER TABLE beatmapset_snapshots_weekly
                    ALTER COLUMN status TYPE text USING CASE status
                        WHEN 1 THEN 'graveyard'
                        WHEN 2 THEN 'wip'
                        WHEN 3 THEN 'pending'
                        WHEN 4 THEN 'ranked'
                        WHEN 5 THEN 'approved'
                        WHEN 6 THEN 'qualified'
                        WHEN 7 THEN 'loved'
                        ELSE 'unknown'
                    END;

                ALTER TABLE osu_user_beatmapsets
                    ALTER COLUMN kind TYPE text USING CASE kind
                        WHEN 1 THEN 'graveyard'
                        WHEN 2 THEN 'guest'
                        WHEN 3 THEN 'loved'
                        WHEN 4 THEN 'nominated'
                        WHEN 5 THEN 'pending'
                        WHEN 6 THEN 'ranked'
                        ELSE 'unknown'
                    END;
                "#,
            )
            .await?;

        Ok(())
    }
}
