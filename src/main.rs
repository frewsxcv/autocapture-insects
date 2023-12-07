use gphoto2::{Context, Result};
use std::{fs, path::Path, io::Write};

fn main() -> Result<()> {
    let context = Context::new()?;
    // Create a new context and detect the first camera from it
    let camera = context
        .autodetect_camera()
        .wait()
        .expect("Failed to autodetect camera");
    let camera_fs = camera.fs();

    // And take pictures
    /*
    let file_path = camera
        .capture_image()
        .wait()
        .expect("Could not capture image");
    camera_fs
        .download_to(
            &file_path.folder(),
            &file_path.name(),
            Path::new(&file_path.name().to_string()),
        )
        .wait()?;
    */
    
    let camera_file = camera
        .capture_preview()
        .wait()
        .expect("Could not capture image");
    let data = camera_file.get_data(&context).wait()?;
    {
        let mut file = fs::File::create(camera_file.name())?;
        file.write_all(&data)?;
    }

    Ok(())
}
