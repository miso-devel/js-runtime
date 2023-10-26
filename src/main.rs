// main.rs
use deno_core::error::AnyError;

use deno_core::op2;
use deno_core::Extension;
use std::env;
use std::rc::Rc;

// #[op2]
// async fn op_read_file(path: String) -> Result<String, AnyError> {
//     let contents = tokio::fs::read_to_string(path).await?;
//     Ok(contents)
// }

// #[op2]
// async fn op_write_file(path: String, contents: String) -> Result<(), AnyError> {
//     tokio::fs::write(path, contents).await?;
//     Ok(())
// }

// #[op2]
// fn op_remove_file(path: String) -> Result<(), AnyError> {
//     std::fs::remove_file(path)?;
//     Ok(())
// }

async fn run_js(file_path: &str) -> Result<(), AnyError> {
    // 現在のpathを取得
    let current_dir: Result<std::path::PathBuf, std::io::Error> = env::current_dir();

    // 与えられたjsファイルのpathと合体！！
    let main_module = deno_core::resolve_path(file_path, current_dir.unwrap().as_path())?;

    // let runjs_extension = Extension::builder()
    //     .ops(vec![
    //         op_read_file::decl(),
    //         op_write_file::decl(),
    //         op_remove_file::decl(),
    //     ])
    //     .build();

    // runtimeの作成
    let mut js_runtime: deno_core::JsRuntime =
        deno_core::JsRuntime::new(deno_core::RuntimeOptions {
            module_loader: Some(Rc::new(deno_core::FsModuleLoader)),
            // extensions: vec![runjs_extension],
            ..Default::default()
        });

    // include_str!はコンパイル時に指定したファイルのパスをその中身の文字列へと置き換える
    let runtime_js_file: &str = include_str!("./runtime.js");

    // execute_scriptの第二引数がFastStringなので変換する
    let runtime_string: deno_core::FastString = deno_core::FastString::Static(runtime_js_file);

    // jsファイルを実行する
    js_runtime
        .execute_script("[runjs:runtime.js]", runtime_string)
        .unwrap();

    // TODO: なんかloadしてる
    let mod_id: usize = js_runtime.load_main_module(&main_module, None).await?;

    // TODO: 評価してそう
    let result: deno_core::futures::channel::oneshot::Receiver<
        Result<(), deno_core::anyhow::Error>,
    > = js_runtime.mod_evaluate(mod_id);

    // TODO: イベントループで何かをしていそう
    js_runtime.run_event_loop(false).await?;

    // TODO: 結果をここで返していそう
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
