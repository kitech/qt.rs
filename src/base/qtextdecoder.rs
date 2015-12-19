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
use super::qbytearray::QByteArray;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  QString QTextDecoder::toUnicode(const char * chars, int len);
  fn _ZN12QTextDecoder9toUnicodeEPKci(qthis: *mut c_void, arg0: *mut c_char, arg1: c_int) -> *mut c_void;
  // proto:  void QTextDecoder::QTextDecoder(const QTextCodec * codec);
  fn _ZN12QTextDecoderC1EPK10QTextCodec(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  bool QTextDecoder::hasFailure();
  fn _ZNK12QTextDecoder10hasFailureEv(qthis: *mut c_void) -> c_char;
  // proto:  void QTextDecoder::~QTextDecoder();
  fn _ZN12QTextDecoderD0Ev(qthis: *mut c_void);
  // proto:  QString QTextDecoder::toUnicode(const QByteArray & ba);
  fn _ZN12QTextDecoder9toUnicodeERK10QByteArray(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QTextDecoder::QTextDecoder(const QTextDecoder & );
  fn _ZN12QTextDecoderC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QTextDecoder::toUnicode(QString * target, const char * chars, int len);
  fn _ZN12QTextDecoder9toUnicodeEP7QStringPKci(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_char, arg2: c_int);
}

// body block begin
// class sizeof(QTextDecoder)=1
pub struct QTextDecoder {
  pub qclsinst: *mut c_void,
}

  // proto:  QString QTextDecoder::toUnicode(const char * chars, int len);
impl /*struct*/ QTextDecoder {
  pub fn toUnicode<RetType, T: QTextDecoder_toUnicode<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.toUnicode(self);
    // return 1;
  }
}

pub trait QTextDecoder_toUnicode<RetType> {
  fn toUnicode(self , rsthis: &mut QTextDecoder) -> RetType;
}

  // proto:  QString QTextDecoder::toUnicode(const char * chars, int len);
impl<'a> /*trait*/ QTextDecoder_toUnicode<QString> for (&'a  String, i32) {
  fn toUnicode(self , rsthis: &mut QTextDecoder) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTextDecoder9toUnicodeEPKci()};
    let arg0 = self.0.as_ptr()  as *mut c_char;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZN12QTextDecoder9toUnicodeEPKci(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QTextDecoder::QTextDecoder(const QTextCodec * codec);
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

  // proto:  void QTextDecoder::QTextDecoder(const QTextCodec * codec);
impl<'a> /*trait*/ QTextDecoder_NewQTextDecoder for (QTextCodec) {
  fn NewQTextDecoder(self) -> QTextDecoder {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTextDecoderC1EPK10QTextCodec()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN12QTextDecoderC1EPK10QTextCodec(qthis, arg0)};
    let rsthis = QTextDecoder{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  bool QTextDecoder::hasFailure();
impl /*struct*/ QTextDecoder {
  pub fn hasFailure<RetType, T: QTextDecoder_hasFailure<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.hasFailure(self);
    // return 1;
  }
}

pub trait QTextDecoder_hasFailure<RetType> {
  fn hasFailure(self , rsthis: &mut QTextDecoder) -> RetType;
}

  // proto:  bool QTextDecoder::hasFailure();
impl<'a> /*trait*/ QTextDecoder_hasFailure<i8> for () {
  fn hasFailure(self , rsthis: &mut QTextDecoder) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTextDecoder10hasFailureEv()};
    let mut ret = unsafe {_ZNK12QTextDecoder10hasFailureEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QTextDecoder::~QTextDecoder();
impl /*struct*/ QTextDecoder {
  pub fn FreeQTextDecoder<RetType, T: QTextDecoder_FreeQTextDecoder<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQTextDecoder(self);
    // return 1;
  }
}

pub trait QTextDecoder_FreeQTextDecoder<RetType> {
  fn FreeQTextDecoder(self , rsthis: &mut QTextDecoder) -> RetType;
}

  // proto:  void QTextDecoder::~QTextDecoder();
impl<'a> /*trait*/ QTextDecoder_FreeQTextDecoder<()> for () {
  fn FreeQTextDecoder(self , rsthis: &mut QTextDecoder) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTextDecoderD0Ev()};
     unsafe {_ZN12QTextDecoderD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QString QTextDecoder::toUnicode(const QByteArray & ba);
impl<'a> /*trait*/ QTextDecoder_toUnicode<QString> for (QByteArray) {
  fn toUnicode(self , rsthis: &mut QTextDecoder) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTextDecoder9toUnicodeERK10QByteArray()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN12QTextDecoder9toUnicodeERK10QByteArray(rsthis.qclsinst, arg0)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QTextDecoder::QTextDecoder(const QTextDecoder & );
impl<'a> /*trait*/ QTextDecoder_NewQTextDecoder for (QTextDecoder) {
  fn NewQTextDecoder(self) -> QTextDecoder {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTextDecoderC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN12QTextDecoderC1ERKS_(qthis, arg0)};
    let rsthis = QTextDecoder{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QTextDecoder::toUnicode(QString * target, const char * chars, int len);
impl<'a> /*trait*/ QTextDecoder_toUnicode<()> for (QString, &'a  String, i32) {
  fn toUnicode(self , rsthis: &mut QTextDecoder) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTextDecoder9toUnicodeEP7QStringPKci()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.as_ptr()  as *mut c_char;
    let arg2 = self.2  as c_int;
     unsafe {_ZN12QTextDecoder9toUnicodeEP7QStringPKci(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

