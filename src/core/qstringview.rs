

// mod ::core::QStringView
// package qtcore
// /usr/include/qt/QtCore/qstringview.h
// #include <qstringview.h>
// #include <QtCore>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 0
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
#[derive(Default)] // class sizeof(QStringView)=16
pub struct QStringView {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QStringView_ITF interface {
//    QStringView_PTR() *QStringView
//}
//func (ptr *QStringView) QStringView_PTR() *QStringView { return ptr }

impl /*struct*/ QStringView {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QStringView {
    return QStringView{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QStringView {
//  type Target = QStringViewBASE;
//
//  fn deref(&self) -> &QStringViewBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QStringViewBASE> for QStringView {
//  fn as_ref(& self) -> & QStringViewBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qstringview.h:172
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void QStringView()

/*
Constructs a null string view.

See also isNull().
*/
// QStringView() ctx.fn_proto_cpp
impl /*struct*/ QStringView {
  pub fn QStringView_0<T: QStringView_QStringView_0>(value: T) -> QStringView {
    let rsthis = value.QStringView_0();
    return rsthis;
    // return 1;
  }
}

pub trait QStringView_QStringView_0 {
  fn QStringView_0(self) -> QStringView;
}
// QStringView() ctx.fn_proto_cpp
impl<'a> /*trait*/ QStringView_QStringView_0 for () {
  fn QStringView_0(self) -> QStringView {
    // unsafe{_ZN11QStringViewC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN11QStringViewC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QStringView{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qstringview.h:215
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QString toString() const

/*
Returns a deep copy of this string view's data as a QString.

The return value will be the null QString if and only if this string view is null.

Warning: QStringView can store strings with more than 230 characters while QString cannot. Calling this function on a string view for which size() returns a value greater than INT_MAX / 2 constitutes undefined behavior.
*/
impl /*struct*/ QStringView {
  pub fn toString_0<RetType, T: QStringView_toString_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toString_0(self);
    // return 1;
  }
}
pub trait QStringView_toString_0<RetType> {
  fn toString_0(self , rsthis: & QStringView) -> RetType;
}
impl<'a> /*trait*/ QStringView_toString_0<usize> for () {
  fn toString_0(self , rsthis: & QStringView) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QStringView8toStringEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstringview.h:217
// index:0
// Public inline Visibility=Default Availability=Available
// [8] qsizetype size() const

/*
Returns the size of this string view, in UTF-16 code points (that is, surrogate pairs count as two for the purposes of this function, the same as in QString and QStringRef).

See also empty(), isEmpty(), isNull(), and length().
*/
impl /*struct*/ QStringView {
  pub fn size_0<RetType, T: QStringView_size_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.size_0(self);
    // return 1;
  }
}
pub trait QStringView_size_0<RetType> {
  fn size_0(self , rsthis: & QStringView) -> RetType;
}
impl<'a> /*trait*/ QStringView_size_0<i64> for () {
  fn size_0(self , rsthis: & QStringView) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QStringView4sizeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstringview.h:218
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QStringView::const_pointer data() const

/*
Returns a const pointer to the first character in the string.

Note: The character array represented by the return value is not null-terminated.

See also begin(), end(), and utf16().
*/
impl /*struct*/ QStringView {
  pub fn data_0<RetType, T: QStringView_data_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.data_0(self);
    // return 1;
  }
}
pub trait QStringView_data_0<RetType> {
  fn data_0(self , rsthis: & QStringView) -> RetType;
}
impl<'a> /*trait*/ QStringView_data_0<usize> for () {
  fn data_0(self , rsthis: & QStringView) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QStringView4dataEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstringview.h:219
// index:0
// Public inline Visibility=Default Availability=Available
// [8] const QStringView::storage_type * utf16() const

/*
Returns a const pointer to the first character in the string.

storage_type is char16_t, except on MSVC 2013 (which lacks char16_t support), where it is wchar_t instead.

Note: The character array represented by the return value is not null-terminated.

See also begin(), end(), and data().
*/
impl /*struct*/ QStringView {
  pub fn utf16_0<RetType, T: QStringView_utf16_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.utf16_0(self);
    // return 1;
  }
}
pub trait QStringView_utf16_0<RetType> {
  fn utf16_0(self , rsthis: & QStringView) -> RetType;
}
impl<'a> /*trait*/ QStringView_utf16_0<usize> for () {
  fn utf16_0(self , rsthis: & QStringView) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QStringView5utf16Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstringview.h:221
// index:0
// Public inline Visibility=Default Availability=Available
// [2] QChar operator[](qsizetype) const

/*

*/
impl /*struct*/ QStringView {
  pub fn operator_get_index_0<RetType, T: QStringView_operator_get_index_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_get_index_0(self);
    // return 1;
  }
}
pub trait QStringView_operator_get_index_0<RetType> {
  fn operator_get_index_0(self , rsthis: & QStringView) -> RetType;
}
impl<'a> /*trait*/ QStringView_operator_get_index_0<usize> for (i64) {
  fn operator_get_index_0(self , rsthis: & QStringView) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i64 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QStringViewixEx", 1,qtrt::FFITY_SINT64,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstringview.h:228
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QByteArray toLatin1() const

/*
Returns a Latin-1 representation of the string as a QByteArray.

The behavior is undefined if the string contains non-Latin1 characters.

See also toUtf8(), toLocal8Bit(), QTextCodec, and qConvertToLatin1().
*/
impl /*struct*/ QStringView {
  pub fn toLatin1_0<RetType, T: QStringView_toLatin1_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toLatin1_0(self);
    // return 1;
  }
}
pub trait QStringView_toLatin1_0<RetType> {
  fn toLatin1_0(self , rsthis: & QStringView) -> RetType;
}
impl<'a> /*trait*/ QStringView_toLatin1_0<usize> for () {
  fn toLatin1_0(self , rsthis: & QStringView) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QStringView8toLatin1Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstringview.h:229
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QByteArray toUtf8() const

/*
Returns a UTF-8 representation of the string as a QByteArray.

UTF-8 is a Unicode codec and can represent all characters in a Unicode string like QString.

See also toLatin1(), toLocal8Bit(), QTextCodec, and qConvertToUtf8().
*/
impl /*struct*/ QStringView {
  pub fn toUtf8_0<RetType, T: QStringView_toUtf8_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toUtf8_0(self);
    // return 1;
  }
}
pub trait QStringView_toUtf8_0<RetType> {
  fn toUtf8_0(self , rsthis: & QStringView) -> RetType;
}
impl<'a> /*trait*/ QStringView_toUtf8_0<usize> for () {
  fn toUtf8_0(self , rsthis: & QStringView) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QStringView6toUtf8Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstringview.h:230
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QByteArray toLocal8Bit() const

