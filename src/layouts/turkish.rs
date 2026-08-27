//! # Turkish Q keyboard support

use crate::{DecodedKey, HandleControl, KeyCode, KeyboardLayout, Modifiers, PhysicalKeyboard};

/// A standard Turkish Q 102/105-key ISO keyboard.
///
/// Has a 2-row high Enter key, with `Oem7` above (ISO layout), matching
/// `KBDTUQ.DLL` ("Turkish Q Keyboard Layout") as documented at
/// <https://kbdlayout.info/KBDTUQ/>.
///
/// The Turkish Q layout is the QWERTY-derived layout most commonly used in
/// Turkey (as opposed to the frequency-optimised Turkish F layout). Notable
/// differences from a US/UK keyboard:
///
/// * `I` / `i` are split into dotted and dotless pairs: [`KeyCode::I`]
///   produces lowercase dotless `ı` / uppercase `I`, while [`KeyCode::Oem3`]
///   (next to `L`) produces lowercase dotted `i` / uppercase dotted `İ`.
/// * [`KeyCode::Oem4`], [`KeyCode::Oem6`], and [`KeyCode::Oem1`] produce
///   `ğ`/`Ğ`, `ü`/`Ü`, and `ş`/`Ş` respectively.
/// * AltGr (`RAltGr`/`ralt`) provides `@`, `€`, `₺`, currency and bracket
///   symbols, `\`, `|`, and a handful of diacritic marks (`^`, `´`, `¨`,
///   `` ` ``, `~`) on the letter/punctuation keys that are dead keys in
///   Windows but are emitted here as bare, non-composing characters, since
///   this crate's [`KeyboardLayout`] trait has no dead-key state machine.
///   Consumers needing full dead-key composition should track a pending
///   diacritic themselves from the [`DecodedKey`] output of this layout.
pub struct TrQwerty;

