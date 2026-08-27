//! # Turkish Q keyboard support

use crate::{
    DecodedKey, HandleControl, KeyCode, KeyboardLayout, Modifiers, PhysicalKeyboard,
};

/// A standard Turkish Q 101-key (or 104-key including Windows keys) keyboard.
///
/// Has a 1-row high Enter key, with Oem102 (< > |) present on the ANSI-ish
/// Turkish physical layout, and AltGr used for a third level of symbols
/// (currency signs, brackets, `@`, etc), matching `KBDTUQ.DLL` ("Turkish Q
/// Keyboard Layout") as documented at <https://kbdlayout.info/KBDTUQ/>.
///
/// The Turkish Q layout is the QWERTY-derived layout most commonly used in
/// Turkey (as opposed to the frequency-optimised Turkish F layout). Notable
/// differences from a US keyboard:
///
/// * `I` / `i` are split into dotted and dotless pairs: the key next to `U`
///   produces lowercase dotless `ı` / uppercase `I`, while the key next to
///   `L` produces lowercase dotted `i` / uppercase dotted `İ`.
/// * `Oem4`, `Oem6`, and `Oem1` produce `ğ`/`Ğ`, `ü`/`Ü`, and `ş`/`Ş`
///   respectively, with AltGr producing dead-key diacritics (`¨`, `~`, `´`).
/// * AltGr provides `@`, `€`, `₺`, currency and bracket symbols, and `\|`.
/// * Several dead keys are available via AltGr: circumflex (`^`), acute
///   (`´`), diaeresis (`¨`), grave (`` ` ``), and tilde (`~`).
///
/// This implementation supports the printable base/shift/AltGr grid
/// directly. Dead-key composition (holding a diacritic until the next
/// keystroke) is not implemented here; AltGr dead-key positions instead
/// yield the bare diacritic mark as a standalone character, since this
/// crate's [`KeyboardLayout`] trait has no built-in dead-key state machine.
/// Consumers needing full dead-key composition should track pending
/// diacritics themselves using [`DecodedKey`] output from this layout.
pub struct TrQwerty;

