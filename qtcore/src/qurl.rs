

// mod ::core::QUrl
// package qtcore
// /usr/include/qt/QtCore/qurl.h
// #include <qurl.h>
// #include <QtCore>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 8
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
#[derive(Default)] // class sizeof(QUrl)=8
pub struct QUrl {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QUrl_ITF interface {
//    QUrl_PTR() *QUrl
//}
//func (ptr *QUrl) QUrl_PTR() *QUrl { return ptr }

impl /*struct*/ QUrl {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QUrl {
    return QUrl{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QUrl {
//  type Target = QUrlBASE;
//
//  fn deref(&self) -> &QUrlBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QUrlBASE> for QUrl {
//  fn as_ref(& self) -> & QUrlBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qurl.h:176
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QUrl()

/*
Constructs an empty QUrl object.
*/
// QUrl() ctx.fn_proto_cpp
impl /*struct*/ QUrl {
  pub fn QUrl_0<T: QUrl_QUrl_0>(value: T) -> QUrl {
    let rsthis = value.QUrl_0();
    return rsthis;
    // return 1;
  }
}

pub trait QUrl_QUrl_0 {
  fn QUrl_0(self) -> QUrl;
}
// QUrl() ctx.fn_proto_cpp
impl<'a> /*trait*/ QUrl_QUrl_0 for () {
  fn QUrl_0(self) -> QUrl {
    // unsafe{_ZN4QUrlC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN4QUrlC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QUrl{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qurl.h:182
// index:1
// Public Visibility=Default Availability=Available
// [-2] void QUrl(const QString &, QUrl::ParsingMode)

/*
Constructs an empty QUrl object.
*/
// QUrl(const QString &, QUrl::ParsingMode) ctx.fn_proto_cpp
impl /*struct*/ QUrl {
  pub fn QUrl_1<T: QUrl_QUrl_1>(value: T) -> QUrl {
    let rsthis = value.QUrl_1();
    return rsthis;
    // return 1;
  }
}

pub trait QUrl_QUrl_1 {
  fn QUrl_1(self) -> QUrl;
}
// QUrl(const QString &, QUrl::ParsingMode) ctx.fn_proto_cpp
impl<'a> /*trait*/ QUrl_QUrl_1 for (usize,i32) {
  fn QUrl_1(self) -> QUrl {
    // unsafe{_ZN4QUrlC2ERK7QStringNS_11ParsingModeE()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN4QUrlC2ERK7QStringNS_11ParsingModeE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QUrl{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qurl.h:178
// index:0
// Public Visibility=Default Availability=Available
// [8] QUrl & operator=(const QUrl &)

/*

*/
impl /*struct*/ QUrl {
  pub fn operator_equal_0<RetType, T: QUrl_operator_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_0(self);
    // return 1;
  }
}
pub trait QUrl_operator_equal_0<RetType> {
  fn operator_equal_0(self , rsthis: & QUrl) -> RetType;
}
impl<'a> /*trait*/ QUrl_operator_equal_0<usize> for (usize) {
  fn operator_equal_0(self , rsthis: & QUrl) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN4QUrlaSERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qurl.h:183
// index:1
// Public Visibility=Default Availability=Available
// [8] QUrl & operator=(const QString &)

/*

*/
impl /*struct*/ QUrl {
  pub fn operator_equal_1<RetType, T: QUrl_operator_equal_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_1(self);
    // return 1;
  }
}
pub trait QUrl_operator_equal_1<RetType> {
  fn operator_equal_1(self , rsthis: & QUrl) -> RetType;
}
impl<'a> /*trait*/ QUrl_operator_equal_1<usize> for (usize) {
  fn operator_equal_1(self , rsthis: & QUrl) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN4QUrlaSERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qurl.h:188
// index:2
// Public inline Visibility=Default Availability=Available
// [8] QUrl & operator=(QUrl &&)

/*

*/
impl /*struct*/ QUrl {
  pub fn operator_equal_2<RetType, T: QUrl_operator_equal_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_2(self);
    // return 1;
  }
}
pub trait QUrl_operator_equal_2<RetType> {
  fn operator_equal_2(self , rsthis: & QUrl) -> RetType;
}
impl<'a> /*trait*/ QUrl_operator_equal_2<usize> for (usize) {
  fn operator_equal_2(self , rsthis: & QUrl) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN4QUrlaSEOS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qurl.h:191
// index:0
// Public Visibility=Default Availability=Available
// [-2] void ~QUrl()

/*

*/
pub fn DeleteQUrl(this :*mut QUrl) {
    // let rv = qtrt::InvokeQtFunc6("_ZN4QUrlD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 8)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtCore/qurl.h:193
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void swap(QUrl &)

/*
Swaps URL other with this URL. This operation is very fast and never fails.

This function was introduced in  Qt 4.8.
*/
impl /*struct*/ QUrl {
  pub fn swap_0<RetType, T: QUrl_swap_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.swap_0(self);
    // return 1;
  }
}
pub trait QUrl_swap_0<RetType> {
  fn swap_0(self , rsthis: & QUrl) -> RetType;
}
impl<'a> /*trait*/ QUrl_swap_0<(/*void*/)> for (usize) {
  fn swap_0(self , rsthis: & QUrl) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN4QUrl4swapERS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qurl.h:195
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setUrl(const QString &, QUrl::ParsingMode)

/*
Parses url and sets this object to that value. QUrl will automatically percent encode all characters that are not allowed in a URL and decode the percent-encoded sequences that represent an unreserved character (letters, digits, hyphens, undercores, dots and tildes). All other characters are left in their original forms.

Parses the url using the parser mode parsingMode. In TolerantMode (the default), QUrl will correct certain mistakes, notably the presence of a percent character ('%') not followed by two hexadecimal digits, and it will accept any character in any position. In StrictMode, encoding mistakes will not be tolerated and QUrl will also check that certain forbidden characters are not present in unencoded form. If an error is detected in StrictMode, isValid() will return false. The parsing mode DecodedMode is not permitted in this context and will produce a run-time warning.

See also url() and toString().
*/
impl /*struct*/ QUrl {
  pub fn setUrl_0<RetType, T: QUrl_setUrl_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setUrl_0(self);
    // return 1;
  }
}
pub trait QUrl_setUrl_0<RetType> {
  fn setUrl_0(self , rsthis: & QUrl) -> RetType;
}
impl<'a> /*trait*/ QUrl_setUrl_0<(/*void*/)> for (usize,i32) {
  fn setUrl_0(self , rsthis: & QUrl) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN4QUrl6setUrlERK7QStringNS_11ParsingModeE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qurl.h:202
// index:0
// Public static Visibility=Default Availability=Available
// [8] QUrl fromEncoded(const QByteArray &, QUrl::ParsingMode)

/*
Parses input and returns the corresponding QUrl. input is assumed to be in encoded form, containing only ASCII characters.

Parses the URL using parsingMode. See setUrl() for more information on this parameter. QUrl::DecodedMode is not permitted in this context.

See also toEncoded() and setUrl().
*/
impl /*struct*/ QUrl {
  pub fn fromEncoded_0<RetType, T: QUrl_fromEncoded_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.fromEncoded_0();
    // return 1;
  }
}
pub trait QUrl_fromEncoded_0<RetType> {
  fn fromEncoded_0(self ) -> RetType;
}
impl<'a> /*trait*/ QUrl_fromEncoded_0<usize> for (usize,i32) {
  fn fromEncoded_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN4QUrl11fromEncodedERK10QByteArrayNS_11ParsingModeE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qurl.h:210
// index:0
// Public static Visibility=Default Availability=Available
// [8] QUrl fromUserInput(const QString &)

/*
Returns a valid URL from a user supplied userInput string if one can be deducted. In the case that is not possible, an invalid QUrl() is returned.

Most applications that can browse the web, allow the user to input a URL in the form of a plain string. This string can be manually typed into a location bar, obtained from the clipboard, or passed in via command line arguments.

When the string is not already a valid URL, a best guess is performed, making various web related assumptions.

In the case the string corresponds to a valid file path on the system, a file:// URL is constructed, using QUrl::fromLocalFile().

If that is not the case, an attempt is made to turn the string into a http:// or ftp:// URL. The latter in the case the string starts with 'ftp'. The result is then passed through QUrl's tolerant parser, and in the case or success, a valid QUrl is returned, or else a QUrl().
*/
impl /*struct*/ QUrl {
  pub fn fromUserInput_0<RetType, T: QUrl_fromUserInput_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.fromUserInput_0();
    // return 1;
  }
}
pub trait QUrl_fromUserInput_0<RetType> {
  fn fromUserInput_0(self ) -> RetType;
}
impl<'a> /*trait*/ QUrl_fromUserInput_0<usize> for (usize) {
  fn fromUserInput_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN4QUrl13fromUserInputERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qurl.h:212
// index:1
// Public static Visibility=Default Availability=Available
// [8] QUrl fromUserInput(const QString &, const QString &, QUrl::UserInputResolutionOptions)

/*
Returns a valid URL from a user supplied userInput string if one can be deducted. In the case that is not possible, an invalid QUrl() is returned.

Most applications that can browse the web, allow the user to input a URL in the form of a plain string. This string can be manually typed into a location bar, obtained from the clipboard, or passed in via command line arguments.

When the string is not already a valid URL, a best guess is performed, making various web related assumptions.

In the case the string corresponds to a valid file path on the system, a file:// URL is constructed, using QUrl::fromLocalFile().

If that is not the case, an attempt is made to turn the string into a http:// or ftp:// URL. The latter in the case the string starts with 'ftp'. The result is then passed through QUrl's tolerant parser, and in the case or success, a valid QUrl is returned, or else a QUrl().
*/
impl /*struct*/ QUrl {
  pub fn fromUserInput_1<RetType, T: QUrl_fromUserInput_1<RetType>>( overload_args: T) -> RetType {
    return overload_args.fromUserInput_1();
    // return 1;
  }
}
pub trait QUrl_fromUserInput_1<RetType> {
  fn fromUserInput_1(self ) -> RetType;
}
impl<'a> /*trait*/ QUrl_fromUserInput_1<usize> for (usize,usize,i32) {
  fn fromUserInput_1(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN4QUrl13fromUserInputERK7QStringS2_6QFlagsINS_25UserInputResolutionOptionEE", 3,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qurl.h:215
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isValid() const

/*
Returns true if the URL is non-empty and valid; otherwise returns false.

The URL is run through a conformance test. Every part of the URL must conform to the standard encoding rules of the URI standard for the URL to be reported as valid.


  bool checkUrl(const QUrl &url) {
      if (!url.isValid()) {
          qDebug("Invalid URL: %s", qUtf8Printable(url.toString()));
          return false;
      }

      return true;
  }
*/
impl /*struct*/ QUrl {
  pub fn isValid_0<RetType, T: QUrl_isValid_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isValid_0(self);
    // return 1;
  }
}
pub trait QUrl_isValid_0<RetType> {
  fn isValid_0(self , rsthis: & QUrl) -> RetType;
}
impl<'a> /*trait*/ QUrl_isValid_0<bool> for () {
  fn isValid_0(self , rsthis: & QUrl) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK4QUrl7isValidEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qurl.h:216
// index:0
// Public Visibility=Default Availability=Available
// [8] QString errorString() const

/*
Returns an error message if the last operation that modified this QUrl object ran into a parsing error. If no error was detected, this function returns an empty string and isValid() returns true.

The error message returned by this function is technical in nature and may not be understood by end users. It is mostly useful to developers trying to understand why QUrl will not accept some input.

This function was introduced in  Qt 4.2.

See also QUrl::ParsingMode.
*/
impl /*struct*/ QUrl {
  pub fn errorString_0<RetType, T: QUrl_errorString_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.errorString_0(self);
    // return 1;
  }
}
pub trait QUrl_errorString_0<RetType> {
  fn errorString_0(self , rsthis: & QUrl) -> RetType;
}
impl<'a> /*trait*/ QUrl_errorString_0<usize> for () {
  fn errorString_0(self , rsthis: & QUrl) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK4QUrl11errorStringEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qurl.h:218
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isEmpty() const

/*
Returns true if the URL has no data; otherwise returns false.

See also clear().
*/
impl /*struct*/ QUrl {
  pub fn isEmpty_0<RetType, T: QUrl_isEmpty_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isEmpty_0(self);
    // return 1;
  }
}
pub trait QUrl_isEmpty_0<RetType> {
  fn isEmpty_0(self , rsthis: & QUrl) -> RetType;
}
impl<'a> /*trait*/ QUrl_isEmpty_0<bool> for () {
  fn isEmpty_0(self , rsthis: & QUrl) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK4QUrl7isEmptyEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qurl.h:219
