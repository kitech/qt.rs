

// mod ::core::QTextCodec
// package qtcore
// /usr/include/qt/QtCore/qtextcodec.h
// #include <qtextcodec.h>
// #include <QtCore>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 16
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
#[derive(Default)] // class sizeof(QTextCodec)=8
pub struct QTextCodec {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QTextCodec_ITF interface {
//    QTextCodec_PTR() *QTextCodec
//}
//func (ptr *QTextCodec) QTextCodec_PTR() *QTextCodec { return ptr }

impl /*struct*/ QTextCodec {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QTextCodec {
    return QTextCodec{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QTextCodec {
//  type Target = QTextCodecBASE;
//
//  fn deref(&self) -> &QTextCodecBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QTextCodecBASE> for QTextCodec {
//  fn as_ref(& self) -> & QTextCodecBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qtextcodec.h:61
// index:0
// Public static Visibility=Default Availability=Available
// [8] QTextCodec * codecForName(const QByteArray &)

/*
Searches all installed QTextCodec objects and returns the one which best matches name; the match is case-insensitive. Returns 0 if no codec matching the name name could be found.

Note: This function is thread-safe.
*/
impl /*struct*/ QTextCodec {
  pub fn codecForName_0<RetType, T: QTextCodec_codecForName_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.codecForName_0();
    // return 1;
  }
}
pub trait QTextCodec_codecForName_0<RetType> {
  fn codecForName_0(self ) -> RetType;
}
impl<'a> /*trait*/ QTextCodec_codecForName_0<usize> for (usize) {
  fn codecForName_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN10QTextCodec12codecForNameERK10QByteArray", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qtextcodec.h:62
// index:1
// Public static inline Visibility=Default Availability=Available
// [8] QTextCodec * codecForName(const char *)

/*
Searches all installed QTextCodec objects and returns the one which best matches name; the match is case-insensitive. Returns 0 if no codec matching the name name could be found.

Note: This function is thread-safe.
*/
impl /*struct*/ QTextCodec {
  pub fn codecForName_1<RetType, T: QTextCodec_codecForName_1<RetType>>( overload_args: T) -> RetType {
    return overload_args.codecForName_1();
    // return 1;
  }
}
pub trait QTextCodec_codecForName_1<RetType> {
  fn codecForName_1(self ) -> RetType;
}
impl<'a> /*trait*/ QTextCodec_codecForName_1<usize> for (usize) {
  fn codecForName_1(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (self) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN10QTextCodec12codecForNameEPKc", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qtextcodec.h:63
// index:0
// Public static Visibility=Default Availability=Available
// [8] QTextCodec * codecForMib(int)

/*
Returns the QTextCodec which matches the MIBenum mib.

Note: This function is thread-safe.
*/
impl /*struct*/ QTextCodec {
  pub fn codecForMib_0<RetType, T: QTextCodec_codecForMib_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.codecForMib_0();
    // return 1;
  }
}
pub trait QTextCodec_codecForMib_0<RetType> {
  fn codecForMib_0(self ) -> RetType;
}
impl<'a> /*trait*/ QTextCodec_codecForMib_0<usize> for (i32) {
  fn codecForMib_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN10QTextCodec11codecForMibEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qtextcodec.h:68
// index:0
// Public static Visibility=Default Availability=Available
// [8] QTextCodec * codecForLocale()

/*
Returns a pointer to the codec most suitable for this locale.

On Windows, the codec will be based on a system locale. On Unix systems, the codec will might fall back to using the iconv library if no builtin codec for the locale can be found.

Note that in these cases the codec's name will be "System".

Note: This function is thread-safe.

See also setCodecForLocale().
*/
impl /*struct*/ QTextCodec {
  pub fn codecForLocale_0<RetType, T: QTextCodec_codecForLocale_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.codecForLocale_0();
    // return 1;
  }
}
pub trait QTextCodec_codecForLocale_0<RetType> {
  fn codecForLocale_0(self ) -> RetType;
}
impl<'a> /*trait*/ QTextCodec_codecForLocale_0<usize> for () {
  fn codecForLocale_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN10QTextCodec14codecForLocaleEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qtextcodec.h:69
// index:0
// Public static Visibility=Default Availability=Available
// [-2] void setCodecForLocale(QTextCodec *)

/*
Set the codec to c; this will be returned by codecForLocale(). If c is a null pointer, the codec is reset to the default.

This might be needed for some applications that want to use their own mechanism for setting the locale.

Warning: This function is not reentrant.

See also codecForLocale().
*/
impl /*struct*/ QTextCodec {
  pub fn setCodecForLocale_0<RetType, T: QTextCodec_setCodecForLocale_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.setCodecForLocale_0();
    // return 1;
  }
}
pub trait QTextCodec_setCodecForLocale_0<RetType> {
  fn setCodecForLocale_0(self ) -> RetType;
}
impl<'a> /*trait*/ QTextCodec_setCodecForLocale_0<(/*void*/)> for (usize) {
  fn setCodecForLocale_0(self ) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN10QTextCodec17setCodecForLocaleEPS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qtextcodec.h:75
// index:0
// Public static Visibility=Default Availability=Available
// [8] QTextCodec * codecForHtml(const QByteArray &)

/*
Tries to detect the encoding of the provided snippet of HTML in the given byte array, ba, by checking the BOM (Byte Order Mark) and the content-type meta header and returns a QTextCodec instance that is capable of decoding the html to unicode. If the codec cannot be detected from the content provided, defaultCodec is returned.

This function was introduced in  Qt 4.4.

See also codecForUtfText().
*/
impl /*struct*/ QTextCodec {
  pub fn codecForHtml_0<RetType, T: QTextCodec_codecForHtml_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.codecForHtml_0();
    // return 1;
  }
}
pub trait QTextCodec_codecForHtml_0<RetType> {
  fn codecForHtml_0(self ) -> RetType;
}
impl<'a> /*trait*/ QTextCodec_codecForHtml_0<usize> for (usize) {
  fn codecForHtml_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN10QTextCodec12codecForHtmlERK10QByteArray", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qtextcodec.h:76
// index:1
// Public static Visibility=Default Availability=Available
// [8] QTextCodec * codecForHtml(const QByteArray &, QTextCodec *)

/*
Tries to detect the encoding of the provided snippet of HTML in the given byte array, ba, by checking the BOM (Byte Order Mark) and the content-type meta header and returns a QTextCodec instance that is capable of decoding the html to unicode. If the codec cannot be detected from the content provided, defaultCodec is returned.

This function was introduced in  Qt 4.4.

See also codecForUtfText().
*/
impl /*struct*/ QTextCodec {
  pub fn codecForHtml_1<RetType, T: QTextCodec_codecForHtml_1<RetType>>( overload_args: T) -> RetType {
    return overload_args.codecForHtml_1();
    // return 1;
  }
}
pub trait QTextCodec_codecForHtml_1<RetType> {
  fn codecForHtml_1(self ) -> RetType;
}
impl<'a> /*trait*/ QTextCodec_codecForHtml_1<usize> for (usize,usize) {
  fn codecForHtml_1(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN10QTextCodec12codecForHtmlERK10QByteArrayPS_", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qtextcodec.h:78
// index:0
// Public static Visibility=Default Availability=Available
// [8] QTextCodec * codecForUtfText(const QByteArray &)

/*
Tries to detect the encoding of the provided snippet ba by using the BOM (Byte Order Mark) and returns a QTextCodec instance that is capable of decoding the text to unicode. If the codec cannot be detected from the content provided, defaultCodec is returned.

This function was introduced in  Qt 4.6.

See also codecForHtml().
*/
impl /*struct*/ QTextCodec {
  pub fn codecForUtfText_0<RetType, T: QTextCodec_codecForUtfText_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.codecForUtfText_0();
    // return 1;
  }
}
pub trait QTextCodec_codecForUtfText_0<RetType> {
  fn codecForUtfText_0(self ) -> RetType;
}
impl<'a> /*trait*/ QTextCodec_codecForUtfText_0<usize> for (usize) {
  fn codecForUtfText_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN10QTextCodec15codecForUtfTextERK10QByteArray", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qtextcodec.h:79
// index:1
// Public static Visibility=Default Availability=Available
// [8] QTextCodec * codecForUtfText(const QByteArray &, QTextCodec *)

/*
Tries to detect the encoding of the provided snippet ba by using the BOM (Byte Order Mark) and returns a QTextCodec instance that is capable of decoding the text to unicode. If the codec cannot be detected from the content provided, defaultCodec is returned.

This function was introduced in  Qt 4.6.

See also codecForHtml().
*/
impl /*struct*/ QTextCodec {
  pub fn codecForUtfText_1<RetType, T: QTextCodec_codecForUtfText_1<RetType>>( overload_args: T) -> RetType {
    return overload_args.codecForUtfText_1();
    // return 1;
  }
}
pub trait QTextCodec_codecForUtfText_1<RetType> {
  fn codecForUtfText_1(self ) -> RetType;
}
impl<'a> /*trait*/ QTextCodec_codecForUtfText_1<usize> for (usize,usize) {
  fn codecForUtfText_1(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN10QTextCodec15codecForUtfTextERK10QByteArrayPS_", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qtextcodec.h:81
// index:0
// Public Visibility=Default Availability=Available
// [1] bool canEncode(QChar) const

/*
Returns true if the Unicode character ch can be fully encoded with this codec; otherwise returns false.
*/
impl /*struct*/ QTextCodec {
  pub fn canEncode_0<RetType, T: QTextCodec_canEncode_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.canEncode_0(self);
    // return 1;
  }
}
pub trait QTextCodec_canEncode_0<RetType> {
  fn canEncode_0(self , rsthis: & QTextCodec) -> RetType;
}
impl<'a> /*trait*/ QTextCodec_canEncode_0<bool> for (usize) {
  fn canEncode_0(self , rsthis: & QTextCodec) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QTextCodec9canEncodeE5QChar", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qtextcodec.h:83
// index:1
// Public Visibility=Default Availability=Available
// [1] bool canEncode(const QString &) const

/*
Returns true if the Unicode character ch can be fully encoded with this codec; otherwise returns false.
*/
impl /*struct*/ QTextCodec {
  pub fn canEncode_1<RetType, T: QTextCodec_canEncode_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.canEncode_1(self);
    // return 1;
  }
}
pub trait QTextCodec_canEncode_1<RetType> {
  fn canEncode_1(self , rsthis: & QTextCodec) -> RetType;
}
impl<'a> /*trait*/ QTextCodec_canEncode_1<bool> for (usize) {
  fn canEncode_1(self , rsthis: & QTextCodec) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QTextCodec9canEncodeERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qtextcodec.h:85
// index:2
// Public Visibility=Default Availability=Available
// [1] bool canEncode(QStringView) const

/*
Returns true if the Unicode character ch can be fully encoded with this codec; otherwise returns false.
*/
impl /*struct*/ QTextCodec {
  pub fn canEncode_2<RetType, T: QTextCodec_canEncode_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.canEncode_2(self);
    // return 1;
  }
}
pub trait QTextCodec_canEncode_2<RetType> {
  fn canEncode_2(self , rsthis: & QTextCodec) -> RetType;
}
impl<'a> /*trait*/ QTextCodec_canEncode_2<bool> for (usize) {
  fn canEncode_2(self , rsthis: & QTextCodec) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QTextCodec9canEncodeE11QStringView", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qtextcodec.h:87
