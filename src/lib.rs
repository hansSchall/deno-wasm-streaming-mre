mod utils;

use std::future::{self, Future};

use tokio::sync::oneshot::channel;
use tokio_with_wasm::task::spawn_blocking;
use utils::set_panic_hook;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

async fn my_spawn<T: 'static>(fut: impl Future<Output = T> + 'static) -> Result<T, ()> {
    let (res_sender, res_recv) = channel();
    spawn_local(async {
        let res = res_sender.send(fut.await);
        assert!(res.is_ok());
        // gloo::timers::callback::Timeout::new(1, || {
        //     log("trigger evloop");
        // })
        // .forget();
    });

    Ok(res_recv.await.unwrap())
}

#[wasm_bindgen]
pub async fn foo() {
    set_panic_hook();
    log("A");

    // spawn_local(async {

    // })

    my_spawn(async {
        log("B");
        my_spawn(async {
            log("C");
        })
        .await
        .unwrap();
        // spawn_blocking(|| {
        //     log("Blocking");
        // })
        // .await
        // .unwrap();
        log("D");
    })
    .await
    .unwrap();

    log("E");
}
