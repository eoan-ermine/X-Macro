use inputbot::MouseButton;
use pest::iterators::Pair;

use crate::parser::entities::{Parse, Rule};

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

impl Parse for MouseButton {
    fn parse(pair: Pair<Rule>) -> Self {
        let name = pair.as_str();
        MouseButton::try_from(name).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use pest::Parser;
    use crate::parser::entities::{GrammarParser, Rule, Parse};
    use inputbot::MouseButton;

    #[test]
    fn keybd_key() {
        assert_eq!(
            MouseButton::parse(GrammarParser::parse(Rule::mouse_button, "MiddleButton").unwrap().next().unwrap()),
            MouseButton::MiddleButton
        );

        assert_eq!(
            MouseButton::parse(GrammarParser::parse(Rule::mouse_button, "OtherButton(256)").unwrap().next().unwrap()),
            MouseButton::OtherButton(256)
        );

    }
}
