use std::path::PathBuf;
use neon::prelude::*;

pub struct AssetGuard {
  pub common_path: PathBuf,
  pub java_exec: PathBuf,
}

declare_types! {
  pub class JsAssetGuard for AssetGuard {
    init(mut cx) {
      let common_path = PathBuf::from(
        cx.argument::<JsString>(0)?.value(),
      );
      let java_exec = PathBuf::from(
        cx.argument::<JsString>(1)?.value(),
      );

      Ok(AssetGuard {
        common_path,
        java_exec,
      })
    }
  }
}

register_module!(mut cx, {
  cx.export_class::<JsAssetGuard>("AssetGuard")?;

  Ok(())
});
