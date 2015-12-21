// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qstring::QString;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  bool QProcessEnvironment::contains(const QString & name);
  fn _ZNK19QProcessEnvironment8containsERK7QString(qthis: *mut c_void, arg0: *mut c_void) -> c_char;
  // proto:  QStringList QProcessEnvironment::keys();
  fn _ZNK19QProcessEnvironment4keysEv(qthis: *mut c_void);
  // proto:  void QProcessEnvironment::remove(const QString & name);
  fn _ZN19QProcessEnvironment6removeERK7QString(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QProcessEnvironment::clear();
  fn _ZN19QProcessEnvironment5clearEv(qthis: *mut c_void);
  // proto:  QString QProcessEnvironment::value(const QString & name, const QString & defaultValue);
  fn _ZNK19QProcessEnvironment5valueERK7QStringS2_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  // proto:  bool QProcessEnvironment::isEmpty();
  fn _ZNK19QProcessEnvironment7isEmptyEv(qthis: *mut c_void) -> c_char;
  // proto:  void QProcessEnvironment::~QProcessEnvironment();
  fn _ZN19QProcessEnvironmentD0Ev(qthis: *mut c_void);
  // proto:  void QProcessEnvironment::swap(QProcessEnvironment & other);
  fn _ZN19QProcessEnvironment4swapERS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QProcessEnvironment::QProcessEnvironment(const QProcessEnvironment & other);
  fn _ZN19QProcessEnvironmentC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto: static QProcessEnvironment QProcessEnvironment::systemEnvironment();
  fn _ZN19QProcessEnvironment17systemEnvironmentEv() -> *mut c_void;
  // proto:  void QProcessEnvironment::insert(const QString & name, const QString & value);
  fn _ZN19QProcessEnvironment6insertERK7QStringS2_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  QStringList QProcessEnvironment::toStringList();
  fn _ZNK19QProcessEnvironment12toStringListEv(qthis: *mut c_void);
  // proto:  void QProcessEnvironment::QProcessEnvironment();
  fn _ZN19QProcessEnvironmentC1Ev(qthis: *mut c_void);
  // proto:  void QProcessEnvironment::insert(const QProcessEnvironment & e);
  fn _ZN19QProcessEnvironment6insertERKS_(qthis: *mut c_void, arg0: *mut c_void);
}

// body block begin
// class sizeof(QProcessEnvironment)=1
pub struct QProcessEnvironment {
  pub qclsinst: *mut c_void,
}

  // proto:  bool QProcessEnvironment::contains(const QString & name);
impl /*struct*/ QProcessEnvironment {
  pub fn contains<RetType, T: QProcessEnvironment_contains<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.contains(self);
    // return 1;
  }
}

pub trait QProcessEnvironment_contains<RetType> {
  fn contains(self , rsthis: &mut QProcessEnvironment) -> RetType;
}

  // proto:  bool QProcessEnvironment::contains(const QString & name);
impl<'a> /*trait*/ QProcessEnvironment_contains<i8> for (QString) {
  fn contains(self , rsthis: &mut QProcessEnvironment) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QProcessEnvironment8containsERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK19QProcessEnvironment8containsERK7QString(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QStringList QProcessEnvironment::keys();
impl /*struct*/ QProcessEnvironment {
  pub fn keys<RetType, T: QProcessEnvironment_keys<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.keys(self);
    // return 1;
  }
}

pub trait QProcessEnvironment_keys<RetType> {
  fn keys(self , rsthis: &mut QProcessEnvironment) -> RetType;
}

  // proto:  QStringList QProcessEnvironment::keys();
impl<'a> /*trait*/ QProcessEnvironment_keys<()> for () {
  fn keys(self , rsthis: &mut QProcessEnvironment) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QProcessEnvironment4keysEv()};
     unsafe {_ZNK19QProcessEnvironment4keysEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QProcessEnvironment::remove(const QString & name);
impl /*struct*/ QProcessEnvironment {
  pub fn remove<RetType, T: QProcessEnvironment_remove<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.remove(self);
    // return 1;
  }
}

