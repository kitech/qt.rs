// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qsize::QSize;
use super::qwidget::QWidget;
use super::qstring::QString;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  const QMetaObject * QCheckBox::metaObject();
  fn _ZNK9QCheckBox10metaObjectEv(qthis: *mut c_void) ;
  // proto:  QSize QCheckBox::minimumSizeHint();
  fn _ZNK9QCheckBox15minimumSizeHintEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QCheckBox::FreeQCheckBox();
  fn _ZN9QCheckBoxD0Ev(qthis: *mut c_void) ;
  // proto:  QSize QCheckBox::sizeHint();
  fn _ZNK9QCheckBox8sizeHintEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QCheckBox::stateChanged(int );
  fn _ZN9QCheckBox12stateChangedEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QCheckBox::setTristate(bool y);
  fn _ZN9QCheckBox11setTristateEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  void QCheckBox::NewQCheckBox(const QCheckBox & );
  fn _ZN9QCheckBoxC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QCheckBox::NewQCheckBox(QWidget * parent);
  fn _ZN9QCheckBoxC1EP7QWidget(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  bool QCheckBox::isTristate();
  fn _ZNK9QCheckBox10isTristateEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QCheckBox::NewQCheckBox(const QString & text, QWidget * parent);
  fn _ZN9QCheckBoxC1ERK7QStringP7QWidget(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) ;
}

// body block begin
// class sizeof(QCheckBox)=1
pub struct QCheckBox {
  pub qclsinst: *mut c_void,
}

// proto:  const QMetaObject * QCheckBox::metaObject();
impl /*struct*/ QCheckBox {
  pub fn metaObject<RetType, T: QCheckBox_metaObject<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QCheckBox_metaObject<RetType> {
  fn metaObject(self , rsthis: &mut QCheckBox) -> RetType;
}

// proto:  const QMetaObject * QCheckBox::metaObject();
impl<'a> /*trait*/ QCheckBox_metaObject<()> for () {
  fn metaObject(self , rsthis: &mut QCheckBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QCheckBox10metaObjectEv()};
     unsafe {_ZNK9QCheckBox10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

// proto:  QSize QCheckBox::minimumSizeHint();
impl /*struct*/ QCheckBox {
  pub fn minimumSizeHint<RetType, T: QCheckBox_minimumSizeHint<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.minimumSizeHint(self);
    // return 1;
  }
}

pub trait QCheckBox_minimumSizeHint<RetType> {
  fn minimumSizeHint(self , rsthis: &mut QCheckBox) -> RetType;
}

// proto:  QSize QCheckBox::minimumSizeHint();
impl<'a> /*trait*/ QCheckBox_minimumSizeHint<QSize> for () {
  fn minimumSizeHint(self , rsthis: &mut QCheckBox) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QCheckBox15minimumSizeHintEv()};
    let mut ret = unsafe {_ZNK9QCheckBox15minimumSizeHintEv(rsthis.qclsinst)};
    let mut ret1 = QSize{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  void QCheckBox::FreeQCheckBox();
impl /*struct*/ QCheckBox {
  pub fn FreeQCheckBox<RetType, T: QCheckBox_FreeQCheckBox<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.FreeQCheckBox(self);
    // return 1;
  }
}

pub trait QCheckBox_FreeQCheckBox<RetType> {
  fn FreeQCheckBox(self , rsthis: &mut QCheckBox) -> RetType;
}

// proto:  void QCheckBox::FreeQCheckBox();
impl<'a> /*trait*/ QCheckBox_FreeQCheckBox<()> for () {
  fn FreeQCheckBox(self , rsthis: &mut QCheckBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QCheckBoxD0Ev()};
     unsafe {_ZN9QCheckBoxD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

// proto:  QSize QCheckBox::sizeHint();
impl /*struct*/ QCheckBox {
  pub fn sizeHint<RetType, T: QCheckBox_sizeHint<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.sizeHint(self);
    // return 1;
  }
}

