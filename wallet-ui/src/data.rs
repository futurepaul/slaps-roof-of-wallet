use std::sync::Arc;
use druid::{Data, Env, EventCtx};
use wallet_core::SlapsCore;

#[derive(Clone, Data)]
pub struct AppState {
    core: Arc<SlapsCore>
}

impl AppState {
    pub fn new() -> Self {
        Self {
            core: Arc::new(SlapsCore::new())
        }
    }

    pub fn print_address(_ctx: &mut EventCtx, data: &mut Self, _env: &Env) {
        data.core.print_address();
    }
}
