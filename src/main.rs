// main.rs
use deno_core::error::AnyError;
use std::env;
use std::rc::Rc;

async fn run_js(file_path: &str) -> Result<(), AnyError> {
    // 現在のpathを取得
    let current_dir: Result<std::path::PathBuf, std::io::Error> = env::current_dir();

    // 与えられたjsファイルのpathと合体！！
    let main_module = deno_core::resolve_path(file_path, current_dir.unwrap().as_path())?;

    // runtimeの作成
    let mut js_runtime: deno_core::JsRuntime =
        deno_core::JsRuntime::new(deno_core::RuntimeOptions {
            module_loader: Some(Rc::new(deno_core::FsModuleLoader)),
            ..Default::default()
        });

    let mod_id: usize = js_runtime.load_main_module(&main_module, None).await?;
    let result: deno_core::futures::channel::oneshot::Receiver<
        Result<(), deno_core::anyhow::Error>,
    > = js_runtime.mod_evaluate(mod_id);
    js_runtime.run_event_loop(false).await?;
    result.await?
}

fn main() {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    if let Err(error) = runtime.block_on(run_js("src/js/example.js")) {
        println!("error: {}", error);
    }
}