pub trait QProcessEnvironment_remove<RetType> {
  fn remove(self , rsthis: &mut QProcessEnvironment) -> RetType;
}

  // proto:  void QProcessEnvironment::remove(const QString & name);
impl<'a> /*trait*/ QProcessEnvironment_remove<()> for (QString) {
  fn remove(self , rsthis: &mut QProcessEnvironment) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QProcessEnvironment6removeERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN19QProcessEnvironment6removeERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QProcessEnvironment::clear();
impl /*struct*/ QProcessEnvironment {
  pub fn clear<RetType, T: QProcessEnvironment_clear<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.clear(self);
    // return 1;
  }
}

pub trait QProcessEnvironment_clear<RetType> {
  fn clear(self , rsthis: &mut QProcessEnvironment) -> RetType;
}

  // proto:  void QProcessEnvironment::clear();
impl<'a> /*trait*/ QProcessEnvironment_clear<()> for () {
  fn clear(self , rsthis: &mut QProcessEnvironment) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QProcessEnvironment5clearEv()};
     unsafe {_ZN19QProcessEnvironment5clearEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QString QProcessEnvironment::value(const QString & name, const QString & defaultValue);
impl /*struct*/ QProcessEnvironment {
  pub fn value<RetType, T: QProcessEnvironment_value<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.value(self);
    // return 1;
  }
}

pub trait QProcessEnvironment_value<RetType> {
  fn value(self , rsthis: &mut QProcessEnvironment) -> RetType;
}

  // proto:  QString QProcessEnvironment::value(const QString & name, const QString & defaultValue);
impl<'a> /*trait*/ QProcessEnvironment_value<QString> for (QString, QString) {
  fn value(self , rsthis: &mut QProcessEnvironment) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QProcessEnvironment5valueERK7QStringS2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK19QProcessEnvironment5valueERK7QStringS2_(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  bool QProcessEnvironment::isEmpty();
impl /*struct*/ QProcessEnvironment {
  pub fn isEmpty<RetType, T: QProcessEnvironment_isEmpty<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isEmpty(self);
    // return 1;
  }
}

pub trait QProcessEnvironment_isEmpty<RetType> {
  fn isEmpty(self , rsthis: &mut QProcessEnvironment) -> RetType;
}

  // proto:  bool QProcessEnvironment::isEmpty();
impl<'a> /*trait*/ QProcessEnvironment_isEmpty<i8> for () {
  fn isEmpty(self , rsthis: &mut QProcessEnvironment) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QProcessEnvironment7isEmptyEv()};
    let mut ret = unsafe {_ZNK19QProcessEnvironment7isEmptyEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QProcessEnvironment::~QProcessEnvironment();
impl /*struct*/ QProcessEnvironment {
  pub fn FreeQProcessEnvironment<RetType, T: QProcessEnvironment_FreeQProcessEnvironment<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQProcessEnvironment(self);
    // return 1;
  }
}

pub trait QProcessEnvironment_FreeQProcessEnvironment<RetType> {
  fn FreeQProcessEnvironment(self , rsthis: &mut QProcessEnvironment) -> RetType;
}

  // proto:  void QProcessEnvironment::~QProcessEnvironment();
impl<'a> /*trait*/ QProcessEnvironment_FreeQProcessEnvironment<()> for () {
  fn FreeQProcessEnvironment(self , rsthis: &mut QProcessEnvironment) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QProcessEnvironmentD0Ev()};
     unsafe {_ZN19QProcessEnvironmentD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QProcessEnvironment::swap(QProcessEnvironment & other);
impl /*struct*/ QProcessEnvironment {
  pub fn swap<RetType, T: QProcessEnvironment_swap<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.swap(self);
    // return 1;
  }
}

pub trait QProcessEnvironment_swap<RetType> {
  fn swap(self , rsthis: &mut QProcessEnvironment) -> RetType;
}

  // proto:  void QProcessEnvironment::swap(QProcessEnvironment & other);
impl<'a> /*trait*/ QProcessEnvironment_swap<()> for (QProcessEnvironment) {
  fn swap(self , rsthis: &mut QProcessEnvironment) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QProcessEnvironment4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN19QProcessEnvironment4swapERS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QProcessEnvironment::QProcessEnvironment(const QProcessEnvironment & other);
