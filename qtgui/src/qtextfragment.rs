

// mod ::gui::QTextFragment
// package qtgui
// /usr/include/qt/QtGui/qtextobject.h
// #include <qtextobject.h>
// #include <QtGui>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 36
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
#[derive(Default)] // class sizeof(QTextFragment)=16
pub struct QTextFragment {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QTextFragment_ITF interface {
//    QTextFragment_PTR() *QTextFragment
//}
//func (ptr *QTextFragment) QTextFragment_PTR() *QTextFragment { return ptr }

impl /*struct*/ QTextFragment {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QTextFragment {
    return QTextFragment{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QTextFragment {
//  type Target = QTextFragmentBASE;
//
//  fn deref(&self) -> &QTextFragmentBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QTextFragmentBASE> for QTextFragment {
//  fn as_ref(& self) -> & QTextFragmentBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtGui/qtextobject.h:307
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void QTextFragment()

/*

*/
// QTextFragment() ctx.fn_proto_cpp
impl /*struct*/ QTextFragment {
  pub fn QTextFragment_0<T: QTextFragment_QTextFragment_0>(value: T) -> QTextFragment {
    let rsthis = value.QTextFragment_0();
    return rsthis;
    // return 1;
  }
}

pub trait QTextFragment_QTextFragment_0 {
  fn QTextFragment_0(self) -> QTextFragment;
}
// QTextFragment() ctx.fn_proto_cpp
impl<'a> /*trait*/ QTextFragment_QTextFragment_0 for () {
  fn QTextFragment_0(self) -> QTextFragment {
    // unsafe{_ZN13QTextFragmentC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN13QTextFragmentC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QTextFragment{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextobject.h:309
// index:0
// Public inline Visibility=Default Availability=Available
// [16] QTextFragment & operator=(const QTextFragment &)

/*

*/
impl /*struct*/ QTextFragment {
  pub fn operator_equal_0<RetType, T: QTextFragment_operator_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_0(self);
    // return 1;
  }
}
pub trait QTextFragment_operator_equal_0<RetType> {
  fn operator_equal_0(self , rsthis: & QTextFragment) -> RetType;
}
impl<'a> /*trait*/ QTextFragment_operator_equal_0<usize> for (usize) {
  fn operator_equal_0(self , rsthis: & QTextFragment) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN13QTextFragmentaSERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextobject.h:311
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool isValid() const

/*

*/
impl /*struct*/ QTextFragment {
  pub fn isValid_0<RetType, T: QTextFragment_isValid_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isValid_0(self);
    // return 1;
  }
}
pub trait QTextFragment_isValid_0<RetType> {
  fn isValid_0(self , rsthis: & QTextFragment) -> RetType;
}
impl<'a> /*trait*/ QTextFragment_isValid_0<bool> for () {
  fn isValid_0(self , rsthis: & QTextFragment) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QTextFragment7isValidEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextobject.h:313
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool operator==(const QTextFragment &) const

/*

*/
impl /*struct*/ QTextFragment {
  pub fn operator_equal_equal_0<RetType, T: QTextFragment_operator_equal_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_equal_0(self);
    // return 1;
  }
}
pub trait QTextFragment_operator_equal_equal_0<RetType> {
  fn operator_equal_equal_0(self , rsthis: & QTextFragment) -> RetType;
}
impl<'a> /*trait*/ QTextFragment_operator_equal_equal_0<bool> for (usize) {
  fn operator_equal_equal_0(self , rsthis: & QTextFragment) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QTextFragmenteqERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextobject.h:314
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool operator!=(const QTextFragment &) const

/*

*/
impl /*struct*/ QTextFragment {
  pub fn operator_not_equal_0<RetType, T: QTextFragment_operator_not_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_not_equal_0(self);
    // return 1;
  }
}
pub trait QTextFragment_operator_not_equal_0<RetType> {
  fn operator_not_equal_0(self , rsthis: & QTextFragment) -> RetType;
}
impl<'a> /*trait*/ QTextFragment_operator_not_equal_0<bool> for (usize) {
  fn operator_not_equal_0(self , rsthis: & QTextFragment) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QTextFragmentneERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextobject.h:315
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool operator<(const QTextFragment &) const

/*

*/
impl /*struct*/ QTextFragment {
  pub fn operator_less_than_0<RetType, T: QTextFragment_operator_less_than_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_less_than_0(self);
    // return 1;
  }
}
pub trait QTextFragment_operator_less_than_0<RetType> {
  fn operator_less_than_0(self , rsthis: & QTextFragment) -> RetType;
}
impl<'a> /*trait*/ QTextFragment_operator_less_than_0<bool> for (usize) {
  fn operator_less_than_0(self , rsthis: & QTextFragment) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QTextFragmentltERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextobject.h:317
// index:0
// Public Visibility=Default Availability=Available
// [4] int position() const

/*

*/
impl /*struct*/ QTextFragment {
  pub fn position_0<RetType, T: QTextFragment_position_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.position_0(self);
    // return 1;
  }
}
pub trait QTextFragment_position_0<RetType> {
  fn position_0(self , rsthis: & QTextFragment) -> RetType;
}
impl<'a> /*trait*/ QTextFragment_position_0<i32> for () {
  fn position_0(self , rsthis: & QTextFragment) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QTextFragment8positionEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextobject.h:318
// index:0
// Public Visibility=Default Availability=Available
// [4] int length() const

/*

*/
impl /*struct*/ QTextFragment {
  pub fn length_0<RetType, T: QTextFragment_length_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.length_0(self);
    // return 1;
  }
}
pub trait QTextFragment_length_0<RetType> {
  fn length_0(self , rsthis: & QTextFragment) -> RetType;
}
impl<'a> /*trait*/ QTextFragment_length_0<i32> for () {
  fn length_0(self , rsthis: & QTextFragment) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QTextFragment6lengthEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextobject.h:319
