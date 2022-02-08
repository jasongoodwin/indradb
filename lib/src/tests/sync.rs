use crate::Datastore;

pub async fn should_sync<D: Datastore>(datastore: &D) {
    // just make sure that it runs fine
    datastore.sync().await.unwrap();
}
