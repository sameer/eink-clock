use ssh2::Session;

use crate::ssh::{amixer_set_master_volume, aplay_audio_nonblocking};

pub fn play_audio_for_hour(session: &mut Session, hour24:u32, hour12: u32) -> Result<(), ssh2::Error> {
    let volume = if hour24 < 7 || hour12 > 22 {
        0
    } else {
        20
    };

    amixer_set_master_volume(session, volume)?;

    match hour12 {
        1 => aplay_audio_nonblocking(session, include_bytes!("../audio/1.wav")),
        2 => aplay_audio_nonblocking(session, include_bytes!("../audio/2.wav")),
        3 => aplay_audio_nonblocking(session, include_bytes!("../audio/3.wav")),
        4 => aplay_audio_nonblocking(session, include_bytes!("../audio/4.wav")),
        5 => aplay_audio_nonblocking(session, include_bytes!("../audio/5.wav")),
        6 => aplay_audio_nonblocking(session, include_bytes!("../audio/6.wav")),
        7 => aplay_audio_nonblocking(session, include_bytes!("../audio/7.wav")),
        8 => aplay_audio_nonblocking(session, include_bytes!("../audio/8.wav")),
        9 => aplay_audio_nonblocking(session, include_bytes!("../audio/9.wav")),
        10 => aplay_audio_nonblocking(session, include_bytes!("../audio/10.wav")),
        11 => aplay_audio_nonblocking(session, include_bytes!("../audio/11.wav")),
        12 => aplay_audio_nonblocking(session, include_bytes!("../audio/12.wav")),
        _ => Ok(()),
    }
}
