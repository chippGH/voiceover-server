// build.rs

use std::{fs, io};
use std::env;
use std::ops::Not;
use std::path::Path;
use std::process::Command;

fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}

fn main() {
    let dir_name = "voices";
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    
    if env::var_os("CARGO_CFG_WINDOWS").is_some() {
        Command::new("python")
        .args(vec![
            Path::new(&manifest_dir)
                .join("voices/parse_all.py")
                .to_str()
                .unwrap()
        ])
        .current_dir(Path::new(&manifest_dir).join(dir_name))
        .spawn().unwrap().wait().unwrap().success().not().then(|| panic!());
    } else {
        Command::new("python3")
        .args(vec![
            Path::new(&manifest_dir)
                .join("voices/parse_all.py")
                .to_str()
                .unwrap()
        ])
        .current_dir(Path::new(&manifest_dir).join(dir_name))
        .spawn().unwrap().wait().unwrap().success().not().then(|| panic!());
    }

    /*for source in fs::read_dir(Path::new("./").join(dir_name).join("sources")).unwrap() {
        let source = source.unwrap();
        let out_file = Path::new(&manifest_dir).join(dir_name).join("jsons").join(source.path().with_extension("json").file_name().unwrap());


        fs::write(out_file, Command::new("python")
                .args(vec![
                    Path::new(&manifest_dir)
                    .join("voices/parse_voiceover.py")
                    .to_str()
                    .unwrap()
                ])
                .stdin(Stdio::from(File::open(source.path()).unwrap()))
                .output().unwrap().stdout.iter().map(|x| char::from(*x)).collect::<String>()
        ).unwrap();
    }
*/
    let dest_path1 = Path::new("./target/debug").join(dir_name);
    let dest_path2 = Path::new("./target/release").join(dir_name);
    copy_dir_all(Path::new("./").join(dir_name), dest_path1).unwrap();
    copy_dir_all(Path::new("./").join(dir_name), dest_path2).unwrap();
    println!("cargo::rerun-if-changed=voices");
}
