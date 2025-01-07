use crate::handlers::user::UserState;
use crate::repositories::db::user::UserRepoDb;
use crate::services::user::UserService;
use crate::{configs, middlewares, router, utils};
use axum::{middleware, Router};
use log::info;
use std::io;
use tower::ServiceBuilder;

pub struct App {
    user_state: UserState,
}

impl App {
    pub async fn init() -> io::Result<Self> {
        let result = configs::setting::Setting::new();
        if let Err(err) = result {
            panic!("panic load setting: {}", err.to_string())
        }

        configs::log::init_logging();

        let mut setting = result.unwrap();

        utils::messages::init_message().await?;

        let result = configs::pg_conn::conn(&mut setting).await;
        if let Err(err) = result {
            panic!("panic database: {}", err.to_string())
        }

        let conn = result.unwrap();

        //repositories
        let user_repo = UserRepoDb::new(conn);

        //services
        let user_service = UserService::new(user_repo);

        //states
        let user_state = UserState {user_service};

        Ok(Self { user_state })
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
        Router::new().nest(
            "/api/v1/users",
            router::user::user(self.user_state.clone()),
        )
    }

    fn init_layer(&self, router: Router) -> Router {
        router.layer(
            ServiceBuilder::new()
                .layer(middleware::from_fn(middlewares::language::accept_language))
                .layer(middleware::from_fn(middlewares::auth::auth_check))
                .layer(middleware::from_fn(middlewares::log::stdout::log_write)),
        )
    }
}
