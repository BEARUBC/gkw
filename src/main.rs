use tokio;

mod kernel;

#[tokio::main(flavor = "multi_thread", worker_threads = 10)]
async fn main() -> ! {
    unreachable!("the `grasp-kernel-wrapper` has exited unexpectedly...")
}
