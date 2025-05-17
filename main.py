import math
import qucirc
from qucirc import ops

circ1 = qucirc.Circuit(2)
circ1 += ops.P(1/3 * math.pi)[1]
circ1 += ops.H[0]

circ2 = qucirc.Circuit()
circ2 += ops.H[0]
circ2 += ops.P(1/3 * math.pi)[1]

assert circ1 == circ2