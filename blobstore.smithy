// blobstore.smithy
// A protocol to be used to support capability providers like local blob storage, Amazon S3, Azure blob storeage, ...


// Tell the code generator how to reference symbols defined in this namespace
metadata package = [ { namespace: "org.auxiliary.interfaces.blobstore", crate: "blobstore_interface" } ]

namespace org.auxiliary.interfaces.blobstore

use org.wasmcloud.model#wasmbus
use org.wasmcloud.model#n
use org.wasmcloud.model#U32
use org.wasmcloud.model#U64

/// The Blobstore interface describes a service that can
/// store and retrieve blobs
@wasmbus(
    contractId: "auxiliary::interfaces::blobstore",
    providerReceive: true )
service Blobstore {
  version: "0.2",
  operations: [ CreateContainer, RemoveContainer, RemoveObject, ListObjects, UploadChunk, StartDownload, StartUpload, GetObjectInfo ]
}

/// The BlobReceiver interface describes
/// an actor interface for handling incoming chunks
/// forwared by a blobstore provider. Chunks may not be received in order
@wasmbus(
    contractId: "auxiliary::interfaces::blobstore",
    actorReceive: true )
service BlobReceiver {
  version: "0.1",
  operations: [ ReceiveChunk ]
}

/// ReceiveChunk - handle a file chunk
operation ReceiveChunk {
    input: FileChunk
}


/// CreateContainer(id: string): Container
operation CreateContainer {
    input: String,
    output: Container,
}

structure Container {
    @required
    @n(0)
    id: String,
}

/// RemoveContainer(id: string) : BlobstoreResult
operation RemoveContainer {
    input: String,
    output: BlobstoreResult,
}

structure BlobstoreResult {
    @required
    @n(0)
    success: Boolean,

    /// optional error message
    @n(1)
    error: String,
}

/// RemoveObject(id: string, container_id: string): BlobstoreResult
operation RemoveObject {
    input: RemoveObjectRequest,
    output: BlobstoreResult,
}

structure RemoveObjectRequest {
    @required
    @n(0)
    id: String,

    @required
    @n(1)
    Container_id: String,
}

/// ListObjects(container_id: string): BlobList
operation ListObjects {
    input: String,
    output: BlobList,
}

structure FileBlob {
    @required
    @n(0)
    id: String,

    @required
    @n(1)
    container: Container,

    @required
    @n(2)
    byteSize: U64,
}

list BlobList {
    member: FileBlob,
}

/// UploadChunk(chunk: FileChunk): BlobstoreResult
operation UploadChunk {
    input: FileChunk,
    output: BlobstoreResult,
}

structure FileChunk {
    @required
    @n(0)
    sequenceNo: U64,

    @required
    @n(1)
    container: Container,

    @required
    @n(2)
    id: String,

    @required
    @n(3)
    totalBytes: U64,

    @required
    @n(4)
    chunkSize: U64,

    @n(5)
    context: String,

    @required
    @n(6)
    chunkBytes: Blob,
}

/// StartDownload(blob_id: string, container_id: string, chunk_size: u64, context: string?): BlobstoreResult
operation StartDownload {
    input: StartDownloadRequest,
    output: BlobstoreResult,
}

structure StartDownloadRequest {
    @required
    @n(0)
    blob_id: String,

    @required
    @n(1)
    container_id: String,

    @required
    @n(2)
    chunk_size: U64,

    @n(3)
    context: String,

    @n(4)
    start_idx: U64,
}

/// StartUpload(chunk: FileChunk): BlobstoreResult
operation StartUpload {
    input: FileChunk,
    output: BlobstoreResult,
}

/// GetObjectInfo(blob_id: string, container_id: string): Blob
operation GetObjectInfo {
    input: GetObjectInfoRequest,
    output: FileBlob,
}

structure GetObjectInfoRequest {
    @required
    @n(0)
    blob_id: String,

    @required
    @n(1)
    container_id: String,
}
