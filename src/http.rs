use std::sync::Arc;
use std::thread;
use std::net::SocketAddr;

use futures::Stream;
use futures::future;
use hyper::server::Http;
use net2::unix::UnixTcpBuilderExt;
use tokio_core::net::TcpListener;
use tokio_core::reactor::Core;
use num_cpus;
use net2;
use signal_hook::SIGTERM;
use signal_hook::iterator::Signals;
use crate::app::App;

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
    let signal = Signals::new(&[SIGTERM]).unwrap().into_async().unwrap().map(|_| None);
    let incoming = listener.incoming().map_err(|err| err.into()).map(|item| Some(item));
    core.run(signal.select(incoming).take_while(|item| future::ok(item.is_some())).for_each(|item| {
        let (socket, addr) = item.unwrap();
        protocol.bind_connection(&handle, socket, addr, App::new(handle.clone()));
        Ok(())
    })).unwrap();
}