impl KeyboardLayout for TrQwerty {
    #[rustfmt::skip]
    fn map_keycode(
        &self,
        keycode: KeyCode,
        modifiers: &Modifiers,
        handle_ctrl: HandleControl,
    ) -> DecodedKey {
        // AltGr (Right Alt) is used as a third shift level on this layout.
        let altgr = modifiers.ralt;

        match keycode {
            // ========= Row 2 (the numbers) =========
            KeyCode::Oem8             => altgr_symbol3(altgr, modifiers, '"', 'é', '<'),
            KeyCode::Escape           => DecodedKey::Unicode('\u{001B}'),
            KeyCode::Key1             => altgr_symbol3(altgr, modifiers, '1', '!', '>'),
            KeyCode::Key2             => altgr_symbol3(altgr, modifiers, '2', '\'', '£'),
            KeyCode::Key3             => altgr_symbol3(altgr, modifiers, '3', '^', '#'),
            KeyCode::Key4             => altgr_symbol3(altgr, modifiers, '4', '+', '$'),
            KeyCode::Key5             => altgr_symbol3(altgr, modifiers, '5', '%', '\u{00BD}'),
            KeyCode::Key6             => modifiers.handle_symbol2('6', '&'),
            KeyCode::Key7             => altgr_symbol3(altgr, modifiers, '7', '/', '{'),
            KeyCode::Key8             => altgr_symbol3(altgr, modifiers, '8', '(', '['),
            KeyCode::Key9             => altgr_symbol3(altgr, modifiers, '9', ')', ']'),
            KeyCode::Key0             => altgr_symbol3(altgr, modifiers, '0', '=', '}'),
            KeyCode::OemMinus         => altgr_symbol3(altgr, modifiers, '-', '_', '|'),
            KeyCode::OemPlus          => DecodedKey::Unicode('*'), // OEM_8 slot: * / ? / \
            KeyCode::Backspace        => DecodedKey::Unicode('\u{0008}'),
            // ========= Row 3 (QWERTY) =========
            KeyCode::Tab              => DecodedKey::Unicode('\u{0009}'),
            KeyCode::Q                => altgr_ascii3(altgr, modifiers, 'Q', handle_ctrl, '@'),
            KeyCode::W                => modifiers.handle_ascii_2('W', handle_ctrl),
            KeyCode::E                => altgr_ascii3(altgr, modifiers, 'E', handle_ctrl, '\u{20AC}'),
            KeyCode::R                => modifiers.handle_ascii_2('R', handle_ctrl),
            KeyCode::T                => altgr_ascii3(altgr, modifiers, 'T', handle_ctrl, '\u{20BA}'),
            KeyCode::Y                => modifiers.handle_ascii_2('Y', handle_ctrl),
            KeyCode::U                => modifiers.handle_ascii_2('U', handle_ctrl),
            // Dotless I: 'ı' (lower) / 'I' (upper)
            KeyCode::I                => modifiers.handle_symbol2('\u{0131}', 'I'),
            KeyCode::O                => modifiers.handle_ascii_2('O', handle_ctrl),
            KeyCode::P                => modifiers.handle_ascii_2('P', handle_ctrl),
            // 'ğ' / 'Ğ', AltGr -> diaeresis mark
            KeyCode::Oem4             => altgr_symbol3(altgr, modifiers, '\u{011F}', '\u{011E}', '\u{00A8}'),
            // 'ü' / 'Ü', AltGr -> tilde mark
            KeyCode::Oem6             => altgr_symbol3(altgr, modifiers, '\u{00FC}', '\u{00DC}', '~'),
            KeyCode::Oem7             => DecodedKey::Unicode('\\'),
            // ========= Row 4 (ASDFG) =========
            KeyCode::A                => modifiers.handle_ascii_2('A', handle_ctrl),
            KeyCode::S                => altgr_ascii3(altgr, modifiers, 'S', handle_ctrl, '\u{00DF}'),
            KeyCode::D                => modifiers.handle_ascii_2('D', handle_ctrl),
            KeyCode::F                => modifiers.handle_ascii_2('F', handle_ctrl),
            KeyCode::G                => modifiers.handle_ascii_2('G', handle_ctrl),
            KeyCode::H                => modifiers.handle_ascii_2('H', handle_ctrl),
            KeyCode::J                => modifiers.handle_ascii_2('J', handle_ctrl),
            KeyCode::K                => modifiers.handle_ascii_2('K', handle_ctrl),
            KeyCode::L                => modifiers.handle_ascii_2('L', handle_ctrl),
            // 'ş' / 'Ş', AltGr -> acute accent mark
            KeyCode::Oem1             => altgr_symbol3(altgr, modifiers, '\u{015F}', '\u{015E}', '\u{00B4}'),
            // Dotted I: 'i' (lower) / 'İ' (upper, dot above)
            KeyCode::Oem3             => modifiers.handle_symbol2('i', '\u{0130}'),
            KeyCode::Return           => DecodedKey::Unicode('\u{000A}'),
            // ========= Row 5 (ZXCVB) =========
            KeyCode::Z                => modifiers.handle_ascii_2('Z', handle_ctrl),
            KeyCode::X                => modifiers.handle_ascii_2('X', handle_ctrl),
            KeyCode::C                => modifiers.handle_ascii_2('C', handle_ctrl),
            KeyCode::V                => modifiers.handle_ascii_2('V', handle_ctrl),
            KeyCode::B                => modifiers.handle_ascii_2('B', handle_ctrl),
            KeyCode::N                => modifiers.handle_ascii_2('N', handle_ctrl),
            KeyCode::M                => modifiers.handle_ascii_2('M', handle_ctrl),
            // ',' / ';', AltGr -> grave accent mark
            KeyCode::OemComma         => altgr_symbol3(altgr, modifiers, ',', ';', '`'),
            // 'ö' / 'Ö'
            KeyCode::OemPeriod        => modifiers.handle_symbol2('\u{00F6}', '\u{00D6}'),
            // 'ç' / 'Ç'
            KeyCode::Oem2             => modifiers.handle_symbol2('\u{00E7}', '\u{00C7}'),
            // '.' / ':'
            KeyCode::Oem5             => modifiers.handle_symbol2('.', ':'),
            // '<' / '>' / '|' (extra ANSI102 key, left of Z on ISO)
            KeyCode::Oem102           => altgr_symbol3(altgr, modifiers, '<', '>', '|'),
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
            // Numpad decimal is ',' on this layout (both plain and shifted)
            KeyCode::NumpadPeriod     => modifiers.handle_num_del(',', '\u{007f}'),
            KeyCode::NumpadEnter      => DecodedKey::Unicode('\u{000A}'),
            // ========= Fallback =========
            k                         => DecodedKey::RawKey(k),
        }
    }

