use wasm_bindgen::{closure::Closure, JsCast as _};
use web_sys::HtmlImageElement;
use yew::callback::Callback;
use yew::services::Task;

/// An image fetching service
pub struct ImageService;

impl ImageService {
    /// Create a new image fetch task.
    pub fn fetch<T>(
        src: &str,
        onload: Callback<(HtmlImageElement, T)>,
        onerror: Callback<(HtmlImageElement, T)>,
        msg: T,
    ) -> ImageTask
    where
        T: Clone + 'static,
    {
        let msg_other = msg.clone();

        // init image
        let image = HtmlImageElement::new().unwrap();

        // setup events
        let image_ref = image.clone().unchecked_into();
        let onload_closure = Closure::once(move || onload.emit((image_ref, msg)));

        image.set_onload(Some(&onload_closure.as_ref().clone().dyn_into().unwrap()));

        let image_ref = image.clone().unchecked_into();
        let onerror_closure = Closure::once(move || onerror.emit((image_ref, msg_other)));

        image.set_onerror(Some(&onerror_closure.as_ref().clone().dyn_into().unwrap()));

        // setup src
        image.set_src(src);

        ImageTask {
            _onload: onload_closure,
            _onerror: onerror_closure,
        }
    }
}

/// Image fetch task.
pub struct ImageTask {
    _onload: Closure<dyn FnMut()>,
    _onerror: Closure<dyn FnMut()>,
}

impl Task for ImageTask {
    fn is_active(&self) -> bool {
        true
    }
}

impl Drop for ImageTask {
    fn drop(&mut self) {
        // we don't have to do anything, closures handle this for us.
    }
}
