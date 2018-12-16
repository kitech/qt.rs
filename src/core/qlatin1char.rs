

// mod ::core::QLatin1Char
// package qtcore
// /usr/include/qt/QtCore/qchar.h
// #include <qchar.h>
// #include <QtCore>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 43
*/
// import "C"
// import "unsafe"
// import "reflect"
// import "fmt"
// import "log"
// import "github.com/kitech/qt.go/qtrt"
use std::default::Default;
use std::ops::Deref;
use super::super::qtrt;
use super::*;
//  ext block end

//  body block begin



/*

*/
#[derive(Default)] // class sizeof(QLatin1Char)=1
pub struct QLatin1Char {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QLatin1Char_ITF interface {
//    QLatin1Char_PTR() *QLatin1Char
//}
//func (ptr *QLatin1Char) QLatin1Char_PTR() *QLatin1Char { return ptr }

impl /*struct*/ QLatin1Char {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QLatin1Char {
    return QLatin1Char{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QLatin1Char {
//  type Target = QLatin1CharBASE;
//
//  fn deref(&self) -> &QLatin1CharBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QLatin1CharBASE> for QLatin1Char {
//  fn as_ref(& self) -> & QLatin1CharBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qchar.h:53
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void QLatin1Char(char)

/*

*/
// QLatin1Char(char) ctx.fn_proto_cpp
impl /*struct*/ QLatin1Char {
  pub fn QLatin1Char_0<T: QLatin1Char_QLatin1Char_0>(value: T) -> QLatin1Char {
    let rsthis = value.QLatin1Char_0();
    return rsthis;
    // return 1;
  }
}

pub trait QLatin1Char_QLatin1Char_0 {
  fn QLatin1Char_0(self) -> QLatin1Char;
}
// QLatin1Char(char) ctx.fn_proto_cpp
impl<'a> /*trait*/ QLatin1Char_QLatin1Char_0 for (i8) {
  fn QLatin1Char_0(self) -> QLatin1Char {
    // unsafe{_ZN11QLatin1CharC2Ec()};
    let arg0 = (&self) as *const i8 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN11QLatin1CharC2Ec", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QLatin1Char{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qchar.h:54
// index:0
// Public inline Visibility=Default Availability=Available
// [1] char toLatin1() const

/*
Returns the Latin-1 character equivalent to the QChar, or 0. This is mainly useful for non-internationalized software.

Note: It is not possible to distinguish a non-Latin-1 character from a Latin-1 0 (NUL) character. Prefer to use unicode(), which does not have this ambiguity.

See also unicode().
*/
impl /*struct*/ QLatin1Char {
  pub fn toLatin1_0<RetType, T: QLatin1Char_toLatin1_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toLatin1_0(self);
    // return 1;
  }
}
pub trait QLatin1Char_toLatin1_0<RetType> {
  fn toLatin1_0(self , rsthis: & QLatin1Char) -> RetType;
}
impl<'a> /*trait*/ QLatin1Char_toLatin1_0<i8> for () {
  fn toLatin1_0(self , rsthis: & QLatin1Char) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QLatin1Char8toLatin1Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i8 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qchar.h:55
// index:0
// Public inline Visibility=Default Availability=Available
// [2] ushort unicode() const

/*
Returns the numeric Unicode value of the QChar.
*/
impl /*struct*/ QLatin1Char {
  pub fn unicode_0<RetType, T: QLatin1Char_unicode_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.unicode_0(self);
    // return 1;
  }
}
pub trait QLatin1Char_unicode_0<RetType> {
  fn unicode_0(self , rsthis: & QLatin1Char) -> RetType;
}
impl<'a> /*trait*/ QLatin1Char_unicode_0<u16> for () {
  fn unicode_0(self , rsthis: & QLatin1Char) -> u16 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QLatin1Char7unicodeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: u16 = Default::default(); return dret;
  }
}


pub fn DeleteQLatin1Char(this :*mut QLatin1Char) {
    // rv, err := qtrt::InvokeQtFunc6("_ZN11QLatin1CharD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis())
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
//  body block end

//  keep block begin

//  keep block end