// index:0
// Public Visibility=Default Availability=Available
// [8] QString toUnicode(const QByteArray &) const

/*
Converts a from the encoding of this codec to Unicode, and returns the result in a QString.
*/
impl /*struct*/ QTextCodec {
  pub fn toUnicode_0<RetType, T: QTextCodec_toUnicode_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toUnicode_0(self);
    // return 1;
  }
}
pub trait QTextCodec_toUnicode_0<RetType> {
  fn toUnicode_0(self , rsthis: & QTextCodec) -> RetType;
}
impl<'a> /*trait*/ QTextCodec_toUnicode_0<usize> for (usize) {
  fn toUnicode_0(self , rsthis: & QTextCodec) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QTextCodec9toUnicodeERK10QByteArray", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qtextcodec.h:88
// index:1
// Public Visibility=Default Availability=Available
// [8] QString toUnicode(const char *) const

/*
Converts a from the encoding of this codec to Unicode, and returns the result in a QString.
*/
impl /*struct*/ QTextCodec {
  pub fn toUnicode_1<RetType, T: QTextCodec_toUnicode_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toUnicode_1(self);
    // return 1;
  }
}
pub trait QTextCodec_toUnicode_1<RetType> {
  fn toUnicode_1(self , rsthis: & QTextCodec) -> RetType;
}
impl<'a> /*trait*/ QTextCodec_toUnicode_1<usize> for (usize) {
  fn toUnicode_1(self , rsthis: & QTextCodec) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (self) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QTextCodec9toUnicodeEPKc", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qtextcodec.h:90
