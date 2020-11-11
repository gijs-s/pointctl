//! Some validators used in the CLI

/// Check if a string can be parsed as a valid integer
pub fn is_integer(v: String) -> Result<(), String> {
    if v.parse::<i32>().is_ok() {
        Ok(())
    } else {
        Err(format!("`{}` is not an integer", &*v))
    }
}

/// Check if a string can be parsed as a valid positive integer
pub fn is_usize(v: String) -> Result<(), String> {
    if v.parse::<usize>().is_ok() {
        Ok(())
    } else {
        Err(format!("`{}` is not an positive integer", &*v))
    }
}

/// Check if a string can be parsed as a valid float
pub fn is_float(v: String) -> Result<(), String> {
    if v.parse::<f32>().is_ok() {
        Ok(())
    } else {
        Err(format!("`{}` is not a float", &*v))
    }
}

/// Check if a string can be parsed as a float value between 1 and 0
pub fn is_norm_float(v: String) -> Result<(), String> {
    if let Ok(v) = v.parse::<f32>() {
        if v >= 0f32 && v <= 1f32 {
            Ok(())
        } else {
            Err(format!("`{}` is not between 1.0 and 0.0", v))
        }
    } else {
        Err(format!("`{}` is not a float", &*v))
    }
}
