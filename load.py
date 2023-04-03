import sys
import serial
from serial.tools import miniterm


def main():
    if len(sys.argv) < 4:
        print("usage: load [serial] [baudrate] [payload]")
        return

    serial_path = sys.argv[1]
    baudrate = int(sys.argv[2])
    payload_path = sys.argv[3]

    with open(payload_path, "rb") as g:
        payload = g.read()

    serial_instance = serial.serial_for_url(serial_path, baudrate, bytesize=8)

    print("[I] waiting for notification...")
    while 1:
        if serial_instance.read()[0] == ord('O') and serial_instance.read()[0] == ord('K'):
            break

    print("[I] device ready for transfer... sending payload")
    serial_instance.write(len(payload).to_bytes(length=4, byteorder='little'))
    serial_instance.write(payload)

    print("[I] starting miniterm...")
    mini_term = miniterm.Miniterm(serial_instance)
    mini_term.set_rx_encoding("UTF-8")
    mini_term.set_tx_encoding("UTF-8")
    mini_term.start()
    mini_term.join()
    mini_term.close()


if __name__ == "__main__":
    main()
