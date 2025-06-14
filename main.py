import serial
import cbor2
from cobs import cobs
import time

class CBOR_UART_Communicator:
    def __init__(self, port, baudrate=9600):
        self.ser = serial.Serial(port, baudrate, timeout=1)
        
    def send_position(self, data):

        data_to_send = {"Sensor": data}
        # Serialize to CBOR
        cbor_data = cbor2.dumps(data_to_send)
        
        # Encode with COBS
        cobs_data = cobs.encode(cbor_data)
        
        # Add null terminator and send
        self.ser.write(cobs_data + b'\x00')

        
    def send_config(self, data):

        data_to_send = {"Config": data}
        # Serialize to CBOR
        cbor_data = cbor2.dumps(data_to_send)
        
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
    comm = CBOR_UART_Communicator('/dev/ttyACM0', 38400)
    
    value = 90
    factor = 90

    # comm.send_config({
    #     "position_steps": 6
    # })
    # time.sleep(3)

    while True:   
        value = 0
        comm.send_position({ 
            "position_c0": value, 
            "position_c1": value, 
            "position_c2": value, 
            "position_c3": value, 
            "position_c4": value, 
            "position_c5": value, 
            "position_c6": value, 
            "position_c7": value, 
            "position_c8": value, 
            "position_c9": value, 
            "position_c10": value, 
            "position_c11": value, 
        }) 

        print(f"Sent value: {value}")
        time.sleep(0.35) 

        value = 180
        comm.send_position({ 
            "position_c0": value, 
            "position_c1": value, 
            "position_c2": value, 
            "position_c3": value, 
            "position_c4": value, 
            "position_c5": value, 
            "position_c6": value, 
            "position_c7": value, 
            "position_c8": value, 
            "position_c9": value, 
            "position_c10": value, 
            "position_c11": value, 
        }) 

        print(f"Sent value: {value}")
        time.sleep(0.35) 


        value = 90
        comm.send_position({ 
            "position_c0": value, 
            "position_c1": value, 
            "position_c2": value, 
            "position_c3": value, 
            "position_c4": value, 
            "position_c5": value, 
            "position_c6": value, 
            "position_c7": value, 
            "position_c8": value, 
            "position_c9": value, 
            "position_c10": value, 
            "position_c11": value, 
        }) 

        print(f"Sent value: {value}")
        time.sleep(0.35) 