/*
Returns a local 8-bit representation of the string as a QByteArray.

QTextCodec::codecForLocale() is used to perform the conversion from Unicode. If the locale's encoding could not be determined, this function does the same as toLatin1().

The behavior is undefined if the string contains characters not supported by the locale's 8-bit encoding.

See also toLatin1(), toUtf8(), and QTextCodec.
*/
impl /*struct*/ QStringView {
  pub fn toLocal8Bit_0<RetType, T: QStringView_toLocal8Bit_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toLocal8Bit_0(self);
    // return 1;
  }
}
pub trait QStringView_toLocal8Bit_0<RetType> {
  fn toLocal8Bit_0(self , rsthis: & QStringView) -> RetType;
}
impl<'a> /*trait*/ QStringView_toLocal8Bit_0<usize> for () {
  fn toLocal8Bit_0(self , rsthis: & QStringView) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QStringView11toLocal8BitEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstringview.h:233
// index:0
// Public inline Visibility=Default Availability=Available
// [2] QChar at(qsizetype) const

/*
Returns the character at position n in this string view.

The behavior is undefined if n is negative or not less than size().

See also operator[](), front(), and back().
*/
impl /*struct*/ QStringView {
  pub fn at_0<RetType, T: QStringView_at_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.at_0(self);
    // return 1;
  }
}
pub trait QStringView_at_0<RetType> {
  fn at_0(self , rsthis: & QStringView) -> RetType;
}
impl<'a> /*trait*/ QStringView_at_0<usize> for (i64) {
  fn at_0(self , rsthis: & QStringView) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i64 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QStringView2atEx", 1,qtrt::FFITY_SINT64,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstringview.h:235