impl KeyboardLayout for TrQwerty {
    #[rustfmt::skip]
    fn map_keycode(
        &self,
        keycode: KeyCode,
        modifiers: &Modifiers,
        handle_ctrl: HandleControl,
    ) -> DecodedKey {
        match keycode {
            // ========= Row 2 (the numbers) =========
            KeyCode::Oem8             => modifiers.handle_symbol3('"', '\u{00E9}', '<'), // " / é / <
            KeyCode::Escape           => DecodedKey::Unicode('\u{001B}'),
            KeyCode::Key1             => modifiers.handle_symbol3('1', '!', '>'),
            KeyCode::Key2             => modifiers.handle_symbol3('2', '\'', '\u{00A3}'), // 2 / ' / £
            KeyCode::Key3             => modifiers.handle_symbol3('3', '^', '#'), // '^' is a dead key on real HW
            KeyCode::Key4             => modifiers.handle_symbol3('4', '+', '$'),
            KeyCode::Key5             => modifiers.handle_symbol3('5', '%', '\u{00BD}'), // 5 / % / ½
            KeyCode::Key6             => modifiers.handle_symbol2('6', '&'),
            KeyCode::Key7             => modifiers.handle_symbol3('7', '/', '{'),
            KeyCode::Key8             => modifiers.handle_symbol3('8', '(', '['),
            KeyCode::Key9             => modifiers.handle_symbol3('9', ')', ']'),
            KeyCode::Key0             => modifiers.handle_symbol3('0', '=', '}'),
            KeyCode::OemMinus         => modifiers.handle_symbol3('*', '?', '\\'), // physical key right of '0'
            KeyCode::OemPlus          => modifiers.handle_symbol3('-', '_', '|'),  // physical key right of that
            KeyCode::Backspace        => DecodedKey::Unicode('\u{0008}'),
            // ========= Row 3 (QWERTY) =========
            KeyCode::Tab              => DecodedKey::Unicode('\u{0009}'),
            KeyCode::Q                => modifiers.handle_ascii_3('Q', '@', handle_ctrl),
            KeyCode::W                => modifiers.handle_ascii_2('W', handle_ctrl),
            KeyCode::E                => modifiers.handle_ascii_3('E', '\u{20AC}', handle_ctrl), // AltGr -> €
            KeyCode::R                => modifiers.handle_ascii_2('R', handle_ctrl),
            KeyCode::T                => modifiers.handle_ascii_3('T', '\u{20BA}', handle_ctrl), // AltGr -> ₺
            KeyCode::Y                => modifiers.handle_ascii_2('Y', handle_ctrl),
            KeyCode::U                => modifiers.handle_ascii_2('U', handle_ctrl),
            // Dotless i: lower 'ı', upper 'I'
            KeyCode::I                => modifiers.handle_letter2('\u{0131}', 'I'),
            KeyCode::O                => modifiers.handle_ascii_2('O', handle_ctrl),
            KeyCode::P                => modifiers.handle_ascii_2('P', handle_ctrl),
            // 'ğ' / 'Ğ', AltGr -> diaeresis mark (dead key on real HW)
            KeyCode::Oem4             => modifiers.handle_symbol3('\u{011F}', '\u{011E}', '\u{00A8}'),
            // 'ü' / 'Ü', AltGr -> tilde mark (dead key on real HW)
            KeyCode::Oem6             => modifiers.handle_symbol3('\u{00FC}', '\u{00DC}', '~'),
            // ISO-only key, left of the 2-row-high Enter: '<' / '>' / '|'
            KeyCode::Oem5             => modifiers.handle_symbol3('<', '>', '|'),
            // ========= Row 4 (ASDFG) =========
            KeyCode::A                => modifiers.handle_ascii_2('A', handle_ctrl),
            KeyCode::S                => modifiers.handle_ascii_3('S', '\u{00DF}', handle_ctrl), // AltGr -> ß
            KeyCode::D                => modifiers.handle_ascii_2('D', handle_ctrl),
            KeyCode::F                => modifiers.handle_ascii_2('F', handle_ctrl),
            KeyCode::G                => modifiers.handle_ascii_2('G', handle_ctrl),
            KeyCode::H                => modifiers.handle_ascii_2('H', handle_ctrl),
            KeyCode::J                => modifiers.handle_ascii_2('J', handle_ctrl),
            KeyCode::K                => modifiers.handle_ascii_2('K', handle_ctrl),
            KeyCode::L                => modifiers.handle_ascii_2('L', handle_ctrl),
            // 'ş' / 'Ş', AltGr -> acute accent mark (dead key on real HW)
            KeyCode::Oem1             => modifiers.handle_symbol3('\u{015F}', '\u{015E}', '\u{00B4}'),
            // Dotted i: lower 'i', upper 'İ' (dotted capital I)
            KeyCode::Oem3             => modifiers.handle_letter2('i', '\u{0130}'),
            // ',' / ';', AltGr -> grave accent mark (dead key on real HW)
            KeyCode::Oem7             => modifiers.handle_symbol3(',', ';', '`'),
            KeyCode::Return           => DecodedKey::Unicode('\u{000A}'),
            // ========= Row 5 (ZXCVB) =========
            KeyCode::Z                => modifiers.handle_ascii_2('Z', handle_ctrl),
            KeyCode::X                => modifiers.handle_ascii_2('X', handle_ctrl),
            KeyCode::C                => modifiers.handle_ascii_2('C', handle_ctrl),
            KeyCode::V                => modifiers.handle_ascii_2('V', handle_ctrl),
            KeyCode::B                => modifiers.handle_ascii_2('B', handle_ctrl),
            KeyCode::N                => modifiers.handle_ascii_2('N', handle_ctrl),
            KeyCode::M                => modifiers.handle_ascii_2('M', handle_ctrl),
            // 'ö' / 'Ö'
            KeyCode::OemComma         => modifiers.handle_symbol2('\u{00F6}', '\u{00D6}'),
            // 'ç' / 'Ç'
            KeyCode::OemPeriod        => modifiers.handle_symbol2('\u{00E7}', '\u{00C7}'),
            // '.' / ':'
            KeyCode::Oem2             => modifiers.handle_symbol2('.', ':'),
            // ========= Unicode Specials =========
            KeyCode::Spacebar         => DecodedKey::Unicode(' '),
            KeyCode::Delete           => DecodedKey::Unicode('\u{007f}'),
            // ========= Numpad =========
            KeyCode::NumpadDivide     => DecodedKey::Unicode('/'),
            KeyCode::NumpadMultiply   => DecodedKey::Unicode('*'),
            KeyCode::NumpadSubtract   => DecodedKey::Unicode('-'),
            KeyCode::Numpad7          => modifiers.handle_num_pad('7', KeyCode::Home),
            KeyCode::Numpad8          => modifiers.handle_num_pad('8', KeyCode::ArrowUp),
            KeyCode::Numpad9          => modifiers.handle_num_pad('9', KeyCode::PageUp),
            KeyCode::NumpadAdd        => DecodedKey::Unicode('+'),
            KeyCode::Numpad4          => modifiers.handle_num_pad('4', KeyCode::ArrowLeft),
            KeyCode::Numpad5          => DecodedKey::Unicode('5'),
            KeyCode::Numpad6          => modifiers.handle_num_pad('6', KeyCode::ArrowRight),
            KeyCode::Numpad1          => modifiers.handle_num_pad('1', KeyCode::End),
            KeyCode::Numpad2          => modifiers.handle_num_pad('2', KeyCode::ArrowDown),
            KeyCode::Numpad3          => modifiers.handle_num_pad('3', KeyCode::PageDown),
            KeyCode::Numpad0          => modifiers.handle_num_pad('0', KeyCode::Insert),
            // Numpad decimal is ',' on this layout (both plain and shifted per KLC)
            KeyCode::NumpadPeriod     => modifiers.handle_num_del(',', ','),
            KeyCode::NumpadEnter      => DecodedKey::Unicode('\u{000A}'),
            // ========= Fallback =========
            k                         => DecodedKey::RawKey(k),
        }
    }

