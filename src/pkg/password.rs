use aes::cipher::{block_padding::Pkcs7, BlockDecryptMut, BlockEncryptMut, KeyIvInit};
use anyhow::anyhow;
use base64::{engine::general_purpose::STANDARD as base64std, Engine};
use cipher::BlockSizeUser;
use rand::{distributions::Uniform, Rng};

use std::time::{SystemTime, UNIX_EPOCH};

type Aes128CbcEnc = cbc::Encryptor<aes::Aes128>;
type Aes128CbcDec = cbc::Decryptor<aes::Aes128>;
type Aes192CbcEnc = cbc::Encryptor<aes::Aes192>;
type Aes192CbcDec = cbc::Decryptor<aes::Aes192>;
type Aes256CbcEnc = cbc::Encryptor<aes::Aes256>;
type Aes256CbcDec = cbc::Decryptor<aes::Aes256>;

pub struct Paypal {
    iv_generator: fn(block_size: usize) -> Result<Vec<u8>, anyhow::Error>,
    iv_checker: fn(iv: &[u8]) -> Result<(), anyhow::Error>,
}

impl Paypal {
    pub fn new() -> Self {
        Self {
            iv_generator: Self::default_iv_generator,
            iv_checker: Self::default_iv_checker,
        }
    }
    pub fn set_iv_generator(
        &mut self,
        f: fn(usize) -> Result<Vec<u8>, anyhow::Error>,
    ) -> &mut Self {
        self.iv_generator = f;
        self
    }
    pub fn set_iv_checker(&mut self, f: fn(&[u8]) -> Result<(), anyhow::Error>) -> &mut Self {
        self.iv_checker = f;
        self
    }

    fn default_iv_generator(block_size: usize) -> Result<Vec<u8>, anyhow::Error> {
        let mut rng = rand::thread_rng();
        let range: Uniform<u8> = Uniform::new(0, 255);
        Ok((0..).map(|_| rng.sample(range)).take(block_size).collect())
    }
    fn default_iv_checker(_: &[u8]) -> Result<(), anyhow::Error> {
        Ok(())
    }

    fn encrypt(&self, key: &[u8], plain_text: &[u8]) -> Result<String, anyhow::Error> {
        let block_size = match key.len() {
            16 => Ok(Aes128CbcEnc::block_size()),
            24 => Ok(Aes192CbcEnc::block_size()),
            32 => Ok(Aes256CbcEnc::block_size()),
            x => Err(anyhow!("invalid key size, {x}")),
        }?;

        let iv = (self.iv_generator)(block_size)?;
        if iv.len() != block_size {
            return Err(anyhow!("iv length must equal block size"));
        }

        let pt_len = plain_text.len();
        let buf_len = (pt_len as f64 / block_size as f64).ceil() as usize * block_size;

        let mut buf = vec![0; buf_len];
        buf[..pt_len].copy_from_slice(plain_text);

        let cipher_text = match key.len() {
            16 => Aes128CbcEnc::new(key.into(), iv.as_slice().into())
                .encrypt_padded_mut::<Pkcs7>(&mut buf, pt_len),
            24 => Aes192CbcEnc::new(key.into(), iv.as_slice().into())
                .encrypt_padded_mut::<Pkcs7>(&mut buf, pt_len),
            32 => Aes256CbcEnc::new(key.into(), iv.as_slice().into())
                .encrypt_padded_mut::<Pkcs7>(&mut buf, pt_len),
            _ => panic!("never reached here"),
        };
        let cipher_text = cipher_text.map_err(|e| anyhow::anyhow!("encrypt failure, {:?}", e))?;

        let mut data = iv;
        data.extend(cipher_text);

        Ok(base64std.encode(data))
    }

    fn decrypt(&self, key: &[u8], cipher_text: &[u8]) -> Result<Vec<u8>, anyhow::Error> {
        let block_size = match key.len() {
            16 => Ok(Aes128CbcEnc::block_size()),
            24 => Ok(Aes192CbcEnc::block_size()),
            32 => Ok(Aes256CbcEnc::block_size()),
            x => Err(anyhow!("invalid key size, {x}")),
        }?;
        let buf: Vec<u8> = base64std.decode(cipher_text)?;
        if buf.is_empty() || (buf.len() % block_size) != 0 {
            return Err(anyhow!("invalid cipher data"));
        }

        let (iv, data) = buf.split_at(block_size);
        let mut data: Vec<_> = data.into();
        // check iv
        (self.iv_checker)(iv)?;

        let plain_text = match key.len() {
            16 => Aes128CbcDec::new(key.into(), iv.into()).decrypt_padded_mut::<Pkcs7>(&mut data),
            24 => Aes192CbcDec::new(key.into(), iv.into()).decrypt_padded_mut::<Pkcs7>(&mut data),
            32 => Aes256CbcDec::new(key.into(), iv.into()).decrypt_padded_mut::<Pkcs7>(&mut data),
            _ => panic!("never reached here"),
        };
        let palin_text = plain_text.map_err(|e| anyhow::anyhow!("decrypt failure, {:?}", e))?;

        Ok(palin_text.into())
    }
}