// index:0
// Public Visibility=Default Availability=Available
// [-2] void clear()

/*
Resets the content of the QUrl. After calling this function, the QUrl is equal to one that has been constructed with the default empty constructor.

See also isEmpty().
*/
impl /*struct*/ QUrl {
  pub fn clear_0<RetType, T: QUrl_clear_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.clear_0(self);
    // return 1;
  }
}
pub trait QUrl_clear_0<RetType> {
  fn clear_0(self , rsthis: & QUrl) -> RetType;
}
impl<'a> /*trait*/ QUrl_clear_0<(/*void*/)> for () {
  fn clear_0(self , rsthis: & QUrl) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN4QUrl5clearEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qurl.h:221
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setScheme(const QString &)

/*
Sets the scheme of the URL to scheme. As a scheme can only contain ASCII characters, no conversion or decoding is done on the input. It must also start with an ASCII letter.

The scheme describes the type (or protocol) of the URL. It's represented by one or more ASCII characters at the start the URL.

A scheme is strictly RFC 3986-compliant: scheme = ALPHA *( ALPHA / DIGIT / "+" / "-" / "." )

The following example shows a URL where the scheme is "ftp":



To set the scheme, the following call is used:


  QUrl url;
  url.setScheme("ftp");



The scheme can also be empty, in which case the URL is interpreted as relative.

See also scheme() and isRelative().
*/
impl /*struct*/ QUrl {
  pub fn setScheme_0<RetType, T: QUrl_setScheme_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setScheme_0(self);
    // return 1;
  }
}
pub trait QUrl_setScheme_0<RetType> {
  fn setScheme_0(self , rsthis: & QUrl) -> RetType;
}
impl<'a> /*trait*/ QUrl_setScheme_0<(/*void*/)> for (usize) {
  fn setScheme_0(self , rsthis: & QUrl) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN4QUrl9setSchemeERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qurl.h:222
