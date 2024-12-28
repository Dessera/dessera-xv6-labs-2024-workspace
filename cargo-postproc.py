import os
import sys
import shutil

CARGO_BUILD_BASE="./target/riscv64gc-unknown-none-elf/debug/deps/"
RSOBJ_SRC="./target/objs/"

def get_object(name: str):
    # get $CARGO_BUILD_BASE/{name}-<hash>.o
    for file in os.listdir(CARGO_BUILD_BASE):
        if (name in file) and file.endswith(".o"):
            return CARGO_BUILD_BASE + file
    return None
          
def move_object(names: list[str]):
    for name in names:
        src_path = get_object(name)
        if src_path is None:
            print(f"Object file {name} not found.")
            return False
        dst_path = RSOBJ_SRC + name + ".o"
        shutil.copyfile(src_path, dst_path)
    return True

def main():
    # mkdir if RSOBJ_SRC does not exist
    if not os.path.exists(RSOBJ_SRC):
        os.makedirs(RSOBJ_SRC)
    # from args get the names of the object files
    objects_name = sys.argv[1:]
    if not move_object(objects_name):
        sys.exit(1)
    
if __name__ == "__main__":
    main()