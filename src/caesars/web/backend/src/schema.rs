use async_graphql::{EmptyMutation, EmptySubscription, MergedObject, Schema};

use crate::domain::{
    health::resolver::HealthQuery, meta::resolver::MetaQuery, rot::resolver::RotQuery,
};

#[derive(MergedObject, Default)]
pub struct Query(MetaQuery, HealthQuery, RotQuery);

pub type AppSchema = Schema<Query, EmptyMutation, EmptySubscription>;
