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
  fn _ZNK9QGroupBox11isCheckableEv(qthis: *mut c_void) -> c_char;
  // proto:  void QGroupBox::setCheckable(bool checkable);
  fn _ZN9QGroupBox12setCheckableEb(qthis: *mut c_void, arg0: c_char);
  // proto:  const QMetaObject * QGroupBox::metaObject();
  fn _ZNK9QGroupBox10metaObjectEv(qthis: *mut c_void);
  // proto:  bool QGroupBox::isFlat();
  fn _ZNK9QGroupBox6isFlatEv(qthis: *mut c_void) -> c_char;
  // proto:  QSize QGroupBox::minimumSizeHint();
  fn _ZNK9QGroupBox15minimumSizeHintEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QGroupBox::setFlat(bool flat);
  fn _ZN9QGroupBox7setFlatEb(qthis: *mut c_void, arg0: c_char);
  // proto:  void QGroupBox::~QGroupBox();
  fn _ZN9QGroupBoxD0Ev(qthis: *mut c_void);
  // proto:  void QGroupBox::QGroupBox(QWidget * parent);
  fn _ZN9QGroupBoxC1EP7QWidget(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QGroupBox::toggled(bool );
  fn _ZN9QGroupBox7toggledEb(qthis: *mut c_void, arg0: c_char);
  // proto:  bool QGroupBox::isChecked();
  fn _ZNK9QGroupBox9isCheckedEv(qthis: *mut c_void) -> c_char;
  // proto:  void QGroupBox::setChecked(bool checked);
  fn _ZN9QGroupBox10setCheckedEb(qthis: *mut c_void, arg0: c_char);
  // proto:  void QGroupBox::QGroupBox(const QGroupBox & );
  fn _ZN9QGroupBoxC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QString QGroupBox::title();
  fn _ZNK9QGroupBox5titleEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QGroupBox::setAlignment(int alignment);
  fn _ZN9QGroupBox12setAlignmentEi(qthis: *mut c_void, arg0: c_int);
  // proto:  void QGroupBox::setTitle(const QString & title);
  fn _ZN9QGroupBox8setTitleERK7QString(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QGroupBox::QGroupBox(const QString & title, QWidget * parent);
  fn _ZN9QGroupBoxC1ERK7QStringP7QWidget(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  void QGroupBox::clicked(bool checked);
  fn _ZN9QGroupBox7clickedEb(qthis: *mut c_void, arg0: c_char);
}

// body block begin
// class sizeof(QGroupBox)=1
pub struct QGroupBox {
  pub qclsinst: *mut c_void,
}

  // proto:  bool QGroupBox::isCheckable();
impl /*struct*/ QGroupBox {
  pub fn isCheckable<RetType, T: QGroupBox_isCheckable<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isCheckable(self);
    // return 1;
  }
}

pub trait QGroupBox_isCheckable<RetType> {
  fn isCheckable(self , rsthis: &mut QGroupBox) -> RetType;
}

  // proto:  bool QGroupBox::isCheckable();
impl<'a> /*trait*/ QGroupBox_isCheckable<i8> for () {
  fn isCheckable(self , rsthis: &mut QGroupBox) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QGroupBox11isCheckableEv()};
    let mut ret = unsafe {_ZNK9QGroupBox11isCheckableEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QGroupBox::setCheckable(bool checkable);
impl /*struct*/ QGroupBox {
  pub fn setCheckable<RetType, T: QGroupBox_setCheckable<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setCheckable(self);
    // return 1;
  }
}

pub trait QGroupBox_setCheckable<RetType> {
  fn setCheckable(self , rsthis: &mut QGroupBox) -> RetType;
}

  // proto:  void QGroupBox::setCheckable(bool checkable);
