use piston_window::*;
use std::path::Path;
use std::error::Error;
pub struct Images {
    context: G2dTextureContext,
    settings: TextureSettings,
    pub sprites: Vec<G2dTexture>
}

impl Images {  
    pub fn new(context: G2dTextureContext, settings: TextureSettings) -> Images {
        Images {
            context: context,
            settings: settings,
            sprites: vec![]
        }
    }

    pub fn load(&mut self) -> Result<(), Box<dyn Error>> {
        let names = [
            "dude_a.png",
            "dude_b.png",
            "dude_c.png"
        ];

        let sprites: Result<Vec<_>, _> = names.into_iter().map(|n| {
            self.get(n)
        }).collect();
        self.sprites = sprites?;

        Ok(())
    }

    pub fn get(&mut self, name: &str) -> Result<G2dTexture, Box<dyn Error>> {
        let path = Path::new("assets").join(Path::new(name));

        let tex: G2dTexture = Texture::from_path(
            &mut self.context,
            &path,
            Flip::None,
            &self.settings
        )?;

        Ok(tex)
    }
 }
