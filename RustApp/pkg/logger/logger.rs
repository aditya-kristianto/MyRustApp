#[allow(dead_code)]
pub async fn http_start(http_host: &str, http_port: u16) {
    println!("HTTP server listening on {}:{}", http_host, http_port);
}

#[allow(dead_code)]
pub async fn tcp_start(tcp_host: &str, tcp_port: u16) {
    println!("TCP server listening on {}:{}", tcp_host, tcp_port);
}
