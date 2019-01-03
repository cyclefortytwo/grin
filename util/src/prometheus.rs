extern crate log;

#[cfg(feature = "monitoring")]
mod prometheus {
	use hyper::rt::Future;
	use hyper::service::service_fn_ok;
	use hyper::{rt, Body, Request, Response, Server};
	use std::thread;

	use lazy_static::lazy_static;
	use prometheus::{
		register_int_gauge, IntGauge, __register_gauge, opts, register_gauge, register_int_counter,
		Encoder, Gauge, Histogram, HistogramTimer, IntCounter, __register_counter, histogram_opts,
		register, register_histogram,
	};
	use std::collections::HashMap;
	use std::sync::RwLock;

	lazy_static! {

			static ref GAUGES: RwLock<HashMap<&'static str, Gauge>> = RwLock::new(HashMap::new());
			static ref INT_GAUGES: RwLock<HashMap<&'static str, IntGauge>> = RwLock::new(HashMap::new());
			static ref INT_COUNTER: RwLock<HashMap<&'static str, IntCounter>> = RwLock::new(HashMap::new());
			static ref HISTOGRAMS: RwLock<HashMap<&'static str, Histogram>> = RwLock::new(HashMap::new());
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
					.map_err(|e| warn!("Prometheus server error: {}", e));

				rt::run(server);
			})
			.map_err(|e| warn!("Failed to spawn a thread with Prometheus server: {}", e));
	}

	fn handler(res: Request<Body>) -> Response<Body> {
		let mut buffer = Vec::new();
		let encoder = prometheus::TextEncoder::new();
		let metric_families = prometheus::gather();
		encoder.encode(&metric_families, &mut buffer).unwrap();
		Response::new(Body::from(buffer.clone()))
	}

	pub fn register_histogram(name: &'static str, help: &str, buckets: Vec<f64>) {
		let mut hm = HISTOGRAMS.write().unwrap();
		if let None = hm.get(name) {
			match register_histogram!(name, help, buckets) {
				Ok(h) => {
					hm.insert(name, h);
				}
				Err(e) => warn!("Cannot create gauge {}", e),
			}
		}
	}

	pub fn run_for_histogram(name: &'static str, mut f: impl FnMut(&Histogram) -> ()) {
		{
			let hm = HISTOGRAMS.read().unwrap();
			if let Some(h) = hm.get(name) {
				f(&h);
				return;
			}
		}

		let mut hm = HISTOGRAMS.write().unwrap();
		match register_histogram!(name, name) {
			Ok(h) => {
				f(&h);
				hm.insert(name, h);
			}
			Err(e) => warn!("Cannot create histogram {}", e),
		}
	}

	pub fn histogram_observe(name: &'static str, t: f64) {
		run_for_histogram(name, |h| h.observe(t));
	}

	pub fn histogram_start_timer(name: &'static str, t: f64) -> HistogramTimer {
		{
			let hm = HISTOGRAMS.read().unwrap();
			if let Some(h) = hm.get(name) {
				return h.start_timer();
			}
		}
		let mut hm = HISTOGRAMS.write().unwrap();
		let h = Histogram::with_opts(histogram_opts!(name, name)).unwrap();
		match register(Box::new(h.clone())) {
			Ok(()) => {
				let timer = h.start_timer();
				hm.insert(name, h);
				timer
			}
			Err(e) => {
				warn!("Cannot create histogram {}", e);
				h.start_timer()
			}
		}
	}

	pub fn register_int_gauge(name: &'static str, help: &str) {
		let mut hm = INT_GAUGES.write().unwrap();
		if let None = hm.get(name) {
			match register_int_gauge!(name, help) {
				Ok(g) => {
					hm.insert(name, g);
				}
				Err(e) => warn!("Cannot create gauge {}", e),
			}
		}
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
		match register_int_gauge!(name, name) {
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

	pub fn register_gauge(name: &'static str, help: &str) {
		let mut hm = GAUGES.write().unwrap();
		if let None = hm.get(name) {
			match register_gauge!(name, help) {
				Ok(g) => {
					hm.insert(name, g);
				}
				Err(e) => warn!("Cannot create gauge {}", e),
			}
		}
	}

	pub fn run_for_gauge(name: &'static str, f: impl Fn(&Gauge) -> ()) {
		{
			let hm = GAUGES.read().unwrap();
			if let Some(g) = hm.get(name) {
				f(&g);
				return;
			}
		}

		let mut hm = GAUGES.write().unwrap();
		match register_gauge!(name, name) {
			Ok(g) => {
				f(&g);
				hm.insert(name, g);
			}
			Err(e) => warn!("Cannot create gauge {}", e),
		}
	}

	pub fn gauge_inc(name: &'static str) {
		info!("INC with monitoring");
		run_for_gauge(name, |g| g.inc());
	}

	pub fn gauge_dec(name: &'static str) {
		info!("DEC with monitoring");
		run_for_gauge(name, |g| g.dec())
	}

	pub fn gauge_add(name: &'static str, n: f64) {
		info!("ADD with monitoring");
		run_for_gauge(name, |g| g.add(n));
	}

	pub fn gauge_sub(name: &'static str, n: f64) {
		info!("SET with monitoring");
		run_for_gauge(name, |g| g.sub(n));
	}

	pub fn gauge_set(name: &'static str, n: f64) {
		info!("SET with monitoring");
		run_for_gauge(name, |g| g.set(n));
	}

	pub fn register_int_counter(name: &'static str, help: &str) {
		let mut hm = INT_COUNTER.write().unwrap();
		if let None = hm.get(name) {
			match register_int_counter!(name, help) {
				Ok(g) => {
					hm.insert(name, g);
				}
				Err(e) => warn!("Cannot create int counter {}", e),
			}
		}
	}

	pub fn run_for_int_counter(name: &'static str, f: impl Fn(&IntCounter) -> ()) {
		{
			let hm = INT_COUNTER.read().unwrap();
			if let Some(m) = hm.get(name) {
				f(&m);
				return;
			}
		}

		let mut hm = INT_COUNTER.write().unwrap();
		match register_int_counter!(name, name) {
			Ok(m) => {
				f(&m);
				hm.insert(name, m);
			}
			Err(e) => warn!("Cannot create counter {}", e),
		}
	}
	pub fn int_counter_inc(name: &'static str) {
		run_for_int_counter(name, |c| c.inc());
	}

}

#[cfg(not(feature = "monitoring"))]
mod empty {
	pub fn register_int_gauge(name: &'static str, help: &str) {}
	pub fn register_gauge(name: &'static str, help: &str) {}
	pub fn register_int_counter(name: &'static str, help: &str) {}
	pub fn int_gauge_inc(name: &'static str) {}
	pub fn int_gauge_dec(name: &'static str) {}
	pub fn int_gauge_add(name: &'static str, n: i64) {}
	pub fn int_gauge_sub(name: &'static str, n: i64) {}
	pub fn int_gauge_set(name: &'static str, n: i64) {}
	pub fn int_counter_inc(name: &'static str) {}
	pub fn start() {}
}

#[cfg(feature = "monitoring")]
pub use crate::prometheus::prometheus::{
	int_counter_inc, int_gauge_add, int_gauge_dec, int_gauge_inc, int_gauge_set, int_gauge_sub,
	register_gauge, register_int_counter, register_int_gauge, start,
};

#[cfg(not(feature = "monitoring"))]
pub use crate::prometheus::empty::{
	int_counter_inc, int_gauge_add, int_gauge_dec, int_gauge_inc, int_gauge_set, int_gauge_sub,
	register_gauge, register_int_counter, register_int_gauge, start,
};
