pub fn is_integer(v: String) -> Result<(), String> {
    if v.parse::<i32>().is_ok() {
        Ok(())
    } else {
        Err(format!("`{}` is not an integer", &*v))
    }
}

pub fn is_usize(v: String) -> Result<(), String> {
    if v.parse::<usize>().is_ok() {
        Ok(())
    } else {
        Err(format!("`{}` is not an positive integer", &*v))
    }
}

pub fn is_float(v: String) -> Result<(), String> {
    if v.parse::<f32>().is_ok() {
        Ok(())
    } else {
        Err(format!("`{}` is not a float", &*v))
    }
}