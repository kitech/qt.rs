

// mod ::core::QAbstractConcatenable
// package qtcore
// /usr/include/qt/QtCore/qstringbuilder.h
// #include <qstringbuilder.h>
// #include <QtCore>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 24
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

// void convertFromAscii(const char *, int, QChar *&)
// func (this *QAbstractConcatenable) InheritConvertFromAscii(f func(a string, len_ int, out *QChar) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "convertFromAscii", f)
// }

// void appendLatin1To(const char *, int, QChar *)
// func (this *QAbstractConcatenable) InheritAppendLatin1To(f func(a string, len_ int, out *QChar/*777 QChar **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "appendLatin1To", f)
// }



/*

*/
#[derive(Default)] // class sizeof(QAbstractConcatenable)=1
pub struct QAbstractConcatenable {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QAbstractConcatenable_ITF interface {
//    QAbstractConcatenable_PTR() *QAbstractConcatenable
//}
//func (ptr *QAbstractConcatenable) QAbstractConcatenable_PTR() *QAbstractConcatenable { return ptr }

impl /*struct*/ QAbstractConcatenable {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QAbstractConcatenable {
    return QAbstractConcatenable{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QAbstractConcatenable {
//  type Target = QAbstractConcatenableBASE;
//
//  fn deref(&self) -> &QAbstractConcatenableBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QAbstractConcatenableBASE> for QAbstractConcatenable {
//  fn as_ref(& self) -> & QAbstractConcatenableBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qstringbuilder.h:61
// index:0
// Protected static Visibility=Default Availability=Available
// [-2] void convertFromAscii(const char *, int, QChar *&)

/*

*/
impl /*struct*/ QAbstractConcatenable {
  pub fn convertFromAscii_0<RetType, T: QAbstractConcatenable_convertFromAscii_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.convertFromAscii_0();
    // return 1;
  }
}
pub trait QAbstractConcatenable_convertFromAscii_0<RetType> {
  fn convertFromAscii_0(self ) -> RetType;
}
impl<'a> /*trait*/ QAbstractConcatenable_convertFromAscii_0<(/*void*/)> for (usize,i32,usize) {
  fn convertFromAscii_0(self ) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (self.0) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN21QAbstractConcatenable16convertFromAsciiEPKciRP5QChar", 3,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qstringbuilder.h:62
// index:1
// Protected static inline Visibility=Default Availability=Available
// [-2] void convertFromAscii(char, QChar *&)

/*

*/
impl /*struct*/ QAbstractConcatenable {
  pub fn convertFromAscii_1<RetType, T: QAbstractConcatenable_convertFromAscii_1<RetType>>( overload_args: T) -> RetType {
    return overload_args.convertFromAscii_1();
    // return 1;
  }
}
pub trait QAbstractConcatenable_convertFromAscii_1<RetType> {
  fn convertFromAscii_1(self ) -> RetType;
}
impl<'a> /*trait*/ QAbstractConcatenable_convertFromAscii_1<(/*void*/)> for (i8,usize) {
  fn convertFromAscii_1(self ) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i8 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN21QAbstractConcatenable16convertFromAsciiEcRP5QChar", 2,qtrt::FFITY_SINT8,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qstringbuilder.h:66
// index:0
// Protected static Visibility=Default Availability=Available
// [-2] void appendLatin1To(const char *, int, QChar *)

/*

*/
impl /*struct*/ QAbstractConcatenable {
  pub fn appendLatin1To_0<RetType, T: QAbstractConcatenable_appendLatin1To_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.appendLatin1To_0();
    // return 1;
  }
}
pub trait QAbstractConcatenable_appendLatin1To_0<RetType> {
  fn appendLatin1To_0(self ) -> RetType;
}
impl<'a> /*trait*/ QAbstractConcatenable_appendLatin1To_0<(/*void*/)> for (usize,i32,usize) {
  fn appendLatin1To_0(self ) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (self.0) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN21QAbstractConcatenable14appendLatin1ToEPKciP5QChar", 3,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}


pub fn DeleteQAbstractConcatenable(this :*mut QAbstractConcatenable) {
    // rv, err := qtrt::InvokeQtFunc6("_ZN21QAbstractConcatenableD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis())
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
//  body block end

//  keep block begin

//  keep block end
