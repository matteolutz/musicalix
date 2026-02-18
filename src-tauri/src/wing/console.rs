use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use libwing::{WingConsole, WingNodeData, WingNodeDef, WingResponse};
use tokio::sync::oneshot;

use crate::wing::{error::WingError, WingChannel, WingChannelId, WingDca, WingDcaId};

#[derive(Default)]
pub struct WingRequests {
    node_data_requests: HashMap<i32, Vec<oneshot::Sender<Arc<WingNodeData>>>>,
    node_def_requests: HashMap<i32, Vec<oneshot::Sender<Arc<WingNodeDef>>>>,
}

impl WingRequests {
    pub fn request_node_data(&mut self, node_id: i32, sender: oneshot::Sender<Arc<WingNodeData>>) {
        self.node_data_requests
            .entry(node_id)
            .or_default()
            .push(sender);
    }

    pub fn request_node_def(&mut self, node_id: i32, sender: oneshot::Sender<Arc<WingNodeDef>>) {
        self.node_def_requests
            .entry(node_id)
            .or_default()
            .push(sender);
    }

    pub fn handle(&mut self, response: WingResponse) {
        match response {
            WingResponse::NodeData(node_id, data) => {
                if let Some(requests) = self.node_data_requests.remove(&node_id) {
                    let data = Arc::new(data);
                    for req in requests {
                        let _ = req.send(data.clone());
                    }
                }
            }
            WingResponse::NodeDef(node_def) => {
                if let Some(requests) = self.node_def_requests.remove(&node_def.id) {
                    let node_def = Arc::new(node_def);
                    for req in requests {
                        let _ = req.send(node_def.clone());
                    }
                }
            }
            WingResponse::RequestEnd => {}
        }
    }
}

#[derive(Clone)]
pub struct Wing {
    console: Arc<Mutex<WingConsole>>,
    requests: Arc<Mutex<WingRequests>>,
}

impl Wing {
    pub fn channel<'a>(&'a self, channel: WingChannelId) -> WingChannel<'a> {
        WingChannel::new(self, channel)
    }

    pub fn dca<'a>(&'a self, dca_id: WingDcaId) -> WingDca<'a> {
        WingDca::new(self, dca_id)
    }
}

impl Wing {
    pub fn handle_incoming(&self) {
        let Ok(response) = self.console.lock().unwrap().read() else {
            return;
        };

        self.requests.lock().unwrap().handle(response);
    }

    pub fn handle_incoming_loop(&self) {
        loop {
            self.handle_incoming();
            thread::sleep(Duration::from_secs_f32(1.0 / 60.0));
        }
    }
}

impl Wing {
    pub async fn request_data<F, R>(&self, node_id: i32, f: F) -> Result<R, WingError>
    where
        F: FnOnce(&WingNodeData) -> R,
    {
        let (tx, rx) = oneshot::channel();
        self.requests.lock().unwrap().request_node_data(node_id, tx);
        self.console.lock().unwrap().request_node_data(node_id)?;

        let res = rx.await.expect("Sender was dropped");
        Ok(f(&res))
    }

    pub async fn request_string(&self, node_id: i32) -> Result<String, WingError> {
        self.request_data(node_id, |data| data.get_string()).await
    }

    pub async fn request_int(&self, node_id: i32) -> Result<i32, WingError> {
        self.request_data(node_id, |data| data.get_int()).await
    }

    pub async fn request_float(&self, node_id: i32) -> Result<f32, WingError> {
        self.request_data(node_id, |data| data.get_float()).await
    }

    pub async fn request_node_def<F, R>(&self, node_id: i32, f: F) -> Result<R, WingError>
    where
        F: FnOnce(&WingNodeDef) -> R,
    {
        let (tx, rx) = oneshot::channel();
        self.requests.lock().unwrap().request_node_def(node_id, tx);
        self.console
            .lock()
            .unwrap()
            .request_node_definition(node_id)?;

        let res = rx.await.expect("Sender was dropped");
        Ok(f(&res))
    }

    pub fn set_string(&self, node_id: i32, value: &str) -> Result<(), WingError> {
        self.console.lock().unwrap().set_string(node_id, value)?;
        Ok(())
    }

    pub fn set_int(&self, node_id: i32, value: i32) -> Result<(), WingError> {
        self.console.lock().unwrap().set_int(node_id, value)?;
        Ok(())
    }

    pub fn set_float(&self, node_id: i32, value: f32) -> Result<(), WingError> {
        self.console.lock().unwrap().set_float(node_id, value)?;
        Ok(())
    }
}

impl From<WingConsole> for Wing {
    fn from(value: WingConsole) -> Self {
        Wing {
            console: Arc::new(Mutex::new(value)),
            requests: Arc::new(Mutex::new(WingRequests::default())),
        }
    }
}
