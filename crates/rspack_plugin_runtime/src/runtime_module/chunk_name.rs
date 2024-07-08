use rspack_core::{
  impl_runtime_module,
  rspack_sources::{BoxSource, RawSource, SourceExt},
  ChunkUkey, Compilation, RuntimeGlobals, RuntimeModule,
};
use rspack_identifier::Identifier;

#[impl_runtime_module]
#[derive(Debug)]
pub struct ChunkNameRuntimeModule {
  id: Identifier,
  chunk: Option<ChunkUkey>,
}

impl Default for ChunkNameRuntimeModule {
  fn default() -> Self {
    Self::with_default(Identifier::from("webpack/runtime/chunk_name"), None)
  }
}

impl RuntimeModule for ChunkNameRuntimeModule {
  fn attach(&mut self, chunk: ChunkUkey) {
    self.chunk = Some(chunk);
  }

  fn name(&self) -> Identifier {
    self.id
  }

  fn generate(&self, compilation: &Compilation) -> rspack_error::Result<BoxSource> {
    if let Some(chunk_ukey) = self.chunk {
      let chunk = compilation.chunk_by_ukey.expect_get(&chunk_ukey);
      Ok(
        RawSource::from(format!(
          "{} = {};",
          RuntimeGlobals::CHUNK_NAME,
          serde_json::to_string(&chunk.name).expect("Invalid json string")
        ))
        .boxed(),
      )
    } else {
      unreachable!("should attach chunk for css_loading")
    }
  }
}
