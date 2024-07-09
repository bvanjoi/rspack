use std::sync::Arc;

use napi_derive::napi;
use rspack_core::ResolverFactory;

use crate::{
  raw_resolve::{
    normalize_raw_resolve_options_with_dependency_type, RawResolveOptionsWithDependencyType,
  },
  JsResolver,
};

#[napi]
pub struct JsResolverFactory {
  resolver_factory: Arc<ResolverFactory>,
  loader_resolver_factory: Arc<ResolverFactory>,
}

#[napi]
impl JsResolverFactory {
  pub fn new(
    resolver_factory: Arc<ResolverFactory>,
    loader_resolver_factory: Arc<ResolverFactory>,
  ) -> Self {
    Self {
      resolver_factory,
      loader_resolver_factory,
    }
  }

  #[napi(ts_args_type = "type: string, options?: RawResolveOptionsWithDependencyType")]
  pub fn get(
    &self,
    r#type: String,
    raw: Option<RawResolveOptionsWithDependencyType>,
  ) -> napi::Result<JsResolver> {
    let (options, resolver_factory) = match r#type.as_str() {
      "normal" => {
        (normalize_raw_resolve_options_with_dependency_type(raw, false), self.resolver_factory.clone())
      }
      "loader" => {
        (normalize_raw_resolve_options_with_dependency_type(raw, false), self.loader_resolver_factory.clone())
      }
      "context" => {
        (normalize_raw_resolve_options_with_dependency_type(raw, true), self.resolver_factory.clone())
      }
      _ => {
        return Err(napi::Error::from_reason(format!("Invalid resolver type '{}' specified. Rspack only supports 'normal', 'context', and 'loader' types.", r#type)))
      }
    };
    match options {
      Ok(options) => Ok(JsResolver::new(resolver_factory, options)),
      Err(e) => Err(napi::Error::from_reason(format!("{e}"))),
    }
  }
}
