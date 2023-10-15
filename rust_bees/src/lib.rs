use godot::engine::RandomNumberGenerator;
use godot::engine::Sprite2D;
use godot::engine::Sprite2DVirtual;
use godot::prelude::*;

struct RustBees;

#[gdextension]
unsafe impl ExtensionLibrary for RustBees {}

#[derive(GodotClass)]
#[class(base=Sprite2D)]
struct BeeRust {
    #[base]
    sprite: Base<Sprite2D>,
    #[export]
    speed: f32,
    #[export]
    radius: f32,

    start_point: Vector2,
    next_point: Vector2,

    random: Gd<RandomNumberGenerator>,
}

#[godot_api]
impl Sprite2DVirtual for BeeRust {
    fn init(sprite: Base<Sprite2D>) -> Self {
        Self {
            sprite,
            speed: 400.0,
            radius: 300.0,
            start_point: Vector2::ZERO,
            next_point: Vector2::ZERO,
            random: RandomNumberGenerator::new(),
        }
    }
    fn ready(&mut self) {
        self.start_point = self.sprite.get_position();
        self.next_point = self.sprite.get_position();
    }
    fn physics_process(&mut self, delta: f64) {
        let p = self.sprite.get_position();
        if p.distance_squared_to(self.next_point) < 144.0 {
            let angle_rad = self
                .random
                .randf_range(-std::f32::consts::PI, std::f32::consts::PI);
            self.next_point = self.start_point
                + Vector2::new(angle_rad.cos(), angle_rad.sin())
                    * self.random.randf_range(0.0, self.radius);
        }
        let angle = p.direction_to(self.next_point);
        let speed = self.speed * (delta as f32);
        let p = p + angle * speed;
        self.sprite.set_position(p);
    }
}

#[godot_api]
impl BeeRust {}
