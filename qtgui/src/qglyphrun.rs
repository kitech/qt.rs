

// mod ::gui::QGlyphRun
// package qtgui
// /usr/include/qt/QtGui/qglyphrun.h
// #include <qglyphrun.h>
// #include <QtGui>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 39
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
#[derive(Default)] // class sizeof(QGlyphRun)=8
pub struct QGlyphRun {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QGlyphRun_ITF interface {
//    QGlyphRun_PTR() *QGlyphRun
//}
//func (ptr *QGlyphRun) QGlyphRun_PTR() *QGlyphRun { return ptr }

impl /*struct*/ QGlyphRun {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QGlyphRun {
    return QGlyphRun{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QGlyphRun {
//  type Target = QGlyphRunBASE;
//
//  fn deref(&self) -> &QGlyphRunBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QGlyphRunBASE> for QGlyphRun {
//  fn as_ref(& self) -> & QGlyphRunBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtGui/qglyphrun.h:67
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QGlyphRun()

/*
Constructs an empty QGlyphRun object.
*/
// QGlyphRun() ctx.fn_proto_cpp
impl /*struct*/ QGlyphRun {
  pub fn QGlyphRun_0<T: QGlyphRun_QGlyphRun_0>(value: T) -> QGlyphRun {
    let rsthis = value.QGlyphRun_0();
    return rsthis;
    // return 1;
  }
}

pub trait QGlyphRun_QGlyphRun_0 {
  fn QGlyphRun_0(self) -> QGlyphRun;
}
// QGlyphRun() ctx.fn_proto_cpp
impl<'a> /*trait*/ QGlyphRun_QGlyphRun_0 for () {
  fn QGlyphRun_0(self) -> QGlyphRun {
    // unsafe{_ZN9QGlyphRunC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN9QGlyphRunC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QGlyphRun{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qglyphrun.h:70
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QGlyphRun & operator=(QGlyphRun &&)

/*

*/
impl /*struct*/ QGlyphRun {
  pub fn operator_equal_0<RetType, T: QGlyphRun_operator_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_0(self);
    // return 1;
  }
}
pub trait QGlyphRun_operator_equal_0<RetType> {
  fn operator_equal_0(self , rsthis: & QGlyphRun) -> RetType;
}
impl<'a> /*trait*/ QGlyphRun_operator_equal_0<usize> for (usize) {
  fn operator_equal_0(self , rsthis: & QGlyphRun) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QGlyphRunaSEOS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qglyphrun.h:72
// index:1
// Public Visibility=Default Availability=Available
// [8] QGlyphRun & operator=(const QGlyphRun &)

/*

*/
impl /*struct*/ QGlyphRun {
  pub fn operator_equal_1<RetType, T: QGlyphRun_operator_equal_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_1(self);
    // return 1;
  }
}
pub trait QGlyphRun_operator_equal_1<RetType> {
  fn operator_equal_1(self , rsthis: & QGlyphRun) -> RetType;
}
impl<'a> /*trait*/ QGlyphRun_operator_equal_1<usize> for (usize) {
  fn operator_equal_1(self , rsthis: & QGlyphRun) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QGlyphRunaSERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qglyphrun.h:73
// index:0
// Public Visibility=Default Availability=Available
// [-2] void ~QGlyphRun()

/*

*/
pub fn DeleteQGlyphRun(this :*mut QGlyphRun) {
    // let rv = qtrt::InvokeQtFunc6("_ZN9QGlyphRunD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 8)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtGui/qglyphrun.h:75
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void swap(QGlyphRun &)

/*
Swaps this glyph run instance with other. This function is very fast and never fails.

This function was introduced in  Qt 5.0.
*/
impl /*struct*/ QGlyphRun {
  pub fn swap_0<RetType, T: QGlyphRun_swap_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.swap_0(self);
    // return 1;
  }
}
pub trait QGlyphRun_swap_0<RetType> {
  fn swap_0(self , rsthis: & QGlyphRun) -> RetType;
}
impl<'a> /*trait*/ QGlyphRun_swap_0<(/*void*/)> for (usize) {
  fn swap_0(self , rsthis: & QGlyphRun) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QGlyphRun4swapERS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qglyphrun.h:77
// index:0
// Public Visibility=Default Availability=Available
// [8] QRawFont rawFont() const

/*
Returns the font selected for this QGlyphRun object.

See also setRawFont().
*/
impl /*struct*/ QGlyphRun {
  pub fn rawFont_0<RetType, T: QGlyphRun_rawFont_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.rawFont_0(self);
    // return 1;
  }
}
pub trait QGlyphRun_rawFont_0<RetType> {
  fn rawFont_0(self , rsthis: & QGlyphRun) -> RetType;
}
impl<'a> /*trait*/ QGlyphRun_rawFont_0<usize> for () {
  fn rawFont_0(self , rsthis: & QGlyphRun) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QGlyphRun7rawFontEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qglyphrun.h:78
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setRawFont(const QRawFont &)

/*
Sets the font in which to look up the glyph indexes to the rawFont specified.

See also rawFont() and setGlyphIndexes().
*/
impl /*struct*/ QGlyphRun {
  pub fn setRawFont_0<RetType, T: QGlyphRun_setRawFont_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setRawFont_0(self);
    // return 1;
  }
}
pub trait QGlyphRun_setRawFont_0<RetType> {
  fn setRawFont_0(self , rsthis: & QGlyphRun) -> RetType;
}
impl<'a> /*trait*/ QGlyphRun_setRawFont_0<(/*void*/)> for (usize) {
  fn setRawFont_0(self , rsthis: & QGlyphRun) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QGlyphRun10setRawFontERK8QRawFont", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qglyphrun.h:80
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setRawData(const quint32 *, const QPointF *, int)

/*
Sets the glyph indexes and positions of this QGlyphRun to use the first size elements in the arrays glyphIndexArray and glyphPositionArray. The data is not copied. The caller must guarantee that the arrays are not deleted as long as this QGlyphRun and any copies of it exists.

See also setGlyphIndexes() and setPositions().
*/
impl /*struct*/ QGlyphRun {
  pub fn setRawData_0<RetType, T: QGlyphRun_setRawData_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setRawData_0(self);
    // return 1;
  }
}
pub trait QGlyphRun_setRawData_0<RetType> {
  fn setRawData_0(self , rsthis: & QGlyphRun) -> RetType;
}
impl<'a> /*trait*/ QGlyphRun_setRawData_0<(/*void*/)> for (usize,usize,i32) {
  fn setRawData_0(self , rsthis: & QGlyphRun) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN9QGlyphRun10setRawDataEPKjPK7QPointFi", 3,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qglyphrun.h:90
// index:0
// Public Visibility=Default Availability=Available
// [-2] void clear()

/*
Clears all data in the QGlyphRun object.
*/
impl /*struct*/ QGlyphRun {
  pub fn clear_0<RetType, T: QGlyphRun_clear_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.clear_0(self);
    // return 1;
  }
}
pub trait QGlyphRun_clear_0<RetType> {
  fn clear_0(self , rsthis: & QGlyphRun) -> RetType;
}
impl<'a> /*trait*/ QGlyphRun_clear_0<(/*void*/)> for () {
  fn clear_0(self , rsthis: & QGlyphRun) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN9QGlyphRun5clearEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qglyphrun.h:92
// index:0
// Public Visibility=Default Availability=Available
// [1] bool operator==(const QGlyphRun &) const

/*

*/
impl /*struct*/ QGlyphRun {
  pub fn operator_equal_equal_0<RetType, T: QGlyphRun_operator_equal_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_equal_0(self);
    // return 1;
  }
}
pub trait QGlyphRun_operator_equal_equal_0<RetType> {
  fn operator_equal_equal_0(self , rsthis: & QGlyphRun) -> RetType;
}
impl<'a> /*trait*/ QGlyphRun_operator_equal_equal_0<bool> for (usize) {
  fn operator_equal_equal_0(self , rsthis: & QGlyphRun) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QGlyphRuneqERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qglyphrun.h:93
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool operator!=(const QGlyphRun &) const

