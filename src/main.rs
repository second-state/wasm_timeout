use wasmedge_sdk::VmBuilder;
use wasmedge_sdk_async_wasi::{wasi::snapshots::WasiCtx, wasi_impls, WasiVm};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    env_logger::init();
    async fn async_ticker() {
        let mut i = 0;
        loop {
            println!("[host] tick {i}");
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            i += 1;
        }
    }

    let _ = tokio::spawn(async_ticker());

    run_demo().await;
    run_quickjs().await;
}

async fn run_demo() {
    let vm = VmBuilder::default().build().unwrap();
    let vm = Box::new(vm);
    let wasi_funcs = wasi_impls();
    let mut wasi_ctx = Box::new(WasiCtx::new());
    wasi_ctx.push_arg("abc".into());
    wasi_ctx.push_env("id", &format!("1"));
    let mut wasi_vm = WasiVm::create(vm, wasi_ctx, wasi_funcs).unwrap();
    wasi_vm
        .as_mut()
        .register_module_from_file_("demo", "demo.wasm")
        .unwrap();

    let r = wasi_vm
        .as_ref()
        .run_func_async_timeout(Some("demo"), "_start", vec![], 10)
        .await;
    println!("[run_demo] {r:?}");
}

async fn run_quickjs() {
    use wasmedge_sdk_async_wasi::wasi::snapshots::env::vfs;

    let vm = VmBuilder::default().build().unwrap();
    let vm = Box::new(vm);
    let wasi_funcs = wasi_impls();
    let mut wasi_ctx = Box::new(WasiCtx::new());
    wasi_ctx.push_arg("quickjs.wasm".into());
    wasi_ctx.push_arg("src/demo.js".into());
    wasi_ctx.push_preopen(vfs::WasiPreOpenDir::new("./js".into(), "src".into()));
    let mut wasi_vm = WasiVm::create(vm, wasi_ctx, wasi_funcs).unwrap();
    wasi_vm
        .as_mut()
        .register_module_from_file_("js", "wasmedge_quickjs.wasm")
        .unwrap();

    let r = wasi_vm
        .as_ref()
        .run_func_async_timeout(Some("js"), "_start", vec![], 10)
        .await;

    println!("[run_quickjs] {r:?}");
}
