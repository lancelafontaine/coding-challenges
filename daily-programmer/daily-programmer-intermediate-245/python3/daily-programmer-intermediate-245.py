import collections
import string
import sys

def process_decode_key(decode_key_line):
    decode_key = {}
    decode_key_list = decode_key_line.split()
    if len(decode_key_list) <=1:
        print("Key provided is not sufficient.")
        sys.exit()

    for i in range(len(decode_key_list)):
        if i % 2 == 1:
            # First check if key has chars that are not G or g
            if not set(decode_key_list[i]).issubset(set("Gg")):
                print("Key is invalid: attempts to parse chars that are not 'G' or 'g'")
                sys.exit()
            # Then make sure we aren't duplicating a key
            if decode_key_list[i] in decode_key:
                print("Key is invalid: attempts to define a key's value twice")
                sys.exit()
            decode_key.update({decode_key_list[i]: decode_key_list[i-1]})
    return decode_key

def process_alien_lang(al, decode_key):
    alien_line = al
    test_string = ""
    decoded_string = ""

    for char in alien_line:
        test_string = test_string + char

        if test_string in decode_key:
            decoded_string = decoded_string + decode_key.get(test_string)
            test_string = ""

        if char in string.whitespace or \
                char in string.punctuation or \
                char in string.digits:
            decoded_string = decoded_string + char
            test_string = ""

    return decoded_string

def decode():
    decode_key = process_decode_key(input("Enter a key:\n"))
    result = process_alien_lang(input("Enter an alien string:\n"), decode_key)
    return result

# ---
def process_human_lang(hl):
    # Construct key
    encode_key = {}
    dec_to_bin = 0
    for char in (string.ascii_letters):
        temp_bin = format(dec_to_bin, '06b')
        bin_gG = str(temp_bin).replace("0","g").replace("1","G")
        encode_key.update({char:bin_gG})
        dec_to_bin += 1

    # convert line to message
    encoded_string = ""
    for char in hl:
        if char in encode_key:
            encoded_string = encoded_string + encode_key.get(char)
        else:
            encoded_string = encoded_string + char
    return (encode_key, encoded_string)



def encode():
    print("Now it's time to encode!")
    (encode_key, encoded_string) = process_human_lang(input("Enter an English string to encode:\n"))

    # Convert key to space-delimited string
    encode_key_string = ""
    for k,v in encode_key.items():
        encode_key_string = encode_key_string + k + " " + v + " "

    return (encode_key_string, encoded_string)


if __name__ == "__main__":
    print(decode()+"\n")
    (key, string) = encode()
    print(key)
    print(string)


