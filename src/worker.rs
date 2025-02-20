use tokio::sync::mpsc::{Receiver, Sender};
use crate::data::DataChunk;
use crate::error::ProcessError;
use rand::Rng;
use std::time::Duration;

pub struct Worker {
    pub id: usize
}


impl Worker {
    fn new(id: usize) -> Self {
        Worker { id}
    }


    pub async fn generate_data(&self, chunk_count: usize, tx: Sender<DataChunk>) -> Result<(), ProcessError> {
        let mut rng = rand::thread_rng();

        for i in 0..chunk_count {
            let data = DataChunk{
                id: i,
                value: rng.gen_range(0..100),
                worker_id: self.id,
            };

            tokio::time::sleep(Duration::from_millis(rng.gen_range(10..50))).await;
            tx.send(data).await.map_err(|_| ProcessError::ChannelError)?;

        }

        Ok(())
    }

    pub async fn transform_data(
        &self,
        mut rx: Receiver<DataChunk>,
        tx: Sender<DataChunk>
    ) -> Result<(), ProcessError> {
        while let Some(mut chunk) = rx.recv().await {

            tokio::time::sleep(Duration::from_millis(20)).await;
            // transform data
            chunk.value *= 20;
            chunk.worker_id = self.id;

            tx.send(chunk).await.map_err(|_| ProcessError::ChannelError)?;
        }

        Ok(())
    }

    pub async fn aggregate_data(
        &self,
        rx: &mut Receiver<DataChunk>,
    ) -> Result<Vec<DataChunk>, ProcessError> {
        let mut results = Vec::new();
        
        while let Some(chunk) = rx.recv().await {
            tokio::time::sleep(Duration::from_millis(30)).await;
            results.push(chunk);
        }
        
        Ok(results)
    }






}