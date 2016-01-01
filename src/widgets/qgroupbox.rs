// auto generated, do not modify.
// created: Fri Jan  1 12:13:41 2016
// src-file: /QtWidgets/qgroupbox.h
// dst-file: /src/widgets/qgroupbox.rs
//

// header block begin =>
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;


// <= header block end

// main block begin =>
// <= main block end

// use block begin =>
use super::qwidget::QWidget; // 773
use std::ops::Deref;
use super::super::core::qsize::QSize; // 771
use super::super::core::qstring::QString; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QGroupBox_Class_Size() -> c_int;
  // proto:  bool QGroupBox::isCheckable();
  fn _ZNK9QGroupBox11isCheckableEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QGroupBox::setCheckable(bool checkable);
  fn _ZN9QGroupBox12setCheckableEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  const QMetaObject * QGroupBox::metaObject();
  fn _ZNK9QGroupBox10metaObjectEv(qthis: u64 /* *mut c_void*/);
  // proto:  bool QGroupBox::isFlat();
  fn _ZNK9QGroupBox6isFlatEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  QSize QGroupBox::minimumSizeHint();
  fn _ZNK9QGroupBox15minimumSizeHintEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QGroupBox::setFlat(bool flat);
  fn _ZN9QGroupBox7setFlatEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  void QGroupBox::~QGroupBox();
  fn _ZN9QGroupBoxD0Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QGroupBox::QGroupBox(QWidget * parent);
  fn dector_ZN9QGroupBoxC1EP7QWidget(arg0: *mut c_void) -> *mut c_void;
  fn _ZN9QGroupBoxC1EP7QWidget(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QGroupBox::toggled(bool );
  fn _ZN9QGroupBox7toggledEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  bool QGroupBox::isChecked();
  fn _ZNK9QGroupBox9isCheckedEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QGroupBox::setChecked(bool checked);
  fn _ZN9QGroupBox10setCheckedEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  void QGroupBox::QGroupBox(const QGroupBox & );
  fn dector_ZN9QGroupBoxC1ERKS_(arg0: *mut c_void) -> *mut c_void;
  fn _ZN9QGroupBoxC1ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QString QGroupBox::title();
  fn _ZNK9QGroupBox5titleEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QGroupBox::setAlignment(int alignment);
  fn _ZN9QGroupBox12setAlignmentEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  void QGroupBox::setTitle(const QString & title);
  fn _ZN9QGroupBox8setTitleERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QGroupBox::QGroupBox(const QString & title, QWidget * parent);
  fn dector_ZN9QGroupBoxC1ERK7QStringP7QWidget(arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  fn _ZN9QGroupBoxC1ERK7QStringP7QWidget(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  void QGroupBox::clicked(bool checked);
  fn _ZN9QGroupBox7clickedEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  fn QGroupBox_SlotProxy_connect__ZN9QGroupBox7clickedEb(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QGroupBox_SlotProxy_connect__ZN9QGroupBox7toggledEb(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QGroupBox)=1
#[derive(Default)]
pub struct QGroupBox {
  qbase: QWidget,
  pub qclsinst: u64 /* *mut c_void*/,
  pub _clicked_1: QGroupBox_clicked_signal,
  pub _toggled_1: QGroupBox_toggled_signal,
}

impl /*struct*/ QGroupBox {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QGroupBox {
    return QGroupBox{qbase: QWidget::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QGroupBox {
  type Target = QWidget;

  fn deref(&self) -> &QWidget {
    return & self.qbase;
  }
}
impl AsRef<QWidget> for QGroupBox {
  fn as_ref(& self) -> & QWidget {
    return & self.qbase;
  }
}
  // proto:  bool QGroupBox::isCheckable();
impl /*struct*/ QGroupBox {
  pub fn isCheckable<RetType, T: QGroupBox_isCheckable<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isCheckable(self);
    // return 1;
  }
}

pub trait QGroupBox_isCheckable<RetType> {
  fn isCheckable(self , rsthis: & QGroupBox) -> RetType;
}

  // proto:  bool QGroupBox::isCheckable();
impl<'a> /*trait*/ QGroupBox_isCheckable<i8> for () {
  fn isCheckable(self , rsthis: & QGroupBox) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QGroupBox11isCheckableEv()};
    let mut ret = unsafe {_ZNK9QGroupBox11isCheckableEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QGroupBox::setCheckable(bool checkable);
impl /*struct*/ QGroupBox {
  pub fn setCheckable<RetType, T: QGroupBox_setCheckable<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setCheckable(self);
    // return 1;
  }
}

pub trait QGroupBox_setCheckable<RetType> {
  fn setCheckable(self , rsthis: & QGroupBox) -> RetType;
}

  // proto:  void QGroupBox::setCheckable(bool checkable);
impl<'a> /*trait*/ QGroupBox_setCheckable<()> for (i8) {
  fn setCheckable(self , rsthis: & QGroupBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QGroupBox12setCheckableEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN9QGroupBox12setCheckableEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  const QMetaObject * QGroupBox::metaObject();
impl /*struct*/ QGroupBox {
  pub fn metaObject<RetType, T: QGroupBox_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QGroupBox_metaObject<RetType> {
  fn metaObject(self , rsthis: & QGroupBox) -> RetType;
}

  // proto:  const QMetaObject * QGroupBox::metaObject();
impl<'a> /*trait*/ QGroupBox_metaObject<()> for () {
  fn metaObject(self , rsthis: & QGroupBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QGroupBox10metaObjectEv()};
     unsafe {_ZNK9QGroupBox10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QGroupBox::isFlat();
impl /*struct*/ QGroupBox {
  pub fn isFlat<RetType, T: QGroupBox_isFlat<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isFlat(self);
    // return 1;
  }
}

pub trait QGroupBox_isFlat<RetType> {
  fn isFlat(self , rsthis: & QGroupBox) -> RetType;
}

  // proto:  bool QGroupBox::isFlat();
impl<'a> /*trait*/ QGroupBox_isFlat<i8> for () {
  fn isFlat(self , rsthis: & QGroupBox) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QGroupBox6isFlatEv()};
    let mut ret = unsafe {_ZNK9QGroupBox6isFlatEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QSize QGroupBox::minimumSizeHint();
impl /*struct*/ QGroupBox {
  pub fn minimumSizeHint<RetType, T: QGroupBox_minimumSizeHint<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.minimumSizeHint(self);
    // return 1;
  }
}

pub trait QGroupBox_minimumSizeHint<RetType> {
  fn minimumSizeHint(self , rsthis: & QGroupBox) -> RetType;
}

  // proto:  QSize QGroupBox::minimumSizeHint();
impl<'a> /*trait*/ QGroupBox_minimumSizeHint<QSize> for () {
  fn minimumSizeHint(self , rsthis: & QGroupBox) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QGroupBox15minimumSizeHintEv()};
    let mut ret = unsafe {_ZNK9QGroupBox15minimumSizeHintEv(rsthis.qclsinst)};
    let mut ret1 = QSize::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QGroupBox::setFlat(bool flat);
impl /*struct*/ QGroupBox {
  pub fn setFlat<RetType, T: QGroupBox_setFlat<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setFlat(self);
    // return 1;
  }
}

pub trait QGroupBox_setFlat<RetType> {
  fn setFlat(self , rsthis: & QGroupBox) -> RetType;
}

  // proto:  void QGroupBox::setFlat(bool flat);
impl<'a> /*trait*/ QGroupBox_setFlat<()> for (i8) {
  fn setFlat(self , rsthis: & QGroupBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QGroupBox7setFlatEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN9QGroupBox7setFlatEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QGroupBox::~QGroupBox();
impl /*struct*/ QGroupBox {
  pub fn free<RetType, T: QGroupBox_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QGroupBox_free<RetType> {
  fn free(self , rsthis: & QGroupBox) -> RetType;
}

  // proto:  void QGroupBox::~QGroupBox();
impl<'a> /*trait*/ QGroupBox_free<()> for () {
  fn free(self , rsthis: & QGroupBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QGroupBoxD0Ev()};
     unsafe {_ZN9QGroupBoxD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QGroupBox::QGroupBox(QWidget * parent);
impl /*struct*/ QGroupBox {
  pub fn new<T: QGroupBox_new>(value: T) -> QGroupBox {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QGroupBox_new {
  fn new(self) -> QGroupBox;
}

  // proto:  void QGroupBox::QGroupBox(QWidget * parent);
impl<'a> /*trait*/ QGroupBox_new for (&'a QWidget) {
  fn new(self) -> QGroupBox {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QGroupBoxC1EP7QWidget()};
    let ctysz: c_int = unsafe{QGroupBox_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN9QGroupBoxC1EP7QWidget(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN9QGroupBoxC1EP7QWidget(arg0)} as u64;
    let rsthis = QGroupBox{qbase: QWidget::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QGroupBox::toggled(bool );
impl /*struct*/ QGroupBox {
  pub fn toggled<RetType, T: QGroupBox_toggled<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toggled(self);
    // return 1;
  }
}

pub trait QGroupBox_toggled<RetType> {
  fn toggled(self , rsthis: & QGroupBox) -> RetType;
}

  // proto:  void QGroupBox::toggled(bool );
impl<'a> /*trait*/ QGroupBox_toggled<()> for (i8) {
  fn toggled(self , rsthis: & QGroupBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QGroupBox7toggledEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN9QGroupBox7toggledEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QGroupBox::isChecked();
impl /*struct*/ QGroupBox {
  pub fn isChecked<RetType, T: QGroupBox_isChecked<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isChecked(self);
    // return 1;
  }
}

pub trait QGroupBox_isChecked<RetType> {
  fn isChecked(self , rsthis: & QGroupBox) -> RetType;
}

  // proto:  bool QGroupBox::isChecked();
impl<'a> /*trait*/ QGroupBox_isChecked<i8> for () {
  fn isChecked(self , rsthis: & QGroupBox) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QGroupBox9isCheckedEv()};
    let mut ret = unsafe {_ZNK9QGroupBox9isCheckedEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QGroupBox::setChecked(bool checked);
impl /*struct*/ QGroupBox {
  pub fn setChecked<RetType, T: QGroupBox_setChecked<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setChecked(self);
    // return 1;
  }
}

pub trait QGroupBox_setChecked<RetType> {
  fn setChecked(self , rsthis: & QGroupBox) -> RetType;
}

  // proto:  void QGroupBox::setChecked(bool checked);
impl<'a> /*trait*/ QGroupBox_setChecked<()> for (i8) {
  fn setChecked(self , rsthis: & QGroupBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QGroupBox10setCheckedEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN9QGroupBox10setCheckedEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QGroupBox::QGroupBox(const QGroupBox & );
impl<'a> /*trait*/ QGroupBox_new for (&'a QGroupBox) {
  fn new(self) -> QGroupBox {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QGroupBoxC1ERKS_()};
    let ctysz: c_int = unsafe{QGroupBox_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN9QGroupBoxC1ERKS_(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN9QGroupBoxC1ERKS_(arg0)} as u64;
    let rsthis = QGroupBox{qbase: QWidget::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  QString QGroupBox::title();
impl /*struct*/ QGroupBox {
  pub fn title<RetType, T: QGroupBox_title<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.title(self);
    // return 1;
  }
}

pub trait QGroupBox_title<RetType> {
  fn title(self , rsthis: & QGroupBox) -> RetType;
}

  // proto:  QString QGroupBox::title();
impl<'a> /*trait*/ QGroupBox_title<QString> for () {
  fn title(self , rsthis: & QGroupBox) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QGroupBox5titleEv()};
    let mut ret = unsafe {_ZNK9QGroupBox5titleEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QGroupBox::setAlignment(int alignment);
impl /*struct*/ QGroupBox {
  pub fn setAlignment<RetType, T: QGroupBox_setAlignment<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setAlignment(self);
    // return 1;
  }
}

pub trait QGroupBox_setAlignment<RetType> {
  fn setAlignment(self , rsthis: & QGroupBox) -> RetType;
}

  // proto:  void QGroupBox::setAlignment(int alignment);
impl<'a> /*trait*/ QGroupBox_setAlignment<()> for (i32) {
  fn setAlignment(self , rsthis: & QGroupBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QGroupBox12setAlignmentEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN9QGroupBox12setAlignmentEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QGroupBox::setTitle(const QString & title);
impl /*struct*/ QGroupBox {
  pub fn setTitle<RetType, T: QGroupBox_setTitle<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setTitle(self);
    // return 1;
  }
}

pub trait QGroupBox_setTitle<RetType> {
  fn setTitle(self , rsthis: & QGroupBox) -> RetType;
}

  // proto:  void QGroupBox::setTitle(const QString & title);
impl<'a> /*trait*/ QGroupBox_setTitle<()> for (&'a QString) {
  fn setTitle(self , rsthis: & QGroupBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QGroupBox8setTitleERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QGroupBox8setTitleERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QGroupBox::QGroupBox(const QString & title, QWidget * parent);
impl<'a> /*trait*/ QGroupBox_new for (&'a QString, &'a QWidget) {
  fn new(self) -> QGroupBox {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QGroupBoxC1ERK7QStringP7QWidget()};
    let ctysz: c_int = unsafe{QGroupBox_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    // unsafe {_ZN9QGroupBoxC1ERK7QStringP7QWidget(qthis, arg0, arg1)};
    let qthis: u64 = unsafe {dector_ZN9QGroupBoxC1ERK7QStringP7QWidget(arg0, arg1)} as u64;
    let rsthis = QGroupBox{qbase: QWidget::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QGroupBox::clicked(bool checked);
impl /*struct*/ QGroupBox {
  pub fn clicked<RetType, T: QGroupBox_clicked<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.clicked(self);
    // return 1;
  }
}

pub trait QGroupBox_clicked<RetType> {
  fn clicked(self , rsthis: & QGroupBox) -> RetType;
}

  // proto:  void QGroupBox::clicked(bool checked);
impl<'a> /*trait*/ QGroupBox_clicked<()> for (i8) {
  fn clicked(self , rsthis: & QGroupBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QGroupBox7clickedEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN9QGroupBox7clickedEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

#[derive(Default)] // for QGroupBox_clicked
pub struct QGroupBox_clicked_signal{poi:u64}
impl /* struct */ QGroupBox {
  pub fn clicked_1(&self) -> QGroupBox_clicked_signal {
     return QGroupBox_clicked_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QGroupBox_clicked_signal {
  pub fn connect<T: QGroupBox_clicked_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QGroupBox_clicked_signal_connect {
  fn connect(self, sigthis: QGroupBox_clicked_signal);
}

#[derive(Default)] // for QGroupBox_toggled
pub struct QGroupBox_toggled_signal{poi:u64}
impl /* struct */ QGroupBox {
  pub fn toggled_1(&self) -> QGroupBox_toggled_signal {
     return QGroupBox_toggled_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QGroupBox_toggled_signal {
  pub fn connect<T: QGroupBox_toggled_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QGroupBox_toggled_signal_connect {
  fn connect(self, sigthis: QGroupBox_toggled_signal);
}

// clicked(_Bool)
extern fn QGroupBox_clicked_signal_connect_cb_0(rsfptr:fn(i8), arg0: c_char) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = arg0 as i8;
  rsfptr(rsarg0);
}
extern fn QGroupBox_clicked_signal_connect_cb_box_0(rsfptr_raw:*mut Fn(i8), arg0: c_char) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = arg0 as i8;
  rsfptr(rsarg0);
}
impl /* trait */ QGroupBox_clicked_signal_connect for fn(i8) {
  fn connect(self, sigthis: QGroupBox_clicked_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QGroupBox_clicked_signal_connect_cb_0 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QGroupBox_SlotProxy_connect__ZN9QGroupBox7clickedEb(arg0, arg1, arg2)};
  }
}
impl /* trait */ QGroupBox_clicked_signal_connect for Box<Fn(i8)> {
  fn connect(self, sigthis: QGroupBox_clicked_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QGroupBox_clicked_signal_connect_cb_box_0 as *mut c_void;
    let arg2 = Box::into_raw(self) as *mut c_void;
    unsafe {QGroupBox_SlotProxy_connect__ZN9QGroupBox7clickedEb(arg0, arg1, arg2)};
  }
}
// toggled(_Bool)
extern fn QGroupBox_toggled_signal_connect_cb_1(rsfptr:fn(i8), arg0: c_char) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = arg0 as i8;
  rsfptr(rsarg0);
}
extern fn QGroupBox_toggled_signal_connect_cb_box_1(rsfptr_raw:*mut Fn(i8), arg0: c_char) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = arg0 as i8;
  rsfptr(rsarg0);
}
impl /* trait */ QGroupBox_toggled_signal_connect for fn(i8) {
  fn connect(self, sigthis: QGroupBox_toggled_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QGroupBox_toggled_signal_connect_cb_1 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QGroupBox_SlotProxy_connect__ZN9QGroupBox7toggledEb(arg0, arg1, arg2)};
  }
}
impl /* trait */ QGroupBox_toggled_signal_connect for Box<Fn(i8)> {
  fn connect(self, sigthis: QGroupBox_toggled_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QGroupBox_toggled_signal_connect_cb_box_1 as *mut c_void;
    let arg2 = Box::into_raw(self) as *mut c_void;
    unsafe {QGroupBox_SlotProxy_connect__ZN9QGroupBox7toggledEb(arg0, arg1, arg2)};
  }
}
// <= body block end