// index:0
// Public inline Visibility=Default Availability=Available
// [16] QStringView mid(qsizetype) const

/*
Returns the substring starting at position start in this object, and extending to the end of the string.

Note: The behavior is undefined when start < 0 or start > size().

See also left(), right(), chopped(), chop(), and truncate().
*/
impl /*struct*/ QStringView {
  pub fn mid_0<RetType, T: QStringView_mid_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mid_0(self);
    // return 1;
  }
}
pub trait QStringView_mid_0<RetType> {
  fn mid_0(self , rsthis: & QStringView) -> RetType;
}
impl<'a> /*trait*/ QStringView_mid_0<usize> for (i64) {
  fn mid_0(self , rsthis: & QStringView) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i64 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QStringView3midEx", 1,qtrt::FFITY_SINT64,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstringview.h:237
// index:1
// Public inline Visibility=Default Availability=Available
// [16] QStringView mid(qsizetype, qsizetype) const

/*
Returns the substring starting at position start in this object, and extending to the end of the string.

Note: The behavior is undefined when start < 0 or start > size().

See also left(), right(), chopped(), chop(), and truncate().
*/
impl /*struct*/ QStringView {
  pub fn mid_1<RetType, T: QStringView_mid_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mid_1(self);
    // return 1;
  }
}
pub trait QStringView_mid_1<RetType> {
  fn mid_1(self , rsthis: & QStringView) -> RetType;
}
impl<'a> /*trait*/ QStringView_mid_1<usize> for (i64,i64) {
  fn mid_1(self , rsthis: & QStringView) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i64 as usize;
    let arg1 = (&self.1) as *const i64 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QStringView3midExx", 2,qtrt::FFITY_SINT64,qtrt::FFITY_SINT64,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstringview.h:239
// index:0
// Public inline Visibility=Default Availability=Available
// [16] QStringView left(qsizetype) const

/*
Returns the substring of length length starting at position 0 in this object.

Note: The behavior is undefined when length < 0 or length > size().

See also mid(), right(), chopped(), chop(), and truncate().
*/
impl /*struct*/ QStringView {
  pub fn left_0<RetType, T: QStringView_left_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.left_0(self);
    // return 1;
  }
}
pub trait QStringView_left_0<RetType> {
  fn left_0(self , rsthis: & QStringView) -> RetType;
}
impl<'a> /*trait*/ QStringView_left_0<usize> for (i64) {
  fn left_0(self , rsthis: & QStringView) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i64 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QStringView4leftEx", 1,qtrt::FFITY_SINT64,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstringview.h:241
// index:0
// Public inline Visibility=Default Availability=Available
// [16] QStringView right(qsizetype) const

/*
Returns the substring of length length starting at position size() - length in this object.

Note: The behavior is undefined when length < 0 or length > size().

See also mid(), left(), chopped(), chop(), and truncate().
*/
impl /*struct*/ QStringView {
  pub fn right_0<RetType, T: QStringView_right_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.right_0(self);
    // return 1;
  }
}
pub trait QStringView_right_0<RetType> {
  fn right_0(self , rsthis: & QStringView) -> RetType;
}
impl<'a> /*trait*/ QStringView_right_0<usize> for (i64) {
  fn right_0(self , rsthis: & QStringView) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i64 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QStringView5rightEx", 1,qtrt::FFITY_SINT64,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstringview.h:243
