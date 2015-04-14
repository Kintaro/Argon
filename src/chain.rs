pub struct Chain {
    anchor: Point,
    color: StoneColor,
    stones: PointList,
    liberties: PointList
}

impl Chain {
    pub fn new(point: Point, color: StoneColor) -> Chain {
        Chain {
            anchor: point,
            color: color,
            stones: vec!(point),
            liberties: Vec::new()
        }
    }

    pub fn color(&self) -> StoneColor {
        self.color
    }

    pub fn stones<'a>(&'a self) -> &'a PointList {
        &self.stones
    }

    pub fn stones_mut<'a>(&'a mut self) -> &'a PointList {
        &mut self.stones
    }
}
