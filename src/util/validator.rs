pub fn is_integer(v: String) -> Result<(), String> {
    if v.parse::<i32>().is_ok() {
        Ok(())
    } else {
        Err(format!("`{}` is not an integer", &*v))
    }
}

pub fn is_float(v: String) -> Result<(), String> {
    if v.parse::<f32>().is_ok() {
        Ok(())
    } else {
        Err(format!("`{}` is not a float", &*v))
    }
}