pub fn is_integer(v: String) -> Result<(), String> {
    if v.parse::<i32>().is_ok() {
        Ok(())
    } else {
        Err(format!("`{}` is not an integer", &*v))
    }
}
