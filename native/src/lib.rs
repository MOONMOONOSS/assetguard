use std::path::PathBuf;
use neon::prelude::*;

pub struct AssetGuard {
  pub common_path: PathBuf,
  pub java_exec: PathBuf,
}

pub fn extract_pack_xz(file_paths: Vec<PathBuf>) -> u64 {
  use anyhow::Result;
  use std::fs;
  use std::io::{Read, Write};
  use xz2::read::XzDecoder;

  let mut error_count = 0u64;
  
  for file in file_paths {
    let mut folder_path = file.clone();
    folder_path.pop();

    let error_prone = || -> Result<()> {
      let file_handle = fs::File::open(&file)
        .expect(&format!("extractPackXZ was unable to read {}", &file.display()));
      
      let mut file_buf: Vec<u8> = vec![];
      
      let mut decompressor = XzDecoder::new(file_handle);
      
      match decompressor.read_to_end(&mut file_buf) {
        Ok(_) => {
          let mut new_file = String::from(file
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap()
          );
          new_file.truncate(new_file.len() - 5);

          let mut f = fs::File::create(format!("{}/{}", folder_path.display(), new_file))
            .expect(&format!("Unable to create new file '{}' from '{}'", new_file, file.display()));

          f.write_all(&file_buf).expect(&format!("Unable to write to {}", new_file));
        },
        Err(e) => println!("Unable to decompress {}: {:#?}", file.display(), e),
      };

      Ok(())
    };

    if let Err(e) = error_prone() {
      println!("Error in iteration! {}", e);
      error_count += 1;
    }
  }

  error_count
}

pub fn extract_pack_xz_js(mut cx: FunctionContext) -> JsResult<JsNumber> {
  let file_array = cx.argument::<JsArray>(0)?;
  let tainted_vec = file_array.to_vec(&mut cx)?;
  let mut cleansed_vec: Vec<PathBuf> = vec![];

  for element in tainted_vec {
    if !element.is_a::<JsString>() {
      panic!("The array is not pure");
    }

    cleansed_vec.push(PathBuf::from(element.downcast_or_throw::<JsString, FunctionContext>(&mut cx)?.value()));
  }

  Ok(cx.number(extract_pack_xz(cleansed_vec) as f64))
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
  cx.export_function("extractPackXZ", extract_pack_xz_js)?;
  cx.export_class::<JsAssetGuard>("AssetGuard")?;

  Ok(())
});
