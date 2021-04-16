use inputbot::KeybdKey;

pub(crate) trait FromName {
    fn try_from(name: &str) -> Result<Self, ()>
    where
        Self: Sized;
}
impl FromName for KeybdKey {
    fn try_from(name: &str) -> Result<Self, ()>
    where
        Self: Sized,
    {
        Ok(match name {
            "BackspaceKey" => KeybdKey::BackspaceKey,
            "TabKey" => KeybdKey::TabKey,
            "EnterKey" => KeybdKey::EnterKey,
            "EscapeKey" => KeybdKey::EscapeKey,
            "SpaceKey" => KeybdKey::SpaceKey,
            "HomeKey" => KeybdKey::HomeKey,
            "LeftKey" => KeybdKey::LeftKey,
            "UpKey" => KeybdKey::UpKey,
            "RightKey" => KeybdKey::RightKey,
            "DownKey" => KeybdKey::DownKey,
            "InsertKey" => KeybdKey::InsertKey,
            "DeleteKey" => KeybdKey::DeleteKey,
            "Numrow0Key" => KeybdKey::Numrow0Key,
            "Numrow1Key" => KeybdKey::Numrow1Key,
            "Numrow2Key" => KeybdKey::Numrow2Key,
            "Numrow3Key" => KeybdKey::Numrow3Key,
            "Numrow4Key" => KeybdKey::Numrow4Key,
            "Numrow5Key" => KeybdKey::Numrow5Key,
            "Numrow6Key" => KeybdKey::Numrow6Key,
            "Numrow7Key" => KeybdKey::Numrow7Key,
            "Numrow8Key" => KeybdKey::Numrow8Key,
            "Numrow9Key" => KeybdKey::Numrow9Key,
            "AKey" => KeybdKey::AKey,
            "BKey" => KeybdKey::BKey,
            "CKey" => KeybdKey::CKey,
            "DKey" => KeybdKey::DKey,
            "EKey" => KeybdKey::EKey,
            "FKey" => KeybdKey::FKey,
            "GKey" => KeybdKey::GKey,
            "HKey" => KeybdKey::HKey,
            "IKey" => KeybdKey::IKey,
            "JKey" => KeybdKey::JKey,
            "KKey" => KeybdKey::KKey,
            "LKey" => KeybdKey::LKey,
            "MKey" => KeybdKey::MKey,
            "NKey" => KeybdKey::NKey,
            "OKey" => KeybdKey::OKey,
            "PKey" => KeybdKey::PKey,
            "QKey" => KeybdKey::QKey,
            "RKey" => KeybdKey::RKey,
            "SKey" => KeybdKey::SKey,
            "TKey" => KeybdKey::TKey,
            "UKey" => KeybdKey::UKey,
            "VKey" => KeybdKey::VKey,
            "WKey" => KeybdKey::WKey,
            "XKey" => KeybdKey::XKey,
            "YKey" => KeybdKey::YKey,
            "ZKey" => KeybdKey::ZKey,
            "Numpad0Key" => KeybdKey::Numpad0Key,
            "Numpad1Key" => KeybdKey::Numpad1Key,
            "Numpad2Key" => KeybdKey::Numpad2Key,
            "Numpad3Key" => KeybdKey::Numpad3Key,
            "Numpad4Key" => KeybdKey::Numpad4Key,
            "Numpad5Key" => KeybdKey::Numpad5Key,
            "Numpad6Key" => KeybdKey::Numpad6Key,
            "Numpad7Key" => KeybdKey::Numpad7Key,
            "Numpad8Key" => KeybdKey::Numpad8Key,
            "Numpad9Key" => KeybdKey::Numpad9Key,
            "F1Key" => KeybdKey::F1Key,
            "F2Key" => KeybdKey::F2Key,
            "F3Key" => KeybdKey::F3Key,
            "F4Key" => KeybdKey::F4Key,
            "F5Key" => KeybdKey::F5Key,
            "F6Key" => KeybdKey::F6Key,
            "F7Key" => KeybdKey::F7Key,
            "F8Key" => KeybdKey::F8Key,
            "F9Key" => KeybdKey::F9Key,
            "F10Key" => KeybdKey::F10Key,
            "F11Key" => KeybdKey::F11Key,
            "F12Key" => KeybdKey::F12Key,
            "F13Key" => KeybdKey::F13Key,
            "F14Key" => KeybdKey::F14Key,
            "F15Key" => KeybdKey::F15Key,
            "F16Key" => KeybdKey::F16Key,
            "F17Key" => KeybdKey::F17Key,
            "F18Key" => KeybdKey::F18Key,
            "F19Key" => KeybdKey::F19Key,
            "F20Key" => KeybdKey::F20Key,
            "F21Key" => KeybdKey::F21Key,
            "F22Key" => KeybdKey::F22Key,
            "F23Key" => KeybdKey::F23Key,
            "F24Key" => KeybdKey::F24Key,
            "NumLockKey" => KeybdKey::NumLockKey,
            "ScrollLockKey" => KeybdKey::ScrollLockKey,
            "CapsLockKey" => KeybdKey::CapsLockKey,
            "LShiftKey" => KeybdKey::LShiftKey,
            "RShiftKey" => KeybdKey::RShiftKey,
            "LControlKey" => KeybdKey::LControlKey,
            "RControlKey" => KeybdKey::RControlKey,
            other => {
                if other.starts_with("OtherKey") {
                    let other = other.strip_prefix("OtherKey(").ok_or(())?;
                    let other = other.strip_suffix(")").ok_or(())?;

                    let key_code = other.parse::<u64>().map_err(|_| ())?;
                    KeybdKey::OtherKey(key_code)
                } else {
                    return Err(());
                }
            }
        })
    }
}
