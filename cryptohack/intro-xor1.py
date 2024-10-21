#!/usr/bin/env python

from pwn import xor

# KEY1 = a6c8b6733c9b22de7bc0253266a3867df55acde8635e19c73313
# KEY2 ^ KEY1 = 37dcb292030faa90d07eec17e3b1c6d8daf94c35d4c9191a5e1e
# KEY2 ^ KEY3 = c1545756687e7573db23aa1c3452a098b71a7fbf0fddddde5fc1
# FLAG ^ KEY1 ^ KEY3 ^ KEY2 = 04ee9855208a2cd59091d04767ae47963170d1660df7f56f5faf

input1 = "a6c8b6733c9b22de7bc0253266a3867df55acde8635e19c73313"
input2 = "37dcb292030faa90d07eec17e3b1c6d8daf94c35d4c9191a5e1e"
input3 = "c1545756687e7573db23aa1c3452a098b71a7fbf0fddddde5fc1"
input4 =  "04ee9855208a2cd59091d04767ae47963170d1660df7f56f5faf"

key1 = bytes.fromhex(input1)
key2 = xor(key1, bytes.fromhex(input2))
key3 = xor(key2, bytes.fromhex(input3))
flag = xor(key3, xor(key2, xor(key1, bytes.fromhex(input4))))

print(flag.decode("ascii"))
