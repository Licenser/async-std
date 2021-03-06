use std::time::Duration;

use async_std::io;
use async_std::task;

#[test]
#[should_panic(expected = "timed out")]
fn io_timeout_timedout() {
    task::block_on(async {
        io::timeout(Duration::from_millis(100), async {
            task::sleep(Duration::from_secs(1)).await;

            Ok(())
        })
        .await
        .unwrap(); // We should panic with a timeout error
    });
}

#[test]
#[should_panic(expected = "My custom error")]
fn io_timeout_future_err() {
    task::block_on(async {
        io::timeout(Duration::from_secs(1), async {
            Err::<(), io::Error>(io::Error::new(io::ErrorKind::Other, "My custom error"))
        })
        .await
        .unwrap(); // We should panic with our own error
    });
}

#[test]
fn io_timeout_future_ok() {
    task::block_on(async {
        io::timeout(Duration::from_secs(1), async { Ok(()) })
            .await
            .unwrap(); // We shouldn't panic at all
    });
}
