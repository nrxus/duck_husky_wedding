pub mod game_data;
mod player;

use errors::*;
use self::player::Player;
use self::game_data::GameData;

use glm;
use moho::errors as moho_errors;
use moho::shape::{Rectangle, Shape};
use moho::input_manager::{InputManager, EventPump};
use moho::renderer::{Font, ColorRGBA, FontDetails, FontTexturizer, FontLoader, Renderer,
                     ResourceLoader, ResourceManager, Scene, Show, Texture};
use moho::timer::Timer;
use sdl2::mouse::MouseButton;

use std::time::Duration;
use std::rc::Rc;

pub struct Button<F, T> {
    font: Rc<F>,
    text: &'static str,
    is_hovering: bool,
    body: Rectangle,
    on_click: Box<FnMut(&mut Player<T>) -> ()>,
}

impl<F: Font, T> Button<F, T> {
    fn new(text: &'static str,
           font: Rc<F>,
           tl: glm::UVec2,
           on_click: Box<FnMut(&mut Player<T>)>)
           -> Self {
        let dims = font.measure(text).unwrap();
        let body = Rectangle {
            top_left: glm::to_dvec2(tl),
            dims: glm::to_dvec2(dims),
        };
        Button {
            font: font,
            text: text,
            is_hovering: false,
            body: body,
            on_click: on_click,
        }
    }

    fn update<E: EventPump>(&mut self, input_manager: &InputManager<E>, player: &mut Player<T>) {
        let mouse = input_manager.mouse_coords();
        self.is_hovering = self.body.contains(&glm::to_dvec2(mouse));
        if self.is_hovering && input_manager.did_click_mouse(MouseButton::Left) {
            (self.on_click)(player);
        }
    }
}

impl<'f, F, T, R> Scene<R> for Button<F, T>
    where F: Font,
          T: Texture,
          R: FontTexturizer<'f, Font = F, Texture = T> + Renderer<Texture = T>
{
    fn show(&self, renderer: &mut R) -> moho_errors::Result<()> {
        let color = if self.is_hovering {
            ColorRGBA(255, 255, 0, 0)

        } else {
            ColorRGBA(255, 255, 255, 0)
        };
        let texture = renderer.texturize(&self.font, self.text, color)?;
        let dst_rect = glm::to_ivec4(glm::dvec4(self.body.top_left.x,
                                                self.body.top_left.y,
                                                self.body.dims.x,
                                                self.body.dims.y));
        renderer.copy(&texture, Some(dst_rect), None)
    }
}

pub struct DuckHuskyWedding<E, R, T, F>
    where E: EventPump
{
    input_manager: InputManager<E>,
    title: T,
    player: Player<T>,
    renderer: R,
    font_manager: ResourceManager<F, FontDetails>,
    button: Button<F, T>,
}

impl<'f, E, R, T, F> DuckHuskyWedding<E, R, T, F>
    where E: EventPump,
          T: Texture,
          R: Renderer<Texture = T> + FontTexturizer<'f, Font = F, Texture = T>,
          F: Font
{
    pub fn load<FL>(renderer: R,
                    font_loader: &'f FL,
                    input_manager: InputManager<E>,
                    game_data: GameData)
                    -> Result<Self>
        where FL: FontLoader<'f, Font = F>,
              R: for<'a> ResourceLoader<Texture = T>
    {
        let mut texture_manager: ResourceManager<T, String> = ResourceManager::new();
        let mut font_manager: ResourceManager<F, FontDetails> = ResourceManager::new();
        let font_details = FontDetails {
            path: "media/fonts/kenpixel_mini.ttf",
            size: 64,
        };
        let font = font_manager.load(&font_details, font_loader)?;
        let title_color = ColorRGBA(255, 255, 0, 255);
        let title = renderer.texturize(&font, "Husky <3's Ducky", title_color)?;
        let file_name: &str = &format!("media/sprites/{}", game_data.duck.file_name);
        let texture = texture_manager.load(file_name, &renderer)?;
        let player = Player::new(game_data.duck, texture);
        let button = Button::new("click me",
                                 font.clone(),
                                 glm::uvec2(100, 100),
                                 Box::new(|p| p.flip()));
        let game = DuckHuskyWedding {
            input_manager: input_manager,
            title: title,
            renderer: renderer,
            player: player,
            button: button,
            font_manager: font_manager,
        };
        Ok(game)
    }

    pub fn run(&mut self) -> Result<()>
        where R: Show
    {
        const GAME_SPEED: u32 = 60;
        const MAX_SKIP: u32 = 10;
        let update_duration = Duration::new(0, 1000000000 / GAME_SPEED);
        let mut timer = Timer::new();
        let mut delta: Duration = Default::default();
        while !self.game_quit() {
            let game_time = timer.update();
            delta += game_time.since_update;
            let mut loops: u32 = 0;
            while delta >= update_duration && loops < MAX_SKIP {
                self.input_manager.update();
                if self.game_quit() {
                    break;
                }
                self.update();
                self.button.update(&self.input_manager, &mut self.player);
                delta -= update_duration;
                loops += 1;
            }
            if self.game_quit() {
                break;
            }
            self.player.animate(game_time.since_update);
            let interpolation = delta.subsec_nanos() as f64 / update_duration.subsec_nanos() as f64;
            self.draw(interpolation)?;
        }
        Ok(())
    }

    fn update(&mut self) {
        self.player.update();
    }

    fn draw(&mut self, interpolation: f64) -> Result<()>
        where R: Show
    {
        let title_dims = self.title.dims();
        let title_rectangle = glm::ivec4(0, 0, title_dims.x as i32, title_dims.y as i32);
        self.renderer.clear();
        self.renderer.show(&self.player)?;
        self.renderer.show(&self.button)?;
        self.renderer
            .copy(&self.title, Some(title_rectangle), None)?;
        self.renderer.present();
        Ok(())
    }

    fn game_quit(&self) -> bool {
        self.input_manager.game_quit()
    }
}
