

// mod ::gui::QTextTableFormat
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
// extern C begin: 31
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
#[derive(Default)] // class sizeof(QTextTableFormat)=16
pub struct QTextTableFormat {
  qbase: QTextFrameFormat,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QTextTableFormat_ITF interface {
//    QTextFrameFormat_ITF
//    QTextTableFormat_PTR() *QTextTableFormat
//}
//func (ptr *QTextTableFormat) QTextTableFormat_PTR() *QTextTableFormat { return ptr }

impl /*struct*/ QTextTableFormat {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QTextTableFormat {
    return QTextTableFormat{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QTextTableFormat {
//  type Target = QTextTableFormatBASE;
//
//  fn deref(&self) -> &QTextTableFormatBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QTextTableFormatBASE> for QTextTableFormat {
//  fn as_ref(& self) -> & QTextTableFormatBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtGui/qtextformat.h:887
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QTextTableFormat()

/*

*/
// QTextTableFormat() ctx.fn_proto_cpp
impl /*struct*/ QTextTableFormat {
  pub fn QTextTableFormat_0<T: QTextTableFormat_QTextTableFormat_0>(value: T) -> QTextTableFormat {
    let rsthis = value.QTextTableFormat_0();
    return rsthis;
    // return 1;
  }
}

pub trait QTextTableFormat_QTextTableFormat_0 {
  fn QTextTableFormat_0(self) -> QTextTableFormat;
}
// QTextTableFormat() ctx.fn_proto_cpp
impl<'a> /*trait*/ QTextTableFormat_QTextTableFormat_0 for () {
  fn QTextTableFormat_0(self) -> QTextTableFormat {
    // unsafe{_ZN16QTextTableFormatC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN16QTextTableFormatC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QTextTableFormat{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:923
// index:1
// Protected Visibility=Default Availability=Available
// [-2] void QTextTableFormat(const QTextFormat &)

/*

*/
// QTextTableFormat(const QTextFormat &) ctx.fn_proto_cpp
impl /*struct*/ QTextTableFormat {
  pub fn QTextTableFormat_1<T: QTextTableFormat_QTextTableFormat_1>(value: T) -> QTextTableFormat {
    let rsthis = value.QTextTableFormat_1();
    return rsthis;
    // return 1;
  }
}

pub trait QTextTableFormat_QTextTableFormat_1 {
  fn QTextTableFormat_1(self) -> QTextTableFormat;
}
// QTextTableFormat(const QTextFormat &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QTextTableFormat_QTextTableFormat_1 for (usize) {
  fn QTextTableFormat_1(self) -> QTextTableFormat {
    // unsafe{_ZN16QTextTableFormatC2ERK11QTextFormat()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN16QTextTableFormatC2ERK11QTextFormat", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QTextTableFormat{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:889
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool isValid() const

/*
Returns true if the format is valid (i.e. is not InvalidFormat); otherwise returns false.
*/
impl /*struct*/ QTextTableFormat {
  pub fn isValid_0<RetType, T: QTextTableFormat_isValid_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isValid_0(self);
    // return 1;
  }
}
pub trait QTextTableFormat_isValid_0<RetType> {
  fn isValid_0(self , rsthis: & QTextTableFormat) -> RetType;
}
impl<'a> /*trait*/ QTextTableFormat_isValid_0<bool> for () {
  fn isValid_0(self , rsthis: & QTextTableFormat) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QTextTableFormat7isValidEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:891
// index:0
// Public inline Visibility=Default Availability=Available
// [4] int columns() const

/*

*/
impl /*struct*/ QTextTableFormat {
  pub fn columns_0<RetType, T: QTextTableFormat_columns_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.columns_0(self);
    // return 1;
  }
}
pub trait QTextTableFormat_columns_0<RetType> {
  fn columns_0(self , rsthis: & QTextTableFormat) -> RetType;
}
impl<'a> /*trait*/ QTextTableFormat_columns_0<i32> for () {
  fn columns_0(self , rsthis: & QTextTableFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QTextTableFormat7columnsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:893
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setColumns(int)

/*

*/
impl /*struct*/ QTextTableFormat {
  pub fn setColumns_0<RetType, T: QTextTableFormat_setColumns_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setColumns_0(self);
    // return 1;
  }
}
pub trait QTextTableFormat_setColumns_0<RetType> {
  fn setColumns_0(self , rsthis: & QTextTableFormat) -> RetType;
}
impl<'a> /*trait*/ QTextTableFormat_setColumns_0<(/*void*/)> for (i32) {
  fn setColumns_0(self , rsthis: & QTextTableFormat) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN16QTextTableFormat10setColumnsEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:901
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void clearColumnWidthConstraints()

/*

*/
impl /*struct*/ QTextTableFormat {
  pub fn clearColumnWidthConstraints_0<RetType, T: QTextTableFormat_clearColumnWidthConstraints_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.clearColumnWidthConstraints_0(self);
    // return 1;
  }
}
pub trait QTextTableFormat_clearColumnWidthConstraints_0<RetType> {
  fn clearColumnWidthConstraints_0(self , rsthis: & QTextTableFormat) -> RetType;
}
impl<'a> /*trait*/ QTextTableFormat_clearColumnWidthConstraints_0<(/*void*/)> for () {
  fn clearColumnWidthConstraints_0(self , rsthis: & QTextTableFormat) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN16QTextTableFormat27clearColumnWidthConstraintsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:904
// index:0
// Public inline Visibility=Default Availability=Available
// [8] qreal cellSpacing() const

/*

*/
impl /*struct*/ QTextTableFormat {
  pub fn cellSpacing_0<RetType, T: QTextTableFormat_cellSpacing_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.cellSpacing_0(self);
    // return 1;
  }
}
pub trait QTextTableFormat_cellSpacing_0<RetType> {
  fn cellSpacing_0(self , rsthis: & QTextTableFormat) -> RetType;
}
impl<'a> /*trait*/ QTextTableFormat_cellSpacing_0<f64> for () {
  fn cellSpacing_0(self , rsthis: & QTextTableFormat) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QTextTableFormat11cellSpacingEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:906
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setCellSpacing(qreal)

/*

*/
impl /*struct*/ QTextTableFormat {
  pub fn setCellSpacing_0<RetType, T: QTextTableFormat_setCellSpacing_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setCellSpacing_0(self);
    // return 1;
  }
}
pub trait QTextTableFormat_setCellSpacing_0<RetType> {
  fn setCellSpacing_0(self , rsthis: & QTextTableFormat) -> RetType;
}
impl<'a> /*trait*/ QTextTableFormat_setCellSpacing_0<(/*void*/)> for (f64) {
  fn setCellSpacing_0(self , rsthis: & QTextTableFormat) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN16QTextTableFormat14setCellSpacingEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:909
// index:0
// Public inline Visibility=Default Availability=Available
// [8] qreal cellPadding() const

/*

*/
impl /*struct*/ QTextTableFormat {
  pub fn cellPadding_0<RetType, T: QTextTableFormat_cellPadding_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.cellPadding_0(self);
    // return 1;
  }
}
pub trait QTextTableFormat_cellPadding_0<RetType> {
  fn cellPadding_0(self , rsthis: & QTextTableFormat) -> RetType;
}
impl<'a> /*trait*/ QTextTableFormat_cellPadding_0<f64> for () {
  fn cellPadding_0(self , rsthis: & QTextTableFormat) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QTextTableFormat11cellPaddingEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:911
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setCellPadding(qreal)

/*

*/
impl /*struct*/ QTextTableFormat {
  pub fn setCellPadding_0<RetType, T: QTextTableFormat_setCellPadding_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setCellPadding_0(self);
    // return 1;
  }
}
pub trait QTextTableFormat_setCellPadding_0<RetType> {
  fn setCellPadding_0(self , rsthis: & QTextTableFormat) -> RetType;
}
impl<'a> /*trait*/ QTextTableFormat_setCellPadding_0<(/*void*/)> for (f64) {
  fn setCellPadding_0(self , rsthis: & QTextTableFormat) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN16QTextTableFormat14setCellPaddingEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:913
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setAlignment(Qt::Alignment)

/*

*/
impl /*struct*/ QTextTableFormat {
  pub fn setAlignment_0<RetType, T: QTextTableFormat_setAlignment_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setAlignment_0(self);
    // return 1;
  }
}
pub trait QTextTableFormat_setAlignment_0<RetType> {
  fn setAlignment_0(self , rsthis: & QTextTableFormat) -> RetType;
}
impl<'a> /*trait*/ QTextTableFormat_setAlignment_0<(/*void*/)> for (i32) {
  fn setAlignment_0(self , rsthis: & QTextTableFormat) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN16QTextTableFormat12setAlignmentE6QFlagsIN2Qt13AlignmentFlagEE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:914
