use async_trait::async_trait;
use rspack_core::{ModuleFactory, ModuleFactoryCreateData, ModuleFactoryResult};
use rspack_error::{internal_error, IntoTWithDiagnosticArray, Result, TWithDiagnosticArray};

use super::{
  container_entry_dependency::ContainerEntryDependency,
  container_entry_module::ContainerEntryModule,
};

#[derive(Debug)]
pub struct ContainerEntryModuleFactory;

#[async_trait]
impl ModuleFactory for ContainerEntryModuleFactory {
  async fn create(
    &self,
    data: ModuleFactoryCreateData,
  ) -> Result<TWithDiagnosticArray<ModuleFactoryResult>> {
    let dep = data
      .dependency
      .downcast_ref::<ContainerEntryDependency>()
      .ok_or_else(|| {
        internal_error!(
          "dependency of ContainerEntryModuleFactory should be ContainerEntryDependency"
        )
      })?;
    Ok(
      ModuleFactoryResult::new(Box::new(ContainerEntryModule::new(
        dep.name.clone(),
        dep.exposes.clone(),
        dep.share_scope.clone(),
      )))
      .with_empty_diagnostic(),
    )
  }
}
