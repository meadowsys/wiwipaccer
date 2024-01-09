use ::std::sync::atomic::{ AtomicU8, Ordering };
use tokio::runtime;

#[inline]
fn incrementing_uwu() -> String {
	static THREAD_COUNT: AtomicU8 = AtomicU8::new(1);
	let count = THREAD_COUNT.fetch_add(1, Ordering::Relaxed);
	format!("uwu thread {count}")
}

#[inline]
pub fn get_rt() -> runtime::Runtime {
	runtime::Builder::new_multi_thread()
		.enable_all()
		.worker_threads(2)
		.thread_name_fn(incrementing_uwu)
		.build()
		.expect("error building async runtime")
}
