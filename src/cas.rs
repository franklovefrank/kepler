use super::codec::SupportedCodecs;
use libipld::cid::Cid;

#[rocket::async_trait]
pub trait ContentAddressedStorage: Send + Sync {
    type Error;
    async fn put(&self, content: &[u8], codec: SupportedCodecs) -> Result<Cid, Self::Error>;
    async fn get(&self, address: &Cid) -> Result<Option<Vec<u8>>, Self::Error>;
    async fn delete(&self, address: &Cid) -> Result<(), Self::Error>;
    async fn list(&self) -> Result<Vec<Cid>, Self::Error>;
}
