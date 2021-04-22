use super::{GLTexture, GL};
use web_sys::HtmlImageElement;

/// An asynchrnonously-loading texture.
pub enum AsyncTexture {
    Loaded(GLTexture),
    Loading(GL, HtmlImageElement),
}

impl AsyncTexture {
    /// Loads a texture from a path.
    pub fn new(gl: GL, path: &str) -> AsyncTexture {
        let image = HtmlImageElement::new().unwrap();

        image.set_src(path);

        AsyncTexture::Loading(gl, image)
    }

    /// Checks if the texture has loaded.
    pub fn loaded(&mut self) -> bool {
        self.try_load().is_some()
    }

    /// Unwraps the texture.
    pub fn unwrap(&self) -> &GLTexture {
        match self {
            AsyncTexture::Loaded(tex) => tex,
            AsyncTexture::Loading(_, _) => panic!("unwrap called on unloaded texture"),
        }
    }

    /// Tries to load the image, returning a `Some(tex)` if it has successfully
    /// loaded.
    pub fn try_load(&mut self) -> Option<&GLTexture> {
        match self {
            AsyncTexture::Loaded(tex) => {
                Some(tex)
            }
            AsyncTexture::Loading(gl, img) => {
                if img.complete() {
                    let tex = gl.create_texture(&img);

                    *self = AsyncTexture::Loaded(tex);
                    Some(self.unwrap())
                } else {
                    None
                }
            },
        }
    }
}

impl GL {
    /// Loads a texture asynchronously.
    pub fn async_load_texture(&self, path: &str) -> AsyncTexture {
        AsyncTexture::new(self.clone(), path)
    }
}
