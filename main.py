import serial
import cbor2
from cobs import cobs
import time

class CBOR_UART_Communicator:
    def __init__(self, port, baudrate=9600):
        self.ser = serial.Serial(port, baudrate, timeout=1)
        
    def send(self, data):

        data_to_send = {"Sensor": data}
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
    
    value = 450
    factor = 20
    while True:  
        #value = value  + factor
        # Send a dictionary
        comm.send({ 
            "position_c0": value, 
            "position_c1": value+1, 
            "position_c2": value+2, 
            "position_c3": value+3, 
            "position_c4": value+4, 
            "position_c5": value+5, 
            "position_c6": value+6, 
            "position_c7": value+7, 
            "position_c8": value+8, 
            "position_c9": value+9, 
            "position_c10": value+10, 
            "position_c11": value+11, 
        }) 

        if value == 450:
            value = 250
        else:  # value is 350
            value = 450
 
        print(f"Sent value: {value}")
        time.sleep(0.2)

    # while True:  
    #     value = value  + factor
    #     # Send a dictionary
    #     comm.send({ 
    #         "position_c0": value, 
    #         "position_c1": value+1, 
    #         "position_c2": value+2, 
    #         "position_c3": value+3, 
    #         "position_c4": value+4, 
    #         "position_c5": value+5, 
    #         "position_c6": value+6, 
    #         "position_c7": value+7, 
    #         "position_c8": value+8, 
    #         "position_c9": value+9, 
    #         "position_c10": value+10, 
    #         "position_c11": value+11, 
    #     }) 

    #     if (value > 450):
    #         factor = -1*factor
    #     if (value < 350):
    #         factor = -1*factor
    #     print(f"Sent value: {value}")
    #     time.sleep(0.2)