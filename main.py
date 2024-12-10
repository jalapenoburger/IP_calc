def DecToBin(dec: int) -> str:
    bin = ""
    while dec > 0:
        bin = str(dec % 2) + bin
        dec //= 2

    while len(bin) != 8:
        bin = "0" + bin

    return bin


if __name__ == '__main__':

    print(DecToBin(96))
