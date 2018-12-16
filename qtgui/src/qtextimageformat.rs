

// mod ::gui::QTextImageFormat
// package qtgui
// /usr/include/qt/QtGui/qtextformat.h
// #include <qtextformat.h>
// #include <QtGui>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 11
*/
// import "C"
// import "unsafe"
// import "reflect"
// import "fmt"
// import "log"
// import "github.com/kitech/qt.go/qtrt"
// import "github.com/kitech/qt.go/qtcore"
use qtcore::*; // super::super::%!s(MISSING)::*;
use std::default::Default;
use std::ops::Deref;
use qtrt; // super::super::qtrt;
use super::*;
//  ext block end

//  body block begin



/*

*/
#[derive(Default)] // class sizeof(QTextImageFormat)=16
pub struct QTextImageFormat {
  qbase: QTextCharFormat,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QTextImageFormat_ITF interface {
//    QTextCharFormat_ITF
//    QTextImageFormat_PTR() *QTextImageFormat
//}
//func (ptr *QTextImageFormat) QTextImageFormat_PTR() *QTextImageFormat { return ptr }

impl /*struct*/ QTextImageFormat {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QTextImageFormat {
    return QTextImageFormat{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QTextImageFormat {
//  type Target = QTextImageFormatBASE;
//
//  fn deref(&self) -> &QTextImageFormatBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QTextImageFormatBASE> for QTextImageFormat {
//  fn as_ref(& self) -> & QTextImageFormatBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtGui/qtextformat.h:735
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QTextImageFormat()

/*

*/
// QTextImageFormat() ctx.fn_proto_cpp
impl /*struct*/ QTextImageFormat {
  pub fn QTextImageFormat_0<T: QTextImageFormat_QTextImageFormat_0>(value: T) -> QTextImageFormat {
    let rsthis = value.QTextImageFormat_0();
    return rsthis;
    // return 1;
  }
}

pub trait QTextImageFormat_QTextImageFormat_0 {
  fn QTextImageFormat_0(self) -> QTextImageFormat;
}
// QTextImageFormat() ctx.fn_proto_cpp
impl<'a> /*trait*/ QTextImageFormat_QTextImageFormat_0 for () {
  fn QTextImageFormat_0(self) -> QTextImageFormat {
    // unsafe{_ZN16QTextImageFormatC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN16QTextImageFormatC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QTextImageFormat{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:752
// index:1
// Protected Visibility=Default Availability=Available
// [-2] void QTextImageFormat(const QTextFormat &)

/*

*/
// QTextImageFormat(const QTextFormat &) ctx.fn_proto_cpp
impl /*struct*/ QTextImageFormat {
  pub fn QTextImageFormat_1<T: QTextImageFormat_QTextImageFormat_1>(value: T) -> QTextImageFormat {
    let rsthis = value.QTextImageFormat_1();
    return rsthis;
    // return 1;
  }
}

pub trait QTextImageFormat_QTextImageFormat_1 {
  fn QTextImageFormat_1(self) -> QTextImageFormat;
}
// QTextImageFormat(const QTextFormat &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QTextImageFormat_QTextImageFormat_1 for (usize) {
  fn QTextImageFormat_1(self) -> QTextImageFormat {
    // unsafe{_ZN16QTextImageFormatC2ERK11QTextFormat()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN16QTextImageFormatC2ERK11QTextFormat", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QTextImageFormat{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:737
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool isValid() const

/*
Returns true if the format is valid (i.e. is not InvalidFormat); otherwise returns false.
*/
impl /*struct*/ QTextImageFormat {
  pub fn isValid_0<RetType, T: QTextImageFormat_isValid_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isValid_0(self);
    // return 1;
  }
}
pub trait QTextImageFormat_isValid_0<RetType> {
  fn isValid_0(self , rsthis: & QTextImageFormat) -> RetType;
}
impl<'a> /*trait*/ QTextImageFormat_isValid_0<bool> for () {
  fn isValid_0(self , rsthis: & QTextImageFormat) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QTextImageFormat7isValidEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:739
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setName(const QString &)

/*

*/
impl /*struct*/ QTextImageFormat {
  pub fn setName_0<RetType, T: QTextImageFormat_setName_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setName_0(self);
    // return 1;
  }
}
pub trait QTextImageFormat_setName_0<RetType> {
  fn setName_0(self , rsthis: & QTextImageFormat) -> RetType;
}
impl<'a> /*trait*/ QTextImageFormat_setName_0<(/*void*/)> for (usize) {
  fn setName_0(self , rsthis: & QTextImageFormat) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN16QTextImageFormat7setNameERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:740
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QString name() const

/*

*/
impl /*struct*/ QTextImageFormat {
  pub fn name_0<RetType, T: QTextImageFormat_name_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.name_0(self);
    // return 1;
  }
}
pub trait QTextImageFormat_name_0<RetType> {
  fn name_0(self , rsthis: & QTextImageFormat) -> RetType;
}
impl<'a> /*trait*/ QTextImageFormat_name_0<usize> for () {
  fn name_0(self , rsthis: & QTextImageFormat) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QTextImageFormat4nameEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:743
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setWidth(qreal)

/*

*/
impl /*struct*/ QTextImageFormat {
  pub fn setWidth_0<RetType, T: QTextImageFormat_setWidth_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setWidth_0(self);
    // return 1;
  }
}
pub trait QTextImageFormat_setWidth_0<RetType> {
  fn setWidth_0(self , rsthis: & QTextImageFormat) -> RetType;
}
impl<'a> /*trait*/ QTextImageFormat_setWidth_0<(/*void*/)> for (f64) {
  fn setWidth_0(self , rsthis: & QTextImageFormat) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN16QTextImageFormat8setWidthEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:744
// index:0
// Public inline Visibility=Default Availability=Available
// [8] qreal width() const

/*

*/
impl /*struct*/ QTextImageFormat {
  pub fn width_0<RetType, T: QTextImageFormat_width_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.width_0(self);
    // return 1;
  }
}
pub trait QTextImageFormat_width_0<RetType> {
  fn width_0(self , rsthis: & QTextImageFormat) -> RetType;
}
impl<'a> /*trait*/ QTextImageFormat_width_0<f64> for () {
  fn width_0(self , rsthis: & QTextImageFormat) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QTextImageFormat5widthEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:747
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setHeight(qreal)

/*

*/
impl /*struct*/ QTextImageFormat {
  pub fn setHeight_0<RetType, T: QTextImageFormat_setHeight_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setHeight_0(self);
    // return 1;
  }
}
pub trait QTextImageFormat_setHeight_0<RetType> {
  fn setHeight_0(self , rsthis: & QTextImageFormat) -> RetType;
}
impl<'a> /*trait*/ QTextImageFormat_setHeight_0<(/*void*/)> for (f64) {
  fn setHeight_0(self , rsthis: & QTextImageFormat) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN16QTextImageFormat9setHeightEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:748
// index:0
// Public inline Visibility=Default Availability=Available
// [8] qreal height() const

/*

*/
impl /*struct*/ QTextImageFormat {
  pub fn height_0<RetType, T: QTextImageFormat_height_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.height_0(self);
    // return 1;
  }
}
pub trait QTextImageFormat_height_0<RetType> {
  fn height_0(self , rsthis: & QTextImageFormat) -> RetType;
}
impl<'a> /*trait*/ QTextImageFormat_height_0<f64> for () {
  fn height_0(self , rsthis: & QTextImageFormat) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QTextImageFormat6heightEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}


pub fn DeleteQTextImageFormat(this :*mut QTextImageFormat) {
    // rv, err := qtrt::InvokeQtFunc6("_ZN16QTextImageFormatD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis())
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
//  body block end

//  keep block begin

//  keep block end
