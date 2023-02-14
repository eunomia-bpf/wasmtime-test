use anyhow::Result;
use wasmtime::component::Component;
use wasmtime::component::Linker;
use wasmtime::component::Val;
use wasmtime::Store;
wasmtime::component::bindgen!("import");
struct MyState {}
#[allow(unused)]
impl ImportImports for MyState {
    fn wasm_bpf_map_operate(
        &mut self,
        fd: i32,
        cmd: i32,
        key: u32,
        value: u32,
        next_key: u32,
        flags: Uint64T,
    ) -> anyhow::Result<i32> {
        todo!()
    }

    fn wasm_bpf_buffer_poll(
        &mut self,
        program: BpfObjectSkel,
        fd: i32,
        sample_func: Int32T,
        ctx: Uint32T,
        data: u32,
        max_size: i32,
        timeout_ms: i32,
    ) -> anyhow::Result<i32> {
        todo!()
    }

    fn wasm_attach_bpf_program(
        &mut self,
        obj: BpfObjectSkel,
        name: u32,
        attach_target: u32,
    ) -> anyhow::Result<i32> {
        todo!()
    }

    fn wasm_load_bpf_object(
        &mut self,
        obj_buf: u32,
        obj_buf_sz: i32,
    ) -> anyhow::Result<BpfObjectSkel> {
        todo!()
    }

    fn wasm_close_bpf_object(&mut self, obj: BpfObjectSkel) -> anyhow::Result<i32> {
        todo!()
    }

    fn wasm_bpf_map_fd_by_name(&mut self, obj: BpfObjectSkel, name: u32) -> anyhow::Result<i32> {
        todo!()
    }
}
fn main() -> Result<()> {
    let mut config = wasmtime::Config::new();
    config.wasm_component_model(true);
    let engine = wasmtime::Engine::new(&config).unwrap();
    let component = Component::from_file(&engine, "./rust-bootstrap-component.wasm").unwrap();
    let mut linker = Linker::new(&engine);
    Import::add_to_linker(&mut linker, |state: &mut MyState| state).unwrap();
    let mut store = Store::new(&engine, MyState { /* ... */ });
    let (_, t) = Import::instantiate(&mut store, &component, &linker).unwrap();

    let mut result = [Val::U32(0)];
    t.get_func(&mut store, "__main_argc_argv")
        .unwrap()
        .call(&mut store, &[Val::U32(0), Val::U32(0)], &mut result)
        .unwrap();

    return Ok(());
}