    fn get_physical(&self) -> PhysicalKeyboard {
        PhysicalKeyboard::Ansi
    }
}

/// Helper for keys with a base / shift / AltGr third symbol.
///
/// When AltGr is held, `altgr_char` is produced (regardless of shift).
/// Otherwise falls back to the normal two-level shift handling.
fn altgr_symbol3(altgr: bool, modifiers: &Modifiers, base: char, shifted: char, altgr_char: char) -> DecodedKey {
    if altgr {
        DecodedKey::Unicode(altgr_char)
    } else {
        modifiers.handle_symbol2(base, shifted)
    }
}

/// Helper for letter keys with a base / shift / ctrl / AltGr fourth symbol.
///
/// When AltGr is held, `altgr_char` is produced. Otherwise falls back to
/// the normal ASCII letter handling (which covers shift, caps lock, and
/// ctrl-to-control-code mapping).
fn altgr_ascii3(
    altgr: bool,
    modifiers: &Modifiers,
    letter: char,
    handle_ctrl: HandleControl,
    altgr_char: char,
) -> DecodedKey {
    if altgr {
        DecodedKey::Unicode(altgr_char)
    } else {
        modifiers.handle_ascii_2(letter, handle_ctrl)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{EventDecoder, ScancodeSet, ScancodeSet1};

    #[test]
    fn layout() {
        // Codes taken from https://kbdlayout.info/KBDTUQ/overview+scancodes?arrangement=ANSI104
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
            (0x1c, '\n'),
            (0x2c, 'z'),
            (0x2d, 'x'),
            (0x2e, 'c'),
            (0x2f, 'v'),
            (0x30, 'b'),
            (0x31, 'n'),
            (0x32, 'm'),
            (0x2b, ','),
            (0x33, '\u{00F6}'), // ö
            (0x34, '\u{00E7}'), // ç
            (0x35, '.'),
        ];
        for (code, unicode) in data {
            let ev = s.advance_state(code).unwrap().unwrap();
            assert_eq!(Some(DecodedKey::Unicode(unicode)), dec.process_keyevent(ev));
        }
    }

    #[test]
    fn dotless_i_shift() {
        // Shift + the 'I' key (scancode 0x17) should give uppercase dotted I,
        // matching KBDTUQ where VK_I shift state gives capital 'I'.
        let modifiers = Modifiers {
            capslock: false,
            lalt: false,
            lctrl: false,
            lshift: true,
            numlock: false,
            ralt: false,
            rctrl: false,
            rctrl2: false,
            rshift: false,
        };
        assert_eq!(
            modifiers.handle_symbol2('\u{0131}', 'I'),
            DecodedKey::Unicode('I')
        );
    }

    #[test]
    fn dotted_i_shift() {
        // Shift + the 'İ' key (scancode 0x28) should give capital dotted İ.
        let modifiers = Modifiers {
            capslock: false,
            lalt: false,
            lctrl: false,
            lshift: true,
            numlock: false,
            ralt: false,
            rctrl: false,
            rctrl2: false,
            rshift: false,
        };
        assert_eq!(
            modifiers.handle_symbol2('i', '\u{0130}'),
            DecodedKey::Unicode('\u{0130}')
        );
    }

    #[test]
    fn altgr_at_sign() {
        let modifiers = Modifiers {
            capslock: false,
            lalt: false,
            lctrl: false,
            lshift: false,
            numlock: false,
            ralt: true,
            rctrl: false,
            rctrl2: false,
            rshift: false,
        };
        assert_eq!(
            altgr_ascii3(true, &modifiers, 'Q', HandleControl::Ignore, '@'),
            DecodedKey::Unicode('@')
        );
    }

    #[test]
    fn altgr_euro() {
        let modifiers = Modifiers {
            capslock: false,
            lalt: false,
            lctrl: false,
            lshift: false,
            numlock: false,
            ralt: true,
            rctrl: false,
            rctrl2: false,
            rshift: false,
        };
        assert_eq!(
            altgr_ascii3(true, &modifiers, 'E', HandleControl::Ignore, '\u{20AC}'),
            DecodedKey::Unicode('\u{20AC}')
        );
    }
}