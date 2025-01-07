use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Default)]
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

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Default)]
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
