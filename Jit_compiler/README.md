# POC in Rust, where a desired program is encrypted and will not work if replicated on a computer which is not authorised (it is better if you want to sell it for example)

## Should be used like:
### Create the crypted binary:
cargo run executable_binary_to_encrypt new_name_of_crypted_binary

### Run the crypted binary:
cargo run new_name_of_crypted_binary

#

The code will be decrypted and runned as an executable buffer of hexadecimal instructions, so that no copy can be easily made to use the software without permission

Only the computer that did the encryption can run the crypted binary as it is decrypted using a hardware key (unique for each computer)

Inspired from https://github.com/jonathandturner/rustyjit
