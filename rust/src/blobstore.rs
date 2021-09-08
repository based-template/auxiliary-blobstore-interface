// This file is generated automatically using wasmcloud-weld and smithy model definitions
//

#![allow(clippy::ptr_arg)]
#[allow(unused_imports)]
use async_trait::async_trait;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::{borrow::Cow, string::ToString};
#[allow(unused_imports)]
use wasmbus_rpc::{
    deserialize, serialize, Context, Message, MessageDispatch, RpcError, RpcResult, SendOpts,
    Transport,
};

pub const SMITHY_VERSION: &str = "1.0";

pub type BlobList = Vec<FileBlob>;

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct BlobstoreResult {
    /// optional error message
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(default)]
    pub success: bool,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct Container {
    #[serde(default)]
    pub id: String,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct FileBlob {
    #[serde(rename = "byteSize")]
    pub byte_size: u64,
    pub container: Container,
    #[serde(default)]
    pub id: String,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct FileChunk {
    #[serde(rename = "chunkBytes")]
    #[serde(with = "serde_bytes")]
    #[serde(default)]
    pub chunk_bytes: Vec<u8>,
    #[serde(rename = "chunkSize")]
    pub chunk_size: u64,
    pub container: Container,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,
    #[serde(default)]
    pub id: String,
    #[serde(rename = "sequenceNo")]
    pub sequence_no: u64,
    #[serde(rename = "totalBytes")]
    pub total_bytes: u64,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct GetObjectInfoRequest {
    #[serde(rename = "Container_id")]
    #[serde(default)]
    pub container_id: String,
    #[serde(default)]
    pub blob_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct RemoveObjectRequest {
    #[serde(rename = "Container_id")]
    #[serde(default)]
    pub container_id: String,
    #[serde(default)]
    pub id: String,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct StartDownloadRequest {
    #[serde(default)]
    pub blob_id: String,
    pub chunk_size: u64,
    #[serde(default)]
    pub container_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,
}

/// The BlobReceiver interface describes
/// an actor interface for handling incoming chunks
/// forwared by a blobstore provider. Chunks may not be received in order
/// wasmbus.contractId: auxiliary::interfaces::blobstore
/// wasmbus.actorReceive
#[async_trait]
pub trait BlobReceiver {
    /// returns the capability contract id for this interface
    fn contract_id() -> &'static str {
        "auxiliary::interfaces::blobstore"
    }
    /// ReceiveChunk - handle a file chunk
    async fn receive_chunk(&self, ctx: &Context, arg: &FileChunk) -> RpcResult<()>;
}

/// BlobReceiverReceiver receives messages defined in the BlobReceiver service trait
/// The BlobReceiver interface describes
/// an actor interface for handling incoming chunks
/// forwared by a blobstore provider. Chunks may not be received in order
#[doc(hidden)]
#[async_trait]
pub trait BlobReceiverReceiver: MessageDispatch + BlobReceiver {
    async fn dispatch(&self, ctx: &Context, message: &Message<'_>) -> RpcResult<Message<'_>> {
        match message.method {
            "ReceiveChunk" => {
                let value: FileChunk = deserialize(message.arg.as_ref())
                    .map_err(|e| RpcError::Deser(format!("message '{}': {}", message.method, e)))?;
                let resp = BlobReceiver::receive_chunk(self, ctx, &value).await?;
                let buf = Cow::Owned(serialize(&resp)?);
                Ok(Message {
                    method: "BlobReceiver.ReceiveChunk",
                    arg: buf,
                })
            }
            _ => Err(RpcError::MethodNotHandled(format!(
                "BlobReceiver::{}",
                message.method
            ))),
        }
    }
}

/// BlobReceiverSender sends messages to a BlobReceiver service
/// The BlobReceiver interface describes
/// an actor interface for handling incoming chunks
/// forwared by a blobstore provider. Chunks may not be received in order
/// client for sending BlobReceiver messages
#[derive(Debug)]
pub struct BlobReceiverSender<T: Transport> {
    transport: T,
}

impl<T: Transport> BlobReceiverSender<T> {
    /// Constructs a BlobReceiverSender with the specified transport
    pub fn via(transport: T) -> Self {
        Self { transport }
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl<'send> BlobReceiverSender<wasmbus_rpc::provider::ProviderTransport<'send>> {
    /// Constructs a Sender using an actor's LinkDefinition,
    /// Uses the provider's HostBridge for rpc
    pub fn for_actor(ld: &'send wasmbus_rpc::core::LinkDefinition) -> Self {
        Self {
            transport: wasmbus_rpc::provider::ProviderTransport::new(ld, None),
        }
    }
}
#[cfg(target_arch = "wasm32")]
impl BlobReceiverSender<wasmbus_rpc::actor::prelude::WasmHost> {
    /// Constructs a client for actor-to-actor messaging
    /// using the recipient actor's public key
    pub fn to_actor(actor_id: &str) -> Self {
        let transport =
            wasmbus_rpc::actor::prelude::WasmHost::to_actor(actor_id.to_string()).unwrap();
        Self { transport }
    }
}
#[async_trait]
impl<T: Transport + std::marker::Sync + std::marker::Send> BlobReceiver for BlobReceiverSender<T> {
    #[allow(unused)]
    /// ReceiveChunk - handle a file chunk
    async fn receive_chunk(&self, ctx: &Context, arg: &FileChunk) -> RpcResult<()> {
        let arg = serialize(arg)?;
        let resp = self
            .transport
            .send(
                ctx,
                Message {
                    method: "BlobReceiver.ReceiveChunk",
                    arg: Cow::Borrowed(&arg),
                },
                None,
            )
            .await?;
        Ok(())
    }
}

/// The Blobstore interface describes a service that can
/// store and retrieve blobs
/// wasmbus.contractId: auxiliary::interfaces::blobstore
/// wasmbus.providerReceive
#[async_trait]
pub trait Blobstore {
    /// returns the capability contract id for this interface
    fn contract_id() -> &'static str {
        "auxiliary::interfaces::blobstore"
    }
    /// CreateContainer(id: string): Container
    async fn create_container<TS: ToString + ?Sized + std::marker::Sync>(
        &self,
        ctx: &Context,
        arg: &TS,
    ) -> RpcResult<Container>;
    /// RemoveContainer(id: string) : BlobstoreResult
    async fn remove_container<TS: ToString + ?Sized + std::marker::Sync>(
        &self,
        ctx: &Context,
        arg: &TS,
    ) -> RpcResult<BlobstoreResult>;
    /// RemoveObject(id: string, container_id: string): BlobstoreResult
    async fn remove_object(
        &self,
        ctx: &Context,
        arg: &RemoveObjectRequest,
    ) -> RpcResult<BlobstoreResult>;
    /// ListObjects(container_id: string): BlobList
    async fn list_objects<TS: ToString + ?Sized + std::marker::Sync>(
        &self,
        ctx: &Context,
        arg: &TS,
    ) -> RpcResult<BlobList>;
    /// UploadChunk(chunk: FileChunk): BlobstoreResult
    async fn upload_chunk(&self, ctx: &Context, arg: &FileChunk) -> RpcResult<BlobstoreResult>;
    /// StartDownload(blob_id: string, container_id: string, chunk_size: u64, context: string?): BlobstoreResult
    async fn start_download(
        &self,
        ctx: &Context,
        arg: &StartDownloadRequest,
    ) -> RpcResult<BlobstoreResult>;
    /// StartUpload(chunk: FileChunk): BlobstoreResult
    async fn start_upload(&self, ctx: &Context, arg: &FileChunk) -> RpcResult<BlobstoreResult>;
    /// GetObjectInfo(blob_id: string, container_id: string): Blob
    async fn get_object_info(
        &self,
        ctx: &Context,
        arg: &GetObjectInfoRequest,
    ) -> RpcResult<FileBlob>;
}

/// BlobstoreReceiver receives messages defined in the Blobstore service trait
/// The Blobstore interface describes a service that can
/// store and retrieve blobs
#[doc(hidden)]
#[async_trait]
pub trait BlobstoreReceiver: MessageDispatch + Blobstore {
    async fn dispatch(&self, ctx: &Context, message: &Message<'_>) -> RpcResult<Message<'_>> {
        match message.method {
            "CreateContainer" => {
                let value: String = deserialize(message.arg.as_ref())
                    .map_err(|e| RpcError::Deser(format!("message '{}': {}", message.method, e)))?;
                let resp = Blobstore::create_container(self, ctx, &value).await?;
                let buf = Cow::Owned(serialize(&resp)?);
                Ok(Message {
                    method: "Blobstore.CreateContainer",
                    arg: buf,
                })
            }
            "RemoveContainer" => {
                let value: String = deserialize(message.arg.as_ref())
                    .map_err(|e| RpcError::Deser(format!("message '{}': {}", message.method, e)))?;
                let resp = Blobstore::remove_container(self, ctx, &value).await?;
                let buf = Cow::Owned(serialize(&resp)?);
                Ok(Message {
                    method: "Blobstore.RemoveContainer",
                    arg: buf,
                })
            }
            "RemoveObject" => {
                let value: RemoveObjectRequest = deserialize(message.arg.as_ref())
                    .map_err(|e| RpcError::Deser(format!("message '{}': {}", message.method, e)))?;
                let resp = Blobstore::remove_object(self, ctx, &value).await?;
                let buf = Cow::Owned(serialize(&resp)?);
                Ok(Message {
                    method: "Blobstore.RemoveObject",
                    arg: buf,
                })
            }
            "ListObjects" => {
                let value: String = deserialize(message.arg.as_ref())
                    .map_err(|e| RpcError::Deser(format!("message '{}': {}", message.method, e)))?;
                let resp = Blobstore::list_objects(self, ctx, &value).await?;
                let buf = Cow::Owned(serialize(&resp)?);
                Ok(Message {
                    method: "Blobstore.ListObjects",
                    arg: buf,
                })
            }
            "UploadChunk" => {
                let value: FileChunk = deserialize(message.arg.as_ref())
                    .map_err(|e| RpcError::Deser(format!("message '{}': {}", message.method, e)))?;
                let resp = Blobstore::upload_chunk(self, ctx, &value).await?;
                let buf = Cow::Owned(serialize(&resp)?);
                Ok(Message {
                    method: "Blobstore.UploadChunk",
                    arg: buf,
                })
            }
            "StartDownload" => {
                let value: StartDownloadRequest = deserialize(message.arg.as_ref())
                    .map_err(|e| RpcError::Deser(format!("message '{}': {}", message.method, e)))?;
                let resp = Blobstore::start_download(self, ctx, &value).await?;
                let buf = Cow::Owned(serialize(&resp)?);
                Ok(Message {
                    method: "Blobstore.StartDownload",
                    arg: buf,
                })
            }
            "StartUpload" => {
                let value: FileChunk = deserialize(message.arg.as_ref())
                    .map_err(|e| RpcError::Deser(format!("message '{}': {}", message.method, e)))?;
                let resp = Blobstore::start_upload(self, ctx, &value).await?;
                let buf = Cow::Owned(serialize(&resp)?);
                Ok(Message {
                    method: "Blobstore.StartUpload",
                    arg: buf,
                })
            }
            "GetObjectInfo" => {
                let value: GetObjectInfoRequest = deserialize(message.arg.as_ref())
                    .map_err(|e| RpcError::Deser(format!("message '{}': {}", message.method, e)))?;
                let resp = Blobstore::get_object_info(self, ctx, &value).await?;
                let buf = Cow::Owned(serialize(&resp)?);
                Ok(Message {
                    method: "Blobstore.GetObjectInfo",
                    arg: buf,
                })
            }
            _ => Err(RpcError::MethodNotHandled(format!(
                "Blobstore::{}",
                message.method
            ))),
        }
    }
}

/// BlobstoreSender sends messages to a Blobstore service
/// The Blobstore interface describes a service that can
/// store and retrieve blobs
/// client for sending Blobstore messages
#[derive(Debug)]
pub struct BlobstoreSender<T: Transport> {
    transport: T,
}

impl<T: Transport> BlobstoreSender<T> {
    /// Constructs a BlobstoreSender with the specified transport
    pub fn via(transport: T) -> Self {
        Self { transport }
    }
}

#[cfg(target_arch = "wasm32")]
impl BlobstoreSender<wasmbus_rpc::actor::prelude::WasmHost> {
    /// Constructs a client for sending to a Blobstore provider
    /// implementing the 'auxiliary::interfaces::blobstore' capability contract, with the "default" link
    pub fn new() -> Self {
        let transport = wasmbus_rpc::actor::prelude::WasmHost::to_provider(
            "auxiliary::interfaces::blobstore",
            "default",
        )
        .unwrap();
        Self { transport }
    }

    /// Constructs a client for sending to a Blobstore provider
    /// implementing the 'auxiliary::interfaces::blobstore' capability contract, with the specified link name
    pub fn new_with_link(link_name: &str) -> wasmbus_rpc::RpcResult<Self> {
        let transport = wasmbus_rpc::actor::prelude::WasmHost::to_provider(
            "auxiliary::interfaces::blobstore",
            link_name,
        )?;
        Ok(Self { transport })
    }
}
#[async_trait]
impl<T: Transport + std::marker::Sync + std::marker::Send> Blobstore for BlobstoreSender<T> {
    #[allow(unused)]
    /// CreateContainer(id: string): Container
    async fn create_container<TS: ToString + ?Sized + std::marker::Sync>(
        &self,
        ctx: &Context,
        arg: &TS,
    ) -> RpcResult<Container> {
        let arg = serialize(&arg.to_string())?;
        let resp = self
            .transport
            .send(
                ctx,
                Message {
                    method: "Blobstore.CreateContainer",
                    arg: Cow::Borrowed(&arg),
                },
                None,
            )
            .await?;
        let value = deserialize(&resp)
            .map_err(|e| RpcError::Deser(format!("response to {}: {}", "CreateContainer", e)))?;
        Ok(value)
    }
    #[allow(unused)]
    /// RemoveContainer(id: string) : BlobstoreResult
    async fn remove_container<TS: ToString + ?Sized + std::marker::Sync>(
        &self,
        ctx: &Context,
        arg: &TS,
    ) -> RpcResult<BlobstoreResult> {
        let arg = serialize(&arg.to_string())?;
        let resp = self
            .transport
            .send(
                ctx,
                Message {
                    method: "Blobstore.RemoveContainer",
                    arg: Cow::Borrowed(&arg),
                },
                None,
            )
            .await?;
        let value = deserialize(&resp)
            .map_err(|e| RpcError::Deser(format!("response to {}: {}", "RemoveContainer", e)))?;
        Ok(value)
    }
    #[allow(unused)]
    /// RemoveObject(id: string, container_id: string): BlobstoreResult
    async fn remove_object(
        &self,
        ctx: &Context,
        arg: &RemoveObjectRequest,
    ) -> RpcResult<BlobstoreResult> {
        let arg = serialize(arg)?;
        let resp = self
            .transport
            .send(
                ctx,
                Message {
                    method: "Blobstore.RemoveObject",
                    arg: Cow::Borrowed(&arg),
                },
                None,
            )
            .await?;
        let value = deserialize(&resp)
            .map_err(|e| RpcError::Deser(format!("response to {}: {}", "RemoveObject", e)))?;
        Ok(value)
    }
    #[allow(unused)]
    /// ListObjects(container_id: string): BlobList
    async fn list_objects<TS: ToString + ?Sized + std::marker::Sync>(
        &self,
        ctx: &Context,
        arg: &TS,
    ) -> RpcResult<BlobList> {
        let arg = serialize(&arg.to_string())?;
        let resp = self
            .transport
            .send(
                ctx,
                Message {
                    method: "Blobstore.ListObjects",
                    arg: Cow::Borrowed(&arg),
                },
                None,
            )
            .await?;
        let value = deserialize(&resp)
            .map_err(|e| RpcError::Deser(format!("response to {}: {}", "ListObjects", e)))?;
        Ok(value)
    }
    #[allow(unused)]
    /// UploadChunk(chunk: FileChunk): BlobstoreResult
    async fn upload_chunk(&self, ctx: &Context, arg: &FileChunk) -> RpcResult<BlobstoreResult> {
        let arg = serialize(arg)?;
        let resp = self
            .transport
            .send(
                ctx,
                Message {
                    method: "Blobstore.UploadChunk",
                    arg: Cow::Borrowed(&arg),
                },
                None,
            )
            .await?;
        let value = deserialize(&resp)
            .map_err(|e| RpcError::Deser(format!("response to {}: {}", "UploadChunk", e)))?;
        Ok(value)
    }
    #[allow(unused)]
    /// StartDownload(blob_id: string, container_id: string, chunk_size: u64, context: string?): BlobstoreResult
    async fn start_download(
        &self,
        ctx: &Context,
        arg: &StartDownloadRequest,
    ) -> RpcResult<BlobstoreResult> {
        let arg = serialize(arg)?;
        let resp = self
            .transport
            .send(
                ctx,
                Message {
                    method: "Blobstore.StartDownload",
                    arg: Cow::Borrowed(&arg),
                },
                None,
            )
            .await?;
        let value = deserialize(&resp)
            .map_err(|e| RpcError::Deser(format!("response to {}: {}", "StartDownload", e)))?;
        Ok(value)
    }
    #[allow(unused)]
    /// StartUpload(chunk: FileChunk): BlobstoreResult
    async fn start_upload(&self, ctx: &Context, arg: &FileChunk) -> RpcResult<BlobstoreResult> {
        let arg = serialize(arg)?;
        let resp = self
            .transport
            .send(
                ctx,
                Message {
                    method: "Blobstore.StartUpload",
                    arg: Cow::Borrowed(&arg),
                },
                None,
            )
            .await?;
        let value = deserialize(&resp)
            .map_err(|e| RpcError::Deser(format!("response to {}: {}", "StartUpload", e)))?;
        Ok(value)
    }
    #[allow(unused)]
    /// GetObjectInfo(blob_id: string, container_id: string): Blob
    async fn get_object_info(
        &self,
        ctx: &Context,
        arg: &GetObjectInfoRequest,
    ) -> RpcResult<FileBlob> {
        let arg = serialize(arg)?;
        let resp = self
            .transport
            .send(
                ctx,
                Message {
                    method: "Blobstore.GetObjectInfo",
                    arg: Cow::Borrowed(&arg),
                },
                None,
            )
            .await?;
        let value = deserialize(&resp)
            .map_err(|e| RpcError::Deser(format!("response to {}: {}", "GetObjectInfo", e)))?;
        Ok(value)
    }
}