// index:0
// Public Visibility=Default Availability=Available
// [8] QString scheme() const

/*
Returns the scheme of the URL. If an empty string is returned, this means the scheme is undefined and the URL is then relative.

The scheme can only contain US-ASCII letters or digits, which means it cannot contain any character that would otherwise require encoding. Additionally, schemes are always returned in lowercase form.

See also setScheme() and isRelative().
*/
impl /*struct*/ QUrl {
  pub fn scheme_0<RetType, T: QUrl_scheme_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.scheme_0(self);
    // return 1;
  }
}
pub trait QUrl_scheme_0<RetType> {
  fn scheme_0(self , rsthis: & QUrl) -> RetType;
}
impl<'a> /*trait*/ QUrl_scheme_0<usize> for () {
  fn scheme_0(self , rsthis: & QUrl) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK4QUrl6schemeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qurl.h:224
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setAuthority(const QString &, QUrl::ParsingMode)

/*
Sets the authority of the URL to authority.

The authority of a URL is the combination of user info, a host name and a port. All of these elements are optional; an empty authority is therefore valid.

The user info and host are separated by a '@', and the host and port are separated by a ':'. If the user info is empty, the '@' must be omitted; although a stray ':' is permitted if the port is empty.

The following example shows a valid authority string:



The authority data is interpreted according to mode: in StrictMode, any '%' characters must be followed by exactly two hexadecimal characters and some characters (including space) are not allowed in undecoded form. In TolerantMode (the default), all characters are accepted in undecoded form and the tolerant parser will correct stray '%' not followed by two hex characters.

This function does not allow mode to be QUrl::DecodedMode. To set fully decoded data, call setUserName(), setPassword(), setHost() and setPort() individually.

See also authority(), setUserInfo(), setHost(), and setPort().
*/
impl /*struct*/ QUrl {
  pub fn setAuthority_0<RetType, T: QUrl_setAuthority_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setAuthority_0(self);
    // return 1;
  }
}
pub trait QUrl_setAuthority_0<RetType> {
  fn setAuthority_0(self , rsthis: & QUrl) -> RetType;
}
impl<'a> /*trait*/ QUrl_setAuthority_0<(/*void*/)> for (usize,i32) {
  fn setAuthority_0(self , rsthis: & QUrl) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN4QUrl12setAuthorityERK7QStringNS_11ParsingModeE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qurl.h:227
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setUserInfo(const QString &, QUrl::ParsingMode)

/*
Sets the user info of the URL to userInfo. The user info is an optional part of the authority of the URL, as described in setAuthority().

The user info consists of a user name and optionally a password, separated by a ':'. If the password is empty, the colon must be omitted. The following example shows a valid user info string:



The userInfo data is interpreted according to mode: in StrictMode, any '%' characters must be followed by exactly two hexadecimal characters and some characters (including space) are not allowed in undecoded form. In TolerantMode (the default), all characters are accepted in undecoded form and the tolerant parser will correct stray '%' not followed by two hex characters.

This function does not allow mode to be QUrl::DecodedMode. To set fully decoded data, call setUserName() and setPassword() individually.

See also userInfo(), setUserName(), setPassword(), and setAuthority().
*/
impl /*struct*/ QUrl {
  pub fn setUserInfo_0<RetType, T: QUrl_setUserInfo_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setUserInfo_0(self);
    // return 1;
  }
}
pub trait QUrl_setUserInfo_0<RetType> {
  fn setUserInfo_0(self , rsthis: & QUrl) -> RetType;
}
impl<'a> /*trait*/ QUrl_setUserInfo_0<(/*void*/)> for (usize,i32) {
  fn setUserInfo_0(self , rsthis: & QUrl) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN4QUrl11setUserInfoERK7QStringNS_11ParsingModeE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qurl.h:230
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setUserName(const QString &, QUrl::ParsingMode)

/*
Sets the URL's user name to userName. The userName is part of the user info element in the authority of the URL, as described in setUserInfo().

The userName data is interpreted according to mode: in StrictMode, any '%' characters must be followed by exactly two hexadecimal characters and some characters (including space) are not allowed in undecoded form. In TolerantMode (the default), all characters are accepted in undecoded form and the tolerant parser will correct stray '%' not followed by two hex characters. In DecodedMode, '%' stand for themselves and encoded characters are not possible.

QUrl::DecodedMode should be used when setting the user name from a data source which is not a URL, such as a password dialog shown to the user or with a user name obtained by calling userName() with the QUrl::FullyDecoded formatting option.

See also userName() and setUserInfo().
*/
impl /*struct*/ QUrl {
  pub fn setUserName_0<RetType, T: QUrl_setUserName_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setUserName_0(self);
    // return 1;
  }
}
pub trait QUrl_setUserName_0<RetType> {
  fn setUserName_0(self , rsthis: & QUrl) -> RetType;
}
impl<'a> /*trait*/ QUrl_setUserName_0<(/*void*/)> for (usize,i32) {
  fn setUserName_0(self , rsthis: & QUrl) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN4QUrl11setUserNameERK7QStringNS_11ParsingModeE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qurl.h:233
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setPassword(const QString &, QUrl::ParsingMode)

/*
Sets the URL's password to password. The password is part of the user info element in the authority of the URL, as described in setUserInfo().

The password data is interpreted according to mode: in StrictMode, any '%' characters must be followed by exactly two hexadecimal characters and some characters (including space) are not allowed in undecoded form. In TolerantMode, all characters are accepted in undecoded form and the tolerant parser will correct stray '%' not followed by two hex characters. In DecodedMode, '%' stand for themselves and encoded characters are not possible.

QUrl::DecodedMode should be used when setting the password from a data source which is not a URL, such as a password dialog shown to the user or with a password obtained by calling password() with the QUrl::FullyDecoded formatting option.

See also password() and setUserInfo().
*/
impl /*struct*/ QUrl {
  pub fn setPassword_0<RetType, T: QUrl_setPassword_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setPassword_0(self);
    // return 1;
  }
}
pub trait QUrl_setPassword_0<RetType> {
  fn setPassword_0(self , rsthis: & QUrl) -> RetType;
}
impl<'a> /*trait*/ QUrl_setPassword_0<(/*void*/)> for (usize,i32) {
  fn setPassword_0(self , rsthis: & QUrl) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN4QUrl11setPasswordERK7QStringNS_11ParsingModeE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qurl.h:236
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setHost(const QString &, QUrl::ParsingMode)

/*
Sets the host of the URL to host. The host is part of the authority.

The host data is interpreted according to mode: in StrictMode, any '%' characters must be followed by exactly two hexadecimal characters and some characters (including space) are not allowed in undecoded form. In TolerantMode, all characters are accepted in undecoded form and the tolerant parser will correct stray '%' not followed by two hex characters. In DecodedMode, '%' stand for themselves and encoded characters are not possible.

Note that, in all cases, the result of the parsing must be a valid hostname according to STD 3 rules, as modified by the Internationalized Resource Identifiers specification (RFC 3987). Invalid hostnames are not permitted and will cause isValid() to become false.

See also host() and setAuthority().
*/
impl /*struct*/ QUrl {
  pub fn setHost_0<RetType, T: QUrl_setHost_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setHost_0(self);
    // return 1;
  }
}
pub trait QUrl_setHost_0<RetType> {
  fn setHost_0(self , rsthis: & QUrl) -> RetType;
}
impl<'a> /*trait*/ QUrl_setHost_0<(/*void*/)> for (usize,i32) {
  fn setHost_0(self , rsthis: & QUrl) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN4QUrl7setHostERK7QStringNS_11ParsingModeE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qurl.h:242
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setPort(int)