impl<'a> /*trait*/ QGroupBox_setCheckable<()> for (i8) {
  fn setCheckable(self , rsthis: &mut QGroupBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QGroupBox12setCheckableEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN9QGroupBox12setCheckableEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  const QMetaObject * QGroupBox::metaObject();
impl /*struct*/ QGroupBox {
  pub fn metaObject<RetType, T: QGroupBox_metaObject<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QGroupBox_metaObject<RetType> {
  fn metaObject(self , rsthis: &mut QGroupBox) -> RetType;
}

  // proto:  const QMetaObject * QGroupBox::metaObject();
impl<'a> /*trait*/ QGroupBox_metaObject<()> for () {
  fn metaObject(self , rsthis: &mut QGroupBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QGroupBox10metaObjectEv()};
     unsafe {_ZNK9QGroupBox10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QGroupBox::isFlat();
impl /*struct*/ QGroupBox {
  pub fn isFlat<RetType, T: QGroupBox_isFlat<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isFlat(self);
    // return 1;
  }
}

pub trait QGroupBox_isFlat<RetType> {
  fn isFlat(self , rsthis: &mut QGroupBox) -> RetType;
}

  // proto:  bool QGroupBox::isFlat();
impl<'a> /*trait*/ QGroupBox_isFlat<i8> for () {
  fn isFlat(self , rsthis: &mut QGroupBox) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QGroupBox6isFlatEv()};
    let mut ret = unsafe {_ZNK9QGroupBox6isFlatEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QSize QGroupBox::minimumSizeHint();
impl /*struct*/ QGroupBox {
  pub fn minimumSizeHint<RetType, T: QGroupBox_minimumSizeHint<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.minimumSizeHint(self);
    // return 1;
  }
}

pub trait QGroupBox_minimumSizeHint<RetType> {
  fn minimumSizeHint(self , rsthis: &mut QGroupBox) -> RetType;
}

  // proto:  QSize QGroupBox::minimumSizeHint();
impl<'a> /*trait*/ QGroupBox_minimumSizeHint<QSize> for () {
  fn minimumSizeHint(self , rsthis: &mut QGroupBox) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QGroupBox15minimumSizeHintEv()};
    let mut ret = unsafe {_ZNK9QGroupBox15minimumSizeHintEv(rsthis.qclsinst)};
    let mut ret1 = QSize{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QGroupBox::setFlat(bool flat);
impl /*struct*/ QGroupBox {
  pub fn setFlat<RetType, T: QGroupBox_setFlat<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setFlat(self);
    // return 1;
  }
}

pub trait QGroupBox_setFlat<RetType> {
  fn setFlat(self , rsthis: &mut QGroupBox) -> RetType;
}

  // proto:  void QGroupBox::setFlat(bool flat);
impl<'a> /*trait*/ QGroupBox_setFlat<()> for (i8) {
  fn setFlat(self , rsthis: &mut QGroupBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QGroupBox7setFlatEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN9QGroupBox7setFlatEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QGroupBox::~QGroupBox();
impl /*struct*/ QGroupBox {
  pub fn FreeQGroupBox<RetType, T: QGroupBox_FreeQGroupBox<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQGroupBox(self);
    // return 1;
  }
}

pub trait QGroupBox_FreeQGroupBox<RetType> {
  fn FreeQGroupBox(self , rsthis: &mut QGroupBox) -> RetType;
}

  // proto:  void QGroupBox::~QGroupBox();
impl<'a> /*trait*/ QGroupBox_FreeQGroupBox<()> for () {
  fn FreeQGroupBox(self , rsthis: &mut QGroupBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QGroupBoxD0Ev()};
     unsafe {_ZN9QGroupBoxD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QGroupBox::QGroupBox(QWidget * parent);
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

  // proto:  void QGroupBox::QGroupBox(QWidget * parent);
impl<'a> /*trait*/ QGroupBox_NewQGroupBox for (QWidget) {
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

  // proto:  void QGroupBox::toggled(bool );
impl /*struct*/ QGroupBox {
  pub fn toggled<RetType, T: QGroupBox_toggled<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.toggled(self);
    // return 1;
  }
}

pub trait QGroupBox_toggled<RetType> {
  fn toggled(self , rsthis: &mut QGroupBox) -> RetType;
}

  // proto:  void QGroupBox::toggled(bool );
impl<'a> /*trait*/ QGroupBox_toggled<()> for (i8) {
  fn toggled(self , rsthis: &mut QGroupBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QGroupBox7toggledEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN9QGroupBox7toggledEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QGroupBox::isChecked();
impl /*struct*/ QGroupBox {
  pub fn isChecked<RetType, T: QGroupBox_isChecked<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isChecked(self);
    // return 1;
  }
}

