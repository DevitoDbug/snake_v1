pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

pub fn _rect_vs_rect_collided(rect1: Rect, rect2: Rect) -> bool {
    if rect1.y == rect2.y {
        let left_bound = rect1.x;
        let right_bound = rect1.x + rect1.width;

        if left_bound <= rect2.x && right_bound > rect2.x {
            return true;
        }

        return false;
    }

    if rect1.x == rect2.x {
        let top_bound = rect1.y;
        let bottom_bound = rect1.y + rect1.height;

        if bottom_bound > rect2.y && top_bound <= rect2.y {
            return true;
        }

        return false;
    }

    return false;
}
