use nanoid::nanoid;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::core::Database;
use crate::errors::DatabaseError;
use crate::secrets::EncryptedSecret;
use crate::users::{Role, User};

use aes_gcm::{
    aead::{Aead, KeyInit, OsRng},
    Aes256Gcm, // Or `Aes128Gcm`
    Key,
};

use crate::core::LockedStatus;

#[derive(Clone)]
pub struct InMemoryDatabase {
    pub sealkey: String,
    pub key: Key<Aes256Gcm>,
    pub secrets: Arc<RwLock<HashMap<String, EncryptedSecret>>>,
    pub users: Arc<RwLock<Vec<User>>>,
    pub lock: LockedStatus,
}

impl Default for InMemoryDatabase {
    fn default() -> Self {
        Self::new()
    }
}

impl InMemoryDatabase {
    pub fn new() -> Self {
        let _sealkey = nanoid::nanoid!(20);
        println!("Your root key is: 111");
        Self {
            sealkey: "111".to_string(),
            key: Aes256Gcm::generate_key(OsRng),
            secrets: Arc::new(RwLock::new(HashMap::new())),
            users: Arc::new(RwLock::new(Vec::new())),
            lock: LockedStatus::default(),
        }
    }
}

#[async_trait::async_trait]
impl Database for InMemoryDatabase {
    async fn create_secret(&self, key: String, value: String) -> Result<(), DatabaseError> {
        let encrypted_secret = EncryptedSecret::new(self.key, key.clone(), value);

        let mut secrets = self.secrets.write().await;
        secrets.insert(key, encrypted_secret);
        Ok(())
    }

    async fn view_all_secrets(&self, user_roles: Role ) -> Result<Vec<String>, DatabaseError> {
        let store = self.secrets.read().await;

        let retrieved_keys = store.keys().cloned().collect::<Vec<String>>();
        
        Ok(retrieved_keys)
    }

    async fn view_secret(&self, _user_roles: Role, key: String) -> Result<String, DatabaseError> {
        let store = self.secrets.read().await;

        let retrieved_key = match store.get(&*key) {
            Some(res) => res,
            None => return Err(DatabaseError::KeyNotFound),
        };

        let key = Aes256Gcm::new(&self.key);
        let plaintext = key.decrypt(&retrieved_key.nonce, retrieved_key.ciphertext.as_ref())?;

        let hehe = std::str::from_utf8(&plaintext)?;

        let meme = String::from(hehe);
        Ok(meme)
    }

    async fn view_users(&self) -> Result<Vec<User>, DatabaseError> {
        let store = self.users.read().await;

        Ok(store.to_vec())
    }

    async fn get_user_from_password(&self, password: String) -> Result<User, DatabaseError> {
        let store = self.users.read().await;

        let user = match store.iter().find(|x| x.password == password) {
            Some(user) => user,
            None => return Err(DatabaseError::UserNotFound),
        };

        Ok(user.clone())
    }

    async fn view_user_by_name(&self, id: String) -> Result<User, DatabaseError> {
        let store = self.users.read().await;

        let user = store.clone().into_iter().find(|x| x.username == id);

        if user.is_none() {
            return Err(DatabaseError::UserNotFound);
        }

        Ok(user.unwrap())
    }

    async fn create_user(&self, name: String) -> Result<String, DatabaseError> {
        let mut store = self.users.write().await;

        let user = User {
            username: name.clone(),
            password: nanoid!(20),
            role: Role::Guest,
        };
        let username_is_taken = store.iter().any(|x| x.username == user.username);
        if !username_is_taken {
            store.push(user.clone());
        } else {
            return Err(DatabaseError::UserAlreadyExists);
        }

        Ok(user.password)
    }

    async fn delete_user(&self, name: String) -> Result<(), DatabaseError> {
        let mut store = self.users.write().await;

        store.retain(|user| user.username == name);

        Ok(())
    }

    async fn unlock(&self, key: String) -> Result<bool, DatabaseError> {
        if key != self.sealkey {
            return Err(DatabaseError::Forbidden);
        }

        let mut state = self.lock.is_sealed.lock().await;

        *state = false;

        Ok(true)
    }
    async fn is_locked(&self) -> bool {
        let state = self.lock.is_sealed.lock().await;

        *state
    }
    fn get_root_key(&self) -> String {
        self.sealkey.clone()
    }
}
