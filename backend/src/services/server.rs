use dotenv::dotenv;
use iron::Iron;
use iron::prelude::*;
use router::Router;
use mount::Mount;
use simplelog::{Config, TermLogger, LogLevelFilter};

use super::RequestLogger;

use services::storage::{PersistanceStorage, DbConnectionPool};

pub use handlers;

macro_rules! handle_endpoint {
    ($router:expr, $pool:expr, $route:expr, $method:ident, $name:expr, $lambda:expr) => {
        {
           let query_pool = $pool.clone();
           $router.$method($route, move |req: &mut Request| -> IronResult<Response> {
               $lambda(req, query_pool.get().unwrap())

           }, $name);
        }
    }
}

macro_rules! route_endpoints {
    ($pool:expr, $mainRouter:expr, $mount_route:expr, $( $name:expr => $method:ident, $route:expr, $handler:expr  ),*) => {
        {
            let mut sub_router = Router::new();
            $(
                handle_endpoint!(sub_router, $pool, $route, $method, $name, $handler);

             )*
                $mainRouter.mount($mount_route, sub_router);

        }

    } 
}

/// declare endpoints takes a pool connection, and mounts endpoint handlers.
pub fn declare_endpoints(pool: DbConnectionPool) -> Mount {
    let mut routes = Mount::new();

    route_endpoints!(
        pool, routes, "/",
        "ping" => get, "/ping", handlers::ping::ping
    );

    route_endpoints!(
        pool, routes, "/user/",
        "get_user" => get, "/:id", handlers::get_user 
        // "get_many_events" => get, "/events", handlers::handle_get_many_events,
        // "post_new_events" => post, "/events", handlers::handle_post_new_event
     );

    routes
}

/// start will initialize terminal logger, setup routes, listen and serve.
pub fn start(port: &str, quiet: bool) {
    // Initialize terminal logger
    let _ = TermLogger::init(LogLevelFilter::Info, Config::default()).unwrap();

    // Initialize database and env variables
    dotenv().ok();
    let db = PersistanceStorage::new();
    let db_pool_conn = db.get_pool();

    // route handlers
    let handlers = declare_endpoints(db_pool_conn);

    // let router = router! {
    //     get_ping: get "/ping" => handlers::ping::Ping,
    //     get_many_events: get "/events" => handlers::handle_get_many_events,
    //     post_new_event: post "/events" => handlers::handle_post_new_event,
    // };

    // chain methods [log]
    let mut chain = Chain::new(handlers);
    if !quiet {
        chain.link_before(RequestLogger);
        chain.link_after(RequestLogger);
    }
    // initiate and serve Iron server
    match Iron::new(chain).http(port) {
        Ok(http) => info!("Listening and serving...{:?}", http),
        Result::Err(err) => panic!("Error starting Iron server: {}", err),
    }
}
