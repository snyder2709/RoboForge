use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::robot::{RobotDescriptor, RobotState};

#[derive(Debug, Clone, PartialEq)]
pub enum FsmState {
    Disconnected,
    Connecting,
    Connected,
    Testing,
    Error(String),
}

impl FsmState {
    pub fn name(&self) -> &'static str {
        match self {
            FsmState::Disconnected => "disconnected",
            FsmState::Connecting => "connecting",
            FsmState::Connected => "connected",
            FsmState::Testing => "testing",
            FsmState::Error(_) => "error",
        }
    }
}

pub struct ZenohInner {
    pub fsm: FsmState,
    pub session: Option<zenoh::Session>,
    pub descriptors: HashMap<u32, RobotDescriptor>,
    pub states: HashMap<u32, RobotState>,
}

pub struct ZenohState(pub Arc<Mutex<ZenohInner>>);

impl ZenohState {
    pub fn new() -> Self {
        ZenohState(Arc::new(Mutex::new(ZenohInner {
            fsm: FsmState::Disconnected,
            session: None,
            descriptors: HashMap::new(),
            states: HashMap::new(),
        })))
    }
}
