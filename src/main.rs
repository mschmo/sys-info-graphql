extern crate iron;
extern crate mount;
#[macro_use] extern crate juniper;
extern crate juniper_iron;

use std::env;

use mount::Mount;
use iron::prelude::*;
use juniper_iron::{GraphQLHandler, GraphiQLHandler};
use juniper::EmptyMutation;

mod models;
mod schema;
use models::{
    System, DiskInformation, LoadAverage, MemoryInformation, ByteUnit, CycleUnit
};


fn context_factory(_: &mut Request) -> System {
    System::new()
}

fn main() {
    let mut mount = Mount::new();

    let graphql_endpoint = GraphQLHandler::new(
        context_factory,
        System::new(),
        EmptyMutation::<System>::new()
    );
    let graphiql_endpoint = GraphiQLHandler::new("/graphql");

    mount.mount("/", graphiql_endpoint);
    mount.mount("/graphql", graphql_endpoint);

    // TODO: Use cli argument instead of env var?
    let host = env::var("SYS_GQL_HOST").unwrap_or("localhost:5000".to_owned());
    match Iron::new(mount).http(&host) {
        Ok(_) => println!("Server started on {}", host),
        Err(e) => println!("Error starting server: {}", e)
    };
}
