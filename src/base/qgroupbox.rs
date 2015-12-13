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
  // proto:  bool QGroupBox::isCheckable();
  fn _ZNK9QGroupBox11isCheckableEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QGroupBox::setCheckable(bool checkable);
  fn _ZN9QGroupBox12setCheckableEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  const QMetaObject * QGroupBox::metaObject();
  fn _ZNK9QGroupBox10metaObjectEv(qthis: *mut c_void) ;
  // proto:  bool QGroupBox::isFlat();
  fn _ZNK9QGroupBox6isFlatEv(qthis: *mut c_void) -> int8_t;
  // proto:  QSize QGroupBox::minimumSizeHint();
  fn _ZNK9QGroupBox15minimumSizeHintEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QGroupBox::setFlat(bool flat);
  fn _ZN9QGroupBox7setFlatEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  void QGroupBox::FreeQGroupBox();
  fn _ZN9QGroupBoxD0Ev(qthis: *mut c_void) ;
  // proto:  void QGroupBox::NewQGroupBox(QWidget * parent);
  fn _ZN9QGroupBoxC1EP7QWidget(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QGroupBox::toggled(bool );
  fn _ZN9QGroupBox7toggledEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  bool QGroupBox::isChecked();
  fn _ZNK9QGroupBox9isCheckedEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QGroupBox::setChecked(bool checked);
  fn _ZN9QGroupBox10setCheckedEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  void QGroupBox::NewQGroupBox(const QGroupBox & );
  fn _ZN9QGroupBoxC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QString QGroupBox::title();
  fn _ZNK9QGroupBox5titleEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QGroupBox::setAlignment(int alignment);
  fn _ZN9QGroupBox12setAlignmentEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QGroupBox::setTitle(const QString & title);
  fn _ZN9QGroupBox8setTitleERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QGroupBox::NewQGroupBox(const QString & title, QWidget * parent);
  fn _ZN9QGroupBoxC1ERK7QStringP7QWidget(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) ;
  // proto:  void QGroupBox::clicked(bool checked);
  fn _ZN9QGroupBox7clickedEb(qthis: *mut c_void, arg0: int8_t) ;
}

