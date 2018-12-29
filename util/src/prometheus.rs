extern crate log;

#[cfg(feature = "monitoring")]
mod prometheus {
	use hyper::rt::Future;
	use hyper::service::service_fn_ok;
	use hyper::{rt, Body, Request, Response, Server};
	use std::thread;

	use lazy_static::lazy_static;
	use prometheus::{register_int_gauge, IntGauge, __register_gauge, opts, Encoder};
	use std::collections::HashMap;
	use std::sync::RwLock;

	lazy_static! {

			static ref INT_GAUGES: RwLock<HashMap<&'static str, IntGauge>> = RwLock::new(HashMap::new());
			//static ref PEERS_CONNECTED_GAUGE: IntGauge =
					//register_int_gauge!("peers_connected_total", "Number of connected peers").unwrap();
	}

	pub fn start() {
		thread::Builder::new()
			.name("prometheus".to_string())
			.spawn(move || {
				let addr = ([127, 0, 0, 1], 3000).into();
				let new_service = || service_fn_ok(handler);
				let server = Server::bind(&addr)
					.serve(new_service)
					// TODO graceful shutdown is unstable, investigate
					//.with_graceful_shutdown(rx)
					.map_err(|e| eprintln!("HTTP API server error: {}", e));

				rt::run(server);
			});
		//.map_err(|_| ErrorKind::Internal("failed to spawn API thread".to_string()).into())
	}

	fn handler(res: Request<Body>) -> Response<Body> {
		let mut buffer = Vec::new();
		let encoder = prometheus::TextEncoder::new();
		let metric_families = prometheus::gather();
		encoder.encode(&metric_families, &mut buffer).unwrap();
		Response::new(Body::from(buffer.clone()))
	}

	pub fn run_for_int_gauge(name: &'static str, f: impl Fn(&IntGauge) -> ()) {
		{
			let hm = INT_GAUGES.read().unwrap();
			if let Some(g) = hm.get(name) {
				f(&g);
				return;
			}
		}

		let mut hm = INT_GAUGES.write().unwrap();
		match register_int_gauge!(name, "help") {
			Ok(g) => {
				f(&g);
				hm.insert(name, g);
			}
			Err(e) => warn!("Cannot create gauge {}", e),
		}
	}

	pub fn int_gauge_inc(name: &'static str) {
		info!("INC with monitoring");
		run_for_int_gauge(name, |g| g.inc());
	}

	pub fn int_gauge_dec(name: &'static str) {
		info!("DEC with monitoring");
		run_for_int_gauge(name, |g| g.dec())
	}

	pub fn int_gauge_add(name: &'static str, n: i64) {
		info!("ADD with monitoring");
		run_for_int_gauge(name, |g| g.add(n));
	}

	pub fn int_gauge_sub(name: &'static str, n: i64) {
		info!("SET with monitoring");
		run_for_int_gauge(name, |g| g.sub(n));
	}

	pub fn int_gauge_set(name: &'static str, n: i64) {
		info!("SET with monitoring");
		run_for_int_gauge(name, |g| g.set(n));
	}

}

#[cfg(not(feature = "monitoring"))]
mod empty {
	pub fn int_gauge_inc(name: &'static str) {
		println!("INC without monitoring");
	}
	pub fn int_gauge_dec(name: &'static str) {
		println!("DEC without monitoring");
	}
	pub fn int_gauge_add(name: &'static str, n: i64) {}

	pub fn int_gauge_sub(name: &'static str, n: i64) {}
	pub fn int_gauge_set(name: &'static str, n: i64) {}
	pub fn start() {}
}

#[cfg(feature = "monitoring")]
pub use crate::prometheus::prometheus::{
	int_gauge_add, int_gauge_dec, int_gauge_inc, int_gauge_set, int_gauge_sub, start,
};

#[cfg(not(feature = "monitoring"))]
pub use crate::prometheus::empty::{
	int_gauge_add, int_gauge_dec, int_gauge_inc, int_gauge_set, int_gauge_sub, start,
};