// index:0
// Public Visibility=Default Availability=Available
// [8] QByteArray fromUnicode(const QString &) const

/*
Converts str from Unicode to the encoding of this codec, and returns the result in a QByteArray.
*/
impl /*struct*/ QTextCodec {
  pub fn fromUnicode_0<RetType, T: QTextCodec_fromUnicode_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.fromUnicode_0(self);
    // return 1;
  }
}
pub trait QTextCodec_fromUnicode_0<RetType> {
  fn fromUnicode_0(self , rsthis: & QTextCodec) -> RetType;
}
impl<'a> /*trait*/ QTextCodec_fromUnicode_0<usize> for (usize) {
  fn fromUnicode_0(self , rsthis: & QTextCodec) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QTextCodec11fromUnicodeERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qtextcodec.h:92
// index:1
// Public Visibility=Default Availability=Available
// [8] QByteArray fromUnicode(QStringView) const

/*
Converts str from Unicode to the encoding of this codec, and returns the result in a QByteArray.
*/
impl /*struct*/ QTextCodec {
  pub fn fromUnicode_1<RetType, T: QTextCodec_fromUnicode_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.fromUnicode_1(self);
    // return 1;
  }
}
pub trait QTextCodec_fromUnicode_1<RetType> {
  fn fromUnicode_1(self , rsthis: & QTextCodec) -> RetType;
}
impl<'a> /*trait*/ QTextCodec_fromUnicode_1<usize> for (usize) {
  fn fromUnicode_1(self , rsthis: & QTextCodec) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QTextCodec11fromUnicodeE11QStringView", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qtextcodec.h:119
