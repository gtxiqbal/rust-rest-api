use crate::models::dto::user::{UserReq, UserRes};
use chrono::{NaiveDate, NaiveDateTime};
use sqlx::postgres::PgRow;
use sqlx::Row;

#[derive(Debug, Default)]
pub struct UserMaster {
    pub userid: String,
    pub branchid: String,
    pub fullname: String,
    pub email: String,
    pub status: i16,
    pub flgcbs: bool,
    pub application: String,
    pub expdate: NaiveDate,
    pub created_at: NaiveDateTime,
    pub created_by: String,
    pub updated_at: Option<NaiveDateTime>,
    pub updated_by: Option<String>,
}

impl UserMaster {
    pub fn row_mapper(row: PgRow) -> Result<Self, sqlx::Error> {
        let user_master = UserMaster {
            userid: row.try_get::<String, &str>("userid")?,
            branchid: row.try_get::<String, _>("branchid")?,
            fullname: row.try_get::<String, _>("fullname")?,
            email: row.try_get::<String, _>("email")?,
            status: row.try_get::<i16, _>("status")?,
            flgcbs: row.try_get::<bool, _>("flgcbs")?,
            application: row.try_get::<String, _>("application")?,
            expdate: row.try_get::<NaiveDate, _>("expdate")?,
            created_at: row.try_get::<NaiveDateTime, _>("created_at")?,
            created_by: row.try_get::<String, _>("created_by")?,
            updated_at: row.try_get::<Option<NaiveDateTime>, _>("updated_at")?,
            updated_by: row.try_get::<Option<String>, _>("updated_by")?,
        };
        Ok(user_master)
    }

    pub fn from_user_req(user_req: UserReq) -> Self {
        Self {
            userid: user_req.userId,
            branchid: user_req.branchId,
            fullname: user_req.fullName,
            email: user_req.email,
            status: user_req.status,
            flgcbs: user_req.flgCbs,
            application: user_req.application,
            expdate: user_req.expDate,
            created_at: chrono::Local::now().naive_local(),
            created_by: "SYSTEM".to_string(),
            updated_at: None,
            updated_by: None,
        }
    }

    pub fn to_user_res(self) -> UserRes {
        UserRes {
            userId: self.userid,
            branchId: self.branchid,
            fullName: self.fullname,
            email: self.email,
            status: self.status,
            flgCbs: self.flgcbs,
            application: self.application,
            expDate: self.expdate,
        }
    }
}
