use crate::errors::{Error, Result};
use crate::models;
use crate::models::{EdgeQueryExt, VertexQueryExt};
use async_trait::async_trait;
use std::vec::Vec;
use uuid::Uuid;

/// Specifies a datastore implementation.
///
/// Note that this trait and its members purposefully do not employ any
/// generic arguments. While that would improve ergonomics, it would remove
/// object safety, which we need for plugins.
///
/// # Errors
/// All methods may return an error if something unexpected happens - e.g.
/// if there was a problem connecting to the underlying database.
#[async_trait] // note async-trait has a cost. see: https://rust-lang.github.io/async-book/07_workarounds/05_async_in_traits.html
pub trait Datastore {
    /// Syncs persisted content. Depending on the datastore implementation,
    /// this has different meanings - including potentially being a no-op.
    async fn sync(&self) -> Result<()> {
        Err(Error::Unsupported)
    }

    /// Creates a new transaction. Some datastore implementations do not
    /// support transactional updates, in which case this will return an
    /// error.
    async fn transaction(&self) -> Result<Self>
    where
        Self: Sized,
    {
        Err(Error::Unsupported)
    }

    /// Creates a new vertex. Returns whether the vertex was successfully
    /// created - if this is false, it's because a vertex with the same UUID
    /// already exists.
    ///
    /// # Arguments
    /// * `vertex`: The vertex to create.
    async fn create_vertex(&self, vertex: &models::Vertex) -> Result<bool>;

    /// Creates a new vertex with just a type specification. As opposed to
    /// `create_vertex`, this is used when you do not want to manually specify
    /// the vertex's UUID. Returns the new vertex's UUID.
    ///
    /// # Arguments
    /// * `t`: The type of the vertex to create.
    async fn create_vertex_from_type(&self, t: models::Identifier) -> Result<Uuid> {
        let v = models::Vertex::new(t);

        if !self.create_vertex(&v).await? {
            Err(Error::UuidTaken)
        } else {
            Ok(v.id)
        }
    }

    /// Gets a range of vertices specified by a query.
    ///
    /// # Arguments
    /// * `q`: The query to run.
    async fn get_vertices(&self, q: models::VertexQuery) -> Result<Vec<models::Vertex>>;

    /// Deletes existing vertices specified by a query.
    ///
    /// # Arguments
    /// * `q`: The query to run.
    async fn delete_vertices(&self, q: models::VertexQuery) -> Result<()>;

    /// Gets the number of vertices in the datastore.
    async fn get_vertex_count(&self) -> Result<u64>;

    /// Creates a new edge. If the edge already exists, this will update it
    /// with a new update datetime. Returns whether the edge was successfully
    /// created - if this is false, it's because one of the specified vertices
    /// is missing.
    ///
    /// # Arguments
    /// * `key`: The edge to create.
    async fn create_edge(&self, key: &models::EdgeKey) -> Result<bool>;

    /// Gets a range of edges specified by a query.
    ///
    /// # Arguments
    /// * `q`: The query to run.
    async fn get_edges(&self, q: models::EdgeQuery) -> Result<Vec<models::Edge>>;

    /// Deletes a set of edges specified by a query.
    ///
    /// # Arguments
    /// * `q`: The query to run.
    async fn delete_edges(&self, q: models::EdgeQuery) -> Result<()>;

    /// Gets the number of edges associated with a vertex.
    ///
    /// # Arguments
    /// * `id`: The id of the vertex.
    /// * `t`: Only get the count for a specified edge type.
    /// * `direction`: The direction of edges to get.
    async fn get_edge_count(
        &self,
        id: Uuid,
        t: Option<&models::Identifier>,
        direction: models::EdgeDirection,
    ) -> Result<u64>;

    /// Gets vertex properties.
    ///
    /// # Arguments
    /// * `q`: The query to run.
    async fn get_vertex_properties(&self, q: models::VertexPropertyQuery) -> Result<Vec<models::VertexProperty>>;

    /// Gets all vertex properties.
    ///
    /// # Arguments
    /// * `q`: The query to run.
    async fn get_all_vertex_properties(&self, q: models::VertexQuery) -> Result<Vec<models::VertexProperties>>;

    /// Sets a vertex properties.
    ///
    /// # Arguments
    /// * `q`: The query to run.
    /// * `value`: The property value.
    async fn set_vertex_properties(&self, q: models::VertexPropertyQuery, value: serde_json::Value) -> Result<()>;

    /// Deletes vertex properties.
    ///
    /// # Arguments
    /// * `q`: The query to run.
    async fn delete_vertex_properties(&self, q: models::VertexPropertyQuery) -> Result<()>;

    /// Gets edge properties.
    ///
    /// # Arguments
    /// * `q`: The query to run.
    async fn get_edge_properties(&self, q: models::EdgePropertyQuery) -> Result<Vec<models::EdgeProperty>>;

    /// Gets all edge properties.
    ///
    /// # Arguments
    /// * `q`: The query to run.
    async fn get_all_edge_properties(&self, q: models::EdgeQuery) -> Result<Vec<models::EdgeProperties>>;

    /// Sets edge properties.
    ///
    /// # Arguments
    /// * `q`: The query to run.
    /// * `value`: The property value.
    async fn set_edge_properties(&self, q: models::EdgePropertyQuery, value: serde_json::Value) -> Result<()>;

    /// Deletes edge properties.
    ///
    /// # Arguments
    /// * `q`: The query to run.
    async fn delete_edge_properties(&self, q: models::EdgePropertyQuery) -> Result<()>;

    /// Bulk inserts many vertices, edges, and/or properties.
    ///
    /// # Arguments
    /// * `items`: The items to insert.
    async fn bulk_insert(&self, items: Vec<models::BulkInsertItem>) -> Result<()> {
        for item in items {
            match item {
                models::BulkInsertItem::Vertex(vertex) => {
                    self.create_vertex(&vertex).await?;
                }
                models::BulkInsertItem::Edge(edge_key) => {
                    self.create_edge(&edge_key).await?;
                }
                models::BulkInsertItem::VertexProperty(id, name, value) => {
                    let query = models::SpecificVertexQuery::single(id).property(name);
                    self.set_vertex_properties(query, value).await?;
                }
                models::BulkInsertItem::EdgeProperty(edge_key, name, value) => {
                    let query = models::SpecificEdgeQuery::single(edge_key).property(name);
                    self.set_edge_properties(query, value).await?;
                }
            }
        }

        Ok(())
    }

    // Enables indexing on a specified property. When indexing is enabled on a
    // property, it's possible to query on its presence and values.
    //
    // # Arguments
    // * `name`: The name of the property to index.
    async fn index_property(&self, name: models::Identifier) -> Result<()>;
}
