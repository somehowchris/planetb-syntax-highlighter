use std::cell::RefCell;
use std::future::Future;
use std::pin::Pin;
use std::rc::Rc;
use std::task::Context;
use std::task::Poll;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::HtmlImageElement;

pub struct ImageFuture {
    image: Option<HtmlImageElement>,
    load_failed: Rc<RefCell<bool>>,
}

impl ImageFuture {
    pub fn new(path: &str) -> Self {
        let image = HtmlImageElement::new().unwrap();
        image.set_src(path);
        ImageFuture {
            image: Some(image),
            load_failed: Rc::new(RefCell::new(false)),
        }
    }
}

impl Future for ImageFuture {
    type Output = Result<HtmlImageElement, ()>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match &self.image {
            Some(image) => {
                return if image.complete() {
                    let image = self.image.take().unwrap();
                    let failed = *self.load_failed.borrow();

                    if failed {
                        Poll::Ready(Err(()))
                    } else {
                        Poll::Ready(Ok(image))
                    }
                } else {
                    let waker = cx.waker().clone();
                    let on_load_closure = Closure::wrap(Box::new(move || {
                        waker.wake_by_ref();
                    }) as Box<dyn FnMut()>);
                    image.set_onload(Some(on_load_closure.as_ref().unchecked_ref()));
                    on_load_closure.forget();

                    let waker = cx.waker().clone();
                    let failed_flag = self.load_failed.clone();
                    let on_error_closure = Closure::wrap(Box::new(move || {
                        *failed_flag.borrow_mut() = true;
                        waker.wake_by_ref();
                    })
                        as Box<dyn FnMut()>);
                    image.set_onerror(Some(on_error_closure.as_ref().unchecked_ref()));
                    on_error_closure.forget();

                    Poll::Pending
                };
            }
            _ => Poll::Ready(Err(())),
        }
    }
}

pub async fn has_webp_support() -> bool {
    let img = ImageFuture::new(
        "data:image/webp;base64,UklGRjIAAABXRUJQVlA4ICYAAACyAgCdASoCAAEALmk0mk0iIiIiIgBoSygABc6zbAAA/v56QAAAAA==",
    );

    

    img.await.is_ok()
}

const CONVERTED_IMAGE_EXTENSIONS: [&str; 2] = ["png", "jpg"];

pub fn build_webp_url(path: &str, default_extension: &str, has_support: bool) -> String {
    if has_support && CONVERTED_IMAGE_EXTENSIONS.contains(&default_extension) {
        let mut url = String::with_capacity(path.len() + 1 + 4);

        url.push_str(path);
        url.push_str(".webp");

        url
    } else {
        let mut url = String::with_capacity(path.len() + 1 + default_extension.len());

        url.push_str(path);
        url.push('.');
        url.push_str(default_extension);

        url
    }
}