// index:0
// Public inline Visibility=Default Availability=Available
// [4] Qt::Alignment alignment() const

/*

*/
impl /*struct*/ QTextTableFormat {
  pub fn alignment_0<RetType, T: QTextTableFormat_alignment_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.alignment_0(self);
    // return 1;
  }
}
pub trait QTextTableFormat_alignment_0<RetType> {
  fn alignment_0(self , rsthis: & QTextTableFormat) -> RetType;
}
impl<'a> /*trait*/ QTextTableFormat_alignment_0<i32> for () {
  fn alignment_0(self , rsthis: & QTextTableFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QTextTableFormat9alignmentEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:917
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setHeaderRowCount(int)

/*

*/
impl /*struct*/ QTextTableFormat {
  pub fn setHeaderRowCount_0<RetType, T: QTextTableFormat_setHeaderRowCount_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setHeaderRowCount_0(self);
    // return 1;
  }
}
pub trait QTextTableFormat_setHeaderRowCount_0<RetType> {
  fn setHeaderRowCount_0(self , rsthis: & QTextTableFormat) -> RetType;
}
impl<'a> /*trait*/ QTextTableFormat_setHeaderRowCount_0<(/*void*/)> for (i32) {
  fn setHeaderRowCount_0(self , rsthis: & QTextTableFormat) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN16QTextTableFormat17setHeaderRowCountEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:919
// index:0
// Public inline Visibility=Default Availability=Available
// [4] int headerRowCount() const

/*

*/
impl /*struct*/ QTextTableFormat {
  pub fn headerRowCount_0<RetType, T: QTextTableFormat_headerRowCount_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.headerRowCount_0(self);
    // return 1;
  }
}
pub trait QTextTableFormat_headerRowCount_0<RetType> {
  fn headerRowCount_0(self , rsthis: & QTextTableFormat) -> RetType;
}
impl<'a> /*trait*/ QTextTableFormat_headerRowCount_0<i32> for () {
  fn headerRowCount_0(self , rsthis: & QTextTableFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QTextTableFormat14headerRowCountEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}


pub fn DeleteQTextTableFormat(this :*mut QTextTableFormat) {
    // rv, err := qtrt::InvokeQtFunc6("_ZN16QTextTableFormatD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis())
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
//  body block end

//  keep block begin

//  keep block end
