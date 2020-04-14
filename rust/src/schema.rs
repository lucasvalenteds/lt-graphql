extern crate rand;

use juniper::{FieldResult, GraphQLEnum, GraphQLInputObject, GraphQLObject, RootNode};
use rand::Rng;
use uuid::Uuid;

#[derive(GraphQLEnum)]
enum EngineStatus {
    On,
    Off,
}

#[derive(GraphQLObject)]
#[graphql(description = "Current state from a car")]
struct Engine {
    code: String,
    status: EngineStatus,
    fuel: i32,
}

#[derive(GraphQLInputObject)]
#[graphql(description = "Current state from a car")]
struct NewEngine {
    status: Option<EngineStatus>,
    fuel: Option<i32>,
}

pub struct RootQuery;

#[juniper::object]
impl RootQuery {
    fn engine(code: String) -> FieldResult<Engine> {
        let mut rng = rand::thread_rng();

        Ok(Engine {
            code: code.to_owned(),
            status: EngineStatus::On,
            fuel: rng.gen_range(0, 100),
        })
    }
}

pub struct RootMutation;

#[juniper::object]
impl RootMutation {
    fn register_engine(new_engine: NewEngine) -> FieldResult<Engine> {
        Ok(Engine {
            code: Uuid::new_v4().to_string(),
            status: match new_engine.status {
                Some(status) => status,
                None => EngineStatus::Off,
            },
            fuel: match new_engine.fuel {
                Some(fuel) => fuel,
                None => {
                    let mut rng = rand::thread_rng();
                    rng.gen_range(0, 100)
                }
            },
        })
    }
}

pub type RootSchema = RootNode<'static, RootQuery, RootMutation>;

pub fn create_schema() -> RootSchema {
    RootSchema::new(RootQuery {}, RootMutation {})
}
