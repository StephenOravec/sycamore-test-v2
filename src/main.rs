use sycamore::prelude::*;

fn main() {
sycamore::render(|| view! {
    h1 { "Hello, world!" }
    p { "This is my second Sycamore app" }
});
}