// Copyright (c) 2024 Murilo Ijanc' <mbsd@m0x.ru>

// Permission to use, copy, modify, and distribute this software for any
// purpose with or without fee is hereby granted, provided that the above
// copyright notice and this permission notice appear in all copies.

// THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
// WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
// MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
// ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
// WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
// ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
// OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.

// use async_trait::async_trait;
use aws_sdk_s3::{
    error::SdkError, operation::put_object::PutObjectError,
    primitives::ByteStream, Client,
};
use aws_smithy_runtime_api::client::orchestrator::HttpResponse;
use derive_new::new;

#[derive(Debug, thiserror::Error)]
pub enum S3Error {}

pub struct S3Adapter {
    bucket: String,
    object: S3Object,
}

impl S3Adapter {
    pub fn new(client: Client, bucket: impl Into<String>) -> Self {
        Self {
            bucket: bucket.into(),
            object: S3Object::new(client),
        }
    }

    pub fn bucket(&self) -> &str {
        self.bucket.as_str()
    }

    pub fn bucket_mut(&mut self) -> &mut str {
        self.bucket.as_mut_str()
    }
}

#[derive(new)]
pub struct S3Object {
    client: Client,
}

impl S3Object {
    async fn save(
        &self,
        bucket: &str,
        key: &str,
        body: Vec<u8>,
    ) -> Result<(), SdkError<PutObjectError, HttpResponse>> {
        self.client
            .put_object()
            .bucket(bucket)
            .key(key)
            .body(ByteStream::from(body))
            .send()
            .await?;
        Ok(())
    }
}