pub trait QCheckBox_sizeHint<RetType> {
  fn sizeHint(self , rsthis: &mut QCheckBox) -> RetType;
}

// proto:  QSize QCheckBox::sizeHint();
impl<'a> /*trait*/ QCheckBox_sizeHint<QSize> for () {
  fn sizeHint(self , rsthis: &mut QCheckBox) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QCheckBox8sizeHintEv()};
    let mut ret = unsafe {_ZNK9QCheckBox8sizeHintEv(rsthis.qclsinst)};
    let mut ret1 = QSize{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  void QCheckBox::stateChanged(int );
impl /*struct*/ QCheckBox {
  pub fn stateChanged<RetType, T: QCheckBox_stateChanged<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.stateChanged(self);
    // return 1;
  }
}

pub trait QCheckBox_stateChanged<RetType> {
  fn stateChanged(self , rsthis: &mut QCheckBox) -> RetType;
}

// proto:  void QCheckBox::stateChanged(int );
impl<'a> /*trait*/ QCheckBox_stateChanged<()> for (i32) {
  fn stateChanged(self , rsthis: &mut QCheckBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QCheckBox12stateChangedEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN9QCheckBox12stateChangedEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  void QCheckBox::setTristate(bool y);
impl /*struct*/ QCheckBox {
  pub fn setTristate<RetType, T: QCheckBox_setTristate<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setTristate(self);
    // return 1;
  }
}

pub trait QCheckBox_setTristate<RetType> {
  fn setTristate(self , rsthis: &mut QCheckBox) -> RetType;
}

// proto:  void QCheckBox::setTristate(bool y);
impl<'a> /*trait*/ QCheckBox_setTristate<()> for (i8) {
  fn setTristate(self , rsthis: &mut QCheckBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QCheckBox11setTristateEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN9QCheckBox11setTristateEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QCheckBox {
  pub fn NewQCheckBox<T: QCheckBox_NewQCheckBox>(value: T) -> QCheckBox {
    let rsthis = value.NewQCheckBox();
    return rsthis;
    // return 1;
  }
}

pub trait QCheckBox_NewQCheckBox {
  fn NewQCheckBox(self) -> QCheckBox;
}

// proto: void QCheckBox::NewQCheckBox(const QCheckBox & );
impl<'a> /*trait*/ QCheckBox_NewQCheckBox for (&'a  QCheckBox) {
  fn NewQCheckBox(self) -> QCheckBox {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QCheckBoxC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN9QCheckBoxC1ERKS_(qthis, arg0)};
    let rsthis = QCheckBox{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: void QCheckBox::NewQCheckBox(QWidget * parent);
impl<'a> /*trait*/ QCheckBox_NewQCheckBox for (&'a mut QWidget) {
  fn NewQCheckBox(self) -> QCheckBox {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QCheckBoxC1EP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN9QCheckBoxC1EP7QWidget(qthis, arg0)};
    let rsthis = QCheckBox{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto:  bool QCheckBox::isTristate();
impl /*struct*/ QCheckBox {
  pub fn isTristate<RetType, T: QCheckBox_isTristate<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.isTristate(self);
    // return 1;
  }
}

pub trait QCheckBox_isTristate<RetType> {
  fn isTristate(self , rsthis: &mut QCheckBox) -> RetType;
}

// proto:  bool QCheckBox::isTristate();
impl<'a> /*trait*/ QCheckBox_isTristate<i8> for () {
  fn isTristate(self , rsthis: &mut QCheckBox) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QCheckBox10isTristateEv()};
    let mut ret = unsafe {_ZNK9QCheckBox10isTristateEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

// proto: void QCheckBox::NewQCheckBox(const QString & text, QWidget * parent);
impl<'a> /*trait*/ QCheckBox_NewQCheckBox for (&'a  QString, &'a mut QWidget) {
  fn NewQCheckBox(self) -> QCheckBox {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QCheckBoxC1ERK7QStringP7QWidget()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN9QCheckBoxC1ERK7QStringP7QWidget(qthis, arg0, arg1)};
    let rsthis = QCheckBox{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

