use crate::model::device::DeviceData;
use mongodb::bson::doc;
use mongodb::{Client, Collection};

pub struct MongoDB {
    col: Collection<DeviceData>,
}

impl MongoDB {
    pub async fn init() -> mongodb::error::Result<Self> {
        println!("Connecting to DB...");
        let uri = "mongodb://localhost:27017".to_owned();
        let client = Client::with_uri_str(uri).await?;
        let db = client.database("iot_backend");
        db.run_command(doc! {"ping": 1}, None).await?;
        println!("Connected to DB successfully.");
        let col: Collection<DeviceData> = db.collection("device_data");
        return Ok(MongoDB { col: col });
    }
}
