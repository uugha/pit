use std::{
    fs,
    io::{self, ErrorKind, Write},
    path::Path,
};

pub struct FileHandler;
impl FileHandler {
    const PATH: &str = ".pit/";

    pub fn create_dir() -> std::io::Result<bool> {
        match fs::create_dir(Self::PATH) {
            Ok(()) => Ok(true),
            Err(e) if e.kind() == ErrorKind::AlreadyExists => Ok(true),
            Err(e) => Err(e),
        }
    }

    pub fn create_new_file(value: &str) -> Result<String, io::Error> {
        Self::create_dir()?;

        if value.is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "value conot be empty",
            ));
        }
        let path = Path::new(Self::PATH).join(value);
        let mut file = fs::File::create_new(&path)?;

        file.write_all(value.as_bytes())?;

        Ok(path.to_string_lossy().into_owned())
    }

    pub fn read_dir() -> std::io::Result<()> {
        for entry in fs::read_dir(Self::PATH)? {
            let dir = entry?;
            println!("{:?}", dir.path());
        }
        Ok(())
    }
}
