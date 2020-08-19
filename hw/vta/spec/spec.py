
from collections import OrderedDict

def round(width: int) -> int:
    if width % 8 == 0:
        return int(width/8)
    else:
        return int(width/8) + 1

class Field():
    def __init__(self, name: str, width: int) -> None:
        self.name = name
        self.width = width
        self.value = [0] * round(width)
        self.check()

    def get_width(self) -> int:
        return self.width

    def get_name(self) -> str:
        return self.name

    def get_value(self, offset: int) -> int:
        return self.value[offset]

    def set_width(self, width: int) -> None:
        self.width = width

    def set_name(self, name: str) -> None:
        self.name = name

    def set_value(self, value: int, offset: int) -> None:
        self.value[offset] |= value

    def check(self) -> None:
        assert self.width > 0, "width must be greater than zero"

class Instruction():
    def __init__(self, width: int) -> None:
        self.width = width
        self.field_map = OrderedDict()

    def get_width(self) -> int:
        return self.width

    def contains_field(self, pos: int) -> bool:
        if pos in self.field_map:
            True
        else:
            False

    def get_field(self, pos: int) -> Field:
        assert self.contains_field(pos), "field does not exists"
        self.field_map[pos]

    def add_field(self, pos: int, field: Field) -> None:
        assert not self.contains_field(pos), "field already exists"
        self.field_map[pos] = field

    def set_field_value(self, pos: int, value: int, offset: int) -> None:
        field = self.get_field(pos)
        self.get_field[pos] = field.set_value(value, offset)

    def check(self) -> None:
        total = 0
        for _, field in self.field_map.items():
            field.check()
            total += field.get_width()
        assert total <= self.width, "fields do not fit instruction width"


if __name__ == "__main__":
    opcode = Field("op", 3)
    load = Instruction(128)
    gemm = Instruction(128)
    store = Instruction(128)
    load.add_field(0, opcode)
    gemm.add_field(0, opcode)
    store.add_field(0, opcode)
    load.check()
    gemm.check()
    store.check()