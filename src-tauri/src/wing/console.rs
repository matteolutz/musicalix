use libwing::{WingConsole, WingNodeData, WingResponse};

use crate::wing::{error::WingError, WingDca, WingDcaId, WingInputChannel};

const NODE_DATA_RETRIES: u8 = 50;

pub trait WingConsoleExt {
    fn input_channel<'a>(&'a mut self, channel: u32) -> WingInputChannel<'a>;
    fn dca<'a>(&'a mut self, dca_id: WingDcaId) -> WingDca<'a>;

    fn request_and_read_data(&mut self, node_id: i32) -> Result<WingNodeData, WingError>;
}

impl WingConsoleExt for WingConsole {
    fn input_channel<'a>(&'a mut self, channel: u32) -> WingInputChannel<'a> {
        WingInputChannel::new(self, channel)
    }

    fn dca<'a>(&'a mut self, dca_id: WingDcaId) -> WingDca<'a> {
        WingDca::new(self, dca_id)
    }

    fn request_and_read_data(&mut self, node_id: i32) -> Result<WingNodeData, WingError> {
        self.request_node_data(node_id)?;

        for _ in 0..NODE_DATA_RETRIES {
            let WingResponse::NodeData(received_node_id, data) = self.read()? else {
                continue;
            };

            if received_node_id != node_id {
                continue;
            }

            return Ok(data);
        }

        Err(WingError::NodeDataRequestTimeout(node_id))
    }
}