/*

*/
impl /*struct*/ QGlyphRun {
  pub fn operator_not_equal_0<RetType, T: QGlyphRun_operator_not_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_not_equal_0(self);
    // return 1;
  }
}
pub trait QGlyphRun_operator_not_equal_0<RetType> {
  fn operator_not_equal_0(self , rsthis: & QGlyphRun) -> RetType;
}
impl<'a> /*trait*/ QGlyphRun_operator_not_equal_0<bool> for (usize) {
  fn operator_not_equal_0(self , rsthis: & QGlyphRun) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QGlyphRunneERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qglyphrun.h:96
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setOverline(bool)

/*
Indicates that this QGlyphRun should be painted with an overline decoration if overline is true. Otherwise the QGlyphRun should be painted with no overline decoration.

See also overline(), setFlag(), and setFlags().
*/
impl /*struct*/ QGlyphRun {
  pub fn setOverline_0<RetType, T: QGlyphRun_setOverline_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setOverline_0(self);
    // return 1;
  }
}
pub trait QGlyphRun_setOverline_0<RetType> {
  fn setOverline_0(self , rsthis: & QGlyphRun) -> RetType;
}
impl<'a> /*trait*/ QGlyphRun_setOverline_0<(/*void*/)> for (bool) {
  fn setOverline_0(self , rsthis: & QGlyphRun) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN9QGlyphRun11setOverlineEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qglyphrun.h:97
