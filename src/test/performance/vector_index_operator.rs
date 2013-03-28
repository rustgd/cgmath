/**
 * Index operator performance test. For best results, compile with the optimise
 * flag (-O). It seems like VecBufSlice is the fastest version.
 */

extern mod std;
use std::time::precise_time_ns;
use cast::{reinterpret_cast, transmute};
use vec::raw::buf_as_slice;
use ptr::to_unsafe_ptr;

pub struct Vector { x: float, y: float, z: float, w: float }

pub struct VecMatch { x: float, y: float, z: float, w: float }

pub impl VecMatch: Index<uint, float> {
    #[inline(always)]
    fn index(i: uint) -> float {
        match i {
            0 => self.x,
            1 => self.y,
            2 => self.z,
            3 => self.w,
            _ => fail!(~"Vector index out of bounds.")
        }
    }
}

pub struct VecReinterpret { x: float, y: float, z: float, w: float }

pub impl VecReinterpret: Index<uint, float> {
    #[inline(always)]
    fn index(i: uint) -> float unsafe {
        (reinterpret_cast::<VecReinterpret, [float * 4]>(&self))[i]
    }
}

pub struct VecTransmute { x: float, y: float, z: float, w: float }

pub impl VecTransmute: Index<uint, float> {
    #[inline(always)]
    fn index(i: uint) -> float unsafe {
        (transmute::<VecTransmute, [float * 4]>(self))[i]
    }
}

pub struct VecBufSlice { x: float, y: float, z: float, w: float }

pub impl VecBufSlice: Index<uint, float> {
    #[inline(always)]
    fn index(i: uint) -> float unsafe {
        do vec::raw::buf_as_slice(
            transmute::<*VecBufSlice, *float>(
                to_unsafe_ptr(&self)), 4) |slice| { slice[i] }
    }
}

fn main() {
    let n_tests = 100000;
    
    let v = Vector { x: 1f, y: 2f, z: 3f, w: 4f };
    
    let vfield_avg = do test_avg_time_ns(n_tests) {
        // io::println(fmt!("[ %?, %?, %?, %? ]", v.x, v.y, v.z, v.w));
        assert(v.x == 1f);
        assert(v.y == 2f);
        assert(v.z == 3f);
        assert(v.w == 4f);
    };
    
    io::println(fmt!("Vector.x,y,z,w:      [%d]", vfield_avg as int));
    
    let vmatch = VecMatch { x: 1f, y: 2f, z: 3f, w: 4f };
    
    let vindex_avg = do test_avg_time_ns(n_tests) {
        assert(vmatch[0] == 1f);
        assert(vmatch[1] == 2f);
        assert(vmatch[2] == 3f);
        assert(vmatch[3] == 4f);
    };
    
    let vreint = VecReinterpret { x: 1f, y: 2f, z: 3f, w: 4f };
    
    let vreint_avg = do test_avg_time_ns(n_tests) {
        assert(vreint[0] == 1f);
        assert(vreint[1] == 2f);
        assert(vreint[2] == 3f);
        assert(vreint[3] == 4f);
    };
    
    let vtrans = VecTransmute { x: 1f, y: 2f, z: 3f, w: 4f };
    
    let vtrans_avg = do test_avg_time_ns(n_tests) {
        assert(vtrans[0] == 1f);
        assert(vtrans[1] == 2f);
        assert(vtrans[2] == 3f);
        assert(vtrans[3] == 4f);
    };
    
    let vbufslice = VecBufSlice { x: 1f, y: 2f, z: 3f, w: 4f };
    
    let vbufslice_avg = do test_avg_time_ns(n_tests) {
        assert(vbufslice[0] == 1f);
        assert(vbufslice[1] == 2f);
        assert(vbufslice[2] == 3f);
        assert(vbufslice[3] == 4f);
    };
    
    let min = [vfield_avg, vindex_avg, vreint_avg, vtrans_avg, vbufslice_avg].min();
    
    io::println(fmt!("Vector.x,y,z,w:    %d = %d", vfield_avg as int, (vfield_avg - min) as int));
    io::println(fmt!("VecMatch[i]:       %d = %d", vindex_avg as int, (vindex_avg - min) as int));
    io::println(fmt!("VecReinterpret[i]: %d = %d", vreint_avg as int, (vreint_avg - min) as int));
    io::println(fmt!("VecTransmute[i]:   %d = %d", vtrans_avg as int, (vtrans_avg - min) as int));
    io::println(fmt!("VecBufSlice[i]:    %d = %d", vbufslice_avg as int, (vbufslice_avg - min) as int));
}

fn test_avg_time_ns(n: uint, f: fn&()) -> u64 {
    
    let mut total = 0;
    for n.times {
        let start_time = precise_time_ns();
        
        f();
        
        total += precise_time_ns() - start_time;
    }
    
    return total / (n as u64);
}