use std::io::Read ;
use std::string::String;
use std::fs::File;
use std::path::Path;
use std::path::PathBuf;


/// used for debug, will be discarded eventually.
pub fn maybe<T>(option: Option<T>, s: &'static str) -> T
{
    assert!(option.is_some(), s);
    option.unwrap()
}

/// Normalize a vector. Will be eventually be discarted.
pub fn normalize_vec(v: (f32, f32, f32)) -> (f32, f32, f32)
{
    let norm = (v.0*v.0 + v.1*v.1 + v.2*v.2).sqrt();
    (v.0/norm, v.1/norm, v.2/norm)
}

/// Some vector product. Will eventually be discarted
pub fn v_prod(u: (f32, f32, f32), v: (f32, f32, f32)) -> (f32, f32, f32)
{
    (
        u.1*v.2 - u.2*v.1,
        -u.0*v.2 + u.2*v.0,
        u.0*v.1 - u.1*v.0
    )
}

/// Reads a file and returns its content
pub fn read_file(file_name: PathBuf) -> String
{
	let mut f = match File::open(file_name)
	{
		Ok(fichier) => fichier,
		_ => panic!("Couldn't open file")
	};
	let mut buffer = String::new();
	match f.read_to_string(&mut buffer)
	{
		Ok(_) => (),
		_ => panic!("Couldn't read file")
	};

	buffer
}




/// Finds the path of the ressources folder
pub fn get_ressources_path() -> PathBuf {
    let args: Vec<String> = std::env::args().collect();
    //the only relevant path we can get is the executable's since the execution dir could be anywhere
    let executable_path = Path::new(&args[0]);
    let crate_path = match executable_path.ancestors().nth(3) {
        Some(root) => root,
        None => panic!(
            "Panic! Can't figure out where we are, did you move the executable out of its folder?"
        ),
    };
    let ressources_path = crate_path.join(Path::new("ressources"));
    ressources_path
}