// index:0
// Public Visibility=Default Availability=Available
// [1] bool overline() const

/*
Returns true if this QGlyphRun should be painted with an overline decoration.

See also setOverline() and flags().
*/
impl /*struct*/ QGlyphRun {
  pub fn overline_0<RetType, T: QGlyphRun_overline_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.overline_0(self);
    // return 1;
  }
}
pub trait QGlyphRun_overline_0<RetType> {
  fn overline_0(self , rsthis: & QGlyphRun) -> RetType;
}
impl<'a> /*trait*/ QGlyphRun_overline_0<bool> for () {
  fn overline_0(self , rsthis: & QGlyphRun) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QGlyphRun8overlineEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qglyphrun.h:99
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setUnderline(bool)

/*
Indicates that this QGlyphRun should be painted with an underline decoration if underline is true. Otherwise the QGlyphRun should be painted with no underline decoration.

See also underline(), setFlag(), and setFlags().
*/
impl /*struct*/ QGlyphRun {
  pub fn setUnderline_0<RetType, T: QGlyphRun_setUnderline_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setUnderline_0(self);
    // return 1;
  }
}
pub trait QGlyphRun_setUnderline_0<RetType> {
  fn setUnderline_0(self , rsthis: & QGlyphRun) -> RetType;
}
impl<'a> /*trait*/ QGlyphRun_setUnderline_0<(/*void*/)> for (bool) {
  fn setUnderline_0(self , rsthis: & QGlyphRun) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN9QGlyphRun12setUnderlineEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qglyphrun.h:100
// index:0
// Public Visibility=Default Availability=Available
// [1] bool underline() const

/*
Returns true if this QGlyphRun should be painted with an underline decoration.

See also setUnderline() and flags().
*/
impl /*struct*/ QGlyphRun {
  pub fn underline_0<RetType, T: QGlyphRun_underline_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.underline_0(self);
    // return 1;
  }
}
pub trait QGlyphRun_underline_0<RetType> {
  fn underline_0(self , rsthis: & QGlyphRun) -> RetType;
}
impl<'a> /*trait*/ QGlyphRun_underline_0<bool> for () {
  fn underline_0(self , rsthis: & QGlyphRun) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QGlyphRun9underlineEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qglyphrun.h:102
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setStrikeOut(bool)

/*
Indicates that this QGlyphRun should be painted with an strike out decoration if strikeOut is true. Otherwise the QGlyphRun should be painted with no strike out decoration.

See also strikeOut(), setFlag(), and setFlags().
*/
impl /*struct*/ QGlyphRun {
  pub fn setStrikeOut_0<RetType, T: QGlyphRun_setStrikeOut_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setStrikeOut_0(self);
    // return 1;
  }
}
pub trait QGlyphRun_setStrikeOut_0<RetType> {
  fn setStrikeOut_0(self , rsthis: & QGlyphRun) -> RetType;
}
impl<'a> /*trait*/ QGlyphRun_setStrikeOut_0<(/*void*/)> for (bool) {
  fn setStrikeOut_0(self , rsthis: & QGlyphRun) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN9QGlyphRun12setStrikeOutEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qglyphrun.h:103
// index:0
// Public Visibility=Default Availability=Available
// [1] bool strikeOut() const

/*
Returns true if this QGlyphRun should be painted with a strike out decoration.

See also setStrikeOut() and flags().
*/
impl /*struct*/ QGlyphRun {
  pub fn strikeOut_0<RetType, T: QGlyphRun_strikeOut_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.strikeOut_0(self);
    // return 1;
  }
}
pub trait QGlyphRun_strikeOut_0<RetType> {
  fn strikeOut_0(self , rsthis: & QGlyphRun) -> RetType;
}
impl<'a> /*trait*/ QGlyphRun_strikeOut_0<bool> for () {
  fn strikeOut_0(self , rsthis: & QGlyphRun) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QGlyphRun9strikeOutEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qglyphrun.h:105
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setRightToLeft(bool)