// index:0
// Public Visibility=Default Availability=Available
// [8] QTextDecoder * makeDecoder(QTextCodec::ConversionFlags) const

/*
Creates a QTextDecoder with a specified flags to decode chunks of char * data to create chunks of Unicode data.

The caller is responsible for deleting the returned object.

This function was introduced in  Qt 4.7.
*/
impl /*struct*/ QTextCodec {
  pub fn makeDecoder_0<RetType, T: QTextCodec_makeDecoder_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.makeDecoder_0(self);
    // return 1;
  }
}
pub trait QTextCodec_makeDecoder_0<RetType> {
  fn makeDecoder_0(self , rsthis: & QTextCodec) -> RetType;
}
impl<'a> /*trait*/ QTextCodec_makeDecoder_0<usize> for (i32) {
  fn makeDecoder_0(self , rsthis: & QTextCodec) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QTextCodec11makeDecoderE6QFlagsINS_14ConversionFlagEE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qtextcodec.h:120
// index:0
// Public Visibility=Default Availability=Available
// [8] QTextEncoder * makeEncoder(QTextCodec::ConversionFlags) const

/*
Creates a QTextEncoder with a specified flags to encode chunks of Unicode data as char * data.

The caller is responsible for deleting the returned object.

This function was introduced in  Qt 4.7.
*/
impl /*struct*/ QTextCodec {
  pub fn makeEncoder_0<RetType, T: QTextCodec_makeEncoder_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.makeEncoder_0(self);
    // return 1;
  }
}
pub trait QTextCodec_makeEncoder_0<RetType> {
  fn makeEncoder_0(self , rsthis: & QTextCodec) -> RetType;
}
impl<'a> /*trait*/ QTextCodec_makeEncoder_0<usize> for (i32) {
  fn makeEncoder_0(self , rsthis: & QTextCodec) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QTextCodec11makeEncoderE6QFlagsINS_14ConversionFlagEE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qtextcodec.h:122
