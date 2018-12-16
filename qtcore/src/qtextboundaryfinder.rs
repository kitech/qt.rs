

// mod ::core::QTextBoundaryFinder
// package qtcore
// /usr/include/qt/QtCore/qtextboundaryfinder.h
// #include <qtextboundaryfinder.h>
// #include <QtCore>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 18
*/
// import "C"
// import "unsafe"
// import "reflect"
// import "fmt"
// import "log"
// import "github.com/kitech/qt.go/qtrt"
use std::default::Default;
use std::ops::Deref;
use qtrt; // super::super::qtrt;
use super::*;
//  ext block end

//  body block begin



/*

*/
#[derive(Default)] // class sizeof(QTextBoundaryFinder)=48
pub struct QTextBoundaryFinder {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QTextBoundaryFinder_ITF interface {
//    QTextBoundaryFinder_PTR() *QTextBoundaryFinder
//}
//func (ptr *QTextBoundaryFinder) QTextBoundaryFinder_PTR() *QTextBoundaryFinder { return ptr }

impl /*struct*/ QTextBoundaryFinder {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QTextBoundaryFinder {
    return QTextBoundaryFinder{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QTextBoundaryFinder {
//  type Target = QTextBoundaryFinderBASE;
//
//  fn deref(&self) -> &QTextBoundaryFinderBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QTextBoundaryFinderBASE> for QTextBoundaryFinder {
//  fn as_ref(& self) -> & QTextBoundaryFinderBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qtextboundaryfinder.h:54
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QTextBoundaryFinder()

/*
Constructs an invalid QTextBoundaryFinder object.
*/
// QTextBoundaryFinder() ctx.fn_proto_cpp
impl /*struct*/ QTextBoundaryFinder {
  pub fn QTextBoundaryFinder_0<T: QTextBoundaryFinder_QTextBoundaryFinder_0>(value: T) -> QTextBoundaryFinder {
    let rsthis = value.QTextBoundaryFinder_0();
    return rsthis;
    // return 1;
  }
}

pub trait QTextBoundaryFinder_QTextBoundaryFinder_0 {
  fn QTextBoundaryFinder_0(self) -> QTextBoundaryFinder;
}
// QTextBoundaryFinder() ctx.fn_proto_cpp
impl<'a> /*trait*/ QTextBoundaryFinder_QTextBoundaryFinder_0 for () {
  fn QTextBoundaryFinder_0(self) -> QTextBoundaryFinder {
    // unsafe{_ZN19QTextBoundaryFinderC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN19QTextBoundaryFinderC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QTextBoundaryFinder{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qtextboundaryfinder.h:76
// index:1
// Public Visibility=Default Availability=Available
// [-2] void QTextBoundaryFinder(QTextBoundaryFinder::BoundaryType, const QString &)

/*
Constructs an invalid QTextBoundaryFinder object.
*/
// QTextBoundaryFinder(QTextBoundaryFinder::BoundaryType, const QString &) ctx.fn_proto_cpp
impl /*struct*/ QTextBoundaryFinder {
  pub fn QTextBoundaryFinder_1<T: QTextBoundaryFinder_QTextBoundaryFinder_1>(value: T) -> QTextBoundaryFinder {
    let rsthis = value.QTextBoundaryFinder_1();
    return rsthis;
    // return 1;
  }
}

pub trait QTextBoundaryFinder_QTextBoundaryFinder_1 {
  fn QTextBoundaryFinder_1(self) -> QTextBoundaryFinder;
}
// QTextBoundaryFinder(QTextBoundaryFinder::BoundaryType, const QString &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QTextBoundaryFinder_QTextBoundaryFinder_1 for (i32,usize) {
  fn QTextBoundaryFinder_1(self) -> QTextBoundaryFinder {
    // unsafe{_ZN19QTextBoundaryFinderC2ENS_12BoundaryTypeERK7QString()};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN19QTextBoundaryFinderC2ENS_12BoundaryTypeERK7QString", 2,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QTextBoundaryFinder{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qtextboundaryfinder.h:77
// index:2
// Public Visibility=Default Availability=Available
// [-2] void QTextBoundaryFinder(QTextBoundaryFinder::BoundaryType, const QChar *, int, unsigned char *, int)

/*
Constructs an invalid QTextBoundaryFinder object.
*/
// QTextBoundaryFinder(QTextBoundaryFinder::BoundaryType, const QChar *, int, unsigned char *, int) ctx.fn_proto_cpp
impl /*struct*/ QTextBoundaryFinder {
  pub fn QTextBoundaryFinder_2<T: QTextBoundaryFinder_QTextBoundaryFinder_2>(value: T) -> QTextBoundaryFinder {
    let rsthis = value.QTextBoundaryFinder_2();
    return rsthis;
    // return 1;
  }
}

pub trait QTextBoundaryFinder_QTextBoundaryFinder_2 {
  fn QTextBoundaryFinder_2(self) -> QTextBoundaryFinder;
}
// QTextBoundaryFinder(QTextBoundaryFinder::BoundaryType, const QChar *, int, unsigned char *, int) ctx.fn_proto_cpp
impl<'a> /*trait*/ QTextBoundaryFinder_QTextBoundaryFinder_2 for (i32,usize,i32,usize,i32) {
  fn QTextBoundaryFinder_2(self) -> QTextBoundaryFinder {
    // unsafe{_ZN19QTextBoundaryFinderC2ENS_12BoundaryTypeEPK5QChariPhi()};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let arg3 = (&self.3) as *const usize as usize;
    let arg4 = (&self.4) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN19QTextBoundaryFinderC2ENS_12BoundaryTypeEPK5QChariPhi", 5,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QTextBoundaryFinder{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qtextboundaryfinder.h:56
// index:0
// Public Visibility=Default Availability=Available
// [48] QTextBoundaryFinder & operator=(const QTextBoundaryFinder &)

/*

*/
impl /*struct*/ QTextBoundaryFinder {
  pub fn operator_equal_0<RetType, T: QTextBoundaryFinder_operator_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_0(self);
    // return 1;
  }
}
pub trait QTextBoundaryFinder_operator_equal_0<RetType> {
  fn operator_equal_0(self , rsthis: & QTextBoundaryFinder) -> RetType;
}
impl<'a> /*trait*/ QTextBoundaryFinder_operator_equal_0<usize> for (usize) {
  fn operator_equal_0(self , rsthis: & QTextBoundaryFinder) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN19QTextBoundaryFinderaSERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qtextboundaryfinder.h:57
// index:0
// Public Visibility=Default Availability=Available
// [-2] void ~QTextBoundaryFinder()

/*

*/
pub fn DeleteQTextBoundaryFinder(this :*mut QTextBoundaryFinder) {
    // let rv = qtrt::InvokeQtFunc6("_ZN19QTextBoundaryFinderD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 48)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtCore/qtextboundaryfinder.h:79
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool isValid() const

/*
Returns true if the text boundary finder is valid; otherwise returns false. A default QTextBoundaryFinder is invalid.
*/
impl /*struct*/ QTextBoundaryFinder {
  pub fn isValid_0<RetType, T: QTextBoundaryFinder_isValid_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isValid_0(self);
    // return 1;
  }
}
pub trait QTextBoundaryFinder_isValid_0<RetType> {
  fn isValid_0(self , rsthis: & QTextBoundaryFinder) -> RetType;
}
impl<'a> /*trait*/ QTextBoundaryFinder_isValid_0<bool> for () {
  fn isValid_0(self , rsthis: & QTextBoundaryFinder) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QTextBoundaryFinder7isValidEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qtextboundaryfinder.h:81
// index:0
// Public inline Visibility=Default Availability=Available
// [4] QTextBoundaryFinder::BoundaryType type() const

/*
Returns the type of the QTextBoundaryFinder.
*/
impl /*struct*/ QTextBoundaryFinder {
  pub fn type__0<RetType, T: QTextBoundaryFinder_type__0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.type__0(self);
    // return 1;
  }
}
pub trait QTextBoundaryFinder_type__0<RetType> {
  fn type__0(self , rsthis: & QTextBoundaryFinder) -> RetType;
}
impl<'a> /*trait*/ QTextBoundaryFinder_type__0<i32> for () {
  fn type__0(self , rsthis: & QTextBoundaryFinder) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QTextBoundaryFinder4typeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qtextboundaryfinder.h:82
// index:0
// Public Visibility=Default Availability=Available
// [8] QString string() const

/*
Returns the string the QTextBoundaryFinder object operates on.
*/
impl /*struct*/ QTextBoundaryFinder {
  pub fn string_0<RetType, T: QTextBoundaryFinder_string_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.string_0(self);
    // return 1;
  }
}
pub trait QTextBoundaryFinder_string_0<RetType> {
  fn string_0(self , rsthis: & QTextBoundaryFinder) -> RetType;
}
impl<'a> /*trait*/ QTextBoundaryFinder_string_0<usize> for () {
  fn string_0(self , rsthis: & QTextBoundaryFinder) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QTextBoundaryFinder6stringEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qtextboundaryfinder.h:84
// index:0
// Public Visibility=Default Availability=Available
// [-2] void toStart()

/*
Moves the finder to the start of the string. This is equivalent to setPosition(0).

See also setPosition() and position().
*/
impl /*struct*/ QTextBoundaryFinder {
  pub fn toStart_0<RetType, T: QTextBoundaryFinder_toStart_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toStart_0(self);
    // return 1;
  }
}
pub trait QTextBoundaryFinder_toStart_0<RetType> {
  fn toStart_0(self , rsthis: & QTextBoundaryFinder) -> RetType;
}
impl<'a> /*trait*/ QTextBoundaryFinder_toStart_0<(/*void*/)> for () {
  fn toStart_0(self , rsthis: & QTextBoundaryFinder) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN19QTextBoundaryFinder7toStartEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qtextboundaryfinder.h:85
// index:0
// Public Visibility=Default Availability=Available
// [-2] void toEnd()

/*
Moves the finder to the end of the string. This is equivalent to setPosition(string.length()).

See also setPosition() and position().
*/
impl /*struct*/ QTextBoundaryFinder {
  pub fn toEnd_0<RetType, T: QTextBoundaryFinder_toEnd_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toEnd_0(self);
    // return 1;
  }
}
pub trait QTextBoundaryFinder_toEnd_0<RetType> {
  fn toEnd_0(self , rsthis: & QTextBoundaryFinder) -> RetType;
}
impl<'a> /*trait*/ QTextBoundaryFinder_toEnd_0<(/*void*/)> for () {
  fn toEnd_0(self , rsthis: & QTextBoundaryFinder) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN19QTextBoundaryFinder5toEndEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qtextboundaryfinder.h:86
// index:0
// Public Visibility=Default Availability=Available
// [4] int position() const

/*
Returns the current position of the QTextBoundaryFinder.

The range is from 0 (the beginning of the string) to the length of the string inclusive.

See also setPosition().
*/
impl /*struct*/ QTextBoundaryFinder {
  pub fn position_0<RetType, T: QTextBoundaryFinder_position_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.position_0(self);
    // return 1;
  }
}
pub trait QTextBoundaryFinder_position_0<RetType> {
  fn position_0(self , rsthis: & QTextBoundaryFinder) -> RetType;
}
impl<'a> /*trait*/ QTextBoundaryFinder_position_0<i32> for () {
  fn position_0(self , rsthis: & QTextBoundaryFinder) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QTextBoundaryFinder8positionEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qtextboundaryfinder.h:87
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setPosition(int)

/*
Sets the current position of the QTextBoundaryFinder to position.

If position is out of bounds, it will be bound to only valid positions. In this case, valid positions are from 0 to the length of the string inclusive.

See also position().
*/
impl /*struct*/ QTextBoundaryFinder {
  pub fn setPosition_0<RetType, T: QTextBoundaryFinder_setPosition_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setPosition_0(self);
    // return 1;
  }
}
pub trait QTextBoundaryFinder_setPosition_0<RetType> {
  fn setPosition_0(self , rsthis: & QTextBoundaryFinder) -> RetType;
}
impl<'a> /*trait*/ QTextBoundaryFinder_setPosition_0<(/*void*/)> for (i32) {
  fn setPosition_0(self , rsthis: & QTextBoundaryFinder) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN19QTextBoundaryFinder11setPositionEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qtextboundaryfinder.h:89
// index:0
// Public Visibility=Default Availability=Available
// [4] int toNextBoundary()

/*
Moves the QTextBoundaryFinder to the next boundary position and returns that position.

Returns -1 if there is no next boundary.
*/
impl /*struct*/ QTextBoundaryFinder {
  pub fn toNextBoundary_0<RetType, T: QTextBoundaryFinder_toNextBoundary_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toNextBoundary_0(self);
    // return 1;
  }
}
pub trait QTextBoundaryFinder_toNextBoundary_0<RetType> {
  fn toNextBoundary_0(self , rsthis: & QTextBoundaryFinder) -> RetType;
}
impl<'a> /*trait*/ QTextBoundaryFinder_toNextBoundary_0<i32> for () {
  fn toNextBoundary_0(self , rsthis: & QTextBoundaryFinder) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN19QTextBoundaryFinder14toNextBoundaryEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qtextboundaryfinder.h:90
// index:0
// Public Visibility=Default Availability=Available
// [4] int toPreviousBoundary()

/*
Moves the QTextBoundaryFinder to the previous boundary position and returns that position.

Returns -1 if there is no previous boundary.
*/
impl /*struct*/ QTextBoundaryFinder {
  pub fn toPreviousBoundary_0<RetType, T: QTextBoundaryFinder_toPreviousBoundary_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toPreviousBoundary_0(self);
    // return 1;
  }
}
pub trait QTextBoundaryFinder_toPreviousBoundary_0<RetType> {
  fn toPreviousBoundary_0(self , rsthis: & QTextBoundaryFinder) -> RetType;
}
impl<'a> /*trait*/ QTextBoundaryFinder_toPreviousBoundary_0<i32> for () {
  fn toPreviousBoundary_0(self , rsthis: & QTextBoundaryFinder) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN19QTextBoundaryFinder18toPreviousBoundaryEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qtextboundaryfinder.h:92
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isAtBoundary() const

