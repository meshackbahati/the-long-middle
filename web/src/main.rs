use oxidite::prelude::*;
use oxidite::template::serve_static;

mod routes;
mod controllers;
mod middleware;
mod models;
mod services;
mod validators;
mod jobs;
mod policies;
mod events;

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::load()
        .map_err(|e| Error::InternalServerError(e.to_string()))?;
    let addr = format!("{}:{}", config.server.host, config.server.port);

    let mut router = Router::new();
    routes::register(&mut router);

    // Static files fallback
    router.get("/*", serve_static);

    let server = Server::new(router);
    println!("🚀 Server running on http://{}", addr);
    server.listen(addr.parse().unwrap()).await
}
