use async_trait::async_trait;
use pingora::prelude::*;
use std::{env, sync::Arc};

pub struct LB(Arc<LoadBalancer<RoundRobin>>);

#[async_trait]
impl ProxyHttp for LB {
    /// For this small example, we don't need context storage
    type CTX = ();
    fn new_ctx(&self) {}

    async fn upstream_peer(&self, _session: &mut Session, _ctx: &mut ()) -> Result<Box<HttpPeer>> {
        let upstream = self
            .0
            .select(b"", 256) // hash doesn't matter for round robin
            .unwrap();

        println!("upstream peer is: {upstream:?}");

        // Set SNI to one.one.one.one
        let peer = Box::new(HttpPeer::new(upstream, false, "".to_string()));
        Ok(peer)
    }
}

fn main() {
    let mut my_server = Server::new(None).unwrap();
    my_server.bootstrap();

    let port = env::var("PORT").expect("PORT envvar not set");
    let servers_raw = env::var("SERVER_LIST").expect("SERVER_LIST envvar not set");
    let server_list = servers_raw.split(',').collect::<Vec<&str>>();

    let upstreams = LoadBalancer::try_from_iter(server_list).unwrap();

    let mut lb = http_proxy_service(&my_server.configuration, LB(Arc::new(upstreams)));
    lb.add_tcp(format!("0.0.0.0:{port}").as_str());

    my_server.add_service(lb);

    my_server.run_forever();
}
