use errors::*;
use duck_husky_wedding::button::Button;
use duck_husky_wedding::game_data::SpriteData;
use duck_husky_wedding::player::Player;

use glm;
use moho::errors as moho_errors;
use moho::input;
use moho::renderer::{ColorRGBA, Font, FontTexturizer, Renderer, Scene, Show, Texture};

use std::rc::Rc;
use std::time::Duration;

enum ScreenKind {
    MENU_SCREEN,
    GAME_PLAY,
}

enum ScreenState {
    FOREGROUND,
    BACKGROUND,
}

pub struct MenuScreen<T> {
    title: T,
    button: Button<T>,
    new_game: Button<T>,
    player: Player<T>,
}

impl<T> MenuScreen<T> {
    pub fn load<'f, F, R>(font: &F,
                          texturizer: &R,
                          data: SpriteData,
                          player_texture: Rc<T>)
                          -> Result<Self>
        where T: Texture,
              F: Font,
              R: FontTexturizer<'f, Font = F, Texture = T>
    {
        let title_color = ColorRGBA(255, 255, 0, 255);
        let title = texturizer
            .texturize(font, "Husky <3 Ducky", &title_color)?;
        let button = Button::from_text("click me", texturizer, font, glm::uvec2(100, 100))?;
        let new_game = Button::from_text("New Game", texturizer, font, glm::uvec2(200, 200))?;
        let player = Player::new(data, player_texture);
        Ok(Self::new(title, button, new_game, player))
    }

    pub fn quack(&mut self) {
        println!("QUACK");
    }

    pub fn new(title: T, button: Button<T>, new_game: Button<T>, player: Player<T>) -> Self {
        MenuScreen {
            title: title,
            button: button,
            new_game: new_game,
            player: player,
        }
    }

    pub fn update(&mut self, input: &input::State) {
        self.player.update();
        if self.button.update(input) {
            self.player.flip();
        }
        if self.new_game.update(input) {
            self.quack();
        }
    }

    pub fn animate(&mut self, delta: Duration) {
        self.player.animate(delta)
    }
}

impl<T, R> Scene<R> for MenuScreen<T>
    where T: Texture,
          R: Renderer<Texture = T> + Show
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
