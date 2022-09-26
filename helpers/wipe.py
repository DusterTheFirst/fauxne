import os

def wipe(dir):
    for entry in os.ilistdir(dir):
        filename = f"{dir}/{entry[0]}"
        type = entry[1]

        is_dir = type == 0x4000

        if is_dir:
            wipe(filename)
            print(f"rmdir {filename}")
            os.rmdir(filename)
        else:
            print(f"rm {filename}")
            os.remove(filename)

wipe(".")