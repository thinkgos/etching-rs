fn main() -> Result<(), Box<dyn std::error::Error>> {
    let passwd = "123456";

    let hashed_passwd = bcrypt::hash(passwd, bcrypt::DEFAULT_COST)?;
    let b = bcrypt::verify(passwd, &hashed_passwd)?;
    assert!(b);
    Ok(())
}
