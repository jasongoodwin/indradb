use crate::{models, Datastore};

use chrono::offset::Utc;
use chrono::DateTime;
use uuid::Uuid;

// TODO: remove &mut

pub async fn create_edge_from<D: Datastore>(datastore: &D, outbound_id: Uuid) -> Uuid {
    let inbound_vertex_t = models::Identifier::new("test_inbound_vertex_type").unwrap();
    let inbound_v = models::Vertex::new(inbound_vertex_t);
    datastore.create_vertex(&inbound_v).await.unwrap();
    let edge_t = models::Identifier::new("test_edge_type").unwrap();
    let key = models::EdgeKey::new(outbound_id, edge_t, inbound_v.id);
    datastore.create_edge(&key).await.unwrap();
    inbound_v.id
}

pub async fn create_edges<D: Datastore>(datastore: &D) -> (Uuid, [Uuid; 5]) {
    let outbound_vertex_t = models::Identifier::new("test_outbound_vertex_type").unwrap();
    let outbound_v = models::Vertex::new(outbound_vertex_t);
    datastore.create_vertex(&outbound_v).await.unwrap();
    let inbound_ids: [Uuid; 5] = [
        create_edge_from(datastore, outbound_v.id),
        create_edge_from(datastore, outbound_v.id),
        create_edge_from(datastore, outbound_v.id),
        create_edge_from(datastore, outbound_v.id),
        create_edge_from(datastore, outbound_v.id),
    ];

    (outbound_v.id, inbound_ids)
}

pub async fn create_time_range_queryable_edges<D: Datastore>(
    datastore: &D,
) -> (Uuid, DateTime<Utc>, DateTime<Utc>, [Uuid; 5]) {
    let outbound_vertex_t = models::Identifier::new("test_outbound_vertex_type").unwrap();
    let outbound_v = models::Vertex::new(outbound_vertex_t);
    datastore.create_vertex(&outbound_v).await.unwrap();

    create_edge_from(datastore, outbound_v.id).await;
    create_edge_from(datastore, outbound_v.id).await;
    create_edge_from(datastore, outbound_v.id).await;
    create_edge_from(datastore, outbound_v.id).await;
    create_edge_from(datastore, outbound_v.id).await;

    let start_time = Utc::now();
    let inbound_ids = [
        create_edge_from(datastore, outbound_v.id).await,
        create_edge_from(datastore, outbound_v.id).await,
        create_edge_from(datastore, outbound_v.id).await,
        create_edge_from(datastore, outbound_v.id).await,
        create_edge_from(datastore, outbound_v.id).await,
    ];
    let end_time = Utc::now();

    create_edge_from(datastore, outbound_v.id).await;
    create_edge_from(datastore, outbound_v.id).await;
    create_edge_from(datastore, outbound_v.id).await;
    create_edge_from(datastore, outbound_v.id).await;
    create_edge_from(datastore, outbound_v.id).await;

    (outbound_v.id, start_time, end_time, inbound_ids)
}