// index:0
// Public inline Visibility=Default Availability=Available
// [16] QStringView chopped(qsizetype) const

/*
Returns the substring of length size() - length starting at the beginning of this object.

Same as left(size() - length).

Note: The behavior is undefined when length < 0 or length > size().

See also mid(), left(), right(), chop(), and truncate().
*/
impl /*struct*/ QStringView {
  pub fn chopped_0<RetType, T: QStringView_chopped_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.chopped_0(self);
    // return 1;
  }
}
pub trait QStringView_chopped_0<RetType> {
  fn chopped_0(self , rsthis: & QStringView) -> RetType;
}
impl<'a> /*trait*/ QStringView_chopped_0<usize> for (i64) {
  fn chopped_0(self , rsthis: & QStringView) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i64 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QStringView7choppedEx", 1,qtrt::FFITY_SINT64,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstringview.h:246
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void truncate(qsizetype)

/*
Truncates this string view to length length.

Same as *this = left(length).

Note: The behavior is undefined when length < 0 or length > size().

See also mid(), left(), right(), chopped(), and chop().
*/
impl /*struct*/ QStringView {
  pub fn truncate_0<RetType, T: QStringView_truncate_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.truncate_0(self);
    // return 1;
  }
}
pub trait QStringView_truncate_0<RetType> {
  fn truncate_0(self , rsthis: & QStringView) -> RetType;
}
impl<'a> /*trait*/ QStringView_truncate_0<(/*void*/)> for (i64) {
  fn truncate_0(self , rsthis: & QStringView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i64 as usize;
     qtrt::InvokeQtFunc6("_ZN11QStringView8truncateEx", 1,qtrt::FFITY_SINT64,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qstringview.h:248
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void chop(qsizetype)

/*
Truncates this string view by length characters.

Same as *this = left(size() - length).

Note: The behavior is undefined when length < 0 or length > size().

See also mid(), left(), right(), chopped(), and truncate().
*/
impl /*struct*/ QStringView {
  pub fn chop_0<RetType, T: QStringView_chop_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.chop_0(self);
    // return 1;
  }
}
pub trait QStringView_chop_0<RetType> {
  fn chop_0(self , rsthis: & QStringView) -> RetType;
}
impl<'a> /*trait*/ QStringView_chop_0<(/*void*/)> for (i64) {
  fn chop_0(self , rsthis: & QStringView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i64 as usize;
     qtrt::InvokeQtFunc6("_ZN11QStringView4chopEx", 1,qtrt::FFITY_SINT64,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qstringview.h:251
// index:0
// Public inline Visibility=Default Availability=Available
// [16] QStringView trimmed() const

/*
Strips leading and trailing whitespace and returns the result.

Whitespace means any character for which QChar::isSpace() returns true. This includes the ASCII characters '\t', '\n', '\v', '\f', '\r', and ' '.
*/
impl /*struct*/ QStringView {
  pub fn trimmed_0<RetType, T: QStringView_trimmed_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.trimmed_0(self);
    // return 1;
  }
}
pub trait QStringView_trimmed_0<RetType> {
  fn trimmed_0(self , rsthis: & QStringView) -> RetType;
}
impl<'a> /*trait*/ QStringView_trimmed_0<usize> for () {
  fn trimmed_0(self , rsthis: & QStringView) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QStringView7trimmedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstringview.h:253
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool startsWith(QStringView, Qt::CaseSensitivity) const

/*
Returns true if this string-view starts with string-view str, Latin-1 string l1, or character ch, respectively; otherwise returns false.

If cs is Qt::CaseSensitive (the default), the search is case-sensitive; otherwise the search is case-insensitive.

See also endsWith().
*/
impl /*struct*/ QStringView {
  pub fn startsWith_0<RetType, T: QStringView_startsWith_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.startsWith_0(self);
    // return 1;
  }
}
pub trait QStringView_startsWith_0<RetType> {
  fn startsWith_0(self , rsthis: & QStringView) -> RetType;
}
impl<'a> /*trait*/ QStringView_startsWith_0<bool> for (usize,i32) {
  fn startsWith_0(self , rsthis: & QStringView) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QStringView10startsWithES_N2Qt15CaseSensitivityE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstringview.h:255
// index:1
// Public inline Visibility=Default Availability=Available
// [1] bool startsWith(QLatin1String, Qt::CaseSensitivity) const

/*
Returns true if this string-view starts with string-view str, Latin-1 string l1, or character ch, respectively; otherwise returns false.

If cs is Qt::CaseSensitive (the default), the search is case-sensitive; otherwise the search is case-insensitive.

See also endsWith().
*/
impl /*struct*/ QStringView {
  pub fn startsWith_1<RetType, T: QStringView_startsWith_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.startsWith_1(self);
    // return 1;
  }
}
pub trait QStringView_startsWith_1<RetType> {
  fn startsWith_1(self , rsthis: & QStringView) -> RetType;
}
impl<'a> /*trait*/ QStringView_startsWith_1<bool> for (usize,i32) {
  fn startsWith_1(self , rsthis: & QStringView) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QStringView10startsWithE13QLatin1StringN2Qt15CaseSensitivityE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstringview.h:256