impl Default for Paypal {
    fn default() -> Self {
        Self::new()
    }
}

pub struct Password;

impl Password {
    // 一个32位key, 由固定16前缀+unique_id+后缀16位,拼接后取前32位作为key
    // 加密方法,
    // cipherPasswd = Base64(AesCbcEncrypt(key,rawPasswd)) // Cbc方法
    // CbcEncrypt 中,
    // IV生成规则: 使用时间戳ms值, 不足16位, 前面补p, 然后拼接到加密后的密文前面

    const PASSWD_KEY_PREFIX: &'static str = "y0C<$?,^!hm4WTEt"; // 前缀16位, 密码加密key
    const PASSWD_KEY_SUFFIX: &'static str = "ogevQ)&ZEo@-yxjK"; // 后缀16位, 密码加密key

    // PASSWD_KEY_PREFIX + unique_id + PASSWD_KEY_SUFFIX, 拼接取前32位
    fn key(unique_id: &str) -> String {
        let mut s = format!(
            "{}{}{}",
            Self::PASSWD_KEY_PREFIX,
            unique_id,
            Self::PASSWD_KEY_SUFFIX
        );
        s.truncate(32);
        s
    }

    fn iv_generator(_: usize) -> Result<Vec<u8>, anyhow::Error> {
        let t = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|_| anyhow!("generator failure"))?
            .as_millis();
        Ok(format!("ppp{}", t).into())
    }

    fn iv_checker(iv: &[u8]) -> Result<(), anyhow::Error> {
        const LIMIT_MILLISECOND: u128 = 5 * 60 * 1000;

        let t = String::from_utf8_lossy(iv)
            .trim_start_matches('p')
            .parse::<u128>()
            .map_err(|_| anyhow!("invalid iv value"))?;
        let t1 = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|_| anyhow!("generator failure"))?
            .as_millis();

        if t + LIMIT_MILLISECOND <= t1 {
            Err(anyhow!("iv已失效"))
        } else {
            Ok(())
        }
    }

    #[inline]
    pub fn encrypt<P: AsRef<[u8]>>(unique_id: &str, passwd: P) -> Result<String, anyhow::Error> {
        Paypal::new()
            .set_iv_generator(Self::iv_generator)
            .set_iv_checker(Self::iv_checker)
            .encrypt(Self::key(unique_id).as_bytes(), passwd.as_ref())
    }

    #[inline]
    pub fn decrypt<P: AsRef<[u8]>>(
        unique_id: &str,
        cipher_passwd: P,
    ) -> Result<String, anyhow::Error> {
        let plain_text = Paypal::new()
            .set_iv_generator(Self::iv_generator)
            .set_iv_checker(Self::iv_checker)
            .decrypt(Self::key(unique_id).as_bytes(), cipher_passwd.as_ref())?;
        Ok(String::from_utf8_lossy(&plain_text).into())
    }

    #[inline]
    pub fn encrypt_raw<P: AsRef<[u8]>>(passwd: P) -> Result<String, anyhow::Error> {
        Self::encrypt("", passwd)
    }

    #[inline]
    pub fn decrypt_raw<P: AsRef<[u8]>>(cipher_passwd: P) -> Result<String, anyhow::Error> {
        Self::decrypt("", cipher_passwd)
    }
}

#[cfg(test)]
mod tests {
    use super::Password;
    use super::Paypal;

    const PLAIN_TEXT: &[u8] = b"hello world! this is my plaintext.";

    #[test]
    fn test_paypal_encrypt() -> Result<(), anyhow::Error> {
        let key = [0x42; 16];
        let paypal = Paypal::new();

        let cipher_text = paypal.encrypt(&key, PLAIN_TEXT)?;
        let plain_text = paypal.decrypt(&key, cipher_text.as_bytes())?;
        assert_eq!(plain_text, PLAIN_TEXT);
        Ok(())
    }

    #[test]
    fn test_passwd_encrypt_decrypt() {
        let passwd = "12345678";

        let cipher_text = Password::encrypt_raw(passwd).unwrap();
        println!("{}", cipher_text);

        let plain_text = Password::decrypt_raw(&cipher_text).unwrap();
        assert_eq!(passwd, plain_text);
    }
}
