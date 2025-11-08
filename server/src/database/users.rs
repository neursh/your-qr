use mongodb::{ IndexModel, bson, options::IndexOptions };
use serde::{ Deserialize, Serialize };

#[derive(Clone, Deserialize, Serialize)]
pub struct UserStore {
  pub email: String,
  pub password_hash: String,
  pub _id: String,
}

#[derive(Clone)]
pub struct UsersCollection {
  collection: mongodb::Collection<UserStore>,
}
impl UsersCollection {
  pub async fn default(
    collection: mongodb::Collection<UserStore>
  ) -> Result<Self, mongodb::error::Error> {
    collection.create_index(
      IndexModel::builder()
        .keys(bson::doc! { "email": 1 })
        .options(IndexOptions::builder().unique(true).build())
        .build()
    ).await?;

    Ok(UsersCollection {
      collection,
    })
  }

  pub async fn add(&self, user: UserStore) -> Result<(), mongodb::error::Error> {
    self.collection.insert_one(user).await?;
    Ok(())
  }
  pub async fn delete(&self, id: String) -> Result<(), mongodb::error::Error> {
    self.collection.delete_one(bson::doc! { "id": id }).await?;
    Ok(())
  }
}