/*
Sets the port of the URL to port. The port is part of the authority of the URL, as described in setAuthority().

port must be between 0 and 65535 inclusive. Setting the port to -1 indicates that the port is unspecified.

See also port().
*/
impl /*struct*/ QUrl {
  pub fn setPort_0<RetType, T: QUrl_setPort_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setPort_0(self);
    // return 1;
  }
}
pub trait QUrl_setPort_0<RetType> {
  fn setPort_0(self , rsthis: & QUrl) -> RetType;
}
impl<'a> /*trait*/ QUrl_setPort_0<(/*void*/)> for (i32) {
  fn setPort_0(self , rsthis: & QUrl) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN4QUrl7setPortEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qurl.h:243
// index:0
// Public Visibility=Default Availability=Available
// [4] int port(int) const

/*
Returns the port of the URL, or defaultPort if the port is unspecified.

Example:


  QTcpSocket sock;
  sock.connectToHost(url.host(), url.port(80));



This function was introduced in  Qt 4.1.

See also setPort().
*/
impl /*struct*/ QUrl {
  pub fn port_0<RetType, T: QUrl_port_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.port_0(self);
    // return 1;
  }
}
pub trait QUrl_port_0<RetType> {
  fn port_0(self , rsthis: & QUrl) -> RetType;
}
impl<'a> /*trait*/ QUrl_port_0<i32> for (i32) {
  fn port_0(self , rsthis: & QUrl) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK4QUrl4portEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qurl.h:245
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setPath(const QString &, QUrl::ParsingMode)

/*
Sets the path of the URL to path. The path is the part of the URL that comes after the authority but before the query string.



For non-hierarchical schemes, the path will be everything following the scheme declaration, as in the following example:



The path data is interpreted according to mode: in StrictMode, any '%' characters must be followed by exactly two hexadecimal characters and some characters (including space) are not allowed in undecoded form. In TolerantMode (the default), all characters are accepted in undecoded form and the tolerant parser will correct stray '%' not followed by two hex characters. In DecodedMode, '%' stand for themselves and encoded characters are not possible.

QUrl::DecodedMode should be used when setting the path from a data source which is not a URL, such as a dialog shown to the user or with a path obtained by calling path() with the QUrl::FullyDecoded formatting option.

See also path().
*/
impl /*struct*/ QUrl {
  pub fn setPath_0<RetType, T: QUrl_setPath_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setPath_0(self);
    // return 1;
  }
}
pub trait QUrl_setPath_0<RetType> {
  fn setPath_0(self , rsthis: & QUrl) -> RetType;
}
impl<'a> /*trait*/ QUrl_setPath_0<(/*void*/)> for (usize,i32) {
  fn setPath_0(self , rsthis: & QUrl) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN4QUrl7setPathERK7QStringNS_11ParsingModeE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qurl.h:249
// index:0
// Public Visibility=Default Availability=Available
// [1] bool hasQuery() const

/*
Returns true if this URL contains a Query (i.e., if ? was seen on it).

This function was introduced in  Qt 4.2.

See also setQuery(), query(), and hasFragment().
*/
impl /*struct*/ QUrl {
  pub fn hasQuery_0<RetType, T: QUrl_hasQuery_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.hasQuery_0(self);
    // return 1;
  }
}
pub trait QUrl_hasQuery_0<RetType> {
  fn hasQuery_0(self , rsthis: & QUrl) -> RetType;
}
impl<'a> /*trait*/ QUrl_hasQuery_0<bool> for () {
  fn hasQuery_0(self , rsthis: & QUrl) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK4QUrl8hasQueryEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qurl.h:250
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setQuery(const QString &, QUrl::ParsingMode)

/*
Sets the query string of the URL to query.

This function is useful if you need to pass a query string that does not fit into the key-value pattern, or that uses a different scheme for encoding special characters than what is suggested by QUrl.

Passing a value of QString() to query (a null QString) unsets the query completely. However, passing a value of QString("") will set the query to an empty value, as if the original URL had a lone "?".

The query data is interpreted according to mode: in StrictMode, any '%' characters must be followed by exactly two hexadecimal characters and some characters (including space) are not allowed in undecoded form. In TolerantMode, all characters are accepted in undecoded form and the tolerant parser will correct stray '%' not followed by two hex characters. In DecodedMode, '%' stand for themselves and encoded characters are not possible.

Query strings often contain percent-encoded sequences, so use of DecodedMode is discouraged. One special sequence to be aware of is that of the plus character ('+'). QUrl does not convert spaces to plus characters, even though HTML forms posted by web browsers do. In order to represent an actual plus character in a query, the sequence "%2B" is usually used. This function will leave "%2B" sequences untouched in TolerantMode or StrictMode.

See also query() and hasQuery().
*/
impl /*struct*/ QUrl {
  pub fn setQuery_0<RetType, T: QUrl_setQuery_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setQuery_0(self);
    // return 1;
  }
}
pub trait QUrl_setQuery_0<RetType> {
  fn setQuery_0(self , rsthis: & QUrl) -> RetType;
}
impl<'a> /*trait*/ QUrl_setQuery_0<(/*void*/)> for (usize,i32) {
  fn setQuery_0(self , rsthis: & QUrl) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN4QUrl8setQueryERK7QStringNS_11ParsingModeE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qurl.h:251
// index:1
// Public Visibility=Default Availability=Available
// [-2] void setQuery(const QUrlQuery &)

/*
Sets the query string of the URL to query.

This function is useful if you need to pass a query string that does not fit into the key-value pattern, or that uses a different scheme for encoding special characters than what is suggested by QUrl.

Passing a value of QString() to query (a null QString) unsets the query completely. However, passing a value of QString("") will set the query to an empty value, as if the original URL had a lone "?".

The query data is interpreted according to mode: in StrictMode, any '%' characters must be followed by exactly two hexadecimal characters and some characters (including space) are not allowed in undecoded form. In TolerantMode, all characters are accepted in undecoded form and the tolerant parser will correct stray '%' not followed by two hex characters. In DecodedMode, '%' stand for themselves and encoded characters are not possible.

Query strings often contain percent-encoded sequences, so use of DecodedMode is discouraged. One special sequence to be aware of is that of the plus character ('+'). QUrl does not convert spaces to plus characters, even though HTML forms posted by web browsers do. In order to represent an actual plus character in a query, the sequence "%2B" is usually used. This function will leave "%2B" sequences untouched in TolerantMode or StrictMode.

See also query() and hasQuery().
*/
impl /*struct*/ QUrl {
  pub fn setQuery_1<RetType, T: QUrl_setQuery_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setQuery_1(self);
    // return 1;
  }
}
pub trait QUrl_setQuery_1<RetType> {
  fn setQuery_1(self , rsthis: & QUrl) -> RetType;
}
impl<'a> /*trait*/ QUrl_setQuery_1<(/*void*/)> for (usize) {
  fn setQuery_1(self , rsthis: & QUrl) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN4QUrl8setQueryERK9QUrlQuery", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qurl.h:254
