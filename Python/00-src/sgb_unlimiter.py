import psutil
from base64 import a85decode as decoder
from colorama import init, Fore, Style


def _n():
    print()
def _ok(msg):
    print(f"{Fore.GREEN}[  OK  ]{Style.RESET_ALL} {msg}")
def _err(msg):
    print(f"{Fore.RED}[ EROR ]{Style.RESET_ALL} {msg}")
def _dbg(msg):
    print(f"{Fore.LIGHTCYAN_EX}[ INFO ]{Style.RESET_ALL} {msg}")

def main(encoded):

    total  = 0
    worked = 0

    targets = {
        each.lower() for each in
        decoder(encoded).decode('utf-8').strip().split('N')
    }

    for process in psutil.process_iter():
        total += 1
        processname = process.name().split('.')[0].lower()

        if processname in targets:
            process.suspend()
            worked += 1

    return worked, total


if __name__ == "__main__":
    init()

    encoded = b"EHQ&*E--2@:2OENF*)G@G@b5lB4Yt&:2OENF*)G@G@b6)G&g>qBP)-n@sWH<:2XZ^GA;AJAn1"

    _dbg("Proceed with prebuilt mode...")

    _n()

    try:
        worked, total = main(encoded)
        _ok(f"Done {Fore.LIGHTBLACK_EX}[ {Fore.CYAN}{worked}{Fore.LIGHTBLACK_EX} / {total} ]{Style.RESET_ALL}")
        if worked < 4:
            _n()
            _err("Some program may not terminated. Please execute again. ")
            _dbg("If you see this message after a few attempts, it's working normally. ")
    except Exception as exc:
        _err(exc)

    _n()
    _dbg(f"Press {Fore.YELLOW}ENTER{Style.RESET_ALL} to exit...")
    input()
