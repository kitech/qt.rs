// auto generated, do not modify.
// created: Wed Dec 30 23:22:52 2015
// src-file: /QtWidgets/qformlayout.h
// dst-file: /src/widgets/qformlayout.rs
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
use super::qlayout::QLayout; // 773
use std::ops::Deref;
use super::qlayoutitem::QLayoutItem; // 773
use super::qwidget::QWidget; // 773
use super::super::core::qstring::QString; // 771
use super::super::core::qrect::QRect; // 771
use super::super::core::qsize::QSize; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QFormLayout_Class_Size() -> c_int;
  // proto:  int QFormLayout::horizontalSpacing();
  fn _ZNK11QFormLayout17horizontalSpacingEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  int QFormLayout::rowCount();
  fn _ZNK11QFormLayout8rowCountEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  QWidget * QFormLayout::labelForField(QLayout * field);
  fn _ZNK11QFormLayout13labelForFieldEP7QLayout(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QFormLayout::addRow(const QString & labelText, QLayout * field);
  fn _ZN11QFormLayout6addRowERK7QStringP7QLayout(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  void QFormLayout::insertRow(int row, const QString & labelText, QLayout * field);
  fn _ZN11QFormLayout9insertRowEiRK7QStringP7QLayout(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: *mut c_void, arg2: *mut c_void);
  // proto:  QWidget * QFormLayout::labelForField(QWidget * field);
  fn _ZNK11QFormLayout13labelForFieldEP7QWidget(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QFormLayout::insertRow(int row, QWidget * label, QLayout * field);
  fn _ZN11QFormLayout9insertRowEiP7QWidgetP7QLayout(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: *mut c_void, arg2: *mut c_void);
  // proto:  int QFormLayout::count();
  fn _ZNK11QFormLayout5countEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  int QFormLayout::spacing();
  fn _ZNK11QFormLayout7spacingEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QFormLayout::QFormLayout(QWidget * parent);
  fn dector_ZN11QFormLayoutC1EP7QWidget(arg0: *mut c_void) -> *mut c_void;
  fn _ZN11QFormLayoutC1EP7QWidget(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QFormLayout::insertRow(int row, QLayout * layout);
  fn _ZN11QFormLayout9insertRowEiP7QLayout(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: *mut c_void);
  // proto:  void QFormLayout::setGeometry(const QRect & rect);
  fn _ZN11QFormLayout11setGeometryERK5QRect(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QFormLayout::setVerticalSpacing(int spacing);
  fn _ZN11QFormLayout18setVerticalSpacingEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  void QFormLayout::setHorizontalSpacing(int spacing);
  fn _ZN11QFormLayout20setHorizontalSpacingEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  void QFormLayout::insertRow(int row, const QString & labelText, QWidget * field);
  fn _ZN11QFormLayout9insertRowEiRK7QStringP7QWidget(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: *mut c_void, arg2: *mut c_void);
  // proto:  const QMetaObject * QFormLayout::metaObject();
  fn _ZNK11QFormLayout10metaObjectEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QFormLayout::insertRow(int row, QWidget * label, QWidget * field);
  fn _ZN11QFormLayout9insertRowEiP7QWidgetS1_(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: *mut c_void, arg2: *mut c_void);
  // proto:  void QFormLayout::setSpacing(int );
  fn _ZN11QFormLayout10setSpacingEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  void QFormLayout::~QFormLayout();
  fn _ZN11QFormLayoutD0Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QFormLayout::addRow(QLayout * layout);
  fn _ZN11QFormLayout6addRowEP7QLayout(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QSize QFormLayout::sizeHint();
  fn _ZNK11QFormLayout8sizeHintEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QFormLayout::invalidate();
  fn _ZN11QFormLayout10invalidateEv(qthis: u64 /* *mut c_void*/);
  // proto:  QLayoutItem * QFormLayout::itemAt(int index);
  fn _ZNK11QFormLayout6itemAtEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> *mut c_void;
  // proto:  QLayoutItem * QFormLayout::takeAt(int index);
  fn _ZN11QFormLayout6takeAtEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> *mut c_void;
  // proto:  void QFormLayout::addRow(const QString & labelText, QWidget * field);
  fn _ZN11QFormLayout6addRowERK7QStringP7QWidget(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  QSize QFormLayout::minimumSize();
  fn _ZNK11QFormLayout11minimumSizeEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QFormLayout::addRow(QWidget * widget);
  fn _ZN11QFormLayout6addRowEP7QWidget(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QFormLayout::addRow(QWidget * label, QLayout * field);
  fn _ZN11QFormLayout6addRowEP7QWidgetP7QLayout(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  int QFormLayout::verticalSpacing();
  fn _ZNK11QFormLayout15verticalSpacingEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  int QFormLayout::heightForWidth(int width);
  fn _ZNK11QFormLayout14heightForWidthEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> c_int;
  // proto:  void QFormLayout::addItem(QLayoutItem * item);
  fn _ZN11QFormLayout7addItemEP11QLayoutItem(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  bool QFormLayout::hasHeightForWidth();
  fn _ZNK11QFormLayout17hasHeightForWidthEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QFormLayout::insertRow(int row, QWidget * widget);
  fn _ZN11QFormLayout9insertRowEiP7QWidget(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: *mut c_void);
  // proto:  void QFormLayout::addRow(QWidget * label, QWidget * field);
  fn _ZN11QFormLayout6addRowEP7QWidgetS1_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QFormLayout)=1
#[derive(Default)]
pub struct QFormLayout {
  qbase: QLayout,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QFormLayout {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QFormLayout {
    return QFormLayout{qbase: QLayout::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QFormLayout {
  type Target = QLayout;

  fn deref(&self) -> &QLayout {
    return & self.qbase;
  }
}
impl AsRef<QLayout> for QFormLayout {
  fn as_ref(& self) -> & QLayout {
    return & self.qbase;
  }
}
  // proto:  int QFormLayout::horizontalSpacing();
impl /*struct*/ QFormLayout {
  pub fn horizontalSpacing<RetType, T: QFormLayout_horizontalSpacing<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.horizontalSpacing(self);
    // return 1;
  }
}

pub trait QFormLayout_horizontalSpacing<RetType> {
  fn horizontalSpacing(self , rsthis: & QFormLayout) -> RetType;
}

  // proto:  int QFormLayout::horizontalSpacing();
impl<'a> /*trait*/ QFormLayout_horizontalSpacing<i32> for () {
  fn horizontalSpacing(self , rsthis: & QFormLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QFormLayout17horizontalSpacingEv()};
    let mut ret = unsafe {_ZNK11QFormLayout17horizontalSpacingEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  int QFormLayout::rowCount();
impl /*struct*/ QFormLayout {
  pub fn rowCount<RetType, T: QFormLayout_rowCount<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.rowCount(self);
    // return 1;
  }
}

pub trait QFormLayout_rowCount<RetType> {
  fn rowCount(self , rsthis: & QFormLayout) -> RetType;
}

  // proto:  int QFormLayout::rowCount();
impl<'a> /*trait*/ QFormLayout_rowCount<i32> for () {
  fn rowCount(self , rsthis: & QFormLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QFormLayout8rowCountEv()};
    let mut ret = unsafe {_ZNK11QFormLayout8rowCountEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  QWidget * QFormLayout::labelForField(QLayout * field);
impl /*struct*/ QFormLayout {
  pub fn labelForField<RetType, T: QFormLayout_labelForField<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.labelForField(self);
    // return 1;
  }
}

pub trait QFormLayout_labelForField<RetType> {
  fn labelForField(self , rsthis: & QFormLayout) -> RetType;
}

  // proto:  QWidget * QFormLayout::labelForField(QLayout * field);
impl<'a> /*trait*/ QFormLayout_labelForField<QWidget> for (&'a QLayout) {
  fn labelForField(self , rsthis: & QFormLayout) -> QWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QFormLayout13labelForFieldEP7QLayout()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK11QFormLayout13labelForFieldEP7QLayout(rsthis.qclsinst, arg0)};
    let mut ret1 = QWidget::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QFormLayout::addRow(const QString & labelText, QLayout * field);
impl /*struct*/ QFormLayout {
  pub fn addRow<RetType, T: QFormLayout_addRow<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.addRow(self);
    // return 1;
  }
}

pub trait QFormLayout_addRow<RetType> {
  fn addRow(self , rsthis: & QFormLayout) -> RetType;
}

  // proto:  void QFormLayout::addRow(const QString & labelText, QLayout * field);
impl<'a> /*trait*/ QFormLayout_addRow<()> for (&'a QString, &'a QLayout) {
  fn addRow(self , rsthis: & QFormLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFormLayout6addRowERK7QStringP7QLayout()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN11QFormLayout6addRowERK7QStringP7QLayout(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QFormLayout::insertRow(int row, const QString & labelText, QLayout * field);
impl /*struct*/ QFormLayout {
  pub fn insertRow<RetType, T: QFormLayout_insertRow<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.insertRow(self);
    // return 1;
  }
}

pub trait QFormLayout_insertRow<RetType> {
  fn insertRow(self , rsthis: & QFormLayout) -> RetType;
}

  // proto:  void QFormLayout::insertRow(int row, const QString & labelText, QLayout * field);
impl<'a> /*trait*/ QFormLayout_insertRow<()> for (i32, &'a QString, &'a QLayout) {
  fn insertRow(self , rsthis: & QFormLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFormLayout9insertRowEiRK7QStringP7QLayout()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
     unsafe {_ZN11QFormLayout9insertRowEiRK7QStringP7QLayout(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  QWidget * QFormLayout::labelForField(QWidget * field);
impl<'a> /*trait*/ QFormLayout_labelForField<QWidget> for (&'a QWidget) {
  fn labelForField(self , rsthis: & QFormLayout) -> QWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QFormLayout13labelForFieldEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK11QFormLayout13labelForFieldEP7QWidget(rsthis.qclsinst, arg0)};
    let mut ret1 = QWidget::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QFormLayout::insertRow(int row, QWidget * label, QLayout * field);
impl<'a> /*trait*/ QFormLayout_insertRow<()> for (i32, &'a QWidget, &'a QLayout) {
  fn insertRow(self , rsthis: & QFormLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFormLayout9insertRowEiP7QWidgetP7QLayout()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
     unsafe {_ZN11QFormLayout9insertRowEiP7QWidgetP7QLayout(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  int QFormLayout::count();
impl /*struct*/ QFormLayout {
  pub fn count<RetType, T: QFormLayout_count<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.count(self);
    // return 1;
  }
}

pub trait QFormLayout_count<RetType> {
  fn count(self , rsthis: & QFormLayout) -> RetType;
}

  // proto:  int QFormLayout::count();
impl<'a> /*trait*/ QFormLayout_count<i32> for () {
  fn count(self , rsthis: & QFormLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QFormLayout5countEv()};
    let mut ret = unsafe {_ZNK11QFormLayout5countEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  int QFormLayout::spacing();
impl /*struct*/ QFormLayout {
  pub fn spacing<RetType, T: QFormLayout_spacing<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.spacing(self);
    // return 1;
  }
}

pub trait QFormLayout_spacing<RetType> {
  fn spacing(self , rsthis: & QFormLayout) -> RetType;
}

  // proto:  int QFormLayout::spacing();
impl<'a> /*trait*/ QFormLayout_spacing<i32> for () {
  fn spacing(self , rsthis: & QFormLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QFormLayout7spacingEv()};
    let mut ret = unsafe {_ZNK11QFormLayout7spacingEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QFormLayout::QFormLayout(QWidget * parent);
impl /*struct*/ QFormLayout {
  pub fn New<T: QFormLayout_New>(value: T) -> QFormLayout {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QFormLayout_New {
  fn New(self) -> QFormLayout;
}

  // proto:  void QFormLayout::QFormLayout(QWidget * parent);
impl<'a> /*trait*/ QFormLayout_New for (&'a QWidget) {
  fn New(self) -> QFormLayout {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFormLayoutC1EP7QWidget()};
    let ctysz: c_int = unsafe{QFormLayout_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN11QFormLayoutC1EP7QWidget(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN11QFormLayoutC1EP7QWidget(arg0)} as u64;
    let rsthis = QFormLayout{qbase: QLayout::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QFormLayout::insertRow(int row, QLayout * layout);
impl<'a> /*trait*/ QFormLayout_insertRow<()> for (i32, &'a QLayout) {
  fn insertRow(self , rsthis: & QFormLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFormLayout9insertRowEiP7QLayout()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN11QFormLayout9insertRowEiP7QLayout(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QFormLayout::setGeometry(const QRect & rect);
impl /*struct*/ QFormLayout {
  pub fn setGeometry<RetType, T: QFormLayout_setGeometry<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setGeometry(self);
    // return 1;
  }
}

pub trait QFormLayout_setGeometry<RetType> {
  fn setGeometry(self , rsthis: & QFormLayout) -> RetType;
}

  // proto:  void QFormLayout::setGeometry(const QRect & rect);
impl<'a> /*trait*/ QFormLayout_setGeometry<()> for (&'a QRect) {
  fn setGeometry(self , rsthis: & QFormLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFormLayout11setGeometryERK5QRect()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QFormLayout11setGeometryERK5QRect(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QFormLayout::setVerticalSpacing(int spacing);
impl /*struct*/ QFormLayout {
  pub fn setVerticalSpacing<RetType, T: QFormLayout_setVerticalSpacing<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setVerticalSpacing(self);
    // return 1;
  }
}

pub trait QFormLayout_setVerticalSpacing<RetType> {
  fn setVerticalSpacing(self , rsthis: & QFormLayout) -> RetType;
}

  // proto:  void QFormLayout::setVerticalSpacing(int spacing);
impl<'a> /*trait*/ QFormLayout_setVerticalSpacing<()> for (i32) {
  fn setVerticalSpacing(self , rsthis: & QFormLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFormLayout18setVerticalSpacingEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN11QFormLayout18setVerticalSpacingEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QFormLayout::setHorizontalSpacing(int spacing);
impl /*struct*/ QFormLayout {
  pub fn setHorizontalSpacing<RetType, T: QFormLayout_setHorizontalSpacing<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setHorizontalSpacing(self);
    // return 1;
  }
}

pub trait QFormLayout_setHorizontalSpacing<RetType> {
  fn setHorizontalSpacing(self , rsthis: & QFormLayout) -> RetType;
}

  // proto:  void QFormLayout::setHorizontalSpacing(int spacing);
impl<'a> /*trait*/ QFormLayout_setHorizontalSpacing<()> for (i32) {
  fn setHorizontalSpacing(self , rsthis: & QFormLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFormLayout20setHorizontalSpacingEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN11QFormLayout20setHorizontalSpacingEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QFormLayout::insertRow(int row, const QString & labelText, QWidget * field);
impl<'a> /*trait*/ QFormLayout_insertRow<()> for (i32, &'a QString, &'a QWidget) {
  fn insertRow(self , rsthis: & QFormLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFormLayout9insertRowEiRK7QStringP7QWidget()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
     unsafe {_ZN11QFormLayout9insertRowEiRK7QStringP7QWidget(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  const QMetaObject * QFormLayout::metaObject();
impl /*struct*/ QFormLayout {
  pub fn metaObject<RetType, T: QFormLayout_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QFormLayout_metaObject<RetType> {
  fn metaObject(self , rsthis: & QFormLayout) -> RetType;
}

  // proto:  const QMetaObject * QFormLayout::metaObject();
impl<'a> /*trait*/ QFormLayout_metaObject<()> for () {
  fn metaObject(self , rsthis: & QFormLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QFormLayout10metaObjectEv()};
     unsafe {_ZNK11QFormLayout10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QFormLayout::insertRow(int row, QWidget * label, QWidget * field);
impl<'a> /*trait*/ QFormLayout_insertRow<()> for (i32, &'a QWidget, &'a QWidget) {
  fn insertRow(self , rsthis: & QFormLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFormLayout9insertRowEiP7QWidgetS1_()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
     unsafe {_ZN11QFormLayout9insertRowEiP7QWidgetS1_(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  void QFormLayout::setSpacing(int );
impl /*struct*/ QFormLayout {
  pub fn setSpacing<RetType, T: QFormLayout_setSpacing<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setSpacing(self);
    // return 1;
  }
}

pub trait QFormLayout_setSpacing<RetType> {
  fn setSpacing(self , rsthis: & QFormLayout) -> RetType;
}

  // proto:  void QFormLayout::setSpacing(int );
impl<'a> /*trait*/ QFormLayout_setSpacing<()> for (i32) {
  fn setSpacing(self , rsthis: & QFormLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFormLayout10setSpacingEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN11QFormLayout10setSpacingEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QFormLayout::~QFormLayout();
impl /*struct*/ QFormLayout {
  pub fn Free<RetType, T: QFormLayout_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QFormLayout_Free<RetType> {
  fn Free(self , rsthis: & QFormLayout) -> RetType;
}

  // proto:  void QFormLayout::~QFormLayout();
impl<'a> /*trait*/ QFormLayout_Free<()> for () {
  fn Free(self , rsthis: & QFormLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFormLayoutD0Ev()};
     unsafe {_ZN11QFormLayoutD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QFormLayout::addRow(QLayout * layout);
impl<'a> /*trait*/ QFormLayout_addRow<()> for (&'a QLayout) {
  fn addRow(self , rsthis: & QFormLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFormLayout6addRowEP7QLayout()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QFormLayout6addRowEP7QLayout(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QSize QFormLayout::sizeHint();
impl /*struct*/ QFormLayout {
  pub fn sizeHint<RetType, T: QFormLayout_sizeHint<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.sizeHint(self);
    // return 1;
  }
}

pub trait QFormLayout_sizeHint<RetType> {
  fn sizeHint(self , rsthis: & QFormLayout) -> RetType;
}

  // proto:  QSize QFormLayout::sizeHint();
impl<'a> /*trait*/ QFormLayout_sizeHint<QSize> for () {
  fn sizeHint(self , rsthis: & QFormLayout) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QFormLayout8sizeHintEv()};
    let mut ret = unsafe {_ZNK11QFormLayout8sizeHintEv(rsthis.qclsinst)};
    let mut ret1 = QSize::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QFormLayout::invalidate();
impl /*struct*/ QFormLayout {
  pub fn invalidate<RetType, T: QFormLayout_invalidate<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.invalidate(self);
    // return 1;
  }
}

pub trait QFormLayout_invalidate<RetType> {
  fn invalidate(self , rsthis: & QFormLayout) -> RetType;
}

  // proto:  void QFormLayout::invalidate();
impl<'a> /*trait*/ QFormLayout_invalidate<()> for () {
  fn invalidate(self , rsthis: & QFormLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFormLayout10invalidateEv()};
     unsafe {_ZN11QFormLayout10invalidateEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QLayoutItem * QFormLayout::itemAt(int index);
impl /*struct*/ QFormLayout {
  pub fn itemAt<RetType, T: QFormLayout_itemAt<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.itemAt(self);
    // return 1;
  }
}

pub trait QFormLayout_itemAt<RetType> {
  fn itemAt(self , rsthis: & QFormLayout) -> RetType;
}

  // proto:  QLayoutItem * QFormLayout::itemAt(int index);
impl<'a> /*trait*/ QFormLayout_itemAt<QLayoutItem> for (i32) {
  fn itemAt(self , rsthis: & QFormLayout) -> QLayoutItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QFormLayout6itemAtEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK11QFormLayout6itemAtEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QLayoutItem::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QLayoutItem * QFormLayout::takeAt(int index);
impl /*struct*/ QFormLayout {
  pub fn takeAt<RetType, T: QFormLayout_takeAt<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.takeAt(self);
    // return 1;
  }
}

pub trait QFormLayout_takeAt<RetType> {
  fn takeAt(self , rsthis: & QFormLayout) -> RetType;
}

  // proto:  QLayoutItem * QFormLayout::takeAt(int index);
impl<'a> /*trait*/ QFormLayout_takeAt<QLayoutItem> for (i32) {
  fn takeAt(self , rsthis: & QFormLayout) -> QLayoutItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFormLayout6takeAtEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZN11QFormLayout6takeAtEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QLayoutItem::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QFormLayout::addRow(const QString & labelText, QWidget * field);
impl<'a> /*trait*/ QFormLayout_addRow<()> for (&'a QString, &'a QWidget) {
  fn addRow(self , rsthis: & QFormLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFormLayout6addRowERK7QStringP7QWidget()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN11QFormLayout6addRowERK7QStringP7QWidget(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  QSize QFormLayout::minimumSize();
impl /*struct*/ QFormLayout {
  pub fn minimumSize<RetType, T: QFormLayout_minimumSize<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.minimumSize(self);
    // return 1;
  }
}

pub trait QFormLayout_minimumSize<RetType> {
  fn minimumSize(self , rsthis: & QFormLayout) -> RetType;
}

  // proto:  QSize QFormLayout::minimumSize();
impl<'a> /*trait*/ QFormLayout_minimumSize<QSize> for () {
  fn minimumSize(self , rsthis: & QFormLayout) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QFormLayout11minimumSizeEv()};
    let mut ret = unsafe {_ZNK11QFormLayout11minimumSizeEv(rsthis.qclsinst)};
    let mut ret1 = QSize::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QFormLayout::addRow(QWidget * widget);
impl<'a> /*trait*/ QFormLayout_addRow<()> for (&'a QWidget) {
  fn addRow(self , rsthis: & QFormLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFormLayout6addRowEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QFormLayout6addRowEP7QWidget(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QFormLayout::addRow(QWidget * label, QLayout * field);
impl<'a> /*trait*/ QFormLayout_addRow<()> for (&'a QWidget, &'a QLayout) {
  fn addRow(self , rsthis: & QFormLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFormLayout6addRowEP7QWidgetP7QLayout()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN11QFormLayout6addRowEP7QWidgetP7QLayout(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  int QFormLayout::verticalSpacing();
impl /*struct*/ QFormLayout {
  pub fn verticalSpacing<RetType, T: QFormLayout_verticalSpacing<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.verticalSpacing(self);
    // return 1;
  }
}

pub trait QFormLayout_verticalSpacing<RetType> {
  fn verticalSpacing(self , rsthis: & QFormLayout) -> RetType;
}

  // proto:  int QFormLayout::verticalSpacing();
impl<'a> /*trait*/ QFormLayout_verticalSpacing<i32> for () {
  fn verticalSpacing(self , rsthis: & QFormLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QFormLayout15verticalSpacingEv()};
    let mut ret = unsafe {_ZNK11QFormLayout15verticalSpacingEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  int QFormLayout::heightForWidth(int width);
impl /*struct*/ QFormLayout {
  pub fn heightForWidth<RetType, T: QFormLayout_heightForWidth<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.heightForWidth(self);
    // return 1;
  }
}

pub trait QFormLayout_heightForWidth<RetType> {
  fn heightForWidth(self , rsthis: & QFormLayout) -> RetType;
}

  // proto:  int QFormLayout::heightForWidth(int width);
impl<'a> /*trait*/ QFormLayout_heightForWidth<i32> for (i32) {
  fn heightForWidth(self , rsthis: & QFormLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QFormLayout14heightForWidthEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK11QFormLayout14heightForWidthEi(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QFormLayout::addItem(QLayoutItem * item);
impl /*struct*/ QFormLayout {
  pub fn addItem<RetType, T: QFormLayout_addItem<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.addItem(self);
    // return 1;
  }
}

pub trait QFormLayout_addItem<RetType> {
  fn addItem(self , rsthis: & QFormLayout) -> RetType;
}

  // proto:  void QFormLayout::addItem(QLayoutItem * item);
impl<'a> /*trait*/ QFormLayout_addItem<()> for (&'a QLayoutItem) {
  fn addItem(self , rsthis: & QFormLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFormLayout7addItemEP11QLayoutItem()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QFormLayout7addItemEP11QLayoutItem(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QFormLayout::hasHeightForWidth();
impl /*struct*/ QFormLayout {
  pub fn hasHeightForWidth<RetType, T: QFormLayout_hasHeightForWidth<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.hasHeightForWidth(self);
    // return 1;
  }
}

pub trait QFormLayout_hasHeightForWidth<RetType> {
  fn hasHeightForWidth(self , rsthis: & QFormLayout) -> RetType;
}

  // proto:  bool QFormLayout::hasHeightForWidth();
impl<'a> /*trait*/ QFormLayout_hasHeightForWidth<i8> for () {
  fn hasHeightForWidth(self , rsthis: & QFormLayout) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QFormLayout17hasHeightForWidthEv()};
    let mut ret = unsafe {_ZNK11QFormLayout17hasHeightForWidthEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QFormLayout::insertRow(int row, QWidget * widget);
impl<'a> /*trait*/ QFormLayout_insertRow<()> for (i32, &'a QWidget) {
  fn insertRow(self , rsthis: & QFormLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFormLayout9insertRowEiP7QWidget()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN11QFormLayout9insertRowEiP7QWidget(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QFormLayout::addRow(QWidget * label, QWidget * field);
impl<'a> /*trait*/ QFormLayout_addRow<()> for (&'a QWidget, &'a QWidget) {
  fn addRow(self , rsthis: & QFormLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFormLayout6addRowEP7QWidgetS1_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN11QFormLayout6addRowEP7QWidgetS1_(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

// <= body block end