/*
Returns true if the object's position() is currently at a valid text boundary.
*/
impl /*struct*/ QTextBoundaryFinder {
  pub fn isAtBoundary_0<RetType, T: QTextBoundaryFinder_isAtBoundary_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isAtBoundary_0(self);
    // return 1;
  }
}
pub trait QTextBoundaryFinder_isAtBoundary_0<RetType> {
  fn isAtBoundary_0(self , rsthis: & QTextBoundaryFinder) -> RetType;
}
impl<'a> /*trait*/ QTextBoundaryFinder_isAtBoundary_0<bool> for () {
  fn isAtBoundary_0(self , rsthis: & QTextBoundaryFinder) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QTextBoundaryFinder12isAtBoundaryEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qtextboundaryfinder.h:93
// index:0
// Public Visibility=Default Availability=Available
// [4] QTextBoundaryFinder::BoundaryReasons boundaryReasons() const

/*
Returns the reasons for the boundary finder to have chosen the current position as a boundary.
*/
impl /*struct*/ QTextBoundaryFinder {
  pub fn boundaryReasons_0<RetType, T: QTextBoundaryFinder_boundaryReasons_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.boundaryReasons_0(self);
    // return 1;
  }
}
pub trait QTextBoundaryFinder_boundaryReasons_0<RetType> {
  fn boundaryReasons_0(self , rsthis: & QTextBoundaryFinder) -> RetType;
}
impl<'a> /*trait*/ QTextBoundaryFinder_boundaryReasons_0<i32> for () {
  fn boundaryReasons_0(self , rsthis: & QTextBoundaryFinder) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QTextBoundaryFinder15boundaryReasonsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}


