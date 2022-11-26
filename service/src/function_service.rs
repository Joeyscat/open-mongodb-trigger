use abi::{
    function::{
        CreateRequest, CreateResponse, DeleteRequest, DeleteResponse, GetRequest, GetResponse,
        QueryRequest, UpdateRequest, UpdateResponse,
    },
    Config, Func, FunctionService,
};
use function::{DefaultFunctionManager, FunctionManager};
use tonic::{async_trait, Request, Response, Status};

use crate::{FuncService, FunctionStream, TonicReceiverStream};

impl FuncService {
    pub fn new(manager: DefaultFunctionManager) -> Self {
        Self { manager }
    }

    pub async fn from_config(config: &Config) -> Result<Self, anyhow::Error> {
        Ok(Self {
            manager: DefaultFunctionManager::from_config(&config.db).await?,
        })
    }
}

#[async_trait]
impl FunctionService for FuncService {
    /// create a function
    async fn create(
        &self,
        request: Request<CreateRequest>,
    ) -> Result<Response<CreateResponse>, Status> {
        let request = request.into_inner();
        if request.function.is_none() {
            return Err(Status::invalid_argument("missing function"));
        }

        let function = self.manager.create(request.function.unwrap()).await?;
        Ok(Response::new(CreateResponse {
            function: Some(function),
        }))
    }
    /// update the function
    async fn update(
        &self,
        request: Request<UpdateRequest>,
    ) -> Result<Response<UpdateResponse>, Status> {
        let request = request.into_inner();
        let func = match request.func {
            Some(abi::function::update_request::Func::Wasm(x)) => Func::Wasm(x),
            None => return Err(Status::invalid_argument("invalid func")),
        };
        let function = self.manager.update_func(request.id, func).await?;
        Ok(Response::new(UpdateResponse {
            function: Some(function),
        }))
    }
    /// delete the function
    async fn delete(
        &self,
        request: Request<DeleteRequest>,
    ) -> Result<Response<DeleteResponse>, Status> {
        let request = request.into_inner();
        let function = self.manager.delete(request.id).await?;
        Ok(Response::new(DeleteResponse {
            function: Some(function),
        }))
    }
    async fn get(&self, request: Request<GetRequest>) -> Result<Response<GetResponse>, Status> {
        let request = request.into_inner();
        let function = self.manager.get(request.id).await?;
        Ok(Response::new(GetResponse {
            function: Some(function),
        }))
    }
    ///Server streaming response type for the query method.
    type queryStream = FunctionStream;
    async fn query(
        &self,
        request: Request<QueryRequest>,
    ) -> Result<Response<Self::queryStream>, Status> {
        let request = request.into_inner();
        if request.query.is_none() {
            return Err(Status::invalid_argument("missing query params"));
        }
        let functions = self.manager.query(request.query.unwrap()).await;
        let stream = TonicReceiverStream::new(functions);
        Ok(Response::new(Box::pin(stream)))
    }
}
