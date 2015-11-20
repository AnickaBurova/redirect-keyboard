

extern crate winapi;
extern crate user32;
use std::mem;
use std::io::{ Result  };
use std::io::prelude::*;


#[derive(Clone,Copy)]
enum Key {
    Press(i32),
    Release(i32)
}

static PressShift : Key = Key::Press(winapi::VK_LSHIFT);
static ReleaseShift : Key = Key::Release(winapi::VK_LSHIFT);

macro_rules! keypress {
    ($ch:ident, $base : expr, $vk : expr, $from : expr, $to : expr) => {
        if($from <= $ch && $ch <= $to){
            let code = $ch as i32 - $base as i32 + $vk;
            return vec!(Key::Press(delta),Key::Release(delta));
        }
    }
    ($ch:ident, $base : expr, $vk : expr, $from : expr, $to : expr, $front : expr, $back : expr) => {
        if($from <= $ch && $ch <= $to){
            let code = $ch as i32 - $base as i32 + $vk;
            return vec!($front,Key::Press(delta),Key::Release(delta),$back);
        }
    }
}

fn char2keys(ch : char) -> Vec<Key>{
    keypress(ch,'A', 0x41, 'A', 'Z', PressShift, ReleaseShift);
    keypress(ch,'a', 0x41, 'a', 'z');
    keypress(ch,'0', 0x41, '0', '9');
	// if 'A' <= ch && ch <= 'Z'{
    //     let deltaA = ch as i32 - 'A' as i32;
    //     return vec!(
    //         PressShift,
    //         Key::Press(deltaA+0x41),
    //         Key::Release(deltaA+0x41),
    //         ReleaseShift
    //     );
	// }
	// if 'a' <= ch && ch <= 'z'{
    //     let deltaA = ch as i32 - 'a' as i32;
    //     return vec!(Key::Press(deltaA+0x41),Key::Release(deltaA+0x41));
	// }
    // if '0' <= ch && ch <= '9'{
    //
    // }
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