// index:0
// Public Visibility=Default Availability=Available
// [1] bool contains(int) const

/*

*/
impl /*struct*/ QTextFragment {
  pub fn contains_0<RetType, T: QTextFragment_contains_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.contains_0(self);
    // return 1;
  }
}
pub trait QTextFragment_contains_0<RetType> {
  fn contains_0(self , rsthis: & QTextFragment) -> RetType;
}
impl<'a> /*trait*/ QTextFragment_contains_0<bool> for (i32) {
  fn contains_0(self , rsthis: & QTextFragment) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QTextFragment8containsEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextobject.h:321
// index:0
// Public Visibility=Default Availability=Available
// [16] QTextCharFormat charFormat() const

/*

*/
impl /*struct*/ QTextFragment {
  pub fn charFormat_0<RetType, T: QTextFragment_charFormat_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.charFormat_0(self);
    // return 1;
  }
}
pub trait QTextFragment_charFormat_0<RetType> {
  fn charFormat_0(self , rsthis: & QTextFragment) -> RetType;
}
impl<'a> /*trait*/ QTextFragment_charFormat_0<usize> for () {
  fn charFormat_0(self , rsthis: & QTextFragment) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QTextFragment10charFormatEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextobject.h:322
// index:0
// Public Visibility=Default Availability=Available
// [4] int charFormatIndex() const

/*

*/
impl /*struct*/ QTextFragment {
  pub fn charFormatIndex_0<RetType, T: QTextFragment_charFormatIndex_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.charFormatIndex_0(self);
    // return 1;
  }
}
pub trait QTextFragment_charFormatIndex_0<RetType> {
  fn charFormatIndex_0(self , rsthis: & QTextFragment) -> RetType;
}
impl<'a> /*trait*/ QTextFragment_charFormatIndex_0<i32> for () {
  fn charFormatIndex_0(self , rsthis: & QTextFragment) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QTextFragment15charFormatIndexEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextobject.h:323
// index:0
// Public Visibility=Default Availability=Available
// [8] QString text() const

/*

*/
impl /*struct*/ QTextFragment {
  pub fn text_0<RetType, T: QTextFragment_text_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.text_0(self);
    // return 1;
  }
}
pub trait QTextFragment_text_0<RetType> {
  fn text_0(self , rsthis: & QTextFragment) -> RetType;
}
impl<'a> /*trait*/ QTextFragment_text_0<usize> for () {
  fn text_0(self , rsthis: & QTextFragment) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QTextFragment4textEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}


pub fn DeleteQTextFragment(this :*mut QTextFragment) {
    // rv, err := qtrt::InvokeQtFunc6("_ZN13QTextFragmentD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis())
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
//  body block end

//  keep block begin

//  keep block end
