use crate::{
    KINDLE_CONNECT_TIMEOUT, KINDLE_IP_ADDRESS, KINDLE_PASSWORD, KINDLE_SSH_PORT, KINDLE_USERNAME,
};

use ssh2::Session;
use std::io::Write;
use std::net::{SocketAddr, SocketAddrV4, TcpStream};
use std::path::Path;
use std::time::Duration;

pub fn open_tcp_connection() -> std::io::Result<TcpStream> {
    let address = SocketAddr::V4(SocketAddrV4::new(KINDLE_IP_ADDRESS, KINDLE_SSH_PORT));
    TcpStream::connect_timeout(&address, Duration::from_millis(KINDLE_CONNECT_TIMEOUT))
}

pub fn eips_show_image(tcp_stream: TcpStream, png: &[u8]) -> Result<(), ssh2::Error> {
    let mut session = Session::new()?;
    session.set_tcp_stream(tcp_stream);
    session.handshake()?;
    session.userauth_password(KINDLE_USERNAME, KINDLE_PASSWORD)?;
    let remote_path = Path::new("/dev/shm/out.png");
    let mut channel = session.scp_send(remote_path, 0o644, png.len() as u64, None)?;
    channel.write_all(png).expect("failed to write png");
    channel.close()?;
    channel.wait_close()?;
    let mut channel = session.channel_session()?;
    channel.exec("/usr/sbin/eips -g /dev/shm/out.png")?;
    channel.close()?;
    channel.wait_close()?;
    session.disconnect(None, "done", None)?;
    // tcp_stream.shutdown(std::net::Shutdown::Both).expect("couldn't shut down tcp stream");
    Ok(())
}
