pub const PROTOCOL_SUPPORT: u8 = 1;

pub struct MessageHeader {
    pub message_type: u8,
    pub message_length: u32,
}

pub struct ProtocolSupport {
    pub max_version: u16,
    pub min_version: u16,
    pub flags: u16,
}

pub struct ProtocolVersion {
    pub version: u16,
    pub flas: u16,
    pub public_key: [u8; 33],
}

pub struct PayoutInfo {
    pub signature: [u8; 64],
    pub message_timestamp: u64,
    pub remaining_payout_script: Vec<u8>,
    pub appended_outputs_count: u8,
    pub appended_outputs: Vec<u8>,
}

pub struct UserAuth {
    pub suggested_target: [u8; 32],
    pub minimum_target: [u8; 32],
    pub user_id: Vec<u8>,
    pub user_auth: Vec<u8>,
}

pub struct AcceptUserAuth {
    pub signature: [u8; 64],
    pub user_id: Vec<u8>,
    pub message_timestamp: u64,
    pub coinbase_postfix: Vec<u8>,
}

pub struct RejectUserAuth {
    pub user_id: Vec<u8>,
}

pub struct DropUser {
    pub user_id: Vec<u8>,
}

pub struct ShareDifficulty {
    pub user_id: Vec<u8>,
    pub message_timestamp: u64,
    pub share_target: [u8; 32],
    pub weak_block_target: [u8; 32],
}

pub struct Share {
    pub header_version: u32,
    pub previous_block: [u8; 32],
    pub header_timestamp: u32,
    pub header_nbits: u32,
    pub header_nonce: u32,
    pub merkle_path: Vec<u8>,
    pub coinbase_tx_length: Vec<u8>,
    pub user_tag_1: Vec<u8>,
    pub user_tag_2: Vec<u8>,
}

pub struct WeakBlock {
    pub header_version: u32,
    pub previous_block: [u8; 32],
    pub header_timestamp: u32,
    pub header_nbits: u32,
    pub header_nonce: u32,
    pub merkle_path: Vec<u8>,
    pub user_tag_1: Vec<u8>,
    pub user_tag_2: Vec<u8>,
    pub extra_block_data: Vec<u8>,
    pub txn_count: u32,
    pub txn_list: Vec<u8>,
}

pub struct WeakBlockStateReset;

pub struct ShareAccepted {
    pub user_tag_1: Vec<u8>,
    pub user_tag_2: Vec<u8>,
}

pub struct ShareRejected {
    pub reason: u8,
    pub user_tag_1: Vec<u8>,
    pub user_tag_2: Vec<u8>,
}

pub struct NewPoolServer {
    pub signature: [u8; 64],
    pub new_host_port: Vec<u8>,
}

pub struct VendorMessage {
    pub flags: u16,
    pub signature: [u8; 64],
    pub vendor: Vec<u8>,
    pub message: Vec<u8>,
}
