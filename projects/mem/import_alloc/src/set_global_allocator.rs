use crate::ImportedGlobalAlloc;

#[global_allocator]
static GLOBAL: ImportedGlobalAlloc = ImportedGlobalAlloc;
