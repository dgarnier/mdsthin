#
# Copyright (c) 2024, Massachusetts Institute of Technology All rights reserved.
#
# Redistribution and use in source and binary forms, with or without
# modification, are permitted provided that the following conditions are met:
#
# Redistributions of source code must retain the above copyright notice, this
# list of conditions and the following disclaimer.
#
# Redistributions in binary form must reproduce the above copyright notice, this
# list of conditions and the following disclaimer in the documentation and/or
# other materials provided with the distribution.
#
# THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
# AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
# IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
# DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE
# FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
# DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
# SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
# CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
# OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
# OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
#

from .connection import Connection, GetMany, PutMany

from .descriptors import (
    Numeric, Descriptor,
    DescriptorS,
        String, Ident, TreeNID, TreePath,
        UInt8, UInt16, UInt32, UInt64,
        Int8, Int16, Int32, Int64,
        Float32, Float64,
    DescriptorA,
        StringArray,
        UInt8Array, UInt16Array, UInt32Array, UInt64Array,
        Int8Array, Int16Array, Int32Array, Int64Array,
        Float32Array, Float64Array,
    DescriptorAPD,
        List, Tuple, Dictionary,
    DescriptorR,
        Signal, Dimension, Window, Slope, Function, Conglom, Range,
        Action, Dispatch, Program, Routine, Procedure, Method,
        Dependency, Condition, WithUnits, Call, WithError, Opaque,
)

from .exceptions import *
from .functions import *

from . import version