// index:2
// Public inline Visibility=Default Availability=Available
// [1] bool startsWith(QChar) const

/*
Returns true if this string-view starts with string-view str, Latin-1 string l1, or character ch, respectively; otherwise returns false.

If cs is Qt::CaseSensitive (the default), the search is case-sensitive; otherwise the search is case-insensitive.

See also endsWith().
*/
impl /*struct*/ QStringView {
  pub fn startsWith_2<RetType, T: QStringView_startsWith_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.startsWith_2(self);
    // return 1;
  }
}
pub trait QStringView_startsWith_2<RetType> {
  fn startsWith_2(self , rsthis: & QStringView) -> RetType;
}
impl<'a> /*trait*/ QStringView_startsWith_2<bool> for (usize) {
  fn startsWith_2(self , rsthis: & QStringView) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QStringView10startsWithE5QChar", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstringview.h:258
// index:3
// Public inline Visibility=Default Availability=Available
// [1] bool startsWith(QChar, Qt::CaseSensitivity) const

/*
Returns true if this string-view starts with string-view str, Latin-1 string l1, or character ch, respectively; otherwise returns false.

If cs is Qt::CaseSensitive (the default), the search is case-sensitive; otherwise the search is case-insensitive.

See also endsWith().
*/
impl /*struct*/ QStringView {
  pub fn startsWith_3<RetType, T: QStringView_startsWith_3<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.startsWith_3(self);
    // return 1;
  }
}
pub trait QStringView_startsWith_3<RetType> {
  fn startsWith_3(self , rsthis: & QStringView) -> RetType;
}
impl<'a> /*trait*/ QStringView_startsWith_3<bool> for (usize,i32) {
  fn startsWith_3(self , rsthis: & QStringView) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QStringView10startsWithE5QCharN2Qt15CaseSensitivityE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstringview.h:261
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool endsWith(QStringView, Qt::CaseSensitivity) const

/*
Returns true if this string-view ends with string-view str, Latin-1 string l1, or character ch, respectively; otherwise returns false.

If cs is Qt::CaseSensitive (the default), the search is case-sensitive; otherwise the search is case-insensitive.

See also startsWith().
*/
impl /*struct*/ QStringView {
  pub fn endsWith_0<RetType, T: QStringView_endsWith_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.endsWith_0(self);
    // return 1;
  }
}
pub trait QStringView_endsWith_0<RetType> {
  fn endsWith_0(self , rsthis: & QStringView) -> RetType;
}
impl<'a> /*trait*/ QStringView_endsWith_0<bool> for (usize,i32) {
  fn endsWith_0(self , rsthis: & QStringView) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QStringView8endsWithES_N2Qt15CaseSensitivityE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstringview.h:263