// index:0
// Public Visibility=Default Availability=Available
// [1] bool hasFragment() const

/*
Returns true if this URL contains a fragment (i.e., if # was seen on it).

This function was introduced in  Qt 4.2.

See also fragment() and setFragment().
*/
impl /*struct*/ QUrl {
  pub fn hasFragment_0<RetType, T: QUrl_hasFragment_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.hasFragment_0(self);
    // return 1;
  }
}
pub trait QUrl_hasFragment_0<RetType> {
  fn hasFragment_0(self , rsthis: & QUrl) -> RetType;
}
impl<'a> /*trait*/ QUrl_hasFragment_0<bool> for () {
  fn hasFragment_0(self , rsthis: & QUrl) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK4QUrl11hasFragmentEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qurl.h:256
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setFragment(const QString &, QUrl::ParsingMode)

/*
Sets the fragment of the URL to fragment. The fragment is the last part of the URL, represented by a '#' followed by a string of characters. It is typically used in HTTP for referring to a certain link or point on a page:



The fragment is sometimes also referred to as the URL "reference".

Passing an argument of QString() (a null QString) will unset the fragment. Passing an argument of QString("") (an empty but not null QString) will set the fragment to an empty string (as if the original URL had a lone "#").

The fragment data is interpreted according to mode: in StrictMode, any '%' characters must be followed by exactly two hexadecimal characters and some characters (including space) are not allowed in undecoded form. In TolerantMode, all characters are accepted in undecoded form and the tolerant parser will correct stray '%' not followed by two hex characters. In DecodedMode, '%' stand for themselves and encoded characters are not possible.

QUrl::DecodedMode should be used when setting the fragment from a data source which is not a URL or with a fragment obtained by calling fragment() with the QUrl::FullyDecoded formatting option.

See also fragment() and hasFragment().
*/
impl /*struct*/ QUrl {
  pub fn setFragment_0<RetType, T: QUrl_setFragment_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setFragment_0(self);
    // return 1;
  }
}
pub trait QUrl_setFragment_0<RetType> {
  fn setFragment_0(self , rsthis: & QUrl) -> RetType;
}
impl<'a> /*trait*/ QUrl_setFragment_0<(/*void*/)> for (usize,i32) {
  fn setFragment_0(self , rsthis: & QUrl) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN4QUrl11setFragmentERK7QStringNS_11ParsingModeE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qurl.h:258
// index:0
// Public Visibility=Default Availability=Available
// [8] QUrl resolved(const QUrl &) const

/*
Returns the result of the merge of this URL with relative. This URL is used as a base to convert relative to an absolute URL.

If relative is not a relative URL, this function will return relative directly. Otherwise, the paths of the two URLs are merged, and the new URL returned has the scheme and authority of the base URL, but with the merged path, as in the following example:


  QUrl baseUrl("http://qt.digia.com/Support/");
  QUrl relativeUrl("../Product/Library/");
  qDebug(baseUrl.resolved(relativeUrl).toString());
  // prints "http://qt.digia.com/Product/Library/"



Calling resolved() with ".." returns a QUrl whose directory is one level higher than the original. Similarly, calling resolved() with "../.." removes two levels from the path. If relative is "/", the path becomes "/".

See also isRelative().
*/
impl /*struct*/ QUrl {
  pub fn resolved_0<RetType, T: QUrl_resolved_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.resolved_0(self);
    // return 1;
  }
}
pub trait QUrl_resolved_0<RetType> {
  fn resolved_0(self , rsthis: & QUrl) -> RetType;
}
impl<'a> /*trait*/ QUrl_resolved_0<usize> for (usize) {
  fn resolved_0(self , rsthis: & QUrl) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK4QUrl8resolvedERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qurl.h:260
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isRelative() const

/*
Returns true if the URL is relative; otherwise returns false. A URL is relative reference if its scheme is undefined; this function is therefore equivalent to calling scheme().isEmpty().

Relative references are defined in RFC 3986 section 4.2.

See also Relative URLs vs Relative Paths.
*/
impl /*struct*/ QUrl {
  pub fn isRelative_0<RetType, T: QUrl_isRelative_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isRelative_0(self);
    // return 1;
  }
}
pub trait QUrl_isRelative_0<RetType> {
  fn isRelative_0(self , rsthis: & QUrl) -> RetType;
}
impl<'a> /*trait*/ QUrl_isRelative_0<bool> for () {
  fn isRelative_0(self , rsthis: & QUrl) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK4QUrl10isRelativeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qurl.h:261
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isParentOf(const QUrl &) const

/*
Returns true if this URL is a parent of childUrl. childUrl is a child of this URL if the two URLs share the same scheme and authority, and this URL's path is a parent of the path of childUrl.
*/
impl /*struct*/ QUrl {
  pub fn isParentOf_0<RetType, T: QUrl_isParentOf_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isParentOf_0(self);
    // return 1;
  }
}
pub trait QUrl_isParentOf_0<RetType> {
  fn isParentOf_0(self , rsthis: & QUrl) -> RetType;
}
impl<'a> /*trait*/ QUrl_isParentOf_0<bool> for (usize) {
  fn isParentOf_0(self , rsthis: & QUrl) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK4QUrl10isParentOfERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qurl.h:263
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isLocalFile() const

/*
Returns true if this URL is pointing to a local file path. A URL is a local file path if the scheme is "file".

Note that this function considers URLs with hostnames to be local file paths, even if the eventual file path cannot be opened with QFile::open().

This function was introduced in  Qt 4.8.

See also fromLocalFile() and toLocalFile().
*/
impl /*struct*/ QUrl {
  pub fn isLocalFile_0<RetType, T: QUrl_isLocalFile_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isLocalFile_0(self);
    // return 1;
  }
}
pub trait QUrl_isLocalFile_0<RetType> {
  fn isLocalFile_0(self , rsthis: & QUrl) -> RetType;
}
impl<'a> /*trait*/ QUrl_isLocalFile_0<bool> for () {
  fn isLocalFile_0(self , rsthis: & QUrl) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK4QUrl11isLocalFileEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qurl.h:264
// index:0
// Public static Visibility=Default Availability=Available
// [8] QUrl fromLocalFile(const QString &)

