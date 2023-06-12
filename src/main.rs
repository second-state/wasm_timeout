use wasmedge_sdk::VmBuilder;
use wasmedge_sdk_async_wasi::{wasi::snapshots::WasiCtx, wasi_impls, WasiVm};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    async fn async_ticker() {
        let mut i = 0;
        loop {
            println!("[host] tick {i}");
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            i += 1;
        }
    }

    let _ = tokio::spawn(async_ticker());

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
    println!("{r:?}");
}