/*
Indicates that this QGlyphRun contains glyphs that should be ordered from the right to left if rightToLeft is true. Otherwise the order of the glyphs is assumed to be left to right.

This function was introduced in  Qt 5.0.

See also isRightToLeft(), setFlag(), and setFlags().
*/
impl /*struct*/ QGlyphRun {
  pub fn setRightToLeft_0<RetType, T: QGlyphRun_setRightToLeft_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setRightToLeft_0(self);
    // return 1;
  }
}
pub trait QGlyphRun_setRightToLeft_0<RetType> {
  fn setRightToLeft_0(self , rsthis: & QGlyphRun) -> RetType;
}
impl<'a> /*trait*/ QGlyphRun_setRightToLeft_0<(/*void*/)> for (bool) {
  fn setRightToLeft_0(self , rsthis: & QGlyphRun) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN9QGlyphRun14setRightToLeftEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qglyphrun.h:106
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isRightToLeft() const

/*
Returns true if this QGlyphRun contains glyphs that are painted from the right to the left.

This function was introduced in  Qt 5.0.

See also setRightToLeft() and flags().
*/
impl /*struct*/ QGlyphRun {
  pub fn isRightToLeft_0<RetType, T: QGlyphRun_isRightToLeft_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isRightToLeft_0(self);
    // return 1;
  }
}
pub trait QGlyphRun_isRightToLeft_0<RetType> {
  fn isRightToLeft_0(self , rsthis: & QGlyphRun) -> RetType;
}
impl<'a> /*trait*/ QGlyphRun_isRightToLeft_0<bool> for () {
  fn isRightToLeft_0(self , rsthis: & QGlyphRun) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QGlyphRun13isRightToLeftEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qglyphrun.h:108
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setFlag(QGlyphRun::GlyphRunFlag, bool)

/*
If enabled is true, then flag is enabled; otherwise, it is disabled.

This function was introduced in  Qt 5.0.

See also flags() and setFlags().
*/
impl /*struct*/ QGlyphRun {
  pub fn setFlag_0<RetType, T: QGlyphRun_setFlag_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setFlag_0(self);
    // return 1;
  }
}
pub trait QGlyphRun_setFlag_0<RetType> {
  fn setFlag_0(self , rsthis: & QGlyphRun) -> RetType;
}
impl<'a> /*trait*/ QGlyphRun_setFlag_0<(/*void*/)> for (i32,bool) {
  fn setFlag_0(self , rsthis: & QGlyphRun) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN9QGlyphRun7setFlagENS_12GlyphRunFlagEb", 2,qtrt::FFITY_INT,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qglyphrun.h:109
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setFlags(QGlyphRun::GlyphRunFlags)

/*
Sets the flags of this QGlyphRun to flags.

This function was introduced in  Qt 5.0.

See also setFlag() and flags().
*/
impl /*struct*/ QGlyphRun {
  pub fn setFlags_0<RetType, T: QGlyphRun_setFlags_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setFlags_0(self);
    // return 1;
  }
}
pub trait QGlyphRun_setFlags_0<RetType> {
  fn setFlags_0(self , rsthis: & QGlyphRun) -> RetType;
}
impl<'a> /*trait*/ QGlyphRun_setFlags_0<(/*void*/)> for (i32) {
  fn setFlags_0(self , rsthis: & QGlyphRun) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN9QGlyphRun8setFlagsE6QFlagsINS_12GlyphRunFlagEE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qglyphrun.h:110
// index:0
// Public Visibility=Default Availability=Available
// [-2] QGlyphRun::GlyphRunFlags flags() const

/*
Returns the flags set for this QGlyphRun.

This function was introduced in  Qt 5.0.

See also setFlags(), setFlag(), and setFlag().
*/
impl /*struct*/ QGlyphRun {
  pub fn flags_0<RetType, T: QGlyphRun_flags_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.flags_0(self);
    // return 1;
  }
}
pub trait QGlyphRun_flags_0<RetType> {
  fn flags_0(self , rsthis: & QGlyphRun) -> RetType;
}
impl<'a> /*trait*/ QGlyphRun_flags_0<i32> for () {
  fn flags_0(self , rsthis: & QGlyphRun) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QGlyphRun5flagsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qglyphrun.h:112
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setBoundingRect(const QRectF &)

