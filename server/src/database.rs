use crate::database::users::UsersCollection;

pub mod users;

#[derive(Clone)]
pub struct KewarCollections {
  pub users: UsersCollection,
}

pub async fn initialize(
  mongodb_connection_string: &str
) -> Result<KewarCollections, mongodb::error::Error> {
  let client = mongodb::Client::with_uri_str(mongodb_connection_string).await?;

  let database = client.database("kewar");

  Ok(KewarCollections {
    users: UsersCollection::default(database.collection("users")).await.unwrap(),
  })
}
