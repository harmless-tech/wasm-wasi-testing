use std::{future::{Future}, pin::Pin};

use wasmtime::{component::{bindgen, Component, Linker}, Config, Engine, Store};
use wasmtime_wasi::preview2::{WasiView, Table, WasiCtx, WasiCtxBuilder};

bindgen!({
    path: "./wit/tcomp.wit",
    async: true
});

struct Link {
    table: Table,
    wasi: WasiCtx,
}
impl Link {
    async fn a_hello() -> wasmtime::Result<u32> {
        Ok(42)
    }
}
impl TestingImports for Link {
    #[must_use]
    #[allow(clippy::type_complexity,clippy::type_repetition_in_bounds)]
    fn hello<'life0, 'async_trait>(&'life0 mut self) ->  Pin<Box<dyn Future<Output = wasmtime::Result<u32>> + Send + 'async_trait>> where 'life0:'async_trait, Self: 'async_trait {
        Box::pin(Link::a_hello())
    }
}
impl WasiView for Link {
    fn table(&self) -> &wasmtime_wasi::preview2::Table {
        &self.table
    }

    fn table_mut(&mut self) -> &mut wasmtime_wasi::preview2::Table {
        &mut self.table
    }

    fn ctx(&self) -> &wasmtime_wasi::preview2::WasiCtx {
        &self.wasi
    }

    fn ctx_mut(&mut self) -> &mut wasmtime_wasi::preview2::WasiCtx {
        &mut self.wasi
    }
}

#[tokio::main]
async fn main() -> wasmtime::Result<()> {
    let mut config = Config::new();
    config.wasm_component_model(true);
    config.async_support(true);
    let engine = Engine::new(&config)?;
    let comp = Component::from_file(&engine, "./target/wasm32-wasi/debug/wasmmm-comp.wasm")?;
    
    let mut linker = Linker::new(&engine);
    wasmtime_wasi::preview2::command::add_to_linker(&mut linker)?;
    Testing::add_to_linker(&mut linker, |state: &mut Link| state)?;
    
    let table = Table::new();
    let wasi = WasiCtxBuilder::new().inherit_stdio().build();
    
    let mut store = Store::new(&engine, Link {
        table,
        wasi,
    });
    let (bindings, _) = Testing::instantiate_async(&mut store, &comp, &linker).await?;
    
    let str = bindings.call_h_world(&mut store).await?;
    println!("Result: {str}");
    
    let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let data = bindings.call_data_process(&mut store, &data).await?;
    println!("Result Data: {data:?}");
    
    Ok(())
}
