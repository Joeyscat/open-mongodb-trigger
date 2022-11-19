use std::pin::Pin;
use std::task::Poll;

use crate::{TonicReceiverStream, TriggerStream, TrigrService};
use abi::trigger::{
    CreateRequest, CreateResponse, DeleteRequest, DeleteResponse, DisableRequest, DisableResponse,
    EnableRequest, EnableResponse, GetRequest, GetResponse, QueryRequest, UpdateRequest,
    UpdateResponse,
};
use abi::{Config, TriggerService};
use function::{DefaultFunctionManager, FunctionManager};
use futures::Stream;
use tonic::{async_trait, Request, Response, Status};
use trigger::{DefaultTriggerManager, TriggerManager};

impl TrigrService {
    pub async fn from_config(config: &Config) -> Result<Self, anyhow::Error> {
        Ok(Self {
            manager: DefaultTriggerManager::from_config(&config.db).await?,
            function_manager: DefaultFunctionManager::from_config(&config.db).await?,
        })
    }
}

#[async_trait]
impl TriggerService for TrigrService {
    /// create a trigger
    async fn create(
        &self,
        request: Request<CreateRequest>,
    ) -> Result<Response<CreateResponse>, Status> {
        let trigger = request
            .into_inner()
            .trigger
            .ok_or_else(|| Status::invalid_argument("missing trigger"))?;

        self.function_manager
            .get(trigger.function_id.clone())
            .await?;

        let new_trigger = self.manager.create(trigger).await?;
        Ok(Response::new(CreateResponse {
            trigger: Some(new_trigger),
        }))
    }
    /// update the trigger
    async fn update(
        &self,
        request: Request<UpdateRequest>,
    ) -> Result<Response<UpdateResponse>, Status> {
        let request = request.into_inner();
        let trigger = self
            .manager
            .change_function(request.id, request.function_id)
            .await?;
        Ok(Response::new(UpdateResponse {
            trigger: Some(trigger),
        }))
    }
    /// enable the trigger
    async fn enable(
        &self,
        request: Request<EnableRequest>,
    ) -> Result<Response<EnableResponse>, Status> {
        let request = request.into_inner();
        let trigger = self.manager.enable(request.id).await?;
        Ok(Response::new(EnableResponse {
            trigger: Some(trigger),
        }))
    }
    /// disable the trigger
    async fn disable(
        &self,
        request: Request<DisableRequest>,
    ) -> Result<Response<DisableResponse>, Status> {
        let request = request.into_inner();
        let trigger = self.manager.disable(request.id).await?;
        Ok(Response::new(DisableResponse {
            trigger: Some(trigger),
        }))
    }
    /// delete the trigger
    async fn delete(
        &self,
        request: Request<DeleteRequest>,
    ) -> Result<Response<DeleteResponse>, Status> {
        let request = request.into_inner();
        let trigger = self.manager.delete(request.id).await?;
        Ok(Response::new(DeleteResponse {
            trigger: Some(trigger),
        }))
    }
    /// get the trigger
    async fn get(&self, request: Request<GetRequest>) -> Result<Response<GetResponse>, Status> {
        let request = request.into_inner();
        let trigger = self.manager.get(request.id).await?;
        Ok(Response::new(GetResponse {
            trigger: Some(trigger),
        }))
    }
    ///Server streaming response type for the query method.
    type queryStream = TriggerStream;
    async fn query(
        &self,
        request: Request<QueryRequest>,
    ) -> Result<Response<Self::queryStream>, Status> {
        let request = request.into_inner();
        if request.query.is_none() {
            return Err(Status::invalid_argument("missing query params"));
        }
        let triggers = self.manager.query(request.query.unwrap()).await;
        let stream = TonicReceiverStream::new(triggers);
        Ok(Response::new(Box::pin(stream)))
    }
}

impl<T> Stream for TonicReceiverStream<T> {
    type Item = Result<T, Status>;

    fn poll_next(
        mut self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        match self.inner.poll_recv(cx) {
            Poll::Ready(Some(Ok(item))) => Poll::Ready(Some(Ok(item))),
            Poll::Ready(Some(Err(e))) => Poll::Ready(Some(Err(e.into()))),
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Pending => Poll::Pending,
        }
    }
}
