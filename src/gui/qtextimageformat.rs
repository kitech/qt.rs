// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qtextformat::QTextFormat;
use super::qstring::QString;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  void QTextImageFormat::QTextImageFormat();
  fn _ZN16QTextImageFormatC1Ev(qthis: *mut c_void);
  // proto:  bool QTextImageFormat::isValid();
  fn _ZNK16QTextImageFormat7isValidEv(qthis: *mut c_void) -> c_char;
  // proto:  qreal QTextImageFormat::width();
  fn _ZNK16QTextImageFormat5widthEv(qthis: *mut c_void) -> c_double;
  // proto:  void QTextImageFormat::QTextImageFormat(const QTextFormat & format);
  fn _ZN16QTextImageFormatC1ERK11QTextFormat(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QTextImageFormat::setHeight(qreal height);
  fn _ZN16QTextImageFormat9setHeightEd(qthis: *mut c_void, arg0: c_double);
  // proto:  void QTextImageFormat::setWidth(qreal width);
  fn _ZN16QTextImageFormat8setWidthEd(qthis: *mut c_void, arg0: c_double);
  // proto:  void QTextImageFormat::setName(const QString & name);
  fn _ZN16QTextImageFormat7setNameERK7QString(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QString QTextImageFormat::name();
  fn _ZNK16QTextImageFormat4nameEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  qreal QTextImageFormat::height();
  fn _ZNK16QTextImageFormat6heightEv(qthis: *mut c_void) -> c_double;
}

// body block begin
// class sizeof(QTextImageFormat)=1
pub struct QTextImageFormat {
  pub qclsinst: *mut c_void,
}

  // proto:  void QTextImageFormat::QTextImageFormat();
impl /*struct*/ QTextImageFormat {
  pub fn NewQTextImageFormat<T: QTextImageFormat_NewQTextImageFormat>(value: T) -> QTextImageFormat {
    let rsthis = value.NewQTextImageFormat();
    return rsthis;
    // return 1;
  }
}

pub trait QTextImageFormat_NewQTextImageFormat {
  fn NewQTextImageFormat(self) -> QTextImageFormat;
}

  // proto:  void QTextImageFormat::QTextImageFormat();
impl<'a> /*trait*/ QTextImageFormat_NewQTextImageFormat for () {
  fn NewQTextImageFormat(self) -> QTextImageFormat {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTextImageFormatC1Ev()};
    unsafe {_ZN16QTextImageFormatC1Ev(qthis)};
    let rsthis = QTextImageFormat{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  bool QTextImageFormat::isValid();
impl /*struct*/ QTextImageFormat {
  pub fn isValid<RetType, T: QTextImageFormat_isValid<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isValid(self);
    // return 1;
  }
}

pub trait QTextImageFormat_isValid<RetType> {
  fn isValid(self , rsthis: &mut QTextImageFormat) -> RetType;
}

  // proto:  bool QTextImageFormat::isValid();
impl<'a> /*trait*/ QTextImageFormat_isValid<i8> for () {
  fn isValid(self , rsthis: &mut QTextImageFormat) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTextImageFormat7isValidEv()};
    let mut ret = unsafe {_ZNK16QTextImageFormat7isValidEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  qreal QTextImageFormat::width();
impl /*struct*/ QTextImageFormat {
  pub fn width<RetType, T: QTextImageFormat_width<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.width(self);
    // return 1;
  }
}

pub trait QTextImageFormat_width<RetType> {
  fn width(self , rsthis: &mut QTextImageFormat) -> RetType;
}

  // proto:  qreal QTextImageFormat::width();
impl<'a> /*trait*/ QTextImageFormat_width<f64> for () {
  fn width(self , rsthis: &mut QTextImageFormat) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTextImageFormat5widthEv()};
    let mut ret = unsafe {_ZNK16QTextImageFormat5widthEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  void QTextImageFormat::QTextImageFormat(const QTextFormat & format);
impl<'a> /*trait*/ QTextImageFormat_NewQTextImageFormat for (QTextFormat) {
  fn NewQTextImageFormat(self) -> QTextImageFormat {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTextImageFormatC1ERK11QTextFormat()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN16QTextImageFormatC1ERK11QTextFormat(qthis, arg0)};
    let rsthis = QTextImageFormat{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QTextImageFormat::setHeight(qreal height);
impl /*struct*/ QTextImageFormat {
  pub fn setHeight<RetType, T: QTextImageFormat_setHeight<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setHeight(self);
    // return 1;
  }
}

pub trait QTextImageFormat_setHeight<RetType> {
  fn setHeight(self , rsthis: &mut QTextImageFormat) -> RetType;
}

  // proto:  void QTextImageFormat::setHeight(qreal height);
impl<'a> /*trait*/ QTextImageFormat_setHeight<()> for (f64) {
  fn setHeight(self , rsthis: &mut QTextImageFormat) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTextImageFormat9setHeightEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN16QTextImageFormat9setHeightEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTextImageFormat::setWidth(qreal width);
impl /*struct*/ QTextImageFormat {
  pub fn setWidth<RetType, T: QTextImageFormat_setWidth<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setWidth(self);
    // return 1;
  }
}

pub trait QTextImageFormat_setWidth<RetType> {
  fn setWidth(self , rsthis: &mut QTextImageFormat) -> RetType;
}

  // proto:  void QTextImageFormat::setWidth(qreal width);
impl<'a> /*trait*/ QTextImageFormat_setWidth<()> for (f64) {
  fn setWidth(self , rsthis: &mut QTextImageFormat) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTextImageFormat8setWidthEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN16QTextImageFormat8setWidthEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTextImageFormat::setName(const QString & name);
impl /*struct*/ QTextImageFormat {
  pub fn setName<RetType, T: QTextImageFormat_setName<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setName(self);
    // return 1;
  }
}

pub trait QTextImageFormat_setName<RetType> {
  fn setName(self , rsthis: &mut QTextImageFormat) -> RetType;
}

  // proto:  void QTextImageFormat::setName(const QString & name);
impl<'a> /*trait*/ QTextImageFormat_setName<()> for (QString) {
  fn setName(self , rsthis: &mut QTextImageFormat) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTextImageFormat7setNameERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN16QTextImageFormat7setNameERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QString QTextImageFormat::name();
impl /*struct*/ QTextImageFormat {
  pub fn name<RetType, T: QTextImageFormat_name<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.name(self);
    // return 1;
  }
}

pub trait QTextImageFormat_name<RetType> {
  fn name(self , rsthis: &mut QTextImageFormat) -> RetType;
}

  // proto:  QString QTextImageFormat::name();
impl<'a> /*trait*/ QTextImageFormat_name<QString> for () {
  fn name(self , rsthis: &mut QTextImageFormat) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTextImageFormat4nameEv()};
    let mut ret = unsafe {_ZNK16QTextImageFormat4nameEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  qreal QTextImageFormat::height();
impl /*struct*/ QTextImageFormat {
  pub fn height<RetType, T: QTextImageFormat_height<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.height(self);
    // return 1;
  }
}

pub trait QTextImageFormat_height<RetType> {
  fn height(self , rsthis: &mut QTextImageFormat) -> RetType;
}

  // proto:  qreal QTextImageFormat::height();
impl<'a> /*trait*/ QTextImageFormat_height<f64> for () {
  fn height(self , rsthis: &mut QTextImageFormat) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTextImageFormat6heightEv()};
    let mut ret = unsafe {_ZNK16QTextImageFormat6heightEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

