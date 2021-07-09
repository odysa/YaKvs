use crate::common::{DataBase, KvsEngine};
use crate::database::LogDataBase;
use crate::error::{Error, Result};
use std::{collections::HashMap, path::Path};

/// Used to store key and value
/// # Example
///
/// ```
/// use kvs::kvs_store::KvStore;
/// use std::path::Path;
/// use kvs::common::KvsEngine;
///
/// let path = Path::new("");
/// let mut kvs = KvStore::open(path).unwrap();
/// kvs.set("key".to_string(), "value".to_string()).unwrap();
/// assert_eq!(kvs.get("key".to_string()).unwrap(),Some("value".to_string()));
/// ```
pub struct KvStore<T: DataBase> {
    map: HashMap<String, String>,
    maintainer: T,
}

impl KvStore<LogDataBase> {
    pub fn open(path: &Path) -> Result<Self> {
        let path = path.join("kvs.db");
        let maintainer = LogDataBase::new(path)?;
        Ok(KvStore {
            maintainer,
            map: HashMap::new(),
        })
    }
}

impl<T: DataBase> KvsEngine for KvStore<T> {
    /// new a key-value store
    /// ```
    /// ```

    /// set the value of a given key
    /// ```
    /// ```
    fn set(&mut self, key: String, value: String) -> Result<()> {
        self.map.insert(key.to_owned(), value.to_owned());
        self.maintainer.set(key, value)?;
        Ok(())
    }
    /// set the value of a given key
    /// ```
    /// ```
    fn get(&mut self, key: String) -> Result<Option<String>> {
        if let Some(v) = self.map.get(&key) {
            return Ok(Some(v.to_owned()));
        }
        match self.maintainer.get(key) {
            Ok(res) => {
                if let Some(value) = res {
                    Ok(Some(value))
                } else {
                    Ok(None)
                }
            }
            Err(e) => Err(e),
        }
    }
    /// remove a given key in store
    /// ```
    /// ```
    fn remove(&mut self, key: String) -> Result<String> {
        self.map.remove(&key).unwrap_or_default();

        match self.maintainer.get(key.to_owned()) {
            Ok(res) => match res {
                None => Err(Error::key_not_found(format!(
                    "key {} you want to remove does not exist",
                    key
                ))),
                Some(value) => {
                    self.maintainer.remove(key)?;
                    Ok(value)
                }
            },
            Err(e) => Err(e),
        }
    }
}