pub trait QGroupBox_isChecked<RetType> {
  fn isChecked(self , rsthis: &mut QGroupBox) -> RetType;
}

  // proto:  bool QGroupBox::isChecked();
impl<'a> /*trait*/ QGroupBox_isChecked<i8> for () {
  fn isChecked(self , rsthis: &mut QGroupBox) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QGroupBox9isCheckedEv()};
    let mut ret = unsafe {_ZNK9QGroupBox9isCheckedEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QGroupBox::setChecked(bool checked);
impl /*struct*/ QGroupBox {
  pub fn setChecked<RetType, T: QGroupBox_setChecked<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setChecked(self);
    // return 1;
  }
}

pub trait QGroupBox_setChecked<RetType> {
  fn setChecked(self , rsthis: &mut QGroupBox) -> RetType;
}

  // proto:  void QGroupBox::setChecked(bool checked);
impl<'a> /*trait*/ QGroupBox_setChecked<()> for (i8) {
  fn setChecked(self , rsthis: &mut QGroupBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QGroupBox10setCheckedEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN9QGroupBox10setCheckedEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QGroupBox::QGroupBox(const QGroupBox & );
impl<'a> /*trait*/ QGroupBox_NewQGroupBox for (QGroupBox) {
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

  // proto:  QString QGroupBox::title();
impl /*struct*/ QGroupBox {
  pub fn title<RetType, T: QGroupBox_title<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.title(self);
    // return 1;
  }
}

pub trait QGroupBox_title<RetType> {
  fn title(self , rsthis: &mut QGroupBox) -> RetType;
}

  // proto:  QString QGroupBox::title();
impl<'a> /*trait*/ QGroupBox_title<QString> for () {
  fn title(self , rsthis: &mut QGroupBox) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QGroupBox5titleEv()};
    let mut ret = unsafe {_ZNK9QGroupBox5titleEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QGroupBox::setAlignment(int alignment);
impl /*struct*/ QGroupBox {
  pub fn setAlignment<RetType, T: QGroupBox_setAlignment<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setAlignment(self);
    // return 1;
  }
}

pub trait QGroupBox_setAlignment<RetType> {
  fn setAlignment(self , rsthis: &mut QGroupBox) -> RetType;
}

  // proto:  void QGroupBox::setAlignment(int alignment);
impl<'a> /*trait*/ QGroupBox_setAlignment<()> for (i32) {
  fn setAlignment(self , rsthis: &mut QGroupBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QGroupBox12setAlignmentEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN9QGroupBox12setAlignmentEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QGroupBox::setTitle(const QString & title);
impl /*struct*/ QGroupBox {
  pub fn setTitle<RetType, T: QGroupBox_setTitle<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setTitle(self);
    // return 1;
  }
}

pub trait QGroupBox_setTitle<RetType> {
  fn setTitle(self , rsthis: &mut QGroupBox) -> RetType;
}

  // proto:  void QGroupBox::setTitle(const QString & title);
impl<'a> /*trait*/ QGroupBox_setTitle<()> for (QString) {
  fn setTitle(self , rsthis: &mut QGroupBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QGroupBox8setTitleERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QGroupBox8setTitleERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QGroupBox::QGroupBox(const QString & title, QWidget * parent);
impl<'a> /*trait*/ QGroupBox_NewQGroupBox for (QString, QWidget) {
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

  // proto:  void QGroupBox::clicked(bool checked);
impl /*struct*/ QGroupBox {
  pub fn clicked<RetType, T: QGroupBox_clicked<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.clicked(self);
    // return 1;
  }
}

pub trait QGroupBox_clicked<RetType> {
  fn clicked(self , rsthis: &mut QGroupBox) -> RetType;
}

  // proto:  void QGroupBox::clicked(bool checked);
impl<'a> /*trait*/ QGroupBox_clicked<()> for (i8) {
  fn clicked(self , rsthis: &mut QGroupBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QGroupBox7clickedEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN9QGroupBox7clickedEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

