use crate::model::device::DeviceData;
use mongodb::{
    bson::doc,
    {Client, Collection},
};

pub struct MongoDB {
    col: Collection<DeviceData>,
}

impl MongoDB {
    pub async fn init_collection(client: Client) -> mongodb::error::Result<Self> {
        let db = client.database("iot_backend");
        db.run_command(doc! {"ping": 1}, None).await?;
        println!("Connected to DB successfully.");

        let col: Collection<DeviceData> = db.collection("device_data");
        return Ok(MongoDB { col: col });
    }

    pub fn get_collection(self) -> Collection<DeviceData> {
        return self.col;
    }
}
