use crate::Arrow;

pub struct ColoredKeys {
    arrow_u_pressed: bool,
    arrow_d_pressed: bool,
    arrow_l_pressed: bool,
    arrow_r_pressed: bool,
}

impl ColoredKeys {
    pub fn default() -> ColoredKeys {
        ColoredKeys {
            arrow_u_pressed: false,
            arrow_d_pressed: false,
            arrow_l_pressed: false,
            arrow_r_pressed: false,
        }
    }

    pub fn press(&mut self, arrow: Arrow) {
        match arrow {
            Arrow::Up => self.arrow_u_pressed = true,
            Arrow::Down => self.arrow_d_pressed = true,
            Arrow::Left => self.arrow_l_pressed = true,
            Arrow::Right => self.arrow_r_pressed = true,
        }
    }

    pub fn release(&mut self, arrow: Arrow) {
        match arrow {
            Arrow::Up => self.arrow_u_pressed = false,
            Arrow::Down => self.arrow_d_pressed = false,
            Arrow::Left => self.arrow_l_pressed = false,
            Arrow::Right => self.arrow_r_pressed = false,
        }
    }

    pub fn get_state(&self, arrow: Arrow) -> bool {
        match arrow {
            Arrow::Up => self.arrow_u_pressed,
            Arrow::Down => self.arrow_d_pressed,
            Arrow::Left => self.arrow_l_pressed,
            Arrow::Right => self.arrow_r_pressed,
        }
    }
}
