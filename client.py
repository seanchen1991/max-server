import argparse
import requests
from dataclasses import asdict, dataclass

@dataclass
class Compare:
    request_id: int
    left: int
    right: int
    ty: str = "compare"

@dataclass
class ComparisonResult:
    request_id: int
    answer: bool
    ty: str = "comp_result"

@dataclass
class ComputeMax:
    length: int
    ty: str = "compute_max"

@dataclass
class ComputeMin:
    length: int
    ty: str = "compute_min"

@dataclass
class Done:
    result: int
    ty: str = "done"

def message_to_struct(message):
    if message["ty"] == "compare":
        return Compare(**message)
    elif message["ty"] == "comp_result":
        return ComparisonResult(**message)
    elif message["ty"] == "compute_min":
        return ComputeMin(**message)
    elif message["ty"] == "compute_max":
        return ComputeMax(**message)
    elif message["ty"] == "done":
        return Done(**message)

class Client:
    def __init__(self, address, log=False):
        self.address = address if address else "http://localhost:8000"
        self.log = log

    def send(self, data):
        response = requests.post(self.address, json=asdict(data))
        json = response.json()
        if self.log:
            print(json)
        return message_to_struct(json)

    def compute(self, values, op):
        req = None
        if op == "min":
            req = ComputeMin(len(values))
        elif op == "max":
            req = ComputeMax(len(values))
        else:
            assert False, "not supported operation: " + op
        next_message = self.send(req)

        while True:
            if next_message.ty == "done":
                return values[next_message.result]
            elif next_message.ty == "compare":
                request_id = next_message.request_id
                left = next_message.left
                right = next_message.right
                next_message = self.send(ComparisonResult(request_id, values[left] < values[right]))
            else:
                raise Exception("Unexpected message: ", next_message)


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description='Run the max computer')
    parser.add_argument('--address', type=str, required=False,
                        help='address of the max computer (defaults to http://localhost:5000)')

    args = parser.parse_args()
    client = Client(args.address, log=True)
    assert 3 == client.compute([1, 2, 3, 1], op="max")
    assert 1 == client.compute([1, 2, 3, 1], op="min")
