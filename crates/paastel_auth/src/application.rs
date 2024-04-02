// copyright (c) 2024 murilo ijanc' <mbsd@m0x.ru>

// permission to use, copy, modify, and distribute this software for any
// purpose with or without fee is hereby granted, provided that the above
// copyright notice and this permission notice appear in all copies.

// THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
// WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
// MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
// ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
// WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
// ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
// OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.

use std::sync::Arc;

use crate::{
    ArcValidateCredentialUseCase, AuthService, Credential, OutArgon2Port,
    OutKubernetesPort, UserSecret,
};

#[allow(dead_code)]
pub struct AuthApplication {
    validate_credential: ArcValidateCredentialUseCase,
}

impl AuthApplication {
    pub fn new(
        kubernetes_port: OutKubernetesPort,
        password_port: OutArgon2Port<Credential, UserSecret>,
    ) -> Self {
        Self {
            validate_credential: Arc::new(AuthService::new(
                kubernetes_port,
                password_port,
            )),
        }
    }
}
