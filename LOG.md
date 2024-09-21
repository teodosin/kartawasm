Using this as a log to write down the resources I am using and the thoughts I'm having while trying to figure this out. 

README will be for the final reflection and the actionable steps. 

First, I'm creating the project by following this tutorial: 
https://sneakycrow.dev/blog/2023-07-30-bevy-game-in-svelte-kit

Also found this, actually. Maybe I should use it instead. 
https://github.com/tobiplay/rust-wasm-sveltekit-starter

Actually no, I better build some muscle memory for this. 

https://www.reddit.com/r/rust/comments/1ahaa7v/is_wasmbindgen_that_essential/
Okay, so wasm can only communicate with the outside world with very low level primitives. What is needed on the JS/TS side is bindings which can convert the more structured stuff in JS/TS into a form wasm can understand and vice versa.

When asking about wasi-filesystem in the Bevy discord's rust channel, I got this reply:
"wasi is actually a totally different WASM target (wasm32-wasi/wasm32-wasip1/wasm32-wasip2) from the normal target for browsers (wasm32-unknown-unknown). I believe the whole wasm-bindgen ecosystem is geared only towards the latter, and there might be other issues trying to run a wasi binary on the browser."

But this repo managed to create the necessary bindings: 
https://github.com/GoogleChromeLabs/wasi-fs-access

 
