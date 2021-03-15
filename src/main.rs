use std::{io, thread::sleep, time::Duration};

#[cfg(target_os = "macos")]
use cocoa_foundation::base::id;
#[cfg(target_os = "macos")]
use cocoa_foundation::foundation::NSRunLoop;
#[cfg(target_os = "macos")]
use objc::{msg_send, sel, sel_impl};

use tts;
use terminal_clipboard;
use inputbot::{KeybdKey::*, *};

fn main() {
    let res = tts::TTS::default();
    let mut text2speech = match res {
        Ok(t2s) => t2s,
        Err(_) => panic!["Backend not found."]
    };
    
    text2speech.set_rate(2.0).expect("Setting rate failed.");
    
    handle_input_events();
    
    loop {
        if EscapeKey.is_pressed() {
            break;
        }
        
        if RKey.is_pressed() {
            // copying to clipboard
            LControlKey.press();
            CKey.press();
            sleep(Duration::from_millis(300));
            CKey.release();
            LControlKey.release();
            
            // get clipboard
            let string_res = terminal_clipboard::get_string();
            
            let text = match string_res {
                Ok(string) => {
                    string.replace("\r", "").replace("-\n", "")
                    .replace(|c: char| {c.is_whitespace()}, " ")
                },
                Err(_) => panic!["Clipboard did not yield text."]
            };
            
            text2speech.speak(text, true).expect("Failed to speak utterance.");
            // set string to original copy

            /*
            let mut _input = String::new();
            // The below is only needed to make the example run on MacOS because there is no NSRunLoop in this context.
            // It shouldn't be needed in an app or game that almost certainly has one already.
            #[cfg(target_os = "macos")]
                {
                    let run_loop: id = unsafe { NSRunLoop::currentRunLoop() };
                    unsafe {
                        let _: () = msg_send![run_loop, run];
                    }
                }
            println!["Speaking"];
            io::stdin().read_line(&mut _input).expect("uh oh");
            */
        }
    }
}