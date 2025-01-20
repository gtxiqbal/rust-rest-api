use crate::handlers::user::UserState;
use crate::repositories::db::user::UserRepoDb;
use crate::services::user::UserService;
use crate::{configs, middlewares, router, utils};
use axum::{middleware, Router};
use log::info;
use std::io;
use sqlx::{Pool, Postgres};
use tokio::signal;
use tower::ServiceBuilder;

pub struct App {
    user_state: UserState,
    db: Pool<Postgres>
}

impl App {
    pub async fn init() -> io::Result<Self> {
        let result = configs::setting::Setting::new();
        if let Err(err) = result {
            panic!("panic load setting: {}", err.to_string())
        }

        configs::logging::init_logging();

        let mut setting = result.unwrap();

        utils::messages::init_message().await?;

        let result = configs::db::init_pg(&mut setting).await;
        if let Err(err) = result {
            panic!("panic database: {}", err.to_string())
        }

        let conn = result.unwrap();

        //repositories
        let user_repo = UserRepoDb::new();

        //services
        let user_service = UserService::new(user_repo);

        //states
        let user_state = UserState { user_service };

        Ok(Self { 
            user_state: user_state,
            db: conn.clone(),
        })
    }

    pub async fn run(&self) -> io::Result<()> {
        let router = self.init_route();

        let app = self.init_layer(router);

        let port = 8080;
        let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await?;
        info!("start server on port {}", port);
        axum::serve(listener, app).with_graceful_shutdown(Self::shutdown_signal()).await
    }

    fn init_route(&self) -> Router {
        Router::new().nest("/api/v1/users", router::user::user(self.user_state.clone()))
    }

    fn init_layer(&self, router: Router) -> Router {
        router.layer(
            ServiceBuilder::new()
                .layer(middleware::from_fn(middlewares::language::accept_language))
                .layer(middleware::from_fn_with_state(self.db.clone(), middlewares::db::inject))
                .layer(middleware::from_fn(middlewares::auth::auth_check))
                .layer(middleware::from_fn(middlewares::logging::stdout::log_write)),
        )
    }

    async fn shutdown_signal() {
        let ctrl_c = async {
            signal::ctrl_c()
                .await
                .expect("failed to install Ctrl+C handler");
        };

        #[cfg(unix)]
        let terminate = async {
            signal::unix::signal(signal::unix::SignalKind::terminate())
                .expect("failed to install signal handler")
                .recv()
                .await;
        };

        #[cfg(not(unix))]
        let terminate = std::future::pending::<()>();
        
        tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
    }
}
