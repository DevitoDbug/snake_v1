pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

pub fn _rect_vs_rect_collided(rect1: Rect, rect2: Rect) -> bool {
    if rect1.y == rect2.y {
        if rect1.x + rect1.width >= rect2.x {
            println!("1");
            return true;
        }
        if rect1.x - rect1.width <= rect2.x {
            println!("2");
            return true;
        }

        return false;
    }

    if rect1.x == rect2.x {
        if rect1.y + rect1.height >= rect2.y {
            println!("4");
            return true;
        }
        if rect1.y - rect1.height <= rect2.y {
            println!("5");
            return true;
        }
        return false;
    }

    return false;
}
