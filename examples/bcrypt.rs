fn main() -> Result<(), Box<dyn std::error::Error>> {
    let hashed_passwd = bcrypt::hash("123456", bcrypt::DEFAULT_COST)?;

    println!("{}", hashed_passwd);

    let b = bcrypt::verify("123456", &hashed_passwd)?;
    println!("{}", b);

    Ok(())
}
