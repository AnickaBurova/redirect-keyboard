

extern crate winapi;
extern crate user32;
use std::mem;
use byteorder::{WriteBytesExt,  LittleEndian};



enum Key {
    Press(u16),
    Release(u16)
}



fn char2keys(ch : char) -> Vec<Key>{
	if 'A' <= ch && ch <= 'Z'{
        let deltaA = ch - 'A';
        return vec!(Key::Press(deltaA+0x41),Key::Release(deltaA+0x41));
	}
	if 'a' <= ch && ch <= 'z'{
        let deltaA = ch - 'a';
        return vec!(Key::Press(deltaA+0x41),Key::Release(deltaA+0x41));
	}
    vec!()
}



pub fn press_character(ch : char) -> Result<()>{
    let mut input = winapi::INPUT{
        type_ : winapi::INPUT_KEYBOARD,
        u : Default::default()
    };
    for key in char2keys(ch).iter(){
        let (code,flags) = match key{
            Press(code) => (code,0),
            Release(code) => (code,2)
        };
        unsafe{
            *input.ki_mut() = winapi::KEYBDINPUT{
                wVk : code,
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
