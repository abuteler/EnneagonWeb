#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Color {
  Blue,
  Crimson,
  DeepPink,
  Gold,
  Green,
  LightBlue,
  LightSeaGreen,
  Orange,
  Violet,
  Yellow,
}
/*
  Some(Color::Violet) => format!("{} bg-[rgb(150,0,160)]", base_style), // rgb(199 21 133)
  Some(Color::DeepPink) => format!("{} bg-[rgb()]", base_style),
*/
impl Color {
  pub fn to_rgb_str(&self) -> &str {
    match self {
      Color::Blue => "bg-[rgb(0,70,210)]",
      Color::Crimson => "bg-[rgb(237,20,61)]",
      Color::DeepPink => "bg-[rgb(255,20,147)]",
      Color::Gold => "bg-[rgb(255,215,0)]",
      Color::Green => "bg-[rgb(0,150,0)]",
      Color::LightBlue => "bg-[rgb(120,180,250)]",
      Color::LightSeaGreen => "bg-[rgb(32,178,170)]",
      Color::Orange => "bg-[rgb(255,165,0)]",
      Color::Violet => "bg-[rgb(190,80,210)]",
      Color::Yellow => "bg-[rgb(240,200,50)]",
    }
  }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub enum CellState {
  #[default] Empty,
  Solid,
  Fluid,
  Exploding,
}

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Cell {
  pub coordinates: (usize, usize),
  pub color: Option<Color>,
  pub state: CellState
}

impl Cell {
  pub fn new(column: usize, row: usize, color: Option<Color>, state: CellState) -> Self {
    Self {
      coordinates: (column, row),
      color,
      state,
    }
  }
  pub fn reset_state(&mut self) {
    self.color = None;
    self.state = CellState::Empty;
  }
}
