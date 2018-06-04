use std::sync::Arc;
use std::thread;
use std::net::SocketAddr;

use futures::Stream;
use hyper::server::Http;
use net2::unix::UnixTcpBuilderExt;
use tokio_core::net::TcpListener;
use tokio_core::reactor::Core;
use num_cpus;
use net2;
use app::App;

pub fn server() {
    let addr = "0.0.0.0:3000".parse().unwrap();
    let protocol = Arc::new(Http::new());

    for _ in 0..num_cpus::get() - 1 {
        let protocol = protocol.clone();
        thread::spawn(move || serve(&addr, &protocol));
    }
    serve(&addr, &protocol);
}

fn serve(addr: &SocketAddr, protocol: &Http) {
    let mut core = Core::new().unwrap();
    let handle = core.handle();
    let listener = net2::TcpBuilder::new_v4().unwrap()
                        .reuse_port(true).unwrap()
                        .bind(addr).unwrap()
                        .listen(128).unwrap();
    let listener = TcpListener::from_listener(listener, addr, &handle).unwrap();
    core.run(listener.incoming().for_each(|(socket, addr)| {
        protocol.bind_connection(&handle, socket, addr, App::new(handle.clone()));
        Ok(())
    })).unwrap();
}
