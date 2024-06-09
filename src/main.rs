use raylib::prelude::*;
use raylib::consts::KeyboardKey::*;

struct QuadraticFunction {
    a: f32,
    b: f32,
    c: f32,
}

impl QuadraticFunction {
    fn fx(&self, x: f32) -> f32 {
        (self.a * ( x * x )) + (self.b * x) + self.c
    }
    fn fdx(&self, x: f32) -> f32 {
        (2.0 * self.a * x) + self.b
    }
}

struct CubicFunction {
    a: f32,
    b: f32,
    c: f32,
    d: f32,
}

impl CubicFunction {
    fn fx(&self, x: f32) -> f32 {
        (self.a * (x * x * x)) + (self.b *  (x * x)) + (self.c * x) + self.d
    }
    fn fdx(&self, x: f32) -> f32 {
        (3.0 * (self.a * (x * x))) + (2.0 * (self.b * x)) + self.c
    }
}

const SCREEN_WIDTH: f32 = 1280.0;
const SCREEN_HEIGHT: f32 = 960.0;
const X_SCALE: f32 = 20.0;
const Y_SCALE: f32 = 5.0;

fn main() {
    println!("Hello Newton Raphson");

    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32)
        .title("Hello Newton Raphson")
        .build();

    let fun = CubicFunction {
        a: 1.0, b: 1.0, c: 0.0, d: 0.0
    };

    let mut x: f32 = 5.0;
    let mut x2: f32 = -5.0;
    while !rl.window_should_close() {
        /* UPDATE STATE */
        if rl.is_key_pressed(KEY_SPACE) {
            x = x - (fun.fx(x) / fun.fdx(x));
            x2 = x2 - (fun.fx(x2) / fun.fdx(x2));
        }
        if rl.is_key_pressed(KEY_R) {
            x = 5.0;
            x2 = -5.0;
        }
        let y = fun.fx(x);
        let y2 = fun.fx(x2);

        /* DRAW STATE */
        let mut d = rl.begin_drawing(&thread);

        let mut iter_x: f32 = -10.0;
        let dx: f32 = 0.1;
        
        while iter_x <= 10.0 {
            let p1x: f32 = iter_x;
            let p1y: f32 = fun.fx(p1x);
            let p2x: f32 = p1x + dx;
            let p2y: f32 = fun.fx(p2x);
            d.draw_line(translate_x(p1x), translate_y(p1y),
                        translate_x(p2x), translate_y(p2y),
                        Color::PURPLE);
            iter_x += dx;
        }

        d.clear_background(Color::BLACK);
        d.draw_text("Newton Raphson", 12, 12, 24, Color::WHITE);
        d.draw_text(format!("x: {} y: {}", x, y).as_str(), 12, 36, 24, Color::WHITE);
        d.draw_text(format!("x: {} y: {}", x2, y2).as_str(), 12, 60, 24, Color::WHITE);

        let center_x = translate_x(x);
        let center_x2 = translate_x(x2);
        let center_y = translate_y(y);
        let center_y2 = translate_y(y2);
        let mut color = Color::RED;
        if y2 == 0.0 && y == 0.0 {
            color = Color::GREEN;
        }
        d.draw_line((SCREEN_WIDTH / 2.0) as i32, 0, (SCREEN_WIDTH / 2.0) as i32, SCREEN_HEIGHT as i32, Color::YELLOW);
        d.draw_line(0, (SCREEN_HEIGHT / 2.0) as i32, SCREEN_WIDTH as i32, (SCREEN_HEIGHT / 2.0) as i32, Color::YELLOW);
        let ball_radius = 5.0;
        d.draw_ellipse(center_x as i32, center_y as i32, ball_radius, ball_radius, color);
        d.draw_ellipse(center_x2 as i32, center_y2 as i32, ball_radius, ball_radius, color);
    }
}

fn translate_x(x: f32) -> i32 {
    ((X_SCALE * x) + (SCREEN_WIDTH / 2.0)) as i32
}

fn translate_y(y: f32) -> i32 {
    ((-1.0 * Y_SCALE * y) + (SCREEN_HEIGHT / 2.0)) as i32
}
