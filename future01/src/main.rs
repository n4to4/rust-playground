use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::{Duration, Instant};

#[tokio::main]
async fn main() {
    let fut = async {
        tokio::time::sleep(Duration::from_millis(500)).await;
    };
    let fut = TimedWrapper::new(fut);
    let (_x, elapsed) = fut.await;
    println!("{:?}", elapsed);
}

#[pin_project::pin_project]
pub struct TimedWrapper<Fut: Future> {
    start: Option<Instant>,
    #[pin]
    future: Fut,
}

impl<Fut: Future> TimedWrapper<Fut> {
    pub fn new(future: Fut) -> Self {
        Self {
            future,
            start: None,
        }
    }
}

impl<Fut: Future> Future for TimedWrapper<Fut> {
    // This future will output a pair of values:
    // 1. The value from the inner future
    // 2. How long it took for the inner future to resolve
    type Output = (Fut::Output, Duration);

    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        // This returns a type with all the same fields, with all the same types,
        // except that the fields defined with #[pin] will be pinned.
        let mut this = self.project();

        // Call the inner poll, measuring how long it took.
        let start = this.start.get_or_insert_with(Instant::now);
        let inner_poll = this.future.as_mut().poll(cx);
        let elapsed = start.elapsed();

        match inner_poll {
            // The inner future needs more time, so this future needs more time too
            Poll::Pending => Poll::Pending,
            // Success!
            Poll::Ready(output) => Poll::Ready((output, elapsed)),
        }
    }
}