impl /*struct*/ QProcessEnvironment {
  pub fn NewQProcessEnvironment<T: QProcessEnvironment_NewQProcessEnvironment>(value: T) -> QProcessEnvironment {
    let rsthis = value.NewQProcessEnvironment();
    return rsthis;
    // return 1;
  }
}

pub trait QProcessEnvironment_NewQProcessEnvironment {
  fn NewQProcessEnvironment(self) -> QProcessEnvironment;
}

  // proto:  void QProcessEnvironment::QProcessEnvironment(const QProcessEnvironment & other);
impl<'a> /*trait*/ QProcessEnvironment_NewQProcessEnvironment for (QProcessEnvironment) {
  fn NewQProcessEnvironment(self) -> QProcessEnvironment {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QProcessEnvironmentC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN19QProcessEnvironmentC1ERKS_(qthis, arg0)};
    let rsthis = QProcessEnvironment{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto: static QProcessEnvironment QProcessEnvironment::systemEnvironment();
impl /*struct*/ QProcessEnvironment {
  pub fn systemEnvironment_s<RetType, T: QProcessEnvironment_systemEnvironment_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.systemEnvironment_s();
    // return 1;
  }
}

pub trait QProcessEnvironment_systemEnvironment_s<RetType> {
  fn systemEnvironment_s(self ) -> RetType;
}

  // proto: static QProcessEnvironment QProcessEnvironment::systemEnvironment();
impl<'a> /*trait*/ QProcessEnvironment_systemEnvironment_s<QProcessEnvironment> for () {
  fn systemEnvironment_s(self ) -> QProcessEnvironment {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QProcessEnvironment17systemEnvironmentEv()};
    let mut ret = unsafe {_ZN19QProcessEnvironment17systemEnvironmentEv()};
    let mut ret1 = QProcessEnvironment{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QProcessEnvironment::insert(const QString & name, const QString & value);
impl /*struct*/ QProcessEnvironment {
  pub fn insert<RetType, T: QProcessEnvironment_insert<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.insert(self);
    // return 1;
  }
}

pub trait QProcessEnvironment_insert<RetType> {
  fn insert(self , rsthis: &mut QProcessEnvironment) -> RetType;
}

  // proto:  void QProcessEnvironment::insert(const QString & name, const QString & value);
impl<'a> /*trait*/ QProcessEnvironment_insert<()> for (QString, QString) {
  fn insert(self , rsthis: &mut QProcessEnvironment) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QProcessEnvironment6insertERK7QStringS2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN19QProcessEnvironment6insertERK7QStringS2_(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  QStringList QProcessEnvironment::toStringList();
impl /*struct*/ QProcessEnvironment {
  pub fn toStringList<RetType, T: QProcessEnvironment_toStringList<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.toStringList(self);
    // return 1;
  }
}

pub trait QProcessEnvironment_toStringList<RetType> {
  fn toStringList(self , rsthis: &mut QProcessEnvironment) -> RetType;
}

  // proto:  QStringList QProcessEnvironment::toStringList();
impl<'a> /*trait*/ QProcessEnvironment_toStringList<()> for () {
  fn toStringList(self , rsthis: &mut QProcessEnvironment) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QProcessEnvironment12toStringListEv()};
     unsafe {_ZNK19QProcessEnvironment12toStringListEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QProcessEnvironment::QProcessEnvironment();
impl<'a> /*trait*/ QProcessEnvironment_NewQProcessEnvironment for () {
  fn NewQProcessEnvironment(self) -> QProcessEnvironment {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QProcessEnvironmentC1Ev()};
    unsafe {_ZN19QProcessEnvironmentC1Ev(qthis)};
    let rsthis = QProcessEnvironment{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QProcessEnvironment::insert(const QProcessEnvironment & e);
impl<'a> /*trait*/ QProcessEnvironment_insert<()> for (QProcessEnvironment) {
  fn insert(self , rsthis: &mut QProcessEnvironment) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QProcessEnvironment6insertERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN19QProcessEnvironment6insertERKS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

