import serial
import cbor2
from cobs import cobs
import time

class CBOR_UART_Communicator:
    def __init__(self, port, baudrate=9600):
        self.ser = serial.Serial(port, baudrate, timeout=1)
        
    def send(self, data):
        # Serialize to CBOR
        cbor_data = cbor2.dumps(data)
        
        # Encode with COBS
        cobs_data = cobs.encode(cbor_data)
        
        # Add null terminator and send
        self.ser.write(cobs_data + b'\x00')
        
    def receive(self):
        # Read until null terminator
        cobs_data = bytearray()
        while True:
            byte = self.ser.read(1)
            if byte == b'\x00':
                break
            if byte:
                cobs_data += byte
        
        if not cobs_data:
            return None
            
        # Decode COBS
        try:
            cbor_data = cobs.decode(cobs_data)
        except cobs.DecodeError:
            print("COBS decode error")
            return None
            
        # Deserialize CBOR
        try:
            return cbor2.loads(cbor_data)
        except cbor2.CBORDecodeError:
            print("CBOR decode error")
            return None
    
    def close(self):
        self.ser.close()

# Example usage
if __name__ == "__main__":
    comm = CBOR_UART_Communicator('/dev/ttyACM0', 9600)
    
    value = 220
    factor = 60
    while True:  
        value = value  + factor
        # Send a dictionary
        comm.send({ 
            "position": value, 
        }) 

        if (value > 500):
            factor = -1*factor
        if (value < 250):
            factor = -1*factor
        print(f"Sent value: {value}")
        time.sleep(0.25)