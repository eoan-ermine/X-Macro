use inputbot::MouseButton;

use super::keybd_key::FromName;

impl FromName for MouseButton {
    fn try_from(name: &str) -> Result<Self, ()>
    where
        Self: Sized,
    {
        Ok(match name {
            "LeftButton" => MouseButton::LeftButton,
            "MiddleButton" => MouseButton::MiddleButton,
            "RightButton" => MouseButton::RightButton,
            "X1Button" => MouseButton::X1Button,
            "X2Button" => MouseButton::X2Button,
            other => {
                if other.starts_with("OtherButton") {
                    let other = other.strip_prefix("OtherButton(").ok_or(())?;
                    let other = other.strip_suffix(")").ok_or(())?;

                    let key_code = other.parse::<u32>().map_err(|_| ())?;
                    MouseButton::OtherButton(key_code)
                } else {
                    return Err(());
                }
            }
        })
    }
}