// index:1
// Public inline Visibility=Default Availability=Available
// [1] bool endsWith(QLatin1String, Qt::CaseSensitivity) const

/*
Returns true if this string-view ends with string-view str, Latin-1 string l1, or character ch, respectively; otherwise returns false.

If cs is Qt::CaseSensitive (the default), the search is case-sensitive; otherwise the search is case-insensitive.

See also startsWith().
*/
impl /*struct*/ QStringView {
  pub fn endsWith_1<RetType, T: QStringView_endsWith_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.endsWith_1(self);
    // return 1;
  }
}
pub trait QStringView_endsWith_1<RetType> {
  fn endsWith_1(self , rsthis: & QStringView) -> RetType;
}
impl<'a> /*trait*/ QStringView_endsWith_1<bool> for (usize,i32) {
  fn endsWith_1(self , rsthis: & QStringView) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QStringView8endsWithE13QLatin1StringN2Qt15CaseSensitivityE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstringview.h:264
// index:2
// Public inline Visibility=Default Availability=Available
// [1] bool endsWith(QChar) const

/*
Returns true if this string-view ends with string-view str, Latin-1 string l1, or character ch, respectively; otherwise returns false.

If cs is Qt::CaseSensitive (the default), the search is case-sensitive; otherwise the search is case-insensitive.

See also startsWith().
*/
impl /*struct*/ QStringView {
  pub fn endsWith_2<RetType, T: QStringView_endsWith_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.endsWith_2(self);
    // return 1;
  }
}
pub trait QStringView_endsWith_2<RetType> {
  fn endsWith_2(self , rsthis: & QStringView) -> RetType;
}
impl<'a> /*trait*/ QStringView_endsWith_2<bool> for (usize) {
  fn endsWith_2(self , rsthis: & QStringView) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QStringView8endsWithE5QChar", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstringview.h:266
// index:3
// Public inline Visibility=Default Availability=Available
// [1] bool endsWith(QChar, Qt::CaseSensitivity) const

/*
Returns true if this string-view ends with string-view str, Latin-1 string l1, or character ch, respectively; otherwise returns false.

If cs is Qt::CaseSensitive (the default), the search is case-sensitive; otherwise the search is case-insensitive.

See also startsWith().
*/
impl /*struct*/ QStringView {
  pub fn endsWith_3<RetType, T: QStringView_endsWith_3<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.endsWith_3(self);
    // return 1;
  }
}
pub trait QStringView_endsWith_3<RetType> {
  fn endsWith_3(self , rsthis: & QStringView) -> RetType;
}
impl<'a> /*trait*/ QStringView_endsWith_3<bool> for (usize,i32) {
  fn endsWith_3(self , rsthis: & QStringView) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QStringView8endsWithE5QCharN2Qt15CaseSensitivityE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstringview.h:272
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QStringView::const_iterator begin() const

/*
Returns a const STL-style iterator pointing to the first character in the string.

This function is provided for STL compatibility.

See also end(), cbegin(), rbegin(), and data().
*/
impl /*struct*/ QStringView {
  pub fn begin_0<RetType, T: QStringView_begin_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.begin_0(self);
    // return 1;
  }
}
pub trait QStringView_begin_0<RetType> {
  fn begin_0(self , rsthis: & QStringView) -> RetType;
}
impl<'a> /*trait*/ QStringView_begin_0<usize> for () {
  fn begin_0(self , rsthis: & QStringView) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QStringView5beginEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstringview.h:273
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QStringView::const_iterator end() const

