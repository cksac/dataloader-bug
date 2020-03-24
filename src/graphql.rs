use dataloader::Loader;
use dataloader::{BatchFn, BatchFuture};
use futures::{future, FutureExt as _};
use juniper::{EmptyMutation, EmptySubscription, IntrospectionFormat};
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Arc;

pub struct Queries;

#[juniper::graphql_object(Context = Context)]
impl Queries {
    async fn user(&self, id: String, ctx: &Context) -> User {
        ctx.user_loader.load(id).await.unwrap()
    }
}

#[derive(Clone)]
pub struct User {
    id: String,
}

#[juniper::graphql_object(Context = Context)]
impl User {
    fn id(&self) -> String {
        self.id.clone()
    }

    async fn friend(&self, ctx: &Context) -> User {
        ctx.user_loader
            .load(format!("friend of {}", self.id))
            .await
            .unwrap()
    }
}

pub struct UserBatch;

impl BatchFn<String, User> for UserBatch {
    type Error = ();

    fn load(&self, keys: &[String]) -> BatchFuture<User, Self::Error> {
        log::debug!("load batch {:?}", keys);
        future::ready(keys.iter().map(|key| User { id: key.clone() }).collect())
            .unit_error()
            .boxed()
    }
}

type UserLoader = Loader<String, User, (), UserBatch>;

//

pub struct Context {
    user_loader: UserLoader,
}

impl juniper::Context for Context {}

impl Context {
    pub fn new() -> Self {
        Context {
            user_loader: Loader::new(UserBatch),
        }
    }
}

type Schema =
    juniper::RootNode<'static, Queries, EmptyMutation<Context>, EmptySubscription<Context>>;

lazy_static! {
    pub static ref ROOT_NODE: Arc<Schema> = Arc::new(Schema::new(
        Queries,
        EmptyMutation::<Context>::new(),
        EmptySubscription::<Context>::new()
    ));
}

lazy_static! {
    pub static ref SCHEMA: serde_json::Value = {
        let ctx = Context::new();

        let (res, _errors) = juniper::introspect(
            &Schema::new(Queries, EmptyMutation::new(), EmptySubscription::new()),
            &ctx,
            IntrospectionFormat::default(),
        )
        .expect("Invalid schema");

        serde_json::to_value(res).expect("Invalid JSON schema")
    };
    pub static ref SCHEMA_JSON: Vec<u8> = serde_json::to_vec(&*SCHEMA).expect("Invalid schema");
}