/*
Sets the bounding rect of the glyphs in this QGlyphRun to be boundingRect. This rectangle will be returned by boundingRect() unless it is empty, in which case the bounding rectangle of the glyphs in the glyph run will be returned instead.

Note: Unless you are implementing text shaping, you should not have to use this function. It is used specifically when the QGlyphRun should represent an area which is smaller than the area of the glyphs it contains. This could happen e.g. if the glyph run is retrieved by calling QTextLayout::glyphRuns() and the specified range only includes part of a ligature (where two or more characters are combined to a single glyph.) When this is the case, the bounding rect should only include the appropriate part of the ligature glyph, based on a calculation of the average width of the characters in the ligature.

In order to support such a case (an example is selections which should be drawn with a different color than the main text color), it is necessary to clip the painting mechanism to the rectangle returned from boundingRect() to avoid drawing the entire ligature glyph.

This function was introduced in  Qt 5.0.

See also boundingRect().
*/
impl /*struct*/ QGlyphRun {
  pub fn setBoundingRect_0<RetType, T: QGlyphRun_setBoundingRect_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setBoundingRect_0(self);
    // return 1;
  }
}
pub trait QGlyphRun_setBoundingRect_0<RetType> {
  fn setBoundingRect_0(self , rsthis: & QGlyphRun) -> RetType;
}
impl<'a> /*trait*/ QGlyphRun_setBoundingRect_0<(/*void*/)> for (usize) {
  fn setBoundingRect_0(self , rsthis: & QGlyphRun) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QGlyphRun15setBoundingRectERK6QRectF", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qglyphrun.h:113
// index:0
// Public Visibility=Default Availability=Available
// [32] QRectF boundingRect() const

/*
Returns the smallest rectangle that contains all glyphs in this QGlyphRun. If a bounding rect has been set using setBoundingRect(), then this will be returned. Otherwise the bounding rect will be calculated based on the font metrics of the glyphs in the glyph run.

This function was introduced in  Qt 5.0.

See also setBoundingRect().
*/
impl /*struct*/ QGlyphRun {
  pub fn boundingRect_0<RetType, T: QGlyphRun_boundingRect_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.boundingRect_0(self);
    // return 1;
  }
}
pub trait QGlyphRun_boundingRect_0<RetType> {
  fn boundingRect_0(self , rsthis: & QGlyphRun) -> RetType;
}
impl<'a> /*trait*/ QGlyphRun_boundingRect_0<usize> for () {
  fn boundingRect_0(self , rsthis: & QGlyphRun) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QGlyphRun12boundingRectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qglyphrun.h:115
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isEmpty() const

/*
Returns true if the QGlyphRun does not contain any glyphs.

This function was introduced in  Qt 5.0.
*/
impl /*struct*/ QGlyphRun {
  pub fn isEmpty_0<RetType, T: QGlyphRun_isEmpty_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isEmpty_0(self);
    // return 1;
  }
}
pub trait QGlyphRun_isEmpty_0<RetType> {
  fn isEmpty_0(self , rsthis: & QGlyphRun) -> RetType;
}
impl<'a> /*trait*/ QGlyphRun_isEmpty_0<bool> for () {
  fn isEmpty_0(self , rsthis: & QGlyphRun) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QGlyphRun7isEmptyEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}


/*


*/
pub type QGlyphRun__GlyphRunFlag = i32;
// 
pub const QGlyphRun__Overline :QGlyphRun__GlyphRunFlag = 1;
// 
pub const QGlyphRun__Underline :QGlyphRun__GlyphRunFlag = 2;
// 
pub const QGlyphRun__StrikeOut :QGlyphRun__GlyphRunFlag = 4;
// 
pub const QGlyphRun__RightToLeft :QGlyphRun__GlyphRunFlag = 8;
// 
pub const QGlyphRun__SplitLigature :QGlyphRun__GlyphRunFlag = 16;
pub fn QGlyphRun_GlyphRunFlagItemName(val: i32) ->String {
  match val {
     QGlyphRun__Overline => // 1
     {return String::from("Overline");}
     QGlyphRun__Underline => // 2
     {return String::from("Underline");}
     QGlyphRun__StrikeOut => // 4
     {return String::from("StrikeOut");}
     QGlyphRun__RightToLeft => // 8
     {return String::from("RightToLeft");}
     QGlyphRun__SplitLigature => // 16
     {return String::from("SplitLigature");}
  _ => {return format!("{}", val);}
}
}
pub fn QGlyphRun_GlyphRunFlagItemName_s(val: i32) ->String {
  //var nilthis *QGlyphRun
  //return nilthis.GlyphRunFlagItemName(val);
  return QGlyphRun_GlyphRunFlagItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
