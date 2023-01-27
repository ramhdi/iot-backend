use mongodb::{Client, Collection};

use crate::model::device::DeviceData;

pub struct MongoDB {
    col: Collection<DeviceData>,
}

impl MongoDB {
    pub async fn init() -> Self {
        let uri = "mongodb://localhost:27017".to_owned();
        let client = Client::with_uri_str(uri).await.unwrap();
        let db = client.database("iot_backend");
        let col: Collection<DeviceData> = db.collection("device_data");
        return MongoDB { col: col };
    }
}
