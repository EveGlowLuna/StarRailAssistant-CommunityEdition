// Windows DPAPI 加密/解密模块
// 与 SRAFrontend 的 EncryptUtil.cs 保持兼容

use base64::{engine::general_purpose, Engine as _};

#[cfg(target_os = "windows")]
use windows::Win32::Security::Cryptography::{CryptProtectData, CryptUnprotectData, CRYPT_INTEGER_BLOB};
#[cfg(target_os = "windows")]
use windows::Win32::Foundation::LocalFree;

/// 使用 Windows DPAPI 加密字符串
/// 与 C# 的 ProtectedData.Protect() 兼容
pub fn encrypt_string(input: &str) -> Result<String, String> {
    if input.is_empty() {
        return Ok(String::new());
    }

    #[cfg(target_os = "windows")]
    {
        let input_bytes = input.as_bytes();
        
        let mut data_in = CRYPT_INTEGER_BLOB {
            cbData: input_bytes.len() as u32,
            pbData: input_bytes.as_ptr() as *mut u8,
        };

        let mut data_out = CRYPT_INTEGER_BLOB {
            cbData: 0,
            pbData: std::ptr::null_mut(),
        };

        unsafe {
            // 调用 CryptProtectData，使用当前用户范围，不使用额外的熵
            let result = CryptProtectData(
                &mut data_in,
                None,                    // 描述字符串
                None,                    // 可选的熵
                None,                    // 保留
                None,                    // 提示结构
                0,                       // 标志（0 = CRYPTPROTECT_UI_FORBIDDEN）
                &mut data_out,
            );

            if result.is_err() {
                return Err("Failed to encrypt data".to_string());
            }

            // 将加密后的数据转换为 Vec<u8>
            let encrypted_bytes = std::slice::from_raw_parts(
                data_out.pbData,
                data_out.cbData as usize,
            ).to_vec();

            // 释放 Windows 分配的内存
            if !data_out.pbData.is_null() {
                let _ = LocalFree(windows::Win32::Foundation::HLOCAL(data_out.pbData as *mut _));
            }

            // 转换为 Base64
            Ok(general_purpose::STANDARD.encode(&encrypted_bytes))
        }
    }

    #[cfg(not(target_os = "windows"))]
    {
        // 非 Windows 平台不加密，直接返回 Base64
        Ok(general_purpose::STANDARD.encode(input.as_bytes()))
    }
}

/// 使用 Windows DPAPI 解密字符串
/// 与 C# 的 ProtectedData.Unprotect() 兼容
pub fn decrypt_string(input: &str) -> Result<String, String> {
    if input.is_empty() {
        return Ok(String::new());
    }

    #[cfg(target_os = "windows")]
    {
        // 从 Base64 解码
        let encrypted_bytes = general_purpose::STANDARD
            .decode(input)
            .map_err(|e| format!("Failed to decode base64: {}", e))?;

        let mut data_in = CRYPT_INTEGER_BLOB {
            cbData: encrypted_bytes.len() as u32,
            pbData: encrypted_bytes.as_ptr() as *mut u8,
        };

        let mut data_out = CRYPT_INTEGER_BLOB {
            cbData: 0,
            pbData: std::ptr::null_mut(),
        };

        unsafe {
            // 调用 CryptUnprotectData
            let result = CryptUnprotectData(
                &mut data_in,
                None,                    // 描述字符串输出
                None,                    // 可选的熵
                None,                    // 保留
                None,                    // 提示结构
                0,                       // 标志
                &mut data_out,
            );

            if result.is_err() {
                return Err("Failed to decrypt data".to_string());
            }

            // 将解密后的数据转换为字符串
            let decrypted_bytes = std::slice::from_raw_parts(
                data_out.pbData,
                data_out.cbData as usize,
            ).to_vec();

            // 释放 Windows 分配的内存
            if !data_out.pbData.is_null() {
                let _ = LocalFree(windows::Win32::Foundation::HLOCAL(data_out.pbData as *mut _));
            }

            // 转换为 UTF-8 字符串
            String::from_utf8(decrypted_bytes)
                .map_err(|e| format!("Failed to convert to UTF-8: {}", e))
        }
    }

    #[cfg(not(target_os = "windows"))]
    {
        // 非 Windows 平台直接从 Base64 解码
        let bytes = general_purpose::STANDARD
            .decode(input)
            .map_err(|e| format!("Failed to decode base64: {}", e))?;
        
        String::from_utf8(bytes)
            .map_err(|e| format!("Failed to convert to UTF-8: {}", e))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_decrypt() {
        let original = "test_password_123";
        let encrypted = encrypt_string(original).unwrap();
        let decrypted = decrypt_string(&encrypted).unwrap();
        assert_eq!(original, decrypted);
    }

    #[test]
    fn test_empty_string() {
        let encrypted = encrypt_string("").unwrap();
        assert_eq!(encrypted, "");
        let decrypted = decrypt_string("").unwrap();
        assert_eq!(decrypted, "");
    }
}
