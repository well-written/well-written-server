use async_graphql::{Object, Context, Schema, EmptyMutation, EmptySubscription};

pub type AppSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

pub struct QueryRoot;

#[Object]
impl QueryRoot {
  async fn hello<'a>(
    &self,
    _ctx: &Context<'a>,
  ) -> String {
    "Hello, world!".to_string()
  }
}
