#!/usr/bin/env python

import hashlib
import itertools

# public static String toSecurityValue(String str, String str2, String str3) {
#     StringBuilder sb = new StringBuilder();
#     String sha512 = sha512(str2 + str + str3);
#     String sha5122 = sha512(str3);
#     int i = 0;
#         for (int i2 = 0; i2 <= 7; i2++) {
#                 i += "0123456789abcdef".indexOf(sha5122.charAt(i));
#                 sb.append(sha512.charAt(i));
#         }
#         return sb.toString();
# }


def generate_s(
    uid: str, from_: str, pin: str = "CypCHG2kSlRkdvr2RG1QF8b2lCWXl7k7"
) -> str:
    sha512 = hashlib.sha512((pin + uid + from_).encode()).hexdigest()
    sha512_2 = hashlib.sha512(from_.encode()).hexdigest()
    i = 0
    sb = []
    hex_chars = "0123456789abcdef"
    for _ in range(8):  # Loop from 0 to 7 (inclusive)
        # The original Java code has a potential bug or unexpected behavior here:
        # i += "0123456789abcdef".indexOf(sha5122.charAt(i));
        # This means 'i' is being modified and then used to access 'sha5122.charAt(i)' in the same loop iteration.
        # If sha512_str3 is shorter than the accumulated 'i', it will cause an IndexError.
        # Assuming the intent is to use 'i2' (the loop counter) for accessing sha512_str3,
        # or that 'i' is expected to stay within bounds.
        # For a direct translation, we replicate the Java behavior, which might lead to errors
        # if 'i' grows beyond the length of sha512_str3.
        # We'll use a try-except block to handle potential index errors gracefully,
        # but the logic itself is a direct translation.
        try:
            char_to_find = sha512_2[i]
            i += hex_chars.index(char_to_find)
        except (IndexError, ValueError):
            # Handle cases where i goes out of bounds for sha512_str3,
            # or char_to_find is not in hex_chars.
            # In a real-world scenario, you'd want to clarify the intended behavior here.
            # For this translation, we'll break or handle as appropriate.
            # For now, we'll just continue if an error occurs to match potentially partial Java execution.
            # A more robust solution would be to understand the exact error handling or intended logic.
            break  # Or handle specific error if needed

        try:
            sb.append(sha512[i])
        except IndexError:
            # Handle cases where 'i' goes out of bounds for sha512_combined
            break  # Or handle specific error if needed

    return "".join(sb)


def main():
    from_ = "12DC195010"
    from_ = "1299295010"
    from_ = "12F1295010"
    pin = "CypCHG2kSlRkdvr2RG1QF8b2lCWXl7k7"
    uid = "1786055427"
    strs = [uid, from_, pin]
    print(generate_s(*strs))


if __name__ == "__main__":
    main()