/*
Returns a const STL-style iterator pointing to the imaginary character after the last character in the list.

This function is provided for STL compatibility.

See also begin(), cend(), and rend().
*/
impl /*struct*/ QStringView {
  pub fn end_0<RetType, T: QStringView_end_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.end_0(self);
    // return 1;
  }
}
pub trait QStringView_end_0<RetType> {
  fn end_0(self , rsthis: & QStringView) -> RetType;
}
impl<'a> /*trait*/ QStringView_end_0<usize> for () {
  fn end_0(self , rsthis: & QStringView) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QStringView3endEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstringview.h:274
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QStringView::const_iterator cbegin() const

/*
Same as begin().

This function is provided for STL compatibility.

See also cend(), begin(), crbegin(), and data().
*/
impl /*struct*/ QStringView {
  pub fn cbegin_0<RetType, T: QStringView_cbegin_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.cbegin_0(self);
    // return 1;
  }
}
pub trait QStringView_cbegin_0<RetType> {
  fn cbegin_0(self , rsthis: & QStringView) -> RetType;
}
impl<'a> /*trait*/ QStringView_cbegin_0<usize> for () {
  fn cbegin_0(self , rsthis: & QStringView) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QStringView6cbeginEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstringview.h:275
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QStringView::const_iterator cend() const

/*
Same as end().

This function is provided for STL compatibility.

See also cbegin(), end(), and crend().
*/
impl /*struct*/ QStringView {
  pub fn cend_0<RetType, T: QStringView_cend_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.cend_0(self);
    // return 1;
  }
}
pub trait QStringView_cend_0<RetType> {
  fn cend_0(self , rsthis: & QStringView) -> RetType;
}
impl<'a> /*trait*/ QStringView_cend_0<usize> for () {
  fn cend_0(self , rsthis: & QStringView) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QStringView4cendEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstringview.h:281
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool empty() const

/*
Returns whether this string view is empty - that is, whether size() == 0.

This function is provided for STL compatibility.

See also isEmpty(), isNull(), size(), and length().
*/
impl /*struct*/ QStringView {
  pub fn empty_0<RetType, T: QStringView_empty_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.empty_0(self);
    // return 1;
  }
}
pub trait QStringView_empty_0<RetType> {
  fn empty_0(self , rsthis: & QStringView) -> RetType;
}
impl<'a> /*trait*/ QStringView_empty_0<bool> for () {
  fn empty_0(self , rsthis: & QStringView) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QStringView5emptyEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstringview.h:282
// index:0
// Public inline Visibility=Default Availability=Available
// [2] QChar front() const

/*
Returns the first character in the string. Same as first().

This function is provided for STL compatibility.

Warning: Calling this function on an empty string view constitutes undefined behavior.

See also back(), first(), and last().
*/
impl /*struct*/ QStringView {
  pub fn front_0<RetType, T: QStringView_front_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.front_0(self);
    // return 1;
  }
}
pub trait QStringView_front_0<RetType> {
  fn front_0(self , rsthis: & QStringView) -> RetType;
}
impl<'a> /*trait*/ QStringView_front_0<usize> for () {
  fn front_0(self , rsthis: & QStringView) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QStringView5frontEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstringview.h:283
// index:0
// Public inline Visibility=Default Availability=Available
// [2] QChar back() const

/*
Returns the last character in the string. Same as last().

This function is provided for STL compatibility.

Warning: Calling this function on an empty string view constitutes undefined behavior.

See also front(), first(), and last().
*/
impl /*struct*/ QStringView {
  pub fn back_0<RetType, T: QStringView_back_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.back_0(self);
    // return 1;
  }
}
pub trait QStringView_back_0<RetType> {
  fn back_0(self , rsthis: & QStringView) -> RetType;
}
impl<'a> /*trait*/ QStringView_back_0<usize> for () {
  fn back_0(self , rsthis: & QStringView) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QStringView4backEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstringview.h:288
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool isNull() const