// body block begin
// class sizeof(QGroupBox)=1
pub struct QGroupBox {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QGroupBox {
  pub fn isCheckable<T: QGroupBox_isCheckable>(&mut self, value: T) -> i8 {
    return value.isCheckable(self);
    // return 1;
  }
}

pub trait QGroupBox_isCheckable {
  fn isCheckable(self, rsthis: &mut QGroupBox) -> i8;
}

// proto:  bool QGroupBox::isCheckable();
impl<'a> /*trait*/ QGroupBox_isCheckable for () {
  fn isCheckable(self, rsthis: &mut QGroupBox) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QGroupBox11isCheckableEv()};
    let mut ret = unsafe {_ZNK9QGroupBox11isCheckableEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QGroupBox {
  pub fn setCheckable<T: QGroupBox_setCheckable>(&mut self, value: T)  {
     value.setCheckable(self);
    // return 1;
  }
}

pub trait QGroupBox_setCheckable {
  fn setCheckable(self, rsthis: &mut QGroupBox) ;
}

// proto:  void QGroupBox::setCheckable(bool checkable);
impl<'a> /*trait*/ QGroupBox_setCheckable for (i8) {
  fn setCheckable(self, rsthis: &mut QGroupBox)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QGroupBox12setCheckableEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN9QGroupBox12setCheckableEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGroupBox {
  pub fn metaObject<T: QGroupBox_metaObject>(&mut self, value: T)  {
     value.metaObject(self);
    // return 1;
  }
}

pub trait QGroupBox_metaObject {
  fn metaObject(self, rsthis: &mut QGroupBox) ;
}

// proto:  const QMetaObject * QGroupBox::metaObject();
impl<'a> /*trait*/ QGroupBox_metaObject for () {
  fn metaObject(self, rsthis: &mut QGroupBox)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QGroupBox10metaObjectEv()};
     unsafe {_ZNK9QGroupBox10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QGroupBox {
  pub fn isFlat<T: QGroupBox_isFlat>(&mut self, value: T) -> i8 {
    return value.isFlat(self);
    // return 1;
  }
}

pub trait QGroupBox_isFlat {
  fn isFlat(self, rsthis: &mut QGroupBox) -> i8;
}

// proto:  bool QGroupBox::isFlat();
impl<'a> /*trait*/ QGroupBox_isFlat for () {
  fn isFlat(self, rsthis: &mut QGroupBox) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QGroupBox6isFlatEv()};
    let mut ret = unsafe {_ZNK9QGroupBox6isFlatEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QGroupBox {
  pub fn minimumSizeHint<T: QGroupBox_minimumSizeHint>(&mut self, value: T) -> QSize {
    return value.minimumSizeHint(self);
    // return 1;
  }
}

pub trait QGroupBox_minimumSizeHint {
  fn minimumSizeHint(self, rsthis: &mut QGroupBox) -> QSize;
}

// proto:  QSize QGroupBox::minimumSizeHint();
impl<'a> /*trait*/ QGroupBox_minimumSizeHint for () {
  fn minimumSizeHint(self, rsthis: &mut QGroupBox) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QGroupBox15minimumSizeHintEv()};
    let mut ret = unsafe {_ZNK9QGroupBox15minimumSizeHintEv(rsthis.qclsinst)};
    let mut ret1 = QSize{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QGroupBox {
  pub fn setFlat<T: QGroupBox_setFlat>(&mut self, value: T)  {
     value.setFlat(self);
    // return 1;
  }
}

pub trait QGroupBox_setFlat {
  fn setFlat(self, rsthis: &mut QGroupBox) ;
}

// proto:  void QGroupBox::setFlat(bool flat);
impl<'a> /*trait*/ QGroupBox_setFlat for (i8) {
  fn setFlat(self, rsthis: &mut QGroupBox)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QGroupBox7setFlatEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN9QGroupBox7setFlatEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGroupBox {
  pub fn FreeQGroupBox<T: QGroupBox_FreeQGroupBox>(&mut self, value: T)  {
     value.FreeQGroupBox(self);
    // return 1;
  }
}

pub trait QGroupBox_FreeQGroupBox {
  fn FreeQGroupBox(self, rsthis: &mut QGroupBox) ;
}

// proto:  void QGroupBox::FreeQGroupBox();
impl<'a> /*trait*/ QGroupBox_FreeQGroupBox for () {
  fn FreeQGroupBox(self, rsthis: &mut QGroupBox)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QGroupBoxD0Ev()};
     unsafe {_ZN9QGroupBoxD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QGroupBox {
  pub fn NewQGroupBox<T: QGroupBox_NewQGroupBox>(value: T) -> QGroupBox {
    let rsthis = value.NewQGroupBox();
    return rsthis;
    // return 1;
  }
}

pub trait QGroupBox_NewQGroupBox {
  fn NewQGroupBox(self) -> QGroupBox;
}

// proto: void QGroupBox::NewQGroupBox(QWidget * parent);
impl<'a> /*trait*/ QGroupBox_NewQGroupBox for (&'a mut QWidget) {
  fn NewQGroupBox(self) -> QGroupBox {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QGroupBoxC1EP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN9QGroupBoxC1EP7QWidget(qthis, arg0)};
    let rsthis = QGroupBox{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QGroupBox {
  pub fn toggled<T: QGroupBox_toggled>(&mut self, value: T)  {
     value.toggled(self);
    // return 1;
  }
}

pub trait QGroupBox_toggled {
  fn toggled(self, rsthis: &mut QGroupBox) ;
}

// proto:  void QGroupBox::toggled(bool );
impl<'a> /*trait*/ QGroupBox_toggled for (i8) {
  fn toggled(self, rsthis: &mut QGroupBox)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QGroupBox7toggledEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN9QGroupBox7toggledEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGroupBox {
  pub fn isChecked<T: QGroupBox_isChecked>(&mut self, value: T) -> i8 {
    return value.isChecked(self);
    // return 1;
  }
}

pub trait QGroupBox_isChecked {
  fn isChecked(self, rsthis: &mut QGroupBox) -> i8;
}

// proto:  bool QGroupBox::isChecked();
impl<'a> /*trait*/ QGroupBox_isChecked for () {
  fn isChecked(self, rsthis: &mut QGroupBox) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QGroupBox9isCheckedEv()};
    let mut ret = unsafe {_ZNK9QGroupBox9isCheckedEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QGroupBox {
  pub fn setChecked<T: QGroupBox_setChecked>(&mut self, value: T)  {
     value.setChecked(self);
    // return 1;
  }
}

pub trait QGroupBox_setChecked {
  fn setChecked(self, rsthis: &mut QGroupBox) ;
}

// proto:  void QGroupBox::setChecked(bool checked);
impl<'a> /*trait*/ QGroupBox_setChecked for (i8) {
  fn setChecked(self, rsthis: &mut QGroupBox)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QGroupBox10setCheckedEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN9QGroupBox10setCheckedEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto: void QGroupBox::NewQGroupBox(const QGroupBox & );
impl<'a> /*trait*/ QGroupBox_NewQGroupBox for (&'a  QGroupBox) {
  fn NewQGroupBox(self) -> QGroupBox {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QGroupBoxC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN9QGroupBoxC1ERKS_(qthis, arg0)};
    let rsthis = QGroupBox{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QGroupBox {
  pub fn title<T: QGroupBox_title>(&mut self, value: T) -> QString {
    return value.title(self);
    // return 1;
  }
}

pub trait QGroupBox_title {
  fn title(self, rsthis: &mut QGroupBox) -> QString;
}

// proto:  QString QGroupBox::title();
impl<'a> /*trait*/ QGroupBox_title for () {
  fn title(self, rsthis: &mut QGroupBox) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QGroupBox5titleEv()};
    let mut ret = unsafe {_ZNK9QGroupBox5titleEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QGroupBox {
  pub fn setAlignment<T: QGroupBox_setAlignment>(&mut self, value: T)  {
     value.setAlignment(self);
    // return 1;
  }
}

pub trait QGroupBox_setAlignment {
  fn setAlignment(self, rsthis: &mut QGroupBox) ;
}

// proto:  void QGroupBox::setAlignment(int alignment);
impl<'a> /*trait*/ QGroupBox_setAlignment for (i32) {
  fn setAlignment(self, rsthis: &mut QGroupBox)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QGroupBox12setAlignmentEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN9QGroupBox12setAlignmentEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGroupBox {
  pub fn setTitle<T: QGroupBox_setTitle>(&mut self, value: T)  {
     value.setTitle(self);
    // return 1;
  }
}

pub trait QGroupBox_setTitle {
  fn setTitle(self, rsthis: &mut QGroupBox) ;
}

// proto:  void QGroupBox::setTitle(const QString & title);
impl<'a> /*trait*/ QGroupBox_setTitle for (&'a  QString) {
  fn setTitle(self, rsthis: &mut QGroupBox)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QGroupBox8setTitleERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QGroupBox8setTitleERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto: void QGroupBox::NewQGroupBox(const QString & title, QWidget * parent);
impl<'a> /*trait*/ QGroupBox_NewQGroupBox for (&'a  QString, &'a mut QWidget) {
  fn NewQGroupBox(self) -> QGroupBox {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QGroupBoxC1ERK7QStringP7QWidget()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN9QGroupBoxC1ERK7QStringP7QWidget(qthis, arg0, arg1)};
    let rsthis = QGroupBox{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QGroupBox {
  pub fn clicked<T: QGroupBox_clicked>(&mut self, value: T)  {
     value.clicked(self);
    // return 1;
  }
}

pub trait QGroupBox_clicked {
  fn clicked(self, rsthis: &mut QGroupBox) ;
}

// proto:  void QGroupBox::clicked(bool checked);
impl<'a> /*trait*/ QGroupBox_clicked for (i8) {
  fn clicked(self, rsthis: &mut QGroupBox)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QGroupBox7clickedEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN9QGroupBox7clickedEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

