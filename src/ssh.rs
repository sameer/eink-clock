use crate::{
    KINDLE_CONNECT_TIMEOUT, KINDLE_IP_ADDRESS, KINDLE_PASSWORD, KINDLE_SSH_PORT, KINDLE_USERNAME,
};

use ssh2::Session;
use std::io::Write;
use std::net::{SocketAddr, TcpStream};
use std::path::Path;
use std::time::Duration;

pub fn open_tcp_connection() -> std::io::Result<TcpStream> {
    let address = SocketAddr::new(KINDLE_IP_ADDRESS, KINDLE_SSH_PORT);
    TcpStream::connect_timeout(&address, Duration::from_millis(KINDLE_CONNECT_TIMEOUT))
}

pub fn open_ssh_session(tcp_stream: TcpStream) -> Result<Session, ssh2::Error> {
    let mut session = Session::new()?;
    session.set_tcp_stream(tcp_stream);
    session.handshake()?;
    session.userauth_password(KINDLE_USERNAME, KINDLE_PASSWORD)?;
    Ok(session)
}

pub fn eips_show_image(
    session: &mut Session,
    png: &[u8],
    clear_screen_before: bool,
) -> Result<(), ssh2::Error> {
    let remote_path = Path::new("/dev/shm/out.png");
    let mut channel = session.scp_send(remote_path, 0o644, png.len() as u64, None)?;
    channel.write_all(png).expect("failed to write png");
    channel.close()?;
    let mut channel = session.channel_session()?;
    channel.exec(&format!(
        "/usr/sbin/eips {} -g /dev/shm/out.png",
        if clear_screen_before { "-f" } else { "" }
    ))?;
    channel.close()?;
    Ok(())
}

pub fn amixer_set_master_volume(session: &mut Session, volume: u8) -> Result<(), ssh2::Error> {
    let mut channel = session.channel_session()?;
    channel.exec(&format!("/usr/bin/amixer set Master {}%", volume))?;
    channel.close()?;
    Ok(())
}

pub fn aplay_audio_nonblocking(session: &mut Session, audio: &[u8]) -> Result<(), ssh2::Error> {
    let mut channel = session.channel_session()?;
    channel.exec("/usr/bin/aplay -v -N -")?;
    channel.write_all(audio).unwrap();
    channel.close()?;
    Ok(())
}
