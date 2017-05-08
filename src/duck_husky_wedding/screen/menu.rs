use errors::*;
use duck_husky_wedding::button::Button;
use duck_husky_wedding::game_data::SpriteData;
use duck_husky_wedding::player::Player;

use glm;
use moho::errors as moho_errors;
use moho::input;
use moho::renderer::{ColorRGBA, FontDetails, FontLoader, FontManager, FontTexturizer, Renderer,
                     Scene, Show, Texture, TextureLoader, TextureManager};

use std::time::Duration;

pub struct Menu<T> {
    title: T,
    button: Button<T>,
    new_game: Button<T>,
    player: Player<T>,
}

impl<T> Menu<T> {
    pub fn load<'f, 't, R, TL, FL>(font_manager: &mut FontManager<'f, FL>,
                                   texture_manager: &mut TextureManager<'t, TL>,
                                   texturizer: &'t R,
                                   data: SpriteData)
                                   -> Result<Self>
        where T: Texture,
              TL: TextureLoader<'t, Texture = T>,
              FL: FontLoader<'f>,
              R: FontTexturizer<'f, 't, Font = FL::Font, Texture = T>
    {
        let font_details = FontDetails {
            path: "media/fonts/kenpixel_mini.ttf",
            size: 64,
        };
        let font = font_manager.load(&font_details)?;
        let file_name: &str = &format!("media/sprites/{}", data.file_name);
        let texture = texture_manager.load(file_name)?;
        let title_color = ColorRGBA(255, 255, 0, 255);
        let title = texturizer
            .texturize(&*font, "Husky <3 Ducky", &title_color)?;
        let button = Button::from_text("click me", texturizer, &*font, glm::uvec2(100, 100))?;
        let new_game = Button::from_text("New Game", texturizer, &*font, glm::uvec2(200, 200))?;
        let player = Player::new(data, texture);
        Ok(Self::new(title, button, new_game, player))
    }

    pub fn new(title: T, button: Button<T>, new_game: Button<T>, player: Player<T>) -> Self {
        Menu {
            title: title,
            button: button,
            new_game: new_game,
            player: player,
        }
    }

    pub fn update(&mut self, delta: Duration, input: &input::State) -> super::Kind {
        self.player.animate(delta);
        self.player.update();
        if self.button.update(input) {
            self.player.flip();
        }
        super::Kind::Menu
    }
}

impl<'t, T, R> Scene<R> for Menu<T>
    where T: Texture,
          R: Renderer<'t, Texture = T> + Show
{
    fn show(&self, renderer: &mut R) -> moho_errors::Result<()> {
        let title_dims = self.title.dims();
        let title_rectangle = glm::ivec4(0, 0, title_dims.x as i32, title_dims.y as i32);
        renderer.show(&self.player)?;
        renderer.show(&self.button)?;
        renderer.show(&self.new_game)?;
        renderer.copy(&self.title, Some(&title_rectangle), None)
    }
}