    fn get_physical(&self) -> PhysicalKeyboard {
        PhysicalKeyboard::Iso
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{EventDecoder, ScancodeSet, ScancodeSet1};

    #[test]
    fn layout() {
        // Scancodes taken from https://kbdlayout.info/KBDTUQ/scancodes
        // (Set 1, unshifted values), cross-checked against this crate's own
        // KeyCode <-> Set-1-scancode table in lib.rs.
        let mut s = ScancodeSet1::new();
        let mut dec = EventDecoder::new(TrQwerty, HandleControl::Ignore);
        let data = [
            (0x29, '"'),
            (0x02, '1'),
            (0x03, '2'),
            (0x04, '3'),
            (0x05, '4'),
            (0x06, '5'),
            (0x07, '6'),
            (0x08, '7'),
            (0x09, '8'),
            (0x0a, '9'),
            (0x0b, '0'),
            (0x0c, '*'),
            (0x0d, '-'),
            (0x0f, '\t'),
            (0x10, 'q'),
            (0x11, 'w'),
            (0x12, 'e'),
            (0x13, 'r'),
            (0x14, 't'),
            (0x15, 'y'),
            (0x16, 'u'),
            (0x17, '\u{0131}'), // ı
            (0x18, 'o'),
            (0x19, 'p'),
            (0x1a, '\u{011F}'), // ğ
            (0x1b, '\u{00FC}'), // ü
            (0x1e, 'a'),
            (0x1f, 's'),
            (0x20, 'd'),
            (0x21, 'f'),
            (0x22, 'g'),
            (0x23, 'h'),
            (0x24, 'j'),
            (0x25, 'k'),
            (0x26, 'l'),
            (0x27, '\u{015F}'), // ş
            (0x28, 'i'),
            (0x2b, ','),
            (0x1c, '\n'),
            (0x2c, 'z'),
            (0x2d, 'x'),
            (0x2e, 'c'),
            (0x2f, 'v'),
            (0x30, 'b'),
            (0x31, 'n'),
            (0x32, 'm'),
            (0x33, '\u{00F6}'), // ö
            (0x34, '\u{00E7}'), // ç
            (0x35, '.'),
            (0x56, '<'),
        ];
        for (code, unicode) in data {
            let ev = s.advance_state(code).unwrap().unwrap();
            assert_eq!(Some(DecodedKey::Unicode(unicode)), dec.process_keyevent(ev));
        }
    }

    #[test]
    fn dotless_i_shift() {
        let modifiers = Modifiers {
            lshift: true,
            ..Default::default()
        };
        assert_eq!(
            modifiers.handle_letter2('\u{0131}', 'I'),
            DecodedKey::Unicode('I')
        );
    }

    #[test]
    fn dotted_i_shift() {
        let modifiers = Modifiers {
            lshift: true,
            ..Default::default()
        };
        assert_eq!(
            modifiers.handle_letter2('i', '\u{0130}'),
            DecodedKey::Unicode('\u{0130}')
        );
    }

    #[test]
    fn altgr_at_sign() {
        let modifiers = Modifiers {
            ralt: true,
            ..Default::default()
        };
        assert_eq!(
            modifiers.handle_ascii_3('Q', '@', HandleControl::Ignore),
            DecodedKey::Unicode('@')
        );
    }

    #[test]
    fn altgr_euro() {
        let modifiers = Modifiers {
            ralt: true,
            ..Default::default()
        };
        assert_eq!(
            modifiers.handle_ascii_3('E', '\u{20AC}', HandleControl::Ignore),
            DecodedKey::Unicode('\u{20AC}')
        );
    }

    #[test]
    fn ctrl_letter() {
        let modifiers = Modifiers {
            lctrl: true,
            ..Default::default()
        };
        assert_eq!(
            modifiers.handle_ascii_2('C', HandleControl::MapLettersToUnicode),
            DecodedKey::Unicode('\u{0003}')
        );
    }
}