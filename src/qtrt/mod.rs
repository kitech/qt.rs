
extern crate libc;
use libc::c_void;


extern {}

mod safedl;

fn aaa() {
    // safedl::dlopeni(); // usage
}

pub fn InvokeQtFunc6(symname :&str, argc :i8,
                     ty0 :i8, ty1 :i8, ty2 :i8, ty3: i8, ty4: i8, ty5: i8, ty6: i8, ty7: i8, ty8: i8, ty9: i8,
                     ty10: i8, ty11: i8, ty12: i8, ty13: i8, ty14: i8,
                     a0 :usize, a1 :usize, a2 :usize, a3 :usize, a4 :usize, a5 :usize,
                     a6 :usize, a7 :usize, a8 :usize, a9 :usize,
                     a10:usize, a11 :usize, a12 :usize, a13 :usize, a14 :usize) -> usize {
    // get sym addr
    // pack args
    // ffi_call
    // conv ret
    a0
}

fn InvokeQtFuncImpl(symname :&str, argc :i8,
                    ty0 :i8, ty1 :i8, ty2 :i8,
                    a0 :*mut c_void, a1 :*mut c_void, a2 :*mut c_void) -> *mut c_void{
    // get sym addr
    // pack args
    // ffi_call
    // conv ret
    &argc as *const i8 as usize;
    let a :*const i8 = (&argc) as *const i8;
    a0 as usize;
    let v = vec![a0, a1, a2];
    let vp: *mut *mut c_void = v.as_ptr() as *mut *mut c_void;
    a0
}

fn atest() {
    let zp = ZPtr();
    InvokeQtFuncImpl("aaa", 0, 0, 1, 2, zp, zp, zp);
}

pub fn ZPtr() -> *mut c_void { 0 as *mut c_void }


pub const FFI_TYPE_VOID :i8 =       0    ;
pub const FFI_TYPE_INT    :i8 =     1;
pub const FFI_TYPE_FLOAT   :i8 =    2    ;
pub const FFI_TYPE_DOUBLE   :i8 =   3;
// #if 1
pub const FFI_TYPE_LONGDOUBLE :i8 = 4;
// #else
// pub const FFI_TYPE_LONGDOUBLE FFI_TYPE_DOUBLE
// #endif
pub const FFI_TYPE_UINT8      :i8 = 5   ;
pub const FFI_TYPE_SINT8      :i8 = 6;
pub const FFI_TYPE_UINT16     :i8 = 7 ;
pub const FFI_TYPE_SINT16    :i8 =  8;
pub const FFI_TYPE_UINT32    :i8 =  9;
pub const FFI_TYPE_SINT32    :i8 =  10;
pub const FFI_TYPE_UINT64    :i8 =  11;
pub const FFI_TYPE_SINT64    :i8 =  12;
pub const FFI_TYPE_STRUCT   :i8 =   13;
pub const FFI_TYPE_POINTER   :i8 =  14;
pub const FFI_TYPE_COMPLEX  :i8 =   15;

pub const FFITY_VOID :i8 =       0    ;
pub const FFITY_INT    :i8 =     1;
pub const FFITY_FLOAT   :i8 =    2    ;
pub const FFITY_DOUBLE   :i8 =   3;
// #if 1
pub const FFITY_LONGDOUBLE :i8 = 4;
// #else
// pub const FFITY_LONGDOUBLE FFITY_DOUBLE
// #endif
pub const FFITY_UINT8      :i8 = 5   ;
pub const FFITY_SINT8      :i8 = 6;
pub const FFITY_UINT16     :i8 = 7 ;
pub const FFITY_SINT16    :i8 =  8;
pub const FFITY_UINT32    :i8 =  9;
pub const FFITY_SINT32    :i8 =  10;
pub const FFITY_UINT64    :i8 =  11;
pub const FFITY_SINT64    :i8 =  12;
pub const FFITY_STRUCT   :i8 =   13;
pub const FFITY_POINTER   :i8 =  14;
pub const FFITY_COMPLEX  :i8 =   15;

//
pub fn GetClassEnumItemName(cls: &str, val: i32) -> String {
    return String::from("");
}
