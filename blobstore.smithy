// blobstore.smithy
// A simple service that calculates the factorial of a whole number


// Tell the code generator how to reference symbols defined in this namespace
metadata package = [ { namespace: "org.auxiliary.interfaces.blobstore", crate: "blobstore_interface" } ]

namespace org.auxiliary.interfaces.blobstore

use org.wasmcloud.model#wasmbus
use org.wasmcloud.model#U32
use org.wasmcloud.model#U64

/// The Blobstore service has a single method, calculate, which
/// calculates the factorial of its whole number parameter.
@wasmbus(
    contractId: "auxiliary::interfaces::blobstore",
    actorReceive: true,
    providerReceive: true )
service Blobstore {
  version: "0.1",
  operations: [ CreateContainer, RemoveContainer, RemoveObject, ListObjects, UploadChunk, StartDownload, StartUpload, GetObjectInfo ]
}

/// CreateContainer(id: string): Container
operation CreateContainer {
    input: String,
    output: Container,
}

structure Container {
    @required
    id: String,
}

/// RemoveContainer(id: string) : BlobstoreResult
operation RemoveContainer {
    input: String,
    output: BlobstoreResult,
}

structure BlobstoreResult {
    @required
    success: Boolean,

    /// optional error message
    error: String,
}

/// RemoveObject(id: string, container_id: string): BlobstoreResult
operation RemoveObject {
    input: RemoveObjectRequest,
    output: BlobstoreResult,
}

structure RemoveObjectRequest {
    @required
    id: String,

    @required
    Container_id: String,
}

/// ListObjects(container_id: string): BlobList
operation ListObjects {
    input: String,
    output: BlobList,
}

structure FileBlob {
    @required
    id: String,

    @required
    container: Container,

    @required
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
    sequenceNo: U64,

    @required
    container: Container,

    @required
    id: String,

    @required
    totalBytes: U64,

    @required
    chunkSize: U64,

    context: String,

    @required
    chunkBytes: Blob,
}

/// StartDownload(blob_id: string, container_id: string, chunk_size: u64, context: string?): BlobstoreResult
operation StartDownload {
    input: StartDownloadRequest,
    output: BlobstoreResult,
}

structure StartDownloadRequest {
    @required
    blob_id: String,

    @required
    container_id: String,

    @required
    chunk_size: U64,

    context: String,
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
    blob_id: String,

    @required
    Container_id: String,
}
