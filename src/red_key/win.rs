

extern crate winapi;
extern crate user32;
use std::mem;
use std::io::{ Result  };
use std::io::prelude::*;



enum Key {
    Press(i32),
    Release(i32)
}

static PressShift : Key = Key::Press(winapi::VK_LSHIFT);
static ReleaseShift : Key = Key::Release(winapi::VK_LSHIFT);



fn char2keys(ch : char) -> Vec<Key>{
	if 'A' <= ch && ch <= 'Z'{
        let deltaA = ch as i32 - 'A' as i32;
        return vec!(
            PressShift,
            Key::Press(deltaA+0x41),
            Key::Release(deltaA+0x41),
            ReleaseShift
        );
	}
	if 'a' <= ch && ch <= 'z'{
        let deltaA = ch as i32 - 'a' as i32;
        return vec!(Key::Press(deltaA+0x41),Key::Release(deltaA+0x41));
	}
    vec!()
}



pub fn press_character(ch : char) -> Result<()>{
    let mut input = winapi::INPUT{
        type_ : winapi::INPUT_KEYBOARD,
        u : Default::default()
    };
    for key in char2keys(ch){
        let (code,flags) = match key{
            Key::Press(code) => (code,0),
            Key::Release(code) => (code,2)
        };
        unsafe{
            *input.ki_mut() = winapi::KEYBDINPUT{
                wVk : code as u16,
                wScan : 0,
                dwFlags : flags,
                time : 0,
                dwExtraInfo : 0
            };

            user32::SendInput(1, &mut input, mem::size_of::<winapi::INPUT>() as i32);
        }

    }
    Ok(())
}
