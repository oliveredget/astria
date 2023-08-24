use anyhow::Result;
use astria_proto::native::sequencer::v1alpha1::Address;
use async_trait::async_trait;
use penumbra_storage::{
    StateRead,
    StateWrite,
};

#[async_trait]
pub(crate) trait ActionHandler {
    fn check_stateless(&self) -> Result<()> {
        Ok(())
    }
    async fn check_stateful<S: StateRead + 'static>(
        &self,
        _state: &S,
        _from: Address,
    ) -> Result<()> {
        Ok(())
    }
    async fn execute<S: StateWrite>(&self, _state: &mut S, _from: Address) -> Result<()> {
        Ok(())
    }
}