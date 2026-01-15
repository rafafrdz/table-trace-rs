use analytics::AnalyticsModule;
use axum::Router;
use common::modules::Module;
use common::{config::Config, server::create_app};
use tokio::net::TcpListener;

pub struct AppModules {
    pub analytics: AnalyticsModule,
}

impl AppModules {
    pub async fn init(config: &Config) -> Self {
        let analytics: AnalyticsModule = AnalyticsModule::create(config).await.unwrap();

        Self { analytics }
    }

    pub fn combined_routes(&self) -> Router {
        let routes: Vec<Router> = vec![self.analytics.routes()];
        routes
            .into_iter()
            .reduce(|acc, router| acc.merge(router))
            .unwrap_or_else(Router::new)
    }
}

pub async fn create_routes(config: &Config) -> Router {
    AppModules::init(config).await.combined_routes()
}
pub async fn run() {
    let config: Config = Config::from_env();
    let router: Router = create_routes(&config).await;
    let app: Router = create_app(router).await;
    let raw_addr: String = format!("0.0.0.0:{}", config.port);
    let addr: TcpListener = TcpListener::bind(raw_addr.clone()).await.unwrap();
    tracing::info!("API listening on {}", raw_addr);
    axum::serve(addr, app).await.unwrap();
}
