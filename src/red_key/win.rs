

extern crate winapi;
extern crate user32;
use std::mem;
use std::io::{ Result  };
use std::io::prelude::*;
use std::collections::{HashMap};

macro_rules! hashmap {
    ($( $key: expr => $val: expr ),*) => {{
         let mut map = ::std::collections::HashMap::new();
         $( map.insert($key, $val); )*
         map
    }}
}


#[derive(Clone,Copy)]
enum Key {
    Press(i32),
    Release(i32)
}

static PressShift : Key = Key::Press(winapi::VK_LSHIFT);
static ReleaseShift : Key = Key::Release(winapi::VK_LSHIFT);

macro_rules! keypress {
    ($vk:expr) => {
        vec!(Key::Press($vk),Key::Release($vk));
    };
    ($ch:ident, $base : expr, $vk : expr, $from : expr, $to : expr) => {
        if($from <= $ch && $ch <= $to){
            let code = $ch as i32 - $base as i32 + $vk;
            return keypress!(code);
        }
    };
    ($ch:ident, $base : expr, $vk : expr, $from : expr, $to : expr, $front : expr, $back : expr) => {
        if($from <= $ch && $ch <= $to){
            let code = $ch as i32 - $base as i32 + $vk;
            return vec!($front,Key::Press(code),Key::Release(code),$back);
        }
    };
    (shift => $ch:ident, $base : expr, $vk : expr, $from : expr, $to : expr) => {
        keypress!($ch,$base,$vk,$from,$to,PressShift, ReleaseShift);
    };
    (map => $ch:ident, $m : $ident) => {
        match $m.get($ch as i32){
            Some(code) => keypress!(code),
            None => ()
        }
    };
    (one => $ch:ident, $code : expr, $vk : expr) => {
        if ($ch as i32 == $code){
            return keypress!(code);
        }
    };
}



fn char2keys(ch : char) -> Vec<Key>{
    keypress!(shift => ch,'A', 0x41, 'A', 'Z');
    keypress!(ch,'a', 0x41, 'a', 'z');
    keypress!(ch,'0', 0x30, '0', '9');
    let keys = hashmap![
            9 => winapi::VK_TAB,
            10 => winapi::VK_RETURN,
            32 => winapi::VK_SPACE,
            127 => winapi::VK_BACK
            ];
    keypress!(map => ch, keys);
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
