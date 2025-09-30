use sqlx::types::chrono::NaiveDateTime;
use uuid::Uuid;

pub enum FriendshipLevel {
    Normal,
    Trusted,
}

impl<'a> From<&'a str> for FriendshipLevel {
    fn from(a: &'a str) -> Self {
        match a {
            "normal" => Self::Normal,
            "trusted" => Self::Trusted,
            _ => Self::Normal,
        }
    }
}

impl FriendshipLevel {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Normal => "normal",
            Self::Trusted => "trusted",
        }
    }
}

#[derive(sqlx::FromRow, Debug)]
pub struct Friendships {
    pub friendship_id: Uuid,
    pub friendship_level: &'static str,
    pub account_in: Uuid,
    pub account_out: Uuid,
    pub created_at: NaiveDateTime,
}

pub const INIT_FRIENDSHIPS_TABLE: &'static str = r#"
    CREATE TABLE IF NOT EXISTS Friendships (
        friendship_id UUID PRIMARY KEY,
        account_in UUID NOT NULL,
        account_out UUID NOT NULL,
        friendship_level VARCHAR(64) NOT NULL,
        created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,

        FOREIGN KEY (account_in) REFERENCES Accounts(account_id) ON DELETE CASCADE,
        FOREIGN KEY (account_out) REFERENCES Accounts(account_id) ON DELETE CASCADE
    );
"#;

pub const INIT_FRIENDSHIPS_INDEX_IN: &'static str = r#"
    CREATE INDEX idx_friendships_in ON Friendships (username);
"#;
pub const INIT_FRIENDSHIPS_INDEX_OUT: &'static str = r#"
    CREATE INDEX idx_friendships_in ON Friendships (username);
"#;
