//! Graphql related stuffs

use crate::graphql::Graphql;
use async_graphql::{ObjectType, Schema, SubscriptionType, http::GraphiQLSource};
use async_graphql_axum::{GraphQL, GraphQLSubscription};
use axum::{
    Router,
    response::{self, IntoResponse},
    routing::get,
};
use std::any::Any;
use tokio::net::TcpListener;
use tower::ServiceBuilder;

impl Graphql {
    /// Start the GraphQL service
    pub async fn start<Query, Mutation, Subscription, Data>(
        &self,
        query: Query,
        mutation: Mutation,
        subscription: Subscription,
        data: Data,
    ) -> anyhow::Result<()>
    where
        Query: ObjectType + 'static,
        Mutation: ObjectType + 'static,
        Subscription: SubscriptionType + 'static,
        Data: Send + Sync + Any,
    {
        let schema = Schema::build(query, mutation, subscription)
            .data(data)
            .finish();

        // Configure CORS using the provided configuration
        let cors = self.cors.layer();
        let app = Router::new()
            .route(
                "/",
                get(graphiql).post_service(GraphQL::new(schema.clone())),
            )
            .route_service("/ws", GraphQLSubscription::new(schema))
            .layer(ServiceBuilder::new().layer(cors).into_inner());

        axum::serve(TcpListener::bind(self.endpoint).await?, app).await?;
        Ok(())
    }
}

async fn graphiql() -> impl IntoResponse {
    response::Html(
        GraphiQLSource::build()
            .endpoint("/")
            .subscription_endpoint("/ws")
            .finish(),
    )
}
