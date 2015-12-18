// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qtextcodec::QTextCodec;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  void QTextDecoder::NewQTextDecoder(const QTextCodec * codec);
  fn _ZN12QTextDecoderC1EPK10QTextCodec(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  bool QTextDecoder::hasFailure();
  fn _ZNK12QTextDecoder10hasFailureEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QTextDecoder::FreeQTextDecoder();
  fn _ZN12QTextDecoderD0Ev(qthis: *mut c_void) ;
  // proto:  void QTextDecoder::NewQTextDecoder(const QTextDecoder & );
  fn _ZN12QTextDecoderC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
}

// body block begin
// class sizeof(QTextDecoder)=1
pub struct QTextDecoder {
  pub qclsinst: *mut c_void,
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
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN12QTextDecoderC1EPK10QTextCodec(qthis, arg0)};
    let rsthis = QTextDecoder{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QTextDecoder {
  pub fn hasFailure<RetType, T: QTextDecoder_hasFailure<RetType>>(&mut self, value: T) -> RetType {
    return value.hasFailure(self);
    // return 1;
  }
}

pub trait QTextDecoder_hasFailure<RetType> {
  fn hasFailure(self, rsthis: &mut QTextDecoder) -> RetType;
}

// proto:  bool QTextDecoder::hasFailure();
impl<'a> /*trait*/ QTextDecoder_hasFailure<i8> for () {
  fn hasFailure(self, rsthis: &mut QTextDecoder) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTextDecoder10hasFailureEv()};
    let mut ret = unsafe {_ZNK12QTextDecoder10hasFailureEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QTextDecoder {
  pub fn FreeQTextDecoder<RetType, T: QTextDecoder_FreeQTextDecoder<RetType>>(&mut self, value: T) -> RetType {
    return value.FreeQTextDecoder(self);
    // return 1;
  }
}

pub trait QTextDecoder_FreeQTextDecoder<RetType> {
  fn FreeQTextDecoder(self, rsthis: &mut QTextDecoder) -> RetType;
}

// proto:  void QTextDecoder::FreeQTextDecoder();
impl<'a> /*trait*/ QTextDecoder_FreeQTextDecoder<()> for () {
  fn FreeQTextDecoder(self, rsthis: &mut QTextDecoder) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTextDecoderD0Ev()};
     unsafe {_ZN12QTextDecoderD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

// proto: void QTextDecoder::NewQTextDecoder(const QTextDecoder & );
impl<'a> /*trait*/ QTextDecoder_NewQTextDecoder for (&'a  QTextDecoder) {
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