/*
Returns a QUrl representation of localFile, interpreted as a local file. This function accepts paths separated by slashes as well as the native separator for this platform.

This function also accepts paths with a doubled leading slash (or backslash) to indicate a remote file, as in "//servername/path/to/file.txt". Note that only certain platforms can actually open this file using QFile::open().

An empty localFile leads to an empty URL (since Qt 5.4).


  qDebug() << QUrl::fromLocalFile("file.txt");            // QUrl("file:file.txt")
  qDebug() << QUrl::fromLocalFile("/home/user/file.txt"); // QUrl("file:///home/user/file.txt")
  qDebug() << QUrl::fromLocalFile("file:file.txt");       // doesn't make sense; expects path, not url with scheme



In the first line in snippet above, a file URL is constructed from a local, relative path. A file URL with a relative path only makes sense if there is a base URL to resolve it against. For example:


  QUrl url = QUrl::fromLocalFile("file.txt");
  QUrl baseUrl = QUrl("file:/home/user/");
  // wrong: prints QUrl("file:file.txt"), as url already has a scheme
  qDebug() << baseUrl.resolved(url);



To resolve such a URL, it's necessary to remove the scheme beforehand:


  // correct: prints QUrl("file:///home/user/file.txt")
  url.setScheme(QString());
  qDebug() << baseUrl.resolved(url);



For this reason, it is better to use a relative URL (that is, no scheme) for relative file paths:


  QUrl url = QUrl("file.txt");
  QUrl baseUrl = QUrl("file:/home/user/");
  // prints QUrl("file:///home/user/file.txt")
  qDebug() << baseUrl.resolved(url);



See also toLocalFile(), isLocalFile(), and QDir::toNativeSeparators().
*/
impl /*struct*/ QUrl {
  pub fn fromLocalFile_0<RetType, T: QUrl_fromLocalFile_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.fromLocalFile_0();
    // return 1;
  }
}
pub trait QUrl_fromLocalFile_0<RetType> {
  fn fromLocalFile_0(self ) -> RetType;
}
impl<'a> /*trait*/ QUrl_fromLocalFile_0<usize> for (usize) {
  fn fromLocalFile_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN4QUrl13fromLocalFileERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qurl.h:265
// index:0
// Public Visibility=Default Availability=Available
// [8] QString toLocalFile() const

/*
Returns the path of this URL formatted as a local file path. The path returned will use forward slashes, even if it was originally created from one with backslashes.

If this URL contains a non-empty hostname, it will be encoded in the returned value in the form found on SMB networks (for example, "//servername/path/to/file.txt").


  qDebug() << QUrl("file:file.txt").toLocalFile();            // "file:file.txt"
  qDebug() << QUrl("file:/home/user/file.txt").toLocalFile(); // "file:///home/user/file.txt"
  qDebug() << QUrl("file.txt").toLocalFile();                 // ""; wasn't a local file as it had no scheme



Note: if the path component of this URL contains a non-UTF-8 binary sequence (such as %80), the behaviour of this function is undefined.

See also fromLocalFile() and isLocalFile().
*/
impl /*struct*/ QUrl {
  pub fn toLocalFile_0<RetType, T: QUrl_toLocalFile_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toLocalFile_0(self);
    // return 1;
  }
}
pub trait QUrl_toLocalFile_0<RetType> {
  fn toLocalFile_0(self , rsthis: & QUrl) -> RetType;
}
impl<'a> /*trait*/ QUrl_toLocalFile_0<usize> for () {
  fn toLocalFile_0(self , rsthis: & QUrl) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK4QUrl11toLocalFileEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qurl.h:267
// index:0
// Public Visibility=Default Availability=Available
// [-2] void detach()

/*

*/
impl /*struct*/ QUrl {
  pub fn detach_0<RetType, T: QUrl_detach_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.detach_0(self);
    // return 1;
  }
}
pub trait QUrl_detach_0<RetType> {
  fn detach_0(self , rsthis: & QUrl) -> RetType;
}
impl<'a> /*trait*/ QUrl_detach_0<(/*void*/)> for () {
  fn detach_0(self , rsthis: & QUrl) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN4QUrl6detachEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qurl.h:268
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isDetached() const

/*

*/
impl /*struct*/ QUrl {
  pub fn isDetached_0<RetType, T: QUrl_isDetached_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isDetached_0(self);
    // return 1;
  }
}
pub trait QUrl_isDetached_0<RetType> {
  fn isDetached_0(self , rsthis: & QUrl) -> RetType;
}
impl<'a> /*trait*/ QUrl_isDetached_0<bool> for () {
  fn isDetached_0(self , rsthis: & QUrl) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK4QUrl10isDetachedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qurl.h:270
// index:0
// Public Visibility=Default Availability=Available
// [1] bool operator<(const QUrl &) const

/*

*/
impl /*struct*/ QUrl {
  pub fn operator_less_than_0<RetType, T: QUrl_operator_less_than_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_less_than_0(self);
    // return 1;
  }
}
pub trait QUrl_operator_less_than_0<RetType> {
  fn operator_less_than_0(self , rsthis: & QUrl) -> RetType;
}
impl<'a> /*trait*/ QUrl_operator_less_than_0<bool> for (usize) {
  fn operator_less_than_0(self , rsthis: & QUrl) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK4QUrlltERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qurl.h:271
// index:0
// Public Visibility=Default Availability=Available
// [1] bool operator==(const QUrl &) const

/*

*/
impl /*struct*/ QUrl {
  pub fn operator_equal_equal_0<RetType, T: QUrl_operator_equal_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_equal_0(self);
    // return 1;
  }
}
pub trait QUrl_operator_equal_equal_0<RetType> {
  fn operator_equal_equal_0(self , rsthis: & QUrl) -> RetType;
}
impl<'a> /*trait*/ QUrl_operator_equal_equal_0<bool> for (usize) {
  fn operator_equal_equal_0(self , rsthis: & QUrl) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK4QUrleqERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qurl.h:272
// index:0
// Public Visibility=Default Availability=Available
// [1] bool operator!=(const QUrl &) const

/*

*/
impl /*struct*/ QUrl {
  pub fn operator_not_equal_0<RetType, T: QUrl_operator_not_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_not_equal_0(self);
    // return 1;
  }
}
pub trait QUrl_operator_not_equal_0<RetType> {
  fn operator_not_equal_0(self , rsthis: & QUrl) -> RetType;
}
impl<'a> /*trait*/ QUrl_operator_not_equal_0<bool> for (usize) {
  fn operator_not_equal_0(self , rsthis: & QUrl) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK4QUrlneERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qurl.h:276
// index:0
// Public static Visibility=Default Availability=Available
// [8] QString fromPercentEncoding(const QByteArray &)

/*
Returns a decoded copy of input. input is first decoded from percent encoding, then converted from UTF-8 to unicode.

Note: Given invalid input (such as a string containing the sequence "%G5", which is not a valid hexadecimal number) the output will be invalid as well. As an example: the sequence "%G5" could be decoded to 'W'.
*/
impl /*struct*/ QUrl {
  pub fn fromPercentEncoding_0<RetType, T: QUrl_fromPercentEncoding_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.fromPercentEncoding_0();
    // return 1;
  }
}
pub trait QUrl_fromPercentEncoding_0<RetType> {
  fn fromPercentEncoding_0(self ) -> RetType;
}
impl<'a> /*trait*/ QUrl_fromPercentEncoding_0<usize> for (usize) {
  fn fromPercentEncoding_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN4QUrl19fromPercentEncodingERK10QByteArray", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qurl.h:277
// index:0
// Public static Visibility=Default Availability=Available
// [8] QByteArray toPercentEncoding(const QString &, const QByteArray &, const QByteArray &)

