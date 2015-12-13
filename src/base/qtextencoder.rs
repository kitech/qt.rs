// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qstring::QString;
use super::qtextcodec::QTextCodec;
use super::qchar::QChar;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  fn _ZN12QTextEncoderD0Ev() -> i32;
  fn _ZN12QTextEncoder11fromUnicodeERK7QString(arg0: *const c_void) -> i32;
  fn _ZNK12QTextEncoder10hasFailureEv() -> i32;
  fn _ZN12QTextEncoderC1EPK10QTextCodec(qthis: *mut c_void, arg0: *const c_void) -> i32;
  fn _ZN12QTextEncoderC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  fn _ZN12QTextEncoder11fromUnicodeEPK5QChari(arg0: *const c_void, arg1: c_int) -> i32;
}

// body block begin
// class sizeof(QTextEncoder)=1
pub struct QTextEncoder {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QTextEncoder {
  pub fn FreeQTextEncoder<T: QTextEncoder_FreeQTextEncoder>(&mut self, value: T) -> i32 {
    value.FreeQTextEncoder(self);
    return 1;
  }
}

pub trait QTextEncoder_FreeQTextEncoder {
  fn FreeQTextEncoder(self, this: &mut QTextEncoder) -> i32;
}

// proto: void QTextEncoder::FreeQTextEncoder();
impl<'a> /*trait*/ QTextEncoder_FreeQTextEncoder for () {
  fn FreeQTextEncoder(self, this: &mut QTextEncoder) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTextEncoderD0Ev()};
    unsafe {_ZN12QTextEncoderD0Ev()};
    return 1;
  }
}

impl /*struct*/ QTextEncoder {
  pub fn fromUnicode<T: QTextEncoder_fromUnicode>(&mut self, value: T) -> i32 {
    value.fromUnicode(self);
    return 1;
  }
}

pub trait QTextEncoder_fromUnicode {
  fn fromUnicode(self, this: &mut QTextEncoder) -> i32;
}

// proto: QByteArray QTextEncoder::fromUnicode(const QString & str);
impl<'a> /*trait*/ QTextEncoder_fromUnicode for (&'a  QString) {
  fn fromUnicode(self, this: &mut QTextEncoder) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTextEncoder11fromUnicodeERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN12QTextEncoder11fromUnicodeERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextEncoder {
  pub fn hasFailure<T: QTextEncoder_hasFailure>(&mut self, value: T) -> i32 {
    value.hasFailure(self);
    return 1;
  }
}

pub trait QTextEncoder_hasFailure {
  fn hasFailure(self, this: &mut QTextEncoder) -> i32;
}

// proto: bool QTextEncoder::hasFailure();
impl<'a> /*trait*/ QTextEncoder_hasFailure for () {
  fn hasFailure(self, this: &mut QTextEncoder) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTextEncoder10hasFailureEv()};
    unsafe {_ZNK12QTextEncoder10hasFailureEv()};
    return 1;
  }
}

impl /*struct*/ QTextEncoder {
  pub fn NewQTextEncoder<T: QTextEncoder_NewQTextEncoder>(value: T) -> QTextEncoder {
    let rsthis = value.NewQTextEncoder();
    return rsthis;
    // return 1;
  }
}

pub trait QTextEncoder_NewQTextEncoder {
  fn NewQTextEncoder(self) -> QTextEncoder;
}

// proto: void QTextEncoder::NewQTextEncoder(const QTextCodec * codec);
impl<'a> /*trait*/ QTextEncoder_NewQTextEncoder for (&'a  QTextCodec) {
  fn NewQTextEncoder(self) -> QTextEncoder {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTextEncoderC1EPK10QTextCodec()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN12QTextEncoderC1EPK10QTextCodec(qthis, arg0)};
    let rsthis = QTextEncoder{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: void QTextEncoder::NewQTextEncoder(const QTextEncoder & );
impl<'a> /*trait*/ QTextEncoder_NewQTextEncoder for (&'a  QTextEncoder) {
  fn NewQTextEncoder(self) -> QTextEncoder {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTextEncoderC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN12QTextEncoderC1ERKS_(qthis, arg0)};
    let rsthis = QTextEncoder{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: QByteArray QTextEncoder::fromUnicode(const QChar * uc, int len);
impl<'a> /*trait*/ QTextEncoder_fromUnicode for (&'a  QChar, i32) {
  fn fromUnicode(self, this: &mut QTextEncoder) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTextEncoder11fromUnicodeEPK5QChari()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1  as c_int;
    unsafe {_ZN12QTextEncoder11fromUnicodeEPK5QChari(arg0, arg1)};
    return 1;
  }
}

