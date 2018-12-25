#[cfg(feature = "monitoring")]
use hyper::rt::Future;
#[cfg(feature = "monitoring")]
use hyper::service::service_fn_ok;
#[cfg(feature = "monitoring")]
use hyper::{rt, Body, Request, Response, Server};
#[cfg(feature = "monitoring")]
use std::thread;

#[cfg(feature = "monitoring")]
use lazy_static::lazy_static;
#[cfg(feature = "monitoring")]
use prometheus::{register_int_gauge, IntGauge, __register_gauge, opts, Encoder};
#[cfg(feature = "monitoring")]
use std::collections::HashMap;
#[cfg(feature = "monitoring")]
use std::sync::Mutex;

#[cfg(feature = "monitoring")]
lazy_static! {

		static ref INT_GAUGES: Mutex<HashMap<&'static str, IntGauge>> = Mutex::new(HashMap::new());
		//static ref PEERS_CONNECTED_GAUGE: IntGauge =
				//register_int_gauge!("peers_connected_total", "Number of connected peers").unwrap();
}

#[cfg(feature = "monitoring")]
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

#[cfg(not(feature = "monitoring"))]
pub fn start() {}

#[cfg(feature = "monitoring")]
fn handler(res: Request<Body>) -> Response<Body> {
	let mut buffer = Vec::new();
	let encoder = prometheus::TextEncoder::new();
	let metric_families = prometheus::gather();
	encoder.encode(&metric_families, &mut buffer).unwrap();
	Response::new(Body::from(buffer.clone()))
}

#[cfg(feature = "monitoring")]
pub fn int_gauge_inc(name: &'static str) {
	eprintln!("INC with monitoring");
	let l = INT_GAUGES.lock();
	eprintln!("INC GOT LOCK");
	let mut hm = l.unwrap();
	eprintln!("INC UNWRAPPED");
	match hm.get(name) {
		Some(g) => {
			eprintln!("EXISTS ");
			g.inc();
			eprintln!("INC");
		}
		None => {
			eprintln!("NOT EXISTS ");
			let g = register_int_gauge!(name, "help").unwrap();
			eprintln!("REGISTERED");
			g.inc();
			eprintln!("INC ");
			hm.insert(name, g);
			eprintln!("INSERT");
		}
	}
	//INT_GAUGES
	//.lock()
	//.unwrap()
	//.entry(name)
	//.or_insert(register_int_gauge!(name, name).unwrap())
	//.inc();
	eprintln!("INC with monitoring | END");
}
#[cfg(not(feature = "monitoring"))]
pub fn int_gauge_inc(name: &'static str) {
	println!("INC without monitoring");
}

#[cfg(feature = "monitoring")]
pub fn int_gauge_dec(name: &'static str) {
	println!("DEC with monitoring");
	INT_GAUGES
		.lock()
		.unwrap()
		.entry(name)
		.or_insert(register_int_gauge!(name, "").unwrap())
		.dec();
}

#[cfg(not(feature = "monitoring"))]
pub fn int_gauge_dec(name: &'static str) {
	println!("DEC without monitoring");
}