/*
Returns an encoded copy of input. input is first converted to UTF-8, and all ASCII-characters that are not in the unreserved group are percent encoded. To prevent characters from being percent encoded pass them to exclude. To force characters to be percent encoded pass them to include.

Unreserved is defined as: ALPHA / DIGIT / "-" / "." / "_" / "~"


  QByteArray ba = QUrl::toPercentEncoding("{a fishy string?}", "{}", "s");
  qDebug(ba.constData());
  // prints "{a fi%73hy %73tring%3F}"
*/
impl /*struct*/ QUrl {
  pub fn toPercentEncoding_0<RetType, T: QUrl_toPercentEncoding_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.toPercentEncoding_0();
    // return 1;
  }
}
pub trait QUrl_toPercentEncoding_0<RetType> {
  fn toPercentEncoding_0(self ) -> RetType;
}
impl<'a> /*trait*/ QUrl_toPercentEncoding_0<usize> for (usize,usize,usize) {
  fn toPercentEncoding_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN4QUrl17toPercentEncodingERK7QStringRK10QByteArrayS5_", 3,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qurl.h:357
// index:0
// Public static Visibility=Default Availability=Available
// [8] QString fromAce(const QByteArray &)

/*
Returns the Unicode form of the given domain name domain, which is encoded in the ASCII Compatible Encoding (ACE). The result of this function is considered equivalent to domain.

If the value in domain cannot be encoded, it will be converted to QString and returned.

The ASCII Compatible Encoding (ACE) is defined by RFC 3490, RFC 3491 and RFC 3492. It is part of the Internationalizing Domain Names in Applications (IDNA) specification, which allows for domain names (like "example.com") to be written using international characters.

This function was introduced in  Qt 4.2.
*/
impl /*struct*/ QUrl {
  pub fn fromAce_0<RetType, T: QUrl_fromAce_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.fromAce_0();
    // return 1;
  }
}
pub trait QUrl_fromAce_0<RetType> {
  fn fromAce_0(self ) -> RetType;
}
impl<'a> /*trait*/ QUrl_fromAce_0<usize> for (usize) {
  fn fromAce_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN4QUrl7fromAceERK10QByteArray", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qurl.h:358
// index:0
// Public static Visibility=Default Availability=Available
// [8] QByteArray toAce(const QString &)

/*
Returns the ASCII Compatible Encoding of the given domain name domain. The result of this function is considered equivalent to domain.

The ASCII-Compatible Encoding (ACE) is defined by RFC 3490, RFC 3491 and RFC 3492. It is part of the Internationalizing Domain Names in Applications (IDNA) specification, which allows for domain names (like "example.com") to be written using international characters.

This function returns an empty QByteArray if domain is not a valid hostname. Note, in particular, that IPv6 literals are not valid domain names.

This function was introduced in  Qt 4.2.
*/
impl /*struct*/ QUrl {
  pub fn toAce_0<RetType, T: QUrl_toAce_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.toAce_0();
    // return 1;
  }
}
pub trait QUrl_toAce_0<RetType> {
  fn toAce_0(self ) -> RetType;
}
impl<'a> /*trait*/ QUrl_toAce_0<usize> for (usize) {
  fn toAce_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN4QUrl5toAceERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qurl.h:359
// index:0
// Public static Visibility=Default Availability=Available
// [8] QStringList idnWhitelist()

/*
Returns the current whitelist of top-level domains that are allowed to have non-ASCII characters in their compositions.

See setIdnWhitelist() for the rationale of this list.

This function was introduced in  Qt 4.2.

See also setIdnWhitelist().
*/
impl /*struct*/ QUrl {
  pub fn idnWhitelist_0<RetType, T: QUrl_idnWhitelist_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.idnWhitelist_0();
    // return 1;
  }
}
pub trait QUrl_idnWhitelist_0<RetType> {
  fn idnWhitelist_0(self ) -> RetType;
}
impl<'a> /*trait*/ QUrl_idnWhitelist_0<usize> for () {
  fn idnWhitelist_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN4QUrl12idnWhitelistEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qurl.h:361
// index:0
// Public static Visibility=Default Availability=Available
// [-2] QList<QUrl> fromStringList(const QStringList &, QUrl::ParsingMode)

/*
Converts a list of strings representing urls into a list of urls, using QUrl(str, mode). Note that this means all strings must be urls, not for instance local paths.

This function was introduced in  Qt 5.1.
*/
impl /*struct*/ QUrl {
  pub fn fromStringList_0<RetType, T: QUrl_fromStringList_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.fromStringList_0();
    // return 1;
  }
}
pub trait QUrl_fromStringList_0<RetType> {
  fn fromStringList_0(self ) -> RetType;
}
impl<'a> /*trait*/ QUrl_fromStringList_0<usize> for (usize,i32) {
  fn fromStringList_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN4QUrl14fromStringListERK11QStringListNS_11ParsingModeE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qurl.h:363
// index:0
// Public static Visibility=Default Availability=Available
// [-2] void setIdnWhitelist(const QStringList &)

/*
Sets the whitelist of Top-Level Domains (TLDs) that are allowed to have non-ASCII characters in domains to the value of list.

Note that if you call this function, you need to do so before you start any threads that might access idnWhitelist().

Qt comes with a default list that contains the Internet top-level domains that have published support for Internationalized Domain Names (IDNs) and rules to guarantee that no deception can happen between similarly-looking characters (such as the Latin lowercase letter 'a' and the Cyrillic equivalent, which in most fonts are visually identical).

This list is periodically maintained, as registrars publish new rules.

This function is provided for those who need to manipulate the list, in order to add or remove a TLD. It is not recommended to change its value for purposes other than testing, as it may expose users to security risks.

This function was introduced in  Qt 4.2.

See also idnWhitelist().
*/
impl /*struct*/ QUrl {
  pub fn setIdnWhitelist_0<RetType, T: QUrl_setIdnWhitelist_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.setIdnWhitelist_0();
    // return 1;
  }
}
pub trait QUrl_setIdnWhitelist_0<RetType> {
  fn setIdnWhitelist_0(self ) -> RetType;
}
impl<'a> /*trait*/ QUrl_setIdnWhitelist_0<(/*void*/)> for (usize) {
  fn setIdnWhitelist_0(self ) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN4QUrl15setIdnWhitelistERK11QStringList", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}


