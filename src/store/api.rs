// This Source Code Form is subject to the terms of
// the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You
// can obtain one at http://mozilla.org/MPL/2.0/.

use byteorder;
use identity::Identity;
use proteus::{DecodeError, EncodeError};
use proteus::keys::{IdentityKeyPair, PreKey, PreKeyId};
use proteus::session::Session;
use std::error::Error;
use std::fmt;
use std::io;

// API //////////////////////////////////////////////////////////////////////

pub type StorageResult<T> = Result<T, StorageError>;

pub trait Store {
    fn load_session<'r>(&self, li: &'r IdentityKeyPair, id: &str) -> StorageResult<Option<Session<'r>>>;
    fn save_session(&self, id: &str, s: &Session) -> StorageResult<()>;
    fn delete_session(&self, id: &str) -> StorageResult<()>;
    fn load_identity<'s>(&self) -> StorageResult<Option<Identity<'s>>>;
    fn save_identity(&self, id: &Identity) -> StorageResult<()>;
    fn load_prekey(&self, id: PreKeyId) -> StorageResult<Option<PreKey>>;
    fn add_prekey(&self, key: &PreKey) -> StorageResult<()>;
    fn delete_prekey(&self, id: PreKeyId) -> StorageResult<()>;
}

// Errors ///////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub struct StorageError {
    pub cause: Box<Error>
}

impl fmt::Display for StorageError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "StorageError: {}", self.cause)
    }
}

impl Error for StorageError {
    fn description(&self) -> &str {
        "StorageError"
    }

    fn cause(&self) -> Option<&Error> {
        Some(&*self.cause)
    }
}

impl From<io::Error> for StorageError {
    fn from(e: io::Error) -> StorageError {
        StorageError { cause: Box::new(e) }
    }
}

impl From<DecodeError> for StorageError {
    fn from(e: DecodeError) -> StorageError {
        StorageError { cause: Box::new(e) }
    }
}

impl From<EncodeError> for StorageError {
    fn from(e: EncodeError) -> StorageError {
        StorageError { cause: Box::new(e) }
    }
}

impl From<byteorder::Error> for StorageError {
    fn from(e: byteorder::Error) -> StorageError {
        StorageError { cause: Box::new(e) }
    }
}