/*
Returns whether this string view is null - that is, whether data() == nullptr.

This functions is provided for compatibility with other Qt containers.

See also empty(), isEmpty(), size(), and length().
*/
impl /*struct*/ QStringView {
  pub fn isNull_0<RetType, T: QStringView_isNull_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isNull_0(self);
    // return 1;
  }
}
pub trait QStringView_isNull_0<RetType> {
  fn isNull_0(self , rsthis: & QStringView) -> RetType;
}
impl<'a> /*trait*/ QStringView_isNull_0<bool> for () {
  fn isNull_0(self , rsthis: & QStringView) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QStringView6isNullEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstringview.h:289
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool isEmpty() const

/*
Returns whether this string view is empty - that is, whether size() == 0.

This function is provided for compatibility with other Qt containers.

See also empty(), isNull(), size(), and length().
*/
impl /*struct*/ QStringView {
  pub fn isEmpty_0<RetType, T: QStringView_isEmpty_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isEmpty_0(self);
    // return 1;
  }
}
pub trait QStringView_isEmpty_0<RetType> {
  fn isEmpty_0(self , rsthis: & QStringView) -> RetType;
}
impl<'a> /*trait*/ QStringView_isEmpty_0<bool> for () {
  fn isEmpty_0(self , rsthis: & QStringView) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QStringView7isEmptyEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstringview.h:290
// index:0
// Public inline Visibility=Default Availability=Available
// [4] int length() const

/*
Same as size(), except returns the result as an int.

This function is provided for compatibility with other Qt containers.

Warning: QStringView can represent strings with more than 231 characters. Calling this function on a string view for which size() returns a value greater than INT_MAX constitutes undefined behavior.

See also empty(), isEmpty(), isNull(), and size().
*/
impl /*struct*/ QStringView {
  pub fn length_0<RetType, T: QStringView_length_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.length_0(self);
    // return 1;
  }
}
pub trait QStringView_length_0<RetType> {
  fn length_0(self , rsthis: & QStringView) -> RetType;
}
impl<'a> /*trait*/ QStringView_length_0<i32> for () {
  fn length_0(self , rsthis: & QStringView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QStringView6lengthEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstringview.h:292
// index:0
// Public inline Visibility=Default Availability=Available
// [2] QChar first() const

/*
Returns the first character in the string. Same as front().

This function is provided for compatibility with other Qt containers.

Warning: Calling this function on an empty string view constitutes undefined behavior.

See also front(), back(), and last().
*/
impl /*struct*/ QStringView {
  pub fn first_0<RetType, T: QStringView_first_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.first_0(self);
    // return 1;
  }
}
pub trait QStringView_first_0<RetType> {
  fn first_0(self , rsthis: & QStringView) -> RetType;
}
impl<'a> /*trait*/ QStringView_first_0<usize> for () {
  fn first_0(self , rsthis: & QStringView) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QStringView5firstEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstringview.h:293
// index:0
// Public inline Visibility=Default Availability=Available
// [2] QChar last() const

/*
Returns the last character in the string. Same as back().

This function is provided for compatibility with other Qt containers.

Warning: Calling this function on an empty string view constitutes undefined behavior.

See also back(), front(), and first().
*/
impl /*struct*/ QStringView {
  pub fn last_0<RetType, T: QStringView_last_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.last_0(self);
    // return 1;
  }
}
pub trait QStringView_last_0<RetType> {
  fn last_0(self , rsthis: & QStringView) -> RetType;
}
impl<'a> /*trait*/ QStringView_last_0<usize> for () {
  fn last_0(self , rsthis: & QStringView) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QStringView4lastEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}


pub fn DeleteQStringView(this :*mut QStringView) {
    // rv, err := qtrt::InvokeQtFunc6("_ZN11QStringViewD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis())
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
//  body block end

//  keep block begin

//  keep block end
