// Copyright 2021 Contributors to the Parsec project.
// SPDX-License-Identifier: Apache-2.0
//! Decrypting data

use crate::context::Function;
use crate::error::{Result, Rv};
use crate::mechanism::Mechanism;
use crate::object::ObjectHandle;
use crate::session::Session;
use cryptoki_sys::*;
use std::convert::TryInto;

impl Session {
    /// Single-part decryption operation
    pub fn decrypt(
        &self,
        mut mechanism: Mechanism,
        key: ObjectHandle,
        encrypted_data: &[u8],
    ) -> Result<Vec<u8>> {
        let mut mechanism: CK_MECHANISM = (&mut mechanism).into();
        let mut data_len = 0;

        unsafe {
            Rv::from(get_pkcs11!(self.client(), C_DecryptInit)(
                self.handle(),
                &mut mechanism as CK_MECHANISM_PTR,
                key.handle(),
            ))
            .into_result(Function::DecryptInit)?;
        }

        // Get the output buffer length
        unsafe {
            Rv::from(get_pkcs11!(self.client(), C_Decrypt)(
                self.handle(),
                // C_Decrypt should not modify this buffer
                encrypted_data.as_ptr() as *mut u8,
                encrypted_data.len().try_into()?,
                std::ptr::null_mut(),
                &mut data_len,
            ))
            .into_result(Function::Decrypt)?;
        }

        let mut data = vec![0; data_len.try_into()?];

        unsafe {
            Rv::from(get_pkcs11!(self.client(), C_Decrypt)(
                self.handle(),
                encrypted_data.as_ptr() as *mut u8,
                encrypted_data.len().try_into()?,
                data.as_mut_ptr(),
                &mut data_len,
            ))
            .into_result(Function::Decrypt)?;
        }

        data.resize(data_len.try_into()?, 0);

        Ok(data)
    }
}
