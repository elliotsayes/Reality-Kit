pub type ArweaveId = String;
pub type ArweaveTxId = ArweaveId;
pub type ArweaveAddress = ArweaveId;
pub type ArweaveTimestamp = u64;
pub type Payload = String;

pub struct Message {
    pub id: ArweaveTxId,
    pub timestamp: ArweaveTimestamp,
    pub sender: ArweaveAddress,
    pub payload: Payload,
}

