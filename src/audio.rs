use ssh2::Session;

use crate::ssh::{amixer_set_master_volume, aplay_audio_nonblocking};

macro_rules! hours {
    ($session: ident, $hour12: ident, $($hour: expr,)*) => {
        match $hour12 {
            $($hour => aplay_audio_nonblocking($session, include_bytes!(concat!("../audio/", stringify!($hour), ".wav"))),)*
            _ => Ok(())
        }
    };
}

pub fn play_audio_for_hour(
    session: &mut Session,
    _hour24: u32,
    hour12: u32,
) -> Result<(), ssh2::Error> {
    let volume = 20;

    amixer_set_master_volume(session, volume)?;
    hours!(session, hour12, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12,)
}
