#![deny(clippy::all)]
mod com;
mod context;
mod custom;
mod device;
mod enumerator;
mod loopback;
mod node;
mod render;
mod session;

use widestring::U16CStr;
use windows::core::PWSTR;

pub use context::Win32Context;
pub use custom::{AudioSessionEvent, SessionState};
pub use parking_lot::RwLock;

fn pwstr_to_string(pwstr: PWSTR) -> String {
    if pwstr.is_null() {
        String::default()
    } else {
        unsafe { U16CStr::from_ptr_str(pwstr.0).to_string_lossy() }
    }
}

type Callback<T> = Box<dyn Fn(T) + Send + Sync>;

#[cfg(test)]
mod tests {
    use std::{thread, time::Duration};

    use super::*;

    #[test]
    fn watch() {
        watch_audio(|state, name| {
            println!("{:?}", state);
            println!("{}", name);
        });
    }

    fn watch_audio<T>(audio: T)
    where
        T: Fn(SessionState, String) + Send + Sync + 'static,
    {
        let context = Win32Context::new(move |event, name| match event {
            AudioSessionEvent::StateChange(state) => {
                audio(state, name);
            }
            _ => {}
        });
        loop {
            let list = context.read().get_active_session_filename();
            println!("list: {list:?}");
            thread::sleep(Duration::from_secs(2));
        }
    }
}