/*

*/
pub type QTextBoundaryFinder__BoundaryType = i32;
// Finds a grapheme which is the smallest boundary. It including letters, punctuation marks, numerals and more.
pub const QTextBoundaryFinder__Grapheme :QTextBoundaryFinder__BoundaryType = 0;
// Finds a word.
pub const QTextBoundaryFinder__Word :QTextBoundaryFinder__BoundaryType = 1;
// Finds sentence boundaries. These include periods, question marks etc.
pub const QTextBoundaryFinder__Sentence :QTextBoundaryFinder__BoundaryType = 2;
// Finds possible positions for breaking the text into multiple lines.
pub const QTextBoundaryFinder__Line :QTextBoundaryFinder__BoundaryType = 3;
pub fn QTextBoundaryFinder_BoundaryTypeItemName(val: i32) ->String {
  match val {
     QTextBoundaryFinder__Grapheme => // 0
     {return String::from("Grapheme");}
     QTextBoundaryFinder__Word => // 1
     {return String::from("Word");}
     QTextBoundaryFinder__Sentence => // 2
     {return String::from("Sentence");}
     QTextBoundaryFinder__Line => // 3
     {return String::from("Line");}
  _ => {return format!("{}", val);}
}
}
pub fn QTextBoundaryFinder_BoundaryTypeItemName_s(val: i32) ->String {
  //var nilthis *QTextBoundaryFinder
  //return nilthis.BoundaryTypeItemName(val);
  return QTextBoundaryFinder_BoundaryTypeItemName(val);
}


/*


*/
pub type QTextBoundaryFinder__BoundaryReason = i32;
// 
pub const QTextBoundaryFinder__NotAtBoundary :QTextBoundaryFinder__BoundaryReason = 0;
// 
pub const QTextBoundaryFinder__BreakOpportunity :QTextBoundaryFinder__BoundaryReason = 31;
// 
pub const QTextBoundaryFinder__StartOfItem :QTextBoundaryFinder__BoundaryReason = 32;
// 
pub const QTextBoundaryFinder__EndOfItem :QTextBoundaryFinder__BoundaryReason = 64;
// 
pub const QTextBoundaryFinder__MandatoryBreak :QTextBoundaryFinder__BoundaryReason = 128;
// 
pub const QTextBoundaryFinder__SoftHyphen :QTextBoundaryFinder__BoundaryReason = 256;
pub fn QTextBoundaryFinder_BoundaryReasonItemName(val: i32) ->String {
  match val {
     QTextBoundaryFinder__NotAtBoundary => // 0
     {return String::from("NotAtBoundary");}
     QTextBoundaryFinder__BreakOpportunity => // 31
     {return String::from("BreakOpportunity");}
     QTextBoundaryFinder__StartOfItem => // 32
     {return String::from("StartOfItem");}
     QTextBoundaryFinder__EndOfItem => // 64
     {return String::from("EndOfItem");}
     QTextBoundaryFinder__MandatoryBreak => // 128
     {return String::from("MandatoryBreak");}
     QTextBoundaryFinder__SoftHyphen => // 256
     {return String::from("SoftHyphen");}
  _ => {return format!("{}", val);}
}
}
pub fn QTextBoundaryFinder_BoundaryReasonItemName_s(val: i32) ->String {
  //var nilthis *QTextBoundaryFinder
  //return nilthis.BoundaryReasonItemName(val);
  return QTextBoundaryFinder_BoundaryReasonItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
