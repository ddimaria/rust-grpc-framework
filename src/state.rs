use crate::database::{pool, DatabasePool};
use crate::error::Result;
use crate::message::grpc::grpc_server::Grpc;
use std::{collections::HashMap, sync::Arc};

pub type Data<'a, T> = HashMap<&'a str, T>;

pub struct State<'a, T> {
    pub data: Arc<Data<'a, T>>,
    pub database_pool: DatabasePool,
}

impl<'a, T> State<'a, T> {
    pub async fn new() -> Result<State<'a, T>> {
        Ok(Self {
            data: Arc::new(Data::new()),
            database_pool: pool().await?,
        })
    }
}

impl<T: 'static + Send + Sync> Grpc for State<'static, T> {}
