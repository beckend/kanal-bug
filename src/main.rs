use std::time::Duration;

use anyhow::{anyhow, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let (sender, receiver) = kanal::bounded_async::<Vec<usize>>(100);
    let limit_messages = 10000000_usize;

    let mut tasks = Vec::new();

    let sender_c = sender.clone_sync();
    tasks.push(tokio::spawn(async move {
        let mut counter = 0_usize;

        loop {
            if counter > limit_messages {
                break;
            }

            tokio::time::sleep(Duration::from_millis(1)).await;

            if let 0 = counter % 2 {
                if sender_c.send(vec![1, 2, 3]).is_err() {
                    break;
                }
            } else {
                if sender_c.send(Default::default()).is_err() {
                    break;
                }
            }

            counter += 1;
        }

        Ok(()) as Result<()>
    }));

    for _ in 1..6 {
        let receiver_c = receiver.clone();

        tasks.push(tokio::spawn(async move {
            loop {
                match receiver_c.recv().await {
                    Err(err) => {
                        return Err(anyhow!("recv error: {}", err));
                    }
                    _ => {}
                };
            }
        }));
    }

    tokio::time::sleep(Duration::from_millis(100)).await;

    for x in tasks.drain(..) {
        x.abort();
    }

    Ok(())
}
