use std::{env, io, mem};
use std::sync::Arc;
use std::sync::atomic::AtomicI64;
use axum::{middleware, Router};
use axum::middleware::FromFnLayer;
use log::info;
use sqlx::{Pool, Postgres};
use tower::layer::util::Stack;
use tower::ServiceBuilder;
use crate::config::setting;
use crate::handlers::user::UserState;
use crate::{handlers, middlewares, repositories, router, services, utils};
use crate::repositories::db::user::UserRepoDb;
use crate::services::user::UserService;

pub struct App {
    pub user_state: Arc<UserState>
}

impl App {
    pub async fn init() ->  io::Result<Self>{
        let result = setting::Setting::new();
        if let Err(err) = result {
            panic!("panic load setting: {}", err.to_string())
        }

        let resources_path = env::var("RESOURCES_PATH").unwrap_or("resources".to_string());
        log4rs::init_file(format!("{resources_path}/log4rs.yaml"), Default::default()).unwrap();

        let setting = result.unwrap();

        utils::messages::init_message().await?;

        let result = crate::config::pg_conn::conn(&setting).await;
        if let Err(err) = result {
            panic!("panic database: {}", err.to_string())
        }

        let conn = result.unwrap();

        //repositories
        let user_repo = Arc::new(UserRepoDb::new(conn));

        //services
        let user_service = Arc::new(UserService::new(user_repo));

        //states
        let user_state = Arc::new(UserState::new(user_service));

        Ok(Self{ user_state })
    }

    pub async fn run(&self) -> io::Result<()> {
        let router = self.init_route();

        let app = self.init_layer(router);

        let port = 8080;
        let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await?;
        info!("start server on port {}", port);
        axum::serve(listener, app).await
    }

    fn init_route(&self) -> Router {
        Router::new()
            .nest("/api/v1/users", router::user::user(Arc::clone(&self.user_state)))
    }

    fn init_layer(&self, router: Router) -> Router {
        router.layer(
            ServiceBuilder::new()
                .layer(middleware::from_fn(middlewares::language::accept_language))
                .layer(middleware::from_fn(middlewares::auth::auth_check))
                .layer(middleware::from_fn(middlewares::log::stdout::log_write))
        )
    }
}