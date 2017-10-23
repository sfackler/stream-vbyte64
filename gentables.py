#!/usr/bin/env python3


def make_lengths():
    n = 8
    lengths = []
    for a in range(1, n + 1):
        for b in range(1, n + 1):
            for c in range(1, n + 1):
                for d in range(1, n + 1):
                    lengths.append((d, c, b, a))
    return lengths


def print_header():
    print(""" #![cfg_attr(rustfmt, rustfmt_skip)]

use stdsimd::simd::u8x32;""")


def print_lengths(lengths):
    print("pub static LENGTH: [u8; {}] = [".format(len(lengths)), end="")

    for i, (a, b, c, d) in enumerate(lengths):
        if i % 32 == 0:
            print("""\n    """, end="")
        else:
            print(" ", end="")
        print("{},".format(a + b + c + d), end="")
    print("\n];")


def print_decode_shuffle_1(lengths):
    print(
        "pub static DECODE_SHUFFLE_1: [u8x32; {}] = [".format(len(lengths)),
        end=""
    )

    for a, b, c, d in lengths:
        print("\n    u8x32::new(", end="")
        first = True
        next_byte = 0
        for n in [a, b]:
            for i in range(0, 8):
                if not first:
                    print(", ", end="")
                first = False

                if n > i:
                    byte = next_byte
                    next_byte += 1
                else:
                    byte = 255

                print("{}".format(byte), end="")

        for n in [c, d]:
            for i in range(0, 8):
                print(", ", end="")

                if n > i:
                    if next_byte >= 16:
                        byte = next_byte - 16
                    else:
                        byte = 255
                    next_byte += 1
                else:
                    byte = 255

                print(byte, end="")
        print("),", end="")

    print("\n];")


def print_decode_shuffle_2(lengths):
    print(
        "pub static DECODE_SHUFFLE_2: [u8x32; {}] = [".format(len(lengths)),
        end=""
    )

    for a, b, c, d in lengths:
        print("\n    u8x32::new(", end="")
        first = True
        next_byte = a + b
        for n in [c, d]:
            for i in range(0, 8):
                if not first:
                    print(", ", end="")
                first = False

                if n > i and next_byte < 16:
                    byte = next_byte
                    next_byte += 1
                else:
                    byte = 255
                print(byte, end="")

        for _ in range(0, 16):
            print(", 255", end="")
        print("),", end="")
    print("\n];")


if __name__ == "__main__":
    lengths = make_lengths()
    print_header()
    print()
    print_lengths(lengths)
    print()
    print_decode_shuffle_1(lengths)
    print()
    print_decode_shuffle_2(lengths)