/*
The parsing mode controls the way QUrl parses strings.



In TolerantMode, the parser has the following behaviour:


Spaces and "%20": unencoded space characters will be accepted and will be treated as equivalent to "%20".
Single "%" characters: Any occurrences of a percent character "%" not followed by exactly two hexadecimal characters (e.g., "13% coverage.html") will be replaced by "%25". Note that one lone "%" character will trigger the correction mode for all percent characters.
Reserved and unreserved characters: An encoded URL should only contain a few characters as literals; all other characters should be percent-encoded. In TolerantMode, these characters will be accepted if they are found in the URL: space / double-quote / "<" / ">" / "" / "^" / "`" / "{" / "|" / "}" Those same characters can be decoded again by passing QUrl::DecodeReserved to toString() or toEncoded(). In the getters of individual components, those characters are often returned in decoded form.


When in StrictMode, if a parsing error is found, isValid() will return false and errorString() will return a message describing the error. If more than one error is detected, it is undefined which error gets reported.

Note that TolerantMode is not usually enough for parsing user input, which often contains more errors and expectations than the parser can deal with. When dealing with data coming directly from the user -- as opposed to data coming from data-transfer sources, such as other programs -- it is recommended to use fromUserInput().

See also fromUserInput(), setUrl(), toString(), toEncoded(), and QUrl::FormattingOptions.

*/
pub type QUrl__ParsingMode = i32;
// QUrl will try to correct some common errors in URLs. This mode is useful for parsing URLs coming from sources not known to be strictly standards-conforming.
pub const QUrl__TolerantMode :QUrl__ParsingMode = 0;
// Only valid URLs are accepted. This mode is useful for general URL validation.
pub const QUrl__StrictMode :QUrl__ParsingMode = 1;
// QUrl will interpret the URL component in the fully-decoded form, where percent characters stand for themselves, not as the beginning of a percent-encoded sequence. This mode is only valid for the setters setting components of a URL; it is not permitted in the QUrl constructor, in fromEncoded() or in setUrl(). For more information on this mode, see the documentation for QUrl::FullyDecoded.
pub const QUrl__DecodedMode :QUrl__ParsingMode = 2;
pub fn QUrl_ParsingModeItemName(val: i32) ->String {
  match val {
     QUrl__TolerantMode => // 0
     {return String::from("TolerantMode");}
     QUrl__StrictMode => // 1
     {return String::from("StrictMode");}
     QUrl__DecodedMode => // 2
     {return String::from("DecodedMode");}
  _ => {return format!("{}", val);}
}
}
pub fn QUrl_ParsingModeItemName_s(val: i32) ->String {
  //var nilthis *QUrl
  //return nilthis.ParsingModeItemName(val);
  return QUrl_ParsingModeItemName(val);
}


/*


*/
pub type QUrl__UrlFormattingOption = i32;
// 
pub const QUrl__None :QUrl__UrlFormattingOption = 0;
// 
pub const QUrl__RemoveScheme :QUrl__UrlFormattingOption = 1;
// 
pub const QUrl__RemovePassword :QUrl__UrlFormattingOption = 2;
// 
pub const QUrl__RemoveUserInfo :QUrl__UrlFormattingOption = 6;
// 
pub const QUrl__RemovePort :QUrl__UrlFormattingOption = 8;
// 
pub const QUrl__RemoveAuthority :QUrl__UrlFormattingOption = 30;
// 
pub const QUrl__RemovePath :QUrl__UrlFormattingOption = 32;
// 
pub const QUrl__RemoveQuery :QUrl__UrlFormattingOption = 64;
// 
pub const QUrl__RemoveFragment :QUrl__UrlFormattingOption = 128;
// 
pub const QUrl__PreferLocalFile :QUrl__UrlFormattingOption = 512;
// 
pub const QUrl__StripTrailingSlash :QUrl__UrlFormattingOption = 1024;
// 
pub const QUrl__RemoveFilename :QUrl__UrlFormattingOption = 2048;
// 
pub const QUrl__NormalizePathSegments :QUrl__UrlFormattingOption = 4096;
pub fn QUrl_UrlFormattingOptionItemName(val: i32) ->String {
  match val {
     QUrl__None => // 0
     {return String::from("None");}
     QUrl__RemoveScheme => // 1
     {return String::from("RemoveScheme");}
     QUrl__RemovePassword => // 2
     {return String::from("RemovePassword");}
     QUrl__RemoveUserInfo => // 6
     {return String::from("RemoveUserInfo");}
     QUrl__RemovePort => // 8
     {return String::from("RemovePort");}
     QUrl__RemoveAuthority => // 30
     {return String::from("RemoveAuthority");}
     QUrl__RemovePath => // 32
     {return String::from("RemovePath");}
     QUrl__RemoveQuery => // 64
     {return String::from("RemoveQuery");}
     QUrl__RemoveFragment => // 128
     {return String::from("RemoveFragment");}
     QUrl__PreferLocalFile => // 512
     {return String::from("PreferLocalFile");}
     QUrl__StripTrailingSlash => // 1024
     {return String::from("StripTrailingSlash");}
     QUrl__RemoveFilename => // 2048
     {return String::from("RemoveFilename");}
     QUrl__NormalizePathSegments => // 4096
     {return String::from("NormalizePathSegments");}
  _ => {return format!("{}", val);}
}
}
pub fn QUrl_UrlFormattingOptionItemName_s(val: i32) ->String {
  //var nilthis *QUrl
  //return nilthis.UrlFormattingOptionItemName(val);
  return QUrl_UrlFormattingOptionItemName(val);
}


/*


*/
pub type QUrl__ComponentFormattingOption = i32;
// 
pub const QUrl__PrettyDecoded :QUrl__ComponentFormattingOption = 0;
// 
pub const QUrl__EncodeSpaces :QUrl__ComponentFormattingOption = 1048576;
// 
pub const QUrl__EncodeUnicode :QUrl__ComponentFormattingOption = 2097152;
// 
pub const QUrl__EncodeDelimiters :QUrl__ComponentFormattingOption = 12582912;
// 
pub const QUrl__EncodeReserved :QUrl__ComponentFormattingOption = 16777216;
// 
pub const QUrl__DecodeReserved :QUrl__ComponentFormattingOption = 33554432;
// 
pub const QUrl__FullyEncoded :QUrl__ComponentFormattingOption = 32505856;
// 
pub const QUrl__FullyDecoded :QUrl__ComponentFormattingOption = 133169152;
pub fn QUrl_ComponentFormattingOptionItemName(val: i32) ->String {
  match val {
     QUrl__PrettyDecoded => // 0
     {return String::from("PrettyDecoded");}
     QUrl__EncodeSpaces => // 1048576
     {return String::from("EncodeSpaces");}
     QUrl__EncodeUnicode => // 2097152
     {return String::from("EncodeUnicode");}
     QUrl__EncodeDelimiters => // 12582912
     {return String::from("EncodeDelimiters");}
     QUrl__EncodeReserved => // 16777216
     {return String::from("EncodeReserved");}
     QUrl__DecodeReserved => // 33554432
     {return String::from("DecodeReserved");}
     QUrl__FullyEncoded => // 32505856
     {return String::from("FullyEncoded");}
     QUrl__FullyDecoded => // 133169152
     {return String::from("FullyDecoded");}
  _ => {return format!("{}", val);}
}
}
pub fn QUrl_ComponentFormattingOptionItemName_s(val: i32) ->String {
  //var nilthis *QUrl
  //return nilthis.ComponentFormattingOptionItemName(val);
  return QUrl_ComponentFormattingOptionItemName(val);
}


/*


*/
pub type QUrl__UserInputResolutionOption = i32;
// 
pub const QUrl__DefaultResolution :QUrl__UserInputResolutionOption = 0;
// 
pub const QUrl__AssumeLocalFile :QUrl__UserInputResolutionOption = 1;
pub fn QUrl_UserInputResolutionOptionItemName(val: i32) ->String {
  match val {
     QUrl__DefaultResolution => // 0
     {return String::from("DefaultResolution");}
     QUrl__AssumeLocalFile => // 1
     {return String::from("AssumeLocalFile");}
  _ => {return format!("{}", val);}
}
}
pub fn QUrl_UserInputResolutionOptionItemName_s(val: i32) ->String {
  //var nilthis *QUrl
  //return nilthis.UserInputResolutionOptionItemName(val);
  return QUrl_UserInputResolutionOptionItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
