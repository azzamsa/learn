interface EncryptResponse {
  data?: EncryptData
}
interface EncryptData {
  encrypt: Encrypt
}
interface Encrypt {
  secret: string
}

interface DecryptResponse {
  data?: DecryptData
}
interface DecryptData {
  decrypt: Decrypt
}
interface Decrypt {
  plain: string
}
