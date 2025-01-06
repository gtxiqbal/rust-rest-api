use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CountReq {
    pub count: i64,
    pub count_loop: i64,
}

#[derive(Debug, Deserialize)]
pub struct UserReq {
    pub userId: String,
    pub branchId: String,
    pub fullName: String,
    pub email: String,
    pub status: i16,
    pub flgCbs: bool,
    pub application: String,
    pub expDate: NaiveDate,
}

impl UserReq{
    pub fn new() -> Self {
        Self {
            userId: "".to_string(),
            branchId: "".to_string(),
            fullName: "".to_string(),
            email: "".to_string(),
            status: 0,
            flgCbs: false,
            application: "".to_string(),
            expDate: Default::default(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct UserRes {
    pub userId: String,
    pub branchId: String,
    pub fullName: String,
    pub email: String,
    pub status: i16,
    pub flgCbs: bool,
    pub application: String,
    pub expDate: NaiveDate,
}

impl UserRes {
    pub fn new() -> Self {
        Self {
            userId: "".to_string(),
            branchId: "".to_string(),
            fullName: "".to_string(),
            email: "".to_string(),
            status: 0,
            flgCbs: false,
            application: "".to_string(),
            expDate: Default::default(),
        }
    }
}