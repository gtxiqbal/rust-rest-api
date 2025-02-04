use crate::configs::db;
use crate::models::entity::user_master::UserMaster;
use crate::repositories::user::UserRepo;
use crate::utils::context::CTX_APP;
use crate::utils::error::ErrorApp;
use sqlx::error::ErrorKind;
use sqlx::Error;

#[derive(Clone, Debug)]
pub struct UserRepoDb {
}

impl UserRepoDb {
    pub fn new() -> Self {
        Self {  }
    }
}

impl UserRepo for UserRepoDb {
    async fn find_all(&self) -> Result<Vec<UserMaster>, ErrorApp> {
        let query = sqlx::query("select * from user_master");
        let result = db::fetch_all(query, UserMaster::row_mapper).await;
        match result {
            Ok(result) => Ok(result),
            Err(err) => Err(ErrorApp::OtherErr(err.to_string())),
        }
    }

    async fn find_by_user_id(&self, user_id: String) -> Result<UserMaster, ErrorApp> {
        let query = sqlx::query("select * from user_master where userid = $1")
            .bind(user_id);
        let result = db::fetch_one(query, UserMaster::row_mapper).await;
        match result {
            Ok(result) => Ok(result),
            Err(err) => Err(match err {
                Error::RowNotFound => ErrorApp::RowNotFound,
                _ => ErrorApp::OtherErr(err.to_string()),
            }),
        }
    }

    async fn create(&self, user_master: &mut UserMaster) -> Result<(), ErrorApp> {
        user_master.created_by = CTX_APP.get().user_id;
        let query = sqlx::query(
            "INSERT INTO public.user_master
(userid, fullname, email, status, expdate, created_at, branchid, created_by, application, flgcbs)
VALUES($1,$2,$3,$4,$5,$6,$7,$8,$9,$10)",
        )
        .bind(&user_master.userid)
        .bind(&user_master.fullname)
        .bind(&user_master.email)
        .bind(user_master.status)
        .bind(user_master.expdate)
        .bind(user_master.created_at)
        .bind(&user_master.branchid)
        .bind(&user_master.created_by)
        .bind(&user_master.application)
        .bind(user_master.flgcbs);
        match db::execute(query).await {
            Ok(_) => Ok(()),
            Err(err) => Err(match err {
                Error::Database(err_db) => match err_db.kind() {
                    ErrorKind::UniqueViolation => ErrorApp::DuplicateKey,
                    _ => ErrorApp::OtherErr(err_db.to_string()),
                },
                _ => ErrorApp::OtherErr(err.to_string()),
            }),
        }
    }

    async fn update(&self, user_master: &mut UserMaster) -> Result<(), ErrorApp> {
        user_master.updated_by = Some(CTX_APP.get().user_id);
        let query = sqlx::query("UPDATE public.user_master
SET fullname=$2, email=$3, status=$4, expdate=$5, branchid=$6, updated_at=now(), updated_by=$7, application=$8, flgcbs=$9
WHERE userid=$1")
            .bind(&user_master.userid)
            .bind(&user_master.fullname)
            .bind(&user_master.email)
            .bind(user_master.status)
            .bind(user_master.expdate)
            .bind(&user_master.branchid)
            .bind(&user_master.updated_by)
            .bind(&user_master.application)
            .bind(user_master.flgcbs);
        match db::execute(query).await {
            Ok(result) => {
                if result.rows_affected() == 0 {
                    Err(ErrorApp::RowNotFound)
                } else {
                    Ok(())
                }
            }
            Err(err) => Err(ErrorApp::OtherErr(err.to_string())),
        }
    }

    async fn delete(&self, user_id: String) -> Result<(), ErrorApp> {
        let query = sqlx::query("delete from user_master where userid = $1")
            .bind(user_id);

        match db::execute(query).await {
            Ok(result) => {
                if result.rows_affected() == 0 {
                    Err(ErrorApp::RowNotFound)
                } else {
                    Ok(())
                }
            }
            Err(err) => Err(ErrorApp::OtherErr(err.to_string())),
        }
    }
}
