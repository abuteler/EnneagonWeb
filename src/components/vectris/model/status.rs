
use std::fmt;

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub enum Status {
    #[default]
    InMenus,
    Paused,
    Playing,
    GameOver,
}
impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Status::InMenus => write!(f, "InMenus"),
            Status::Paused => write!(f, "Paused"),
            Status::Playing => write!(f, "Playing"),
            Status::GameOver => write!(f, "GameOver"),
        }
    }
}
