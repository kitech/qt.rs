// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qtextcodec::QTextCodec;
use super::qbytearray::QByteArray;
use super::qstring::QString;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  fn _ZN12QTextDecoder9toUnicodeEPKci(arg0: *const c_char, arg1: c_int) -> i32;
  fn _ZN12QTextDecoderC1EPK10QTextCodec(qthis: *mut c_void, arg0: *const c_void) -> i32;
  fn _ZNK12QTextDecoder10hasFailureEv() -> i32;
  fn _ZN12QTextDecoderD0Ev() -> i32;
  fn _ZN12QTextDecoder9toUnicodeERK10QByteArray(arg0: *const c_void) -> i32;
  fn _ZN12QTextDecoderC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  fn _ZN12QTextDecoder9toUnicodeEP7QStringPKci(arg0: *mut c_void, arg1: *const c_char, arg2: c_int) -> i32;
}

// body block begin
// class sizeof(QTextDecoder)=1
pub struct QTextDecoder {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QTextDecoder {
  pub fn toUnicode<T: QTextDecoder_toUnicode>(&mut self, value: T) -> i32 {
    value.toUnicode(self);
    return 1;
  }
}

pub trait QTextDecoder_toUnicode {
  fn toUnicode(self, this: &mut QTextDecoder) -> i32;
}

// proto: QString QTextDecoder::toUnicode(const char * chars, int len);
impl<'a> /*trait*/ QTextDecoder_toUnicode for (&'a  String, i32) {
  fn toUnicode(self, this: &mut QTextDecoder) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTextDecoder9toUnicodeEPKci()};
    let arg0 = self.0.as_ptr()  as *const c_char;
    let arg1 = self.1  as c_int;
    unsafe {_ZN12QTextDecoder9toUnicodeEPKci(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QTextDecoder {
  pub fn NewQTextDecoder<T: QTextDecoder_NewQTextDecoder>(value: T) -> QTextDecoder {
    let rsthis = value.NewQTextDecoder();
    return rsthis;
    // return 1;
  }
}

pub trait QTextDecoder_NewQTextDecoder {
  fn NewQTextDecoder(self) -> QTextDecoder;
}

// proto: void QTextDecoder::NewQTextDecoder(const QTextCodec * codec);
impl<'a> /*trait*/ QTextDecoder_NewQTextDecoder for (&'a  QTextCodec) {
  fn NewQTextDecoder(self) -> QTextDecoder {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTextDecoderC1EPK10QTextCodec()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN12QTextDecoderC1EPK10QTextCodec(qthis, arg0)};
    let rsthis = QTextDecoder{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QTextDecoder {
  pub fn hasFailure<T: QTextDecoder_hasFailure>(&mut self, value: T) -> i32 {
    value.hasFailure(self);
    return 1;
  }
}

pub trait QTextDecoder_hasFailure {
  fn hasFailure(self, this: &mut QTextDecoder) -> i32;
}

// proto: bool QTextDecoder::hasFailure();
impl<'a> /*trait*/ QTextDecoder_hasFailure for () {
  fn hasFailure(self, this: &mut QTextDecoder) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTextDecoder10hasFailureEv()};
    unsafe {_ZNK12QTextDecoder10hasFailureEv()};
    return 1;
  }
}

impl /*struct*/ QTextDecoder {
  pub fn FreeQTextDecoder<T: QTextDecoder_FreeQTextDecoder>(&mut self, value: T) -> i32 {
    value.FreeQTextDecoder(self);
    return 1;
  }
}

pub trait QTextDecoder_FreeQTextDecoder {
  fn FreeQTextDecoder(self, this: &mut QTextDecoder) -> i32;
}

// proto: void QTextDecoder::FreeQTextDecoder();
impl<'a> /*trait*/ QTextDecoder_FreeQTextDecoder for () {
  fn FreeQTextDecoder(self, this: &mut QTextDecoder) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTextDecoderD0Ev()};
    unsafe {_ZN12QTextDecoderD0Ev()};
    return 1;
  }
}

// proto: QString QTextDecoder::toUnicode(const QByteArray & ba);
impl<'a> /*trait*/ QTextDecoder_toUnicode for (&'a  QByteArray) {
  fn toUnicode(self, this: &mut QTextDecoder) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTextDecoder9toUnicodeERK10QByteArray()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN12QTextDecoder9toUnicodeERK10QByteArray(arg0)};
    return 1;
  }
}

// proto: void QTextDecoder::NewQTextDecoder(const QTextDecoder & );
impl<'a> /*trait*/ QTextDecoder_NewQTextDecoder for (&'a  QTextDecoder) {
  fn NewQTextDecoder(self) -> QTextDecoder {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTextDecoderC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN12QTextDecoderC1ERKS_(qthis, arg0)};
    let rsthis = QTextDecoder{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: void QTextDecoder::toUnicode(QString * target, const char * chars, int len);
impl<'a> /*trait*/ QTextDecoder_toUnicode for (&'a mut QString, &'a  String, i32) {
  fn toUnicode(self, this: &mut QTextDecoder) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTextDecoder9toUnicodeEP7QStringPKci()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.as_ptr()  as *const c_char;
    let arg2 = self.2  as c_int;
    unsafe {_ZN12QTextDecoder9toUnicodeEP7QStringPKci(arg0, arg1, arg2)};
    return 1;
  }
}

