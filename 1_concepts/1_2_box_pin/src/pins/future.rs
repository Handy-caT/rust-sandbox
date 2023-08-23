use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use crate::pins::base_struct::MeasurableFuture;

impl<Fut> Future for MeasurableFuture<Fut>
where Fut: Future
{
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();
        if this.started_at.is_none() {
            *this.started_at = Some(std::time::Instant::now());
        }
        let inner_future = this.inner_future;
        match inner_future.poll(cx) {
            Poll::Ready(_) => {
                let elapsed = this.started_at.unwrap().elapsed();
                println!("elapsed: {:?}", elapsed);
                Poll::Ready(())
            },
            Poll::Pending => Poll::Pending,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::thread::sleep;
    use crate::pins::base_struct::MeasurableFuture;

    #[test]
    fn test_future() {
        let fut = async {
            println!("started");
            sleep(std::time::Duration::from_secs(1));
            println!("finished");
        };

        let mut fut = MeasurableFuture {
            inner_future: fut,
            started_at: None,
        };

        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            fut.await;
        });
    }
}