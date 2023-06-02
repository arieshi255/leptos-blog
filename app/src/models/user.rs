use cfg_if::cfg_if;
use std::collections::HashSet;
use serde::{Serialize, Deserialize};
use validator::Validate;

#[derive(Debug, Validate, Serialize, Deserialize, Clone)]
pub struct SignupUser {
  #[validate(length(min = 4, max = 32, message = "Username length too long or short"))]
  pub username: String,
  #[validate(must_match(other = "password_confirm", message = "Passwords don't match"), length(min = 8, message = "Password length too short"))]
  pub password: String,
  pub password_confirm: String
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct User {
  pub id: i64,
  pub username: String,
  pub display_name: String,
  pub password: String,
  pub created_at: i64,
  pub created_at_pretty: String,
  pub updated_at: i64,
  pub updated_at_pretty: String,
  pub permissions: HashSet<String>,
}

cfg_if! {
  if #[cfg(feature = "ssr")] {
    use sqlx::SqlitePool;
    use chrono::naive::NaiveDateTime;
    use crate::functions::auth::SqlPermissionTokens;

    #[derive(sqlx::FromRow, Debug, Clone)]
    pub struct SqlUser {
      pub id: i64,
      pub username: String,
      pub display_name: String,
      pub password: String,
      pub created_at: i64,
      pub updated_at: i64,
    }

    impl SqlUser {
      pub fn into_user(self, sql_user_perms: Option<Vec<SqlPermissionTokens>>) -> User {
        User {
          id: self.id,
          username: self.username,
          display_name: self.display_name,
          password: self.password,
          created_at: self.created_at,
          created_at_pretty: NaiveDateTime::from_timestamp_opt(self.created_at, 0).unwrap_or_default().to_string(),
          updated_at: self.updated_at,
          updated_at_pretty: NaiveDateTime::from_timestamp_opt(self.created_at, 0).unwrap_or_default().to_string(),

          permissions: if let Some(user_perms) = sql_user_perms {
            user_perms
              .into_iter()
              .map(|x| x.token)
              .collect::<HashSet<String>>()
            } else {
                HashSet::<String>::new()
            },
        }
      }
    }

    impl User {
      pub async fn get(id: i64, pool: &SqlitePool) -> Option<Self> {
        let sqluser = sqlx::query_as::<_, SqlUser>("SELECT * FROM users WHERE id = ?")
          .bind(id)
          .fetch_one(pool)
          .await
          .ok()?;

        //lets just get all the tokens the user can use, we will only use the full permissions if modifing them.
        // let sql_user_perms = sqlx::query_as::<_, SqlPermissionTokens>(
        //   "SELECT token FROM user_permissions WHERE user_id = ?;",
        // )
        //   .bind(id)
        //   .fetch_all(pool)
        //   .await
        //   .ok()?;

        Some(sqluser.into_user(None))
      }

      pub async fn get_from_username(name: String, pool: &SqlitePool) -> Option<Self> {
        let sqluser = sqlx::query_as::<_, SqlUser>("SELECT * FROM users WHERE username = ?")
          .bind(name)
          .fetch_one(pool)
          .await
          .ok()?;

        Some(sqluser.into_user(None))
      }
    }
  }
}