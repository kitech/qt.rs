

// mod ::gui::QKeySequence
// package qtgui
// /usr/include/qt/QtGui/qkeysequence.h
// #include <qkeysequence.h>
// #include <QtGui>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 48
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
#[derive(Default)] // class sizeof(QKeySequence)=8
pub struct QKeySequence {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QKeySequence_ITF interface {
//    QKeySequence_PTR() *QKeySequence
//}
//func (ptr *QKeySequence) QKeySequence_PTR() *QKeySequence { return ptr }

impl /*struct*/ QKeySequence {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QKeySequence {
    return QKeySequence{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QKeySequence {
//  type Target = QKeySequenceBASE;
//
//  fn deref(&self) -> &QKeySequenceBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QKeySequenceBASE> for QKeySequence {
//  fn as_ref(& self) -> & QKeySequenceBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtGui/qkeysequence.h:156
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QKeySequence()

/*
Constructs an empty key sequence.
*/
// QKeySequence() ctx.fn_proto_cpp
impl /*struct*/ QKeySequence {
  pub fn QKeySequence_0<T: QKeySequence_QKeySequence_0>(value: T) -> QKeySequence {
    let rsthis = value.QKeySequence_0();
    return rsthis;
    // return 1;
  }
}

pub trait QKeySequence_QKeySequence_0 {
  fn QKeySequence_0(self) -> QKeySequence;
}
// QKeySequence() ctx.fn_proto_cpp
impl<'a> /*trait*/ QKeySequence_QKeySequence_0 for () {
  fn QKeySequence_0(self) -> QKeySequence {
    // unsafe{_ZN12QKeySequenceC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN12QKeySequenceC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QKeySequence{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qkeysequence.h:157
// index:1
// Public Visibility=Default Availability=Available
// [-2] void QKeySequence(const QString &, QKeySequence::SequenceFormat)

/*
Constructs an empty key sequence.
*/
// QKeySequence(const QString &, QKeySequence::SequenceFormat) ctx.fn_proto_cpp
impl /*struct*/ QKeySequence {
  pub fn QKeySequence_1<T: QKeySequence_QKeySequence_1>(value: T) -> QKeySequence {
    let rsthis = value.QKeySequence_1();
    return rsthis;
    // return 1;
  }
}

pub trait QKeySequence_QKeySequence_1 {
  fn QKeySequence_1(self) -> QKeySequence;
}
// QKeySequence(const QString &, QKeySequence::SequenceFormat) ctx.fn_proto_cpp
impl<'a> /*trait*/ QKeySequence_QKeySequence_1 for (usize,i32) {
  fn QKeySequence_1(self) -> QKeySequence {
    // unsafe{_ZN12QKeySequenceC2ERK7QStringNS_14SequenceFormatE()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN12QKeySequenceC2ERK7QStringNS_14SequenceFormatE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QKeySequence{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qkeysequence.h:158
// index:2
// Public Visibility=Default Availability=Available
// [-2] void QKeySequence(int, int, int, int)

/*
Constructs an empty key sequence.
*/
// QKeySequence(int, int, int, int) ctx.fn_proto_cpp
impl /*struct*/ QKeySequence {
  pub fn QKeySequence_2<T: QKeySequence_QKeySequence_2>(value: T) -> QKeySequence {
    let rsthis = value.QKeySequence_2();
    return rsthis;
    // return 1;
  }
}

pub trait QKeySequence_QKeySequence_2 {
  fn QKeySequence_2(self) -> QKeySequence;
}
// QKeySequence(int, int, int, int) ctx.fn_proto_cpp
impl<'a> /*trait*/ QKeySequence_QKeySequence_2 for (i32,i32,i32,i32) {
  fn QKeySequence_2(self) -> QKeySequence {
    // unsafe{_ZN12QKeySequenceC2Eiiii()};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let arg3 = (&self.3) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN12QKeySequenceC2Eiiii", 4,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QKeySequence{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qkeysequence.h:160
// index:3
// Public Visibility=Default Availability=Available
// [-2] void QKeySequence(QKeySequence::StandardKey)

/*
Constructs an empty key sequence.
*/
// QKeySequence(QKeySequence::StandardKey) ctx.fn_proto_cpp
impl /*struct*/ QKeySequence {
  pub fn QKeySequence_3<T: QKeySequence_QKeySequence_3>(value: T) -> QKeySequence {
    let rsthis = value.QKeySequence_3();
    return rsthis;
    // return 1;
  }
}

pub trait QKeySequence_QKeySequence_3 {
  fn QKeySequence_3(self) -> QKeySequence;
}
// QKeySequence(QKeySequence::StandardKey) ctx.fn_proto_cpp
impl<'a> /*trait*/ QKeySequence_QKeySequence_3 for (i32) {
  fn QKeySequence_3(self) -> QKeySequence {
    // unsafe{_ZN12QKeySequenceC2ENS_11StandardKeyE()};
    let arg0 = (&self) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN12QKeySequenceC2ENS_11StandardKeyE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QKeySequence{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qkeysequence.h:161
// index:0
// Public Visibility=Default Availability=Available
// [-2] void ~QKeySequence()

/*

*/
pub fn DeleteQKeySequence(this :*mut QKeySequence) {
    // let rv = qtrt::InvokeQtFunc6("_ZN12QKeySequenceD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 8)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtGui/qkeysequence.h:163
// index:0
// Public Visibility=Default Availability=Available
// [4] int count() const

/*
Returns the number of keys in the key sequence. The maximum is 4.
*/
impl /*struct*/ QKeySequence {
  pub fn count_0<RetType, T: QKeySequence_count_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.count_0(self);
    // return 1;
  }
}
pub trait QKeySequence_count_0<RetType> {
  fn count_0(self , rsthis: & QKeySequence) -> RetType;
}
impl<'a> /*trait*/ QKeySequence_count_0<i32> for () {
  fn count_0(self , rsthis: & QKeySequence) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QKeySequence5countEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qkeysequence.h:164
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isEmpty() const

/*
Returns true if the key sequence is empty; otherwise returns false.
*/
impl /*struct*/ QKeySequence {
  pub fn isEmpty_0<RetType, T: QKeySequence_isEmpty_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isEmpty_0(self);
    // return 1;
  }
}
pub trait QKeySequence_isEmpty_0<RetType> {
  fn isEmpty_0(self , rsthis: & QKeySequence) -> RetType;
}
impl<'a> /*trait*/ QKeySequence_isEmpty_0<bool> for () {
  fn isEmpty_0(self , rsthis: & QKeySequence) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QKeySequence7isEmptyEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qkeysequence.h:172
// index:0
// Public Visibility=Default Availability=Available
// [8] QString toString(QKeySequence::SequenceFormat) const

/*
Return a string representation of the key sequence, based on format.

For example, the value Qt::CTRL+Qt::Key_O results in "Ctrl+O". If the key sequence has multiple key codes, each is separated by commas in the string returned, such as "Alt+X, Ctrl+Y, Z". The strings, "Ctrl", "Shift", etc. are translated using QObject::tr() in the "QShortcut" context.

If the key sequence has no keys, an empty string is returned.

On macOS, the string returned resembles the sequence that is shown in the menu bar if format is QKeySequence::NativeText; otherwise, the string uses the "portable" format, suitable for writing to a file.

This function was introduced in  Qt 4.1.

See also fromString().
*/
impl /*struct*/ QKeySequence {
  pub fn toString_0<RetType, T: QKeySequence_toString_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toString_0(self);
    // return 1;
  }
}
pub trait QKeySequence_toString_0<RetType> {
  fn toString_0(self , rsthis: & QKeySequence) -> RetType;
}
impl<'a> /*trait*/ QKeySequence_toString_0<usize> for (i32) {
  fn toString_0(self , rsthis: & QKeySequence) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QKeySequence8toStringENS_14SequenceFormatE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qkeysequence.h:173
// index:0
// Public static Visibility=Default Availability=Available
// [8] QKeySequence fromString(const QString &, QKeySequence::SequenceFormat)

/*
Return a QKeySequence from the string str based on format.

This function was introduced in  Qt 4.1.

See also toString().
*/
impl /*struct*/ QKeySequence {
  pub fn fromString_0<RetType, T: QKeySequence_fromString_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.fromString_0();
    // return 1;
  }
}
pub trait QKeySequence_fromString_0<RetType> {
  fn fromString_0(self ) -> RetType;
}
impl<'a> /*trait*/ QKeySequence_fromString_0<usize> for (usize,i32) {
  fn fromString_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN12QKeySequence10fromStringERK7QStringNS_14SequenceFormatE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qkeysequence.h:178
// index:0
// Public Visibility=Default Availability=Available
// [4] QKeySequence::SequenceMatch matches(const QKeySequence &) const

/*
Matches the sequence with seq. Returns ExactMatch if successful, PartialMatch if seq matches incompletely, and NoMatch if the sequences have nothing in common. Returns NoMatch if seq is shorter.
*/
impl /*struct*/ QKeySequence {
  pub fn matches_0<RetType, T: QKeySequence_matches_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.matches_0(self);
    // return 1;
  }
}
pub trait QKeySequence_matches_0<RetType> {
  fn matches_0(self , rsthis: & QKeySequence) -> RetType;
}
impl<'a> /*trait*/ QKeySequence_matches_0<i32> for (usize) {
  fn matches_0(self , rsthis: & QKeySequence) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QKeySequence7matchesERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qkeysequence.h:179
// index:0
// Public static Visibility=Default Availability=Available
// [8] QKeySequence mnemonic(const QString &)

/*
Returns the shortcut key sequence for the mnemonic in text, or an empty key sequence if no mnemonics are found.

For example, mnemonic("E&xit") returns Qt::ALT+Qt::Key_X, mnemonic("&Quit") returns ALT+Key_Q, and mnemonic("Quit") returns an empty QKeySequence.

We provide a list of common mnemonics in English. At the time of writing, Microsoft and Open Group do not appear to have issued equivalent recommendations for other languages.
*/
impl /*struct*/ QKeySequence {
  pub fn mnemonic_0<RetType, T: QKeySequence_mnemonic_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.mnemonic_0();
    // return 1;
  }
}
pub trait QKeySequence_mnemonic_0<RetType> {
  fn mnemonic_0(self ) -> RetType;
}
impl<'a> /*trait*/ QKeySequence_mnemonic_0<usize> for (usize) {
  fn mnemonic_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN12QKeySequence8mnemonicERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qkeysequence.h:187
// index:0
// Public Visibility=Default Availability=Available
// [4] int operator[](uint) const

/*

*/
impl /*struct*/ QKeySequence {
  pub fn operator_get_index_0<RetType, T: QKeySequence_operator_get_index_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_get_index_0(self);
    // return 1;
  }
}
pub trait QKeySequence_operator_get_index_0<RetType> {
  fn operator_get_index_0(self , rsthis: & QKeySequence) -> RetType;
}
impl<'a> /*trait*/ QKeySequence_operator_get_index_0<i32> for (u32) {
  fn operator_get_index_0(self , rsthis: & QKeySequence) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const u32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QKeySequenceixEj", 1,qtrt::FFITY_UINT32,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qkeysequence.h:188
// index:0
// Public Visibility=Default Availability=Available
// [8] QKeySequence & operator=(const QKeySequence &)

/*

*/
impl /*struct*/ QKeySequence {
  pub fn operator_equal_0<RetType, T: QKeySequence_operator_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_0(self);
    // return 1;
  }
}
pub trait QKeySequence_operator_equal_0<RetType> {
  fn operator_equal_0(self , rsthis: & QKeySequence) -> RetType;
}
impl<'a> /*trait*/ QKeySequence_operator_equal_0<usize> for (usize) {
  fn operator_equal_0(self , rsthis: & QKeySequence) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN12QKeySequenceaSERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qkeysequence.h:190
// index:1
// Public inline Visibility=Default Availability=Available
// [8] QKeySequence & operator=(QKeySequence &&)

/*

*/
impl /*struct*/ QKeySequence {
  pub fn operator_equal_1<RetType, T: QKeySequence_operator_equal_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_1(self);
    // return 1;
  }
}
pub trait QKeySequence_operator_equal_1<RetType> {
  fn operator_equal_1(self , rsthis: & QKeySequence) -> RetType;
}
impl<'a> /*trait*/ QKeySequence_operator_equal_1<usize> for (usize) {
  fn operator_equal_1(self , rsthis: & QKeySequence) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN12QKeySequenceaSEOS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qkeysequence.h:192
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void swap(QKeySequence &)

/*
Swaps key sequence other with this key sequence. This operation is very fast and never fails.

This function was introduced in  Qt 4.8.
*/
impl /*struct*/ QKeySequence {
  pub fn swap_0<RetType, T: QKeySequence_swap_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.swap_0(self);
    // return 1;
  }
}
pub trait QKeySequence_swap_0<RetType> {
  fn swap_0(self , rsthis: & QKeySequence) -> RetType;
}
impl<'a> /*trait*/ QKeySequence_swap_0<(/*void*/)> for (usize) {
  fn swap_0(self , rsthis: & QKeySequence) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN12QKeySequence4swapERS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qkeysequence.h:194
// index:0
// Public Visibility=Default Availability=Available
// [1] bool operator==(const QKeySequence &) const

/*

*/
impl /*struct*/ QKeySequence {
  pub fn operator_equal_equal_0<RetType, T: QKeySequence_operator_equal_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_equal_0(self);
    // return 1;
  }
}
pub trait QKeySequence_operator_equal_equal_0<RetType> {
  fn operator_equal_equal_0(self , rsthis: & QKeySequence) -> RetType;
}
impl<'a> /*trait*/ QKeySequence_operator_equal_equal_0<bool> for (usize) {
  fn operator_equal_equal_0(self , rsthis: & QKeySequence) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QKeySequenceeqERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qkeysequence.h:195
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool operator!=(const QKeySequence &) const

/*

*/
impl /*struct*/ QKeySequence {
  pub fn operator_not_equal_0<RetType, T: QKeySequence_operator_not_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_not_equal_0(self);
    // return 1;
  }
}
pub trait QKeySequence_operator_not_equal_0<RetType> {
  fn operator_not_equal_0(self , rsthis: & QKeySequence) -> RetType;
}
impl<'a> /*trait*/ QKeySequence_operator_not_equal_0<bool> for (usize) {
  fn operator_not_equal_0(self , rsthis: & QKeySequence) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QKeySequenceneERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qkeysequence.h:197
// index:0
// Public Visibility=Default Availability=Available
// [1] bool operator<(const QKeySequence &) const

/*

*/
impl /*struct*/ QKeySequence {
  pub fn operator_less_than_0<RetType, T: QKeySequence_operator_less_than_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_less_than_0(self);
    // return 1;
  }
}
pub trait QKeySequence_operator_less_than_0<RetType> {
  fn operator_less_than_0(self , rsthis: & QKeySequence) -> RetType;
}
impl<'a> /*trait*/ QKeySequence_operator_less_than_0<bool> for (usize) {
  fn operator_less_than_0(self , rsthis: & QKeySequence) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QKeySequenceltERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qkeysequence.h:198
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool operator>(const QKeySequence &) const

/*

*/
impl /*struct*/ QKeySequence {
  pub fn operator_greater_than_0<RetType, T: QKeySequence_operator_greater_than_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_greater_than_0(self);
    // return 1;
  }
}
pub trait QKeySequence_operator_greater_than_0<RetType> {
  fn operator_greater_than_0(self , rsthis: & QKeySequence) -> RetType;
}
impl<'a> /*trait*/ QKeySequence_operator_greater_than_0<bool> for (usize) {
  fn operator_greater_than_0(self , rsthis: & QKeySequence) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QKeySequencegtERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qkeysequence.h:200
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool operator<=(const QKeySequence &) const

/*

*/
impl /*struct*/ QKeySequence {
  pub fn operator_less_than_equal_0<RetType, T: QKeySequence_operator_less_than_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_less_than_equal_0(self);
    // return 1;
  }
}
pub trait QKeySequence_operator_less_than_equal_0<RetType> {
  fn operator_less_than_equal_0(self , rsthis: & QKeySequence) -> RetType;
}
impl<'a> /*trait*/ QKeySequence_operator_less_than_equal_0<bool> for (usize) {
  fn operator_less_than_equal_0(self , rsthis: & QKeySequence) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QKeySequenceleERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qkeysequence.h:202
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool operator>=(const QKeySequence &) const

/*

*/
impl /*struct*/ QKeySequence {
  pub fn operator_greater_than_equal_0<RetType, T: QKeySequence_operator_greater_than_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_greater_than_equal_0(self);
    // return 1;
  }
}
pub trait QKeySequence_operator_greater_than_equal_0<RetType> {
  fn operator_greater_than_equal_0(self , rsthis: & QKeySequence) -> RetType;
}
impl<'a> /*trait*/ QKeySequence_operator_greater_than_equal_0<bool> for (usize) {
  fn operator_greater_than_equal_0(self , rsthis: & QKeySequence) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QKeySequencegeERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qkeysequence.h:205
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isDetached() const

/*

*/
impl /*struct*/ QKeySequence {
  pub fn isDetached_0<RetType, T: QKeySequence_isDetached_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isDetached_0(self);
    // return 1;
  }
}
pub trait QKeySequence_isDetached_0<RetType> {
  fn isDetached_0(self , rsthis: & QKeySequence) -> RetType;
}
impl<'a> /*trait*/ QKeySequence_isDetached_0<bool> for () {
  fn isDetached_0(self , rsthis: & QKeySequence) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QKeySequence10isDetachedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}


/*
This enum represent standard key bindings. They can be used to assign platform dependent keyboard shortcuts to a QAction.

Note that the key bindings are platform dependent. The currently bound shortcuts can be queried using keyBindings().



This enum was introduced or modified in  Qt 4.2.

*/
pub type QKeySequence__StandardKey = i32;
// Unbound key.
pub const QKeySequence__UnknownKey :QKeySequence__StandardKey = 0;
// Open help contents.
pub const QKeySequence__HelpContents :QKeySequence__StandardKey = 1;
// Activate "what's this".
pub const QKeySequence__WhatsThis :QKeySequence__StandardKey = 2;
// Open document.
pub const QKeySequence__Open :QKeySequence__StandardKey = 3;
// Close document/tab.
pub const QKeySequence__Close :QKeySequence__StandardKey = 4;
// Save document.
pub const QKeySequence__Save :QKeySequence__StandardKey = 5;
// Create new document.
pub const QKeySequence__New :QKeySequence__StandardKey = 6;
// Delete.
pub const QKeySequence__Delete :QKeySequence__StandardKey = 7;
// Cut.
pub const QKeySequence__Cut :QKeySequence__StandardKey = 8;
// Copy.
pub const QKeySequence__Copy :QKeySequence__StandardKey = 9;
// 
pub const QKeySequence__Paste :QKeySequence__StandardKey = 10;
// 
pub const QKeySequence__Undo :QKeySequence__StandardKey = 11;
// 
pub const QKeySequence__Redo :QKeySequence__StandardKey = 12;
// 
pub const QKeySequence__Back :QKeySequence__StandardKey = 13;
// 
pub const QKeySequence__Forward :QKeySequence__StandardKey = 14;
// 
pub const QKeySequence__Refresh :QKeySequence__StandardKey = 15;
// 
pub const QKeySequence__ZoomIn :QKeySequence__StandardKey = 16;
// 
pub const QKeySequence__ZoomOut :QKeySequence__StandardKey = 17;
// 
pub const QKeySequence__Print :QKeySequence__StandardKey = 18;
// 
pub const QKeySequence__AddTab :QKeySequence__StandardKey = 19;
// 
pub const QKeySequence__NextChild :QKeySequence__StandardKey = 20;
// 
pub const QKeySequence__PreviousChild :QKeySequence__StandardKey = 21;
// 
pub const QKeySequence__Find :QKeySequence__StandardKey = 22;
// 
pub const QKeySequence__FindNext :QKeySequence__StandardKey = 23;
// 
pub const QKeySequence__FindPrevious :QKeySequence__StandardKey = 24;
// 
pub const QKeySequence__Replace :QKeySequence__StandardKey = 25;
// 
pub const QKeySequence__SelectAll :QKeySequence__StandardKey = 26;
// 
pub const QKeySequence__Bold :QKeySequence__StandardKey = 27;
// 
pub const QKeySequence__Italic :QKeySequence__StandardKey = 28;
// 
pub const QKeySequence__Underline :QKeySequence__StandardKey = 29;
// 
pub const QKeySequence__MoveToNextChar :QKeySequence__StandardKey = 30;
// 
pub const QKeySequence__MoveToPreviousChar :QKeySequence__StandardKey = 31;
// 
pub const QKeySequence__MoveToNextWord :QKeySequence__StandardKey = 32;
// 
pub const QKeySequence__MoveToPreviousWord :QKeySequence__StandardKey = 33;
// 
pub const QKeySequence__MoveToNextLine :QKeySequence__StandardKey = 34;
// 
pub const QKeySequence__MoveToPreviousLine :QKeySequence__StandardKey = 35;
// 
pub const QKeySequence__MoveToNextPage :QKeySequence__StandardKey = 36;
// 
pub const QKeySequence__MoveToPreviousPage :QKeySequence__StandardKey = 37;
// 
pub const QKeySequence__MoveToStartOfLine :QKeySequence__StandardKey = 38;
// 
pub const QKeySequence__MoveToEndOfLine :QKeySequence__StandardKey = 39;
// 
pub const QKeySequence__MoveToStartOfBlock :QKeySequence__StandardKey = 40;
// 
pub const QKeySequence__MoveToEndOfBlock :QKeySequence__StandardKey = 41;
// 
pub const QKeySequence__MoveToStartOfDocument :QKeySequence__StandardKey = 42;
// 
pub const QKeySequence__MoveToEndOfDocument :QKeySequence__StandardKey = 43;
// 
pub const QKeySequence__SelectNextChar :QKeySequence__StandardKey = 44;
// 
pub const QKeySequence__SelectPreviousChar :QKeySequence__StandardKey = 45;
// 
pub const QKeySequence__SelectNextWord :QKeySequence__StandardKey = 46;
// 
pub const QKeySequence__SelectPreviousWord :QKeySequence__StandardKey = 47;
// 
pub const QKeySequence__SelectNextLine :QKeySequence__StandardKey = 48;
// 
pub const QKeySequence__SelectPreviousLine :QKeySequence__StandardKey = 49;
// 
pub const QKeySequence__SelectNextPage :QKeySequence__StandardKey = 50;
// 
pub const QKeySequence__SelectPreviousPage :QKeySequence__StandardKey = 51;
// 
pub const QKeySequence__SelectStartOfLine :QKeySequence__StandardKey = 52;
// 
pub const QKeySequence__SelectEndOfLine :QKeySequence__StandardKey = 53;
// 
pub const QKeySequence__SelectStartOfBlock :QKeySequence__StandardKey = 54;
// 
pub const QKeySequence__SelectEndOfBlock :QKeySequence__StandardKey = 55;
// 
pub const QKeySequence__SelectStartOfDocument :QKeySequence__StandardKey = 56;
// 
pub const QKeySequence__SelectEndOfDocument :QKeySequence__StandardKey = 57;
// 
pub const QKeySequence__DeleteStartOfWord :QKeySequence__StandardKey = 58;
// 
pub const QKeySequence__DeleteEndOfWord :QKeySequence__StandardKey = 59;
// 
pub const QKeySequence__DeleteEndOfLine :QKeySequence__StandardKey = 60;
// 
pub const QKeySequence__InsertParagraphSeparator :QKeySequence__StandardKey = 61;
// 
pub const QKeySequence__InsertLineSeparator :QKeySequence__StandardKey = 62;
// 
pub const QKeySequence__SaveAs :QKeySequence__StandardKey = 63;
// 
pub const QKeySequence__Preferences :QKeySequence__StandardKey = 64;
// 
pub const QKeySequence__Quit :QKeySequence__StandardKey = 65;
// 
pub const QKeySequence__FullScreen :QKeySequence__StandardKey = 66;
// 
pub const QKeySequence__Deselect :QKeySequence__StandardKey = 67;
// 
pub const QKeySequence__DeleteCompleteLine :QKeySequence__StandardKey = 68;
// 
pub const QKeySequence__Backspace :QKeySequence__StandardKey = 69;
// 
pub const QKeySequence__Cancel :QKeySequence__StandardKey = 70;
pub fn QKeySequence_StandardKeyItemName(val: i32) ->String {
  match val {
     QKeySequence__UnknownKey => // 0
     {return String::from("UnknownKey");}
     QKeySequence__HelpContents => // 1
     {return String::from("HelpContents");}
     QKeySequence__WhatsThis => // 2
     {return String::from("WhatsThis");}
     QKeySequence__Open => // 3
     {return String::from("Open");}
     QKeySequence__Close => // 4
     {return String::from("Close");}
     QKeySequence__Save => // 5
     {return String::from("Save");}
     QKeySequence__New => // 6
     {return String::from("New");}
     QKeySequence__Delete => // 7
     {return String::from("Delete");}
     QKeySequence__Cut => // 8
     {return String::from("Cut");}
     QKeySequence__Copy => // 9
     {return String::from("Copy");}
     QKeySequence__Paste => // 10
     {return String::from("Paste");}
     QKeySequence__Undo => // 11
     {return String::from("Undo");}
     QKeySequence__Redo => // 12
     {return String::from("Redo");}
     QKeySequence__Back => // 13
     {return String::from("Back");}
     QKeySequence__Forward => // 14
     {return String::from("Forward");}
     QKeySequence__Refresh => // 15
     {return String::from("Refresh");}
     QKeySequence__ZoomIn => // 16
     {return String::from("ZoomIn");}
     QKeySequence__ZoomOut => // 17
     {return String::from("ZoomOut");}
     QKeySequence__Print => // 18
     {return String::from("Print");}
     QKeySequence__AddTab => // 19
     {return String::from("AddTab");}
     QKeySequence__NextChild => // 20
     {return String::from("NextChild");}
     QKeySequence__PreviousChild => // 21
     {return String::from("PreviousChild");}
     QKeySequence__Find => // 22
     {return String::from("Find");}
     QKeySequence__FindNext => // 23
     {return String::from("FindNext");}
     QKeySequence__FindPrevious => // 24
     {return String::from("FindPrevious");}
     QKeySequence__Replace => // 25
     {return String::from("Replace");}
     QKeySequence__SelectAll => // 26
     {return String::from("SelectAll");}
     QKeySequence__Bold => // 27
     {return String::from("Bold");}
     QKeySequence__Italic => // 28
     {return String::from("Italic");}
     QKeySequence__Underline => // 29
     {return String::from("Underline");}
     QKeySequence__MoveToNextChar => // 30
     {return String::from("MoveToNextChar");}
     QKeySequence__MoveToPreviousChar => // 31
     {return String::from("MoveToPreviousChar");}
     QKeySequence__MoveToNextWord => // 32
     {return String::from("MoveToNextWord");}
     QKeySequence__MoveToPreviousWord => // 33
     {return String::from("MoveToPreviousWord");}
     QKeySequence__MoveToNextLine => // 34
     {return String::from("MoveToNextLine");}
     QKeySequence__MoveToPreviousLine => // 35
     {return String::from("MoveToPreviousLine");}
     QKeySequence__MoveToNextPage => // 36
     {return String::from("MoveToNextPage");}
     QKeySequence__MoveToPreviousPage => // 37
     {return String::from("MoveToPreviousPage");}
     QKeySequence__MoveToStartOfLine => // 38
     {return String::from("MoveToStartOfLine");}
     QKeySequence__MoveToEndOfLine => // 39
     {return String::from("MoveToEndOfLine");}
     QKeySequence__MoveToStartOfBlock => // 40
     {return String::from("MoveToStartOfBlock");}
     QKeySequence__MoveToEndOfBlock => // 41
     {return String::from("MoveToEndOfBlock");}
     QKeySequence__MoveToStartOfDocument => // 42
     {return String::from("MoveToStartOfDocument");}
     QKeySequence__MoveToEndOfDocument => // 43
     {return String::from("MoveToEndOfDocument");}
     QKeySequence__SelectNextChar => // 44
     {return String::from("SelectNextChar");}
     QKeySequence__SelectPreviousChar => // 45
     {return String::from("SelectPreviousChar");}
     QKeySequence__SelectNextWord => // 46
     {return String::from("SelectNextWord");}
     QKeySequence__SelectPreviousWord => // 47
     {return String::from("SelectPreviousWord");}
     QKeySequence__SelectNextLine => // 48
     {return String::from("SelectNextLine");}
     QKeySequence__SelectPreviousLine => // 49
     {return String::from("SelectPreviousLine");}
     QKeySequence__SelectNextPage => // 50
     {return String::from("SelectNextPage");}
     QKeySequence__SelectPreviousPage => // 51
     {return String::from("SelectPreviousPage");}
     QKeySequence__SelectStartOfLine => // 52
     {return String::from("SelectStartOfLine");}
     QKeySequence__SelectEndOfLine => // 53
     {return String::from("SelectEndOfLine");}
     QKeySequence__SelectStartOfBlock => // 54
     {return String::from("SelectStartOfBlock");}
     QKeySequence__SelectEndOfBlock => // 55
     {return String::from("SelectEndOfBlock");}
     QKeySequence__SelectStartOfDocument => // 56
     {return String::from("SelectStartOfDocument");}
     QKeySequence__SelectEndOfDocument => // 57
     {return String::from("SelectEndOfDocument");}
     QKeySequence__DeleteStartOfWord => // 58
     {return String::from("DeleteStartOfWord");}
     QKeySequence__DeleteEndOfWord => // 59
     {return String::from("DeleteEndOfWord");}
     QKeySequence__DeleteEndOfLine => // 60
     {return String::from("DeleteEndOfLine");}
     QKeySequence__InsertParagraphSeparator => // 61
     {return String::from("InsertParagraphSeparator");}
     QKeySequence__InsertLineSeparator => // 62
     {return String::from("InsertLineSeparator");}
     QKeySequence__SaveAs => // 63
     {return String::from("SaveAs");}
     QKeySequence__Preferences => // 64
     {return String::from("Preferences");}
     QKeySequence__Quit => // 65
     {return String::from("Quit");}
     QKeySequence__FullScreen => // 66
     {return String::from("FullScreen");}
     QKeySequence__Deselect => // 67
     {return String::from("Deselect");}
     QKeySequence__DeleteCompleteLine => // 68
     {return String::from("DeleteCompleteLine");}
     QKeySequence__Backspace => // 69
     {return String::from("Backspace");}
     QKeySequence__Cancel => // 70
     {return String::from("Cancel");}
  _ => {return format!("{}", val);}
}
}
pub fn QKeySequence_StandardKeyItemName_s(val: i32) ->String {
  //var nilthis *QKeySequence
  //return nilthis.StandardKeyItemName(val);
  return QKeySequence_StandardKeyItemName(val);
}


/*

*/
pub type QKeySequence__SequenceFormat = i32;
// The key sequence as a platform specific string. This means that it will be shown translated and on the Mac it will resemble a key sequence from the menu bar. This enum is best used when you want to display the string to the user.
pub const QKeySequence__NativeText :QKeySequence__SequenceFormat = 0;
// 
pub const QKeySequence__PortableText :QKeySequence__SequenceFormat = 1;
pub fn QKeySequence_SequenceFormatItemName(val: i32) ->String {
  match val {
     QKeySequence__NativeText => // 0
     {return String::from("NativeText");}
     QKeySequence__PortableText => // 1
     {return String::from("PortableText");}
  _ => {return format!("{}", val);}
}
}
pub fn QKeySequence_SequenceFormatItemName_s(val: i32) ->String {
  //var nilthis *QKeySequence
  //return nilthis.SequenceFormatItemName(val);
  return QKeySequence_SequenceFormatItemName(val);
}


/*

*/
pub type QKeySequence__SequenceMatch = i32;
// The key sequences are different; not even partially matching.
pub const QKeySequence__NoMatch :QKeySequence__SequenceMatch = 0;
// The key sequences match partially, but are not the same.
pub const QKeySequence__PartialMatch :QKeySequence__SequenceMatch = 1;
// The key sequences are the same.
pub const QKeySequence__ExactMatch :QKeySequence__SequenceMatch = 2;
pub fn QKeySequence_SequenceMatchItemName(val: i32) ->String {
  match val {
     QKeySequence__NoMatch => // 0
     {return String::from("NoMatch");}
     QKeySequence__PartialMatch => // 1
     {return String::from("PartialMatch");}
     QKeySequence__ExactMatch => // 2
     {return String::from("ExactMatch");}
  _ => {return format!("{}", val);}
}
}
pub fn QKeySequence_SequenceMatchItemName_s(val: i32) ->String {
  //var nilthis *QKeySequence
  //return nilthis.SequenceMatchItemName(val);
  return QKeySequence_SequenceMatchItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
