#!/usr/bin/env python3


def make_lengths():
    n = 8
    lengths = []
    for d in range(1, n + 1):
        for c in range(1, n + 1):
            for b in range(1, n + 1):
                for a in range(1, n + 1):
                    lengths.append((a, b, c, d))
    return lengths


def print_header():
    print("""#![cfg_attr(rustfmt, rustfmt_skip)]

#[cfg(target_arch = "x86")]
use std::arch::x86::__m256i;
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::__m256i;

#[repr(C)]
pub union Hack {
    pub v: __m256i,
    b: [i8; 32],
}""")


def print_lengths(lengths):
    print("pub static LENGTH: [u8; {}] = [".format(len(lengths)), end="")

    for i, (a, b, c, d) in enumerate(lengths):
        if i % 32 == 0:
            print("\n    ", end="")
        else:
            print(" ", end="")
        print("{},".format(a + b + c + d), end="")
    print("\n];")


def print_decode_shuffle_1(lengths):
    print(
        "pub static DECODE_SHUFFLE_1: [Hack; {}] = [".format(len(lengths)),
        end=""
    )

    for a, b, c, d in lengths:
        print("\n    Hack { b: [", end="")
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
                    byte = -1

                print("{}".format(byte), end="")

        for n in [c, d]:
            for i in range(0, 8):
                print(", ", end="")

                if n > i:
                    if next_byte >= 16:
                        byte = next_byte - 16
                    else:
                        byte = -1
                    next_byte += 1
                else:
                    byte = -1

                print(byte, end="")
        print("] },", end="")

    print("\n];")


def print_decode_shuffle_2(lengths):
    print(
        "pub static DECODE_SHUFFLE_2: [Hack; {}] = [".format(len(lengths)),
        end=""
    )

    for a, b, c, d in lengths:
        print("\n    Hack { b: [", end="")
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
                    byte = -1
                print(byte, end="")

        for _ in range(0, 16):
            print(", -1", end="")
        print("] },", end="")
    print("\n];")


def print_encode_shuffle_1(lengths):
    print(
        "pub static ENCODE_SHUFFLE_1: [Hack; {}] = [".format(len(lengths)),
        end=""
    )

    for a, b, c, d in lengths:
        print("\n    Hack { b: [", end="")
        first = True
        base = 0
        next_byte = 0
        for n in [a, b]:
            for i in range(0, n):
                if not first:
                    print(", ", end="")
                first = False

                byte = base + i
                print(byte, end="")
                next_byte += 1
            base += 8

        for _ in range(a + b, 16):
            print(", -1", end="")

        base = 0
        written = 0
        for n in [c, d]:
            for i in range(0, n):
                if next_byte >= 16:
                    byte = base + i
                    written += 1
                    print(",", byte, end="")
                next_byte += 1
            base += 8

        for _ in range(written, 16):
            print(", -1", end="")
        print("] },", end="")
    print("\n];")


def print_encode_shuffle_2(lengths):
    print(
        "pub static ENCODE_SHUFFLE_2: [Hack; {}] = [".format(len(lengths)),
        end=""
    )

    for a, b, c, d in lengths:
        print("\n    Hack { b: [", end="")

        written = 16 + a + b
        for i in range(0, written):
            if i != 0:
                print(", ", end="")
            print("-1", end="")

        next_byte = a + b
        for idx, n in enumerate([c, d]):
            for i in range(0, n):
                if written < 32:
                    if next_byte < 16:
                        byte = idx * 8 + i
                    else:
                        byte = -1
                    print(",", byte, end="")
                    written += 1
                    next_byte += 1

        for _ in range(written, 32):
            print(", -1", end="")
        print("] },", end="")
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
    print()
    print_encode_shuffle_1(lengths)
    print()
    print_encode_shuffle_2(lengths)