// index:0
// Public purevirtual virtual Visibility=Default Availability=Available
// [8] QByteArray name() const

/*
QTextCodec subclasses must reimplement this function. It returns the name of the encoding supported by the subclass.

If the codec is registered as a character set in the IANA character-sets encoding file this method should return the preferred mime name for the codec if defined, otherwise its name.
*/
impl /*struct*/ QTextCodec {
  pub fn name_0<RetType, T: QTextCodec_name_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.name_0(self);
    // return 1;
  }
}
pub trait QTextCodec_name_0<RetType> {
  fn name_0(self , rsthis: & QTextCodec) -> RetType;
}
impl<'a> /*trait*/ QTextCodec_name_0<usize> for () {
  fn name_0(self , rsthis: & QTextCodec) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QTextCodec4nameEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qtextcodec.h:124
// index:0
// Public purevirtual virtual Visibility=Default Availability=Available
// [4] int mibEnum() const

/*
Subclasses of QTextCodec must reimplement this function. It returns the MIBenum (see IANA character-sets encoding file for more information). It is important that each QTextCodec subclass returns the correct unique value for this function.
*/
impl /*struct*/ QTextCodec {
  pub fn mibEnum_0<RetType, T: QTextCodec_mibEnum_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mibEnum_0(self);
    // return 1;
  }
}
pub trait QTextCodec_mibEnum_0<RetType> {
  fn mibEnum_0(self , rsthis: & QTextCodec) -> RetType;
}
impl<'a> /*trait*/ QTextCodec_mibEnum_0<i32> for () {
  fn mibEnum_0(self , rsthis: & QTextCodec) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QTextCodec7mibEnumEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qtextcodec.h:130
// index:0
// Protected Visibility=Default Availability=Available
// [-2] void QTextCodec()

/*
Constructs a QTextCodec, and gives it the highest precedence. The QTextCodec should always be constructed on the heap (i.e. with new). Qt takes ownership and will delete it when the application terminates.
*/
// QTextCodec() ctx.fn_proto_cpp
impl /*struct*/ QTextCodec {
  pub fn QTextCodec_0<T: QTextCodec_QTextCodec_0>(value: T) -> QTextCodec {
    let rsthis = value.QTextCodec_0();
    return rsthis;
    // return 1;
  }
}

pub trait QTextCodec_QTextCodec_0 {
  fn QTextCodec_0(self) -> QTextCodec;
}
// QTextCodec() ctx.fn_proto_cpp
impl<'a> /*trait*/ QTextCodec_QTextCodec_0 for () {
  fn QTextCodec_0(self) -> QTextCodec {
    // unsafe{_ZN10QTextCodecC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN10QTextCodecC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QTextCodec{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qtextcodec.h:131
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void ~QTextCodec()

/*

*/
pub fn DeleteQTextCodec(this :*mut QTextCodec) {
    // let rv = qtrt::InvokeQtFunc6("_ZN10QTextCodecD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 8)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}

/*


*/
pub type QTextCodec__ConversionFlag = i32;
// 
pub const QTextCodec__DefaultConversion :QTextCodec__ConversionFlag = 0;
// 
pub const QTextCodec__ConvertInvalidToNull :QTextCodec__ConversionFlag = -2147483648;
// 
pub const QTextCodec__IgnoreHeader :QTextCodec__ConversionFlag = 1;
// 
pub const QTextCodec__FreeFunction :QTextCodec__ConversionFlag = 2;
pub fn QTextCodec_ConversionFlagItemName(val: i32) ->String {
  match val {
     QTextCodec__DefaultConversion => // 0
     {return String::from("DefaultConversion");}
     QTextCodec__ConvertInvalidToNull => // -2147483648
     {return String::from("ConvertInvalidToNull");}
     QTextCodec__IgnoreHeader => // 1
     {return String::from("IgnoreHeader");}
     QTextCodec__FreeFunction => // 2
     {return String::from("FreeFunction");}
  _ => {return format!("{}", val);}
}
}
pub fn QTextCodec_ConversionFlagItemName_s(val: i32) ->String {
  //var nilthis *QTextCodec
  //return nilthis.ConversionFlagItemName(val);
  return QTextCodec_ConversionFlagItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
