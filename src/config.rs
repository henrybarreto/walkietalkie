use std::fs::File;
use std::path::Path;

use serde::de::DeserializeOwned;

pub trait Config<C>
where
    C: DeserializeOwned,
{
    fn load_config_file(path_config_file: String) -> File {
        let path = Path::new(&path_config_file);
        if !path.exists() {
            Self::generate_config(&path);
        }
        File::open(path).expect("Could not load the configuration file")
    }
    fn generate_config(path: &Path);
    fn convert_config_to_struct(config_file: File) -> C {
        ron::de::from_reader(config_file).expect("")
    }
    fn config(config_file_name: String) -> C {
        Self::convert_config_to_struct(Self::load_config_file(config_file_name))
    }
}
