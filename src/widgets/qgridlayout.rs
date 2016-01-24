// auto generated, do not modify.
// created: Sun Jan 24 17:41:38 2016
// src-file: /QtWidgets/qgridlayout.h
// dst-file: /src/widgets/qgridlayout.rs
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
use super::super::core::qsize::QSize; // 771
use super::super::core::qrect::QRect; // 771
use super::qwidget::QWidget; // 773
use super::super::core::qobjectdefs::QMetaObject; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QGridLayout_Class_Size() -> c_int;
  // proto:  void QGridLayout::setRowMinimumHeight(int row, int minSize);
  fn C_ZN11QGridLayout19setRowMinimumHeightEii(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_int);
  // proto:  QLayoutItem * QGridLayout::takeAt(int index);
  fn C_ZN11QGridLayout6takeAtEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> *mut c_void;
  // proto:  void QGridLayout::getItemPosition(int idx, int * row, int * column, int * rowSpan, int * columnSpan);
  fn C_ZNK11QGridLayout15getItemPositionEiPiS0_S0_S0_(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: *mut c_int, arg2: *mut c_int, arg3: *mut c_int, arg4: *mut c_int);
  // proto:  int QGridLayout::minimumHeightForWidth(int );
  fn C_ZNK11QGridLayout21minimumHeightForWidthEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> c_int;
  // proto:  int QGridLayout::rowMinimumHeight(int row);
  fn C_ZNK11QGridLayout16rowMinimumHeightEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> c_int;
  // proto:  void QGridLayout::invalidate();
  fn C_ZN11QGridLayout10invalidateEv(qthis: u64 /* *mut c_void*/);
  // proto:  int QGridLayout::count();
  fn C_ZNK11QGridLayout5countEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QGridLayout::setColumnStretch(int column, int stretch);
  fn C_ZN11QGridLayout16setColumnStretchEii(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_int);
  // proto:  int QGridLayout::spacing();
  fn C_ZNK11QGridLayout7spacingEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  int QGridLayout::rowStretch(int row);
  fn C_ZNK11QGridLayout10rowStretchEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> c_int;
  // proto:  QSize QGridLayout::sizeHint();
  fn C_ZNK11QGridLayout8sizeHintEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  int QGridLayout::rowCount();
  fn C_ZNK11QGridLayout8rowCountEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QGridLayout::setGeometry(const QRect & );
  fn C_ZN11QGridLayout11setGeometryERK5QRect(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QGridLayout::setVerticalSpacing(int spacing);
  fn C_ZN11QGridLayout18setVerticalSpacingEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  void QGridLayout::setHorizontalSpacing(int spacing);
  fn C_ZN11QGridLayout20setHorizontalSpacingEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  int QGridLayout::columnStretch(int column);
  fn C_ZNK11QGridLayout13columnStretchEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> c_int;
  // proto:  int QGridLayout::columnCount();
  fn C_ZNK11QGridLayout11columnCountEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  int QGridLayout::columnMinimumWidth(int column);
  fn C_ZNK11QGridLayout18columnMinimumWidthEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> c_int;
  // proto:  QSize QGridLayout::minimumSize();
  fn C_ZNK11QGridLayout11minimumSizeEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  bool QGridLayout::hasHeightForWidth();
  fn C_ZNK11QGridLayout17hasHeightForWidthEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  QRect QGridLayout::cellRect(int row, int column);
  fn C_ZNK11QGridLayout8cellRectEii(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_int) -> *mut c_void;
  // proto:  void QGridLayout::setRowStretch(int row, int stretch);
  fn C_ZN11QGridLayout13setRowStretchEii(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_int);
  // proto:  QLayoutItem * QGridLayout::itemAtPosition(int row, int column);
  fn C_ZNK11QGridLayout14itemAtPositionEii(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_int) -> *mut c_void;
  // proto:  const QMetaObject * QGridLayout::metaObject();
  fn C_ZNK11QGridLayout10metaObjectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  int QGridLayout::verticalSpacing();
  fn C_ZNK11QGridLayout15verticalSpacingEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QGridLayout::QGridLayout(QWidget * parent);
  fn C_ZN11QGridLayoutC2EP7QWidget(arg0: *mut c_void) -> u64;
  // proto:  int QGridLayout::horizontalSpacing();
  fn C_ZNK11QGridLayout17horizontalSpacingEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QGridLayout::setColumnMinimumWidth(int column, int minSize);
  fn C_ZN11QGridLayout21setColumnMinimumWidthEii(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_int);
  // proto:  void QGridLayout::QGridLayout();
  fn C_ZN11QGridLayoutC2Ev() -> u64;
  // proto:  int QGridLayout::heightForWidth(int );
  fn C_ZNK11QGridLayout14heightForWidthEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> c_int;
  // proto:  void QGridLayout::~QGridLayout();
  fn C_ZN11QGridLayoutD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QGridLayout::setSpacing(int spacing);
  fn C_ZN11QGridLayout10setSpacingEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  void QGridLayout::addWidget(QWidget * w);
  fn C_ZN11QGridLayout9addWidgetEP7QWidget(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QLayoutItem * QGridLayout::itemAt(int index);
  fn C_ZNK11QGridLayout6itemAtEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> *mut c_void;
  // proto:  QSize QGridLayout::maximumSize();
  fn C_ZNK11QGridLayout11maximumSizeEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
} // <= ext block end

// body block begin =>
// class sizeof(QGridLayout)=1
#[derive(Default)]
pub struct QGridLayout {
  qbase: QLayout,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QGridLayout {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QGridLayout {
    return QGridLayout{qbase: QLayout::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QGridLayout {
  type Target = QLayout;

  fn deref(&self) -> &QLayout {
    return & self.qbase;
  }
}
impl AsRef<QLayout> for QGridLayout {
  fn as_ref(& self) -> & QLayout {
    return & self.qbase;
  }
}
  // proto:  void QGridLayout::setRowMinimumHeight(int row, int minSize);
impl /*struct*/ QGridLayout {
  pub fn setRowMinimumHeight<RetType, T: QGridLayout_setRowMinimumHeight<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setRowMinimumHeight(self);
    // return 1;
  }
}

pub trait QGridLayout_setRowMinimumHeight<RetType> {
  fn setRowMinimumHeight(self , rsthis: & QGridLayout) -> RetType;
}

  // proto:  void QGridLayout::setRowMinimumHeight(int row, int minSize);
impl<'a> /*trait*/ QGridLayout_setRowMinimumHeight<()> for (i32, i32) {
  fn setRowMinimumHeight(self , rsthis: & QGridLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QGridLayout19setRowMinimumHeightEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {C_ZN11QGridLayout19setRowMinimumHeightEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  QLayoutItem * QGridLayout::takeAt(int index);
impl /*struct*/ QGridLayout {
  pub fn takeAt<RetType, T: QGridLayout_takeAt<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.takeAt(self);
    // return 1;
  }
}

pub trait QGridLayout_takeAt<RetType> {
  fn takeAt(self , rsthis: & QGridLayout) -> RetType;
}

  // proto:  QLayoutItem * QGridLayout::takeAt(int index);
impl<'a> /*trait*/ QGridLayout_takeAt<QLayoutItem> for (i32) {
  fn takeAt(self , rsthis: & QGridLayout) -> QLayoutItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QGridLayout6takeAtEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {C_ZN11QGridLayout6takeAtEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QLayoutItem::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QGridLayout::getItemPosition(int idx, int * row, int * column, int * rowSpan, int * columnSpan);
impl /*struct*/ QGridLayout {
  pub fn getItemPosition<RetType, T: QGridLayout_getItemPosition<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.getItemPosition(self);
    // return 1;
  }
}

pub trait QGridLayout_getItemPosition<RetType> {
  fn getItemPosition(self , rsthis: & QGridLayout) -> RetType;
}

  // proto:  void QGridLayout::getItemPosition(int idx, int * row, int * column, int * rowSpan, int * columnSpan);
impl<'a> /*trait*/ QGridLayout_getItemPosition<()> for (i32, &'a mut Vec<i32>, &'a mut Vec<i32>, &'a mut Vec<i32>, &'a mut Vec<i32>) {
  fn getItemPosition(self , rsthis: & QGridLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QGridLayout15getItemPositionEiPiS0_S0_S0_()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.as_ptr()  as *mut c_int;
    let arg2 = self.2.as_ptr()  as *mut c_int;
    let arg3 = self.3.as_ptr()  as *mut c_int;
    let arg4 = self.4.as_ptr()  as *mut c_int;
     unsafe {C_ZNK11QGridLayout15getItemPositionEiPiS0_S0_S0_(rsthis.qclsinst, arg0, arg1, arg2, arg3, arg4)};
    // return 1;
  }
}

  // proto:  int QGridLayout::minimumHeightForWidth(int );
impl /*struct*/ QGridLayout {
  pub fn minimumHeightForWidth<RetType, T: QGridLayout_minimumHeightForWidth<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.minimumHeightForWidth(self);
    // return 1;
  }
}

pub trait QGridLayout_minimumHeightForWidth<RetType> {
  fn minimumHeightForWidth(self , rsthis: & QGridLayout) -> RetType;
}

  // proto:  int QGridLayout::minimumHeightForWidth(int );
impl<'a> /*trait*/ QGridLayout_minimumHeightForWidth<i32> for (i32) {
  fn minimumHeightForWidth(self , rsthis: & QGridLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QGridLayout21minimumHeightForWidthEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {C_ZNK11QGridLayout21minimumHeightForWidthEi(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  int QGridLayout::rowMinimumHeight(int row);
impl /*struct*/ QGridLayout {
  pub fn rowMinimumHeight<RetType, T: QGridLayout_rowMinimumHeight<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.rowMinimumHeight(self);
    // return 1;
  }
}

pub trait QGridLayout_rowMinimumHeight<RetType> {
  fn rowMinimumHeight(self , rsthis: & QGridLayout) -> RetType;
}

  // proto:  int QGridLayout::rowMinimumHeight(int row);
impl<'a> /*trait*/ QGridLayout_rowMinimumHeight<i32> for (i32) {
  fn rowMinimumHeight(self , rsthis: & QGridLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QGridLayout16rowMinimumHeightEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {C_ZNK11QGridLayout16rowMinimumHeightEi(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QGridLayout::invalidate();
impl /*struct*/ QGridLayout {
  pub fn invalidate<RetType, T: QGridLayout_invalidate<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.invalidate(self);
    // return 1;
  }
}

pub trait QGridLayout_invalidate<RetType> {
  fn invalidate(self , rsthis: & QGridLayout) -> RetType;
}

  // proto:  void QGridLayout::invalidate();
impl<'a> /*trait*/ QGridLayout_invalidate<()> for () {
  fn invalidate(self , rsthis: & QGridLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QGridLayout10invalidateEv()};
     unsafe {C_ZN11QGridLayout10invalidateEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  int QGridLayout::count();
impl /*struct*/ QGridLayout {
  pub fn count<RetType, T: QGridLayout_count<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.count(self);
    // return 1;
  }
}

pub trait QGridLayout_count<RetType> {
  fn count(self , rsthis: & QGridLayout) -> RetType;
}

  // proto:  int QGridLayout::count();
impl<'a> /*trait*/ QGridLayout_count<i32> for () {
  fn count(self , rsthis: & QGridLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QGridLayout5countEv()};
    let mut ret = unsafe {C_ZNK11QGridLayout5countEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QGridLayout::setColumnStretch(int column, int stretch);
impl /*struct*/ QGridLayout {
  pub fn setColumnStretch<RetType, T: QGridLayout_setColumnStretch<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setColumnStretch(self);
    // return 1;
  }
}

pub trait QGridLayout_setColumnStretch<RetType> {
  fn setColumnStretch(self , rsthis: & QGridLayout) -> RetType;
}

  // proto:  void QGridLayout::setColumnStretch(int column, int stretch);
impl<'a> /*trait*/ QGridLayout_setColumnStretch<()> for (i32, i32) {
  fn setColumnStretch(self , rsthis: & QGridLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QGridLayout16setColumnStretchEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {C_ZN11QGridLayout16setColumnStretchEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  int QGridLayout::spacing();
impl /*struct*/ QGridLayout {
  pub fn spacing<RetType, T: QGridLayout_spacing<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.spacing(self);
    // return 1;
  }
}

pub trait QGridLayout_spacing<RetType> {
  fn spacing(self , rsthis: & QGridLayout) -> RetType;
}

  // proto:  int QGridLayout::spacing();
impl<'a> /*trait*/ QGridLayout_spacing<i32> for () {
  fn spacing(self , rsthis: & QGridLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QGridLayout7spacingEv()};
    let mut ret = unsafe {C_ZNK11QGridLayout7spacingEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  int QGridLayout::rowStretch(int row);
impl /*struct*/ QGridLayout {
  pub fn rowStretch<RetType, T: QGridLayout_rowStretch<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.rowStretch(self);
    // return 1;
  }
}

pub trait QGridLayout_rowStretch<RetType> {
  fn rowStretch(self , rsthis: & QGridLayout) -> RetType;
}

  // proto:  int QGridLayout::rowStretch(int row);
impl<'a> /*trait*/ QGridLayout_rowStretch<i32> for (i32) {
  fn rowStretch(self , rsthis: & QGridLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QGridLayout10rowStretchEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {C_ZNK11QGridLayout10rowStretchEi(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  QSize QGridLayout::sizeHint();
impl /*struct*/ QGridLayout {
  pub fn sizeHint<RetType, T: QGridLayout_sizeHint<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.sizeHint(self);
    // return 1;
  }
}

pub trait QGridLayout_sizeHint<RetType> {
  fn sizeHint(self , rsthis: & QGridLayout) -> RetType;
}

  // proto:  QSize QGridLayout::sizeHint();
impl<'a> /*trait*/ QGridLayout_sizeHint<QSize> for () {
  fn sizeHint(self , rsthis: & QGridLayout) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QGridLayout8sizeHintEv()};
    let mut ret = unsafe {C_ZNK11QGridLayout8sizeHintEv(rsthis.qclsinst)};
    let mut ret1 = QSize::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  int QGridLayout::rowCount();
impl /*struct*/ QGridLayout {
  pub fn rowCount<RetType, T: QGridLayout_rowCount<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.rowCount(self);
    // return 1;
  }
}

pub trait QGridLayout_rowCount<RetType> {
  fn rowCount(self , rsthis: & QGridLayout) -> RetType;
}

  // proto:  int QGridLayout::rowCount();
impl<'a> /*trait*/ QGridLayout_rowCount<i32> for () {
  fn rowCount(self , rsthis: & QGridLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QGridLayout8rowCountEv()};
    let mut ret = unsafe {C_ZNK11QGridLayout8rowCountEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QGridLayout::setGeometry(const QRect & );
impl /*struct*/ QGridLayout {
  pub fn setGeometry<RetType, T: QGridLayout_setGeometry<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setGeometry(self);
    // return 1;
  }
}

pub trait QGridLayout_setGeometry<RetType> {
  fn setGeometry(self , rsthis: & QGridLayout) -> RetType;
}

  // proto:  void QGridLayout::setGeometry(const QRect & );
impl<'a> /*trait*/ QGridLayout_setGeometry<()> for (&'a QRect) {
  fn setGeometry(self , rsthis: & QGridLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QGridLayout11setGeometryERK5QRect()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN11QGridLayout11setGeometryERK5QRect(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QGridLayout::setVerticalSpacing(int spacing);
impl /*struct*/ QGridLayout {
  pub fn setVerticalSpacing<RetType, T: QGridLayout_setVerticalSpacing<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setVerticalSpacing(self);
    // return 1;
  }
}

pub trait QGridLayout_setVerticalSpacing<RetType> {
  fn setVerticalSpacing(self , rsthis: & QGridLayout) -> RetType;
}

  // proto:  void QGridLayout::setVerticalSpacing(int spacing);
impl<'a> /*trait*/ QGridLayout_setVerticalSpacing<()> for (i32) {
  fn setVerticalSpacing(self , rsthis: & QGridLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QGridLayout18setVerticalSpacingEi()};
    let arg0 = self  as c_int;
     unsafe {C_ZN11QGridLayout18setVerticalSpacingEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QGridLayout::setHorizontalSpacing(int spacing);
impl /*struct*/ QGridLayout {
  pub fn setHorizontalSpacing<RetType, T: QGridLayout_setHorizontalSpacing<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setHorizontalSpacing(self);
    // return 1;
  }
}

pub trait QGridLayout_setHorizontalSpacing<RetType> {
  fn setHorizontalSpacing(self , rsthis: & QGridLayout) -> RetType;
}

  // proto:  void QGridLayout::setHorizontalSpacing(int spacing);
impl<'a> /*trait*/ QGridLayout_setHorizontalSpacing<()> for (i32) {
  fn setHorizontalSpacing(self , rsthis: & QGridLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QGridLayout20setHorizontalSpacingEi()};
    let arg0 = self  as c_int;
     unsafe {C_ZN11QGridLayout20setHorizontalSpacingEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QGridLayout::columnStretch(int column);
impl /*struct*/ QGridLayout {
  pub fn columnStretch<RetType, T: QGridLayout_columnStretch<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.columnStretch(self);
    // return 1;
  }
}

pub trait QGridLayout_columnStretch<RetType> {
  fn columnStretch(self , rsthis: & QGridLayout) -> RetType;
}

  // proto:  int QGridLayout::columnStretch(int column);
impl<'a> /*trait*/ QGridLayout_columnStretch<i32> for (i32) {
  fn columnStretch(self , rsthis: & QGridLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QGridLayout13columnStretchEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {C_ZNK11QGridLayout13columnStretchEi(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  int QGridLayout::columnCount();
impl /*struct*/ QGridLayout {
  pub fn columnCount<RetType, T: QGridLayout_columnCount<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.columnCount(self);
    // return 1;
  }
}

pub trait QGridLayout_columnCount<RetType> {
  fn columnCount(self , rsthis: & QGridLayout) -> RetType;
}

  // proto:  int QGridLayout::columnCount();
impl<'a> /*trait*/ QGridLayout_columnCount<i32> for () {
  fn columnCount(self , rsthis: & QGridLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QGridLayout11columnCountEv()};
    let mut ret = unsafe {C_ZNK11QGridLayout11columnCountEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  int QGridLayout::columnMinimumWidth(int column);
impl /*struct*/ QGridLayout {
  pub fn columnMinimumWidth<RetType, T: QGridLayout_columnMinimumWidth<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.columnMinimumWidth(self);
    // return 1;
  }
}

pub trait QGridLayout_columnMinimumWidth<RetType> {
  fn columnMinimumWidth(self , rsthis: & QGridLayout) -> RetType;
}

  // proto:  int QGridLayout::columnMinimumWidth(int column);
impl<'a> /*trait*/ QGridLayout_columnMinimumWidth<i32> for (i32) {
  fn columnMinimumWidth(self , rsthis: & QGridLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QGridLayout18columnMinimumWidthEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {C_ZNK11QGridLayout18columnMinimumWidthEi(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  QSize QGridLayout::minimumSize();
impl /*struct*/ QGridLayout {
  pub fn minimumSize<RetType, T: QGridLayout_minimumSize<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.minimumSize(self);
    // return 1;
  }
}

pub trait QGridLayout_minimumSize<RetType> {
  fn minimumSize(self , rsthis: & QGridLayout) -> RetType;
}

  // proto:  QSize QGridLayout::minimumSize();
impl<'a> /*trait*/ QGridLayout_minimumSize<QSize> for () {
  fn minimumSize(self , rsthis: & QGridLayout) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QGridLayout11minimumSizeEv()};
    let mut ret = unsafe {C_ZNK11QGridLayout11minimumSizeEv(rsthis.qclsinst)};
    let mut ret1 = QSize::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QGridLayout::hasHeightForWidth();
impl /*struct*/ QGridLayout {
  pub fn hasHeightForWidth<RetType, T: QGridLayout_hasHeightForWidth<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.hasHeightForWidth(self);
    // return 1;
  }
}

pub trait QGridLayout_hasHeightForWidth<RetType> {
  fn hasHeightForWidth(self , rsthis: & QGridLayout) -> RetType;
}

  // proto:  bool QGridLayout::hasHeightForWidth();
impl<'a> /*trait*/ QGridLayout_hasHeightForWidth<i8> for () {
  fn hasHeightForWidth(self , rsthis: & QGridLayout) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QGridLayout17hasHeightForWidthEv()};
    let mut ret = unsafe {C_ZNK11QGridLayout17hasHeightForWidthEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QRect QGridLayout::cellRect(int row, int column);
impl /*struct*/ QGridLayout {
  pub fn cellRect<RetType, T: QGridLayout_cellRect<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.cellRect(self);
    // return 1;
  }
}

pub trait QGridLayout_cellRect<RetType> {
  fn cellRect(self , rsthis: & QGridLayout) -> RetType;
}

  // proto:  QRect QGridLayout::cellRect(int row, int column);
impl<'a> /*trait*/ QGridLayout_cellRect<QRect> for (i32, i32) {
  fn cellRect(self , rsthis: & QGridLayout) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QGridLayout8cellRectEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {C_ZNK11QGridLayout8cellRectEii(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QRect::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QGridLayout::setRowStretch(int row, int stretch);
impl /*struct*/ QGridLayout {
  pub fn setRowStretch<RetType, T: QGridLayout_setRowStretch<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setRowStretch(self);
    // return 1;
  }
}

pub trait QGridLayout_setRowStretch<RetType> {
  fn setRowStretch(self , rsthis: & QGridLayout) -> RetType;
}

  // proto:  void QGridLayout::setRowStretch(int row, int stretch);
impl<'a> /*trait*/ QGridLayout_setRowStretch<()> for (i32, i32) {
  fn setRowStretch(self , rsthis: & QGridLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QGridLayout13setRowStretchEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {C_ZN11QGridLayout13setRowStretchEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  QLayoutItem * QGridLayout::itemAtPosition(int row, int column);
impl /*struct*/ QGridLayout {
  pub fn itemAtPosition<RetType, T: QGridLayout_itemAtPosition<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.itemAtPosition(self);
    // return 1;
  }
}

pub trait QGridLayout_itemAtPosition<RetType> {
  fn itemAtPosition(self , rsthis: & QGridLayout) -> RetType;
}

  // proto:  QLayoutItem * QGridLayout::itemAtPosition(int row, int column);
impl<'a> /*trait*/ QGridLayout_itemAtPosition<QLayoutItem> for (i32, i32) {
  fn itemAtPosition(self , rsthis: & QGridLayout) -> QLayoutItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QGridLayout14itemAtPositionEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {C_ZNK11QGridLayout14itemAtPositionEii(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QLayoutItem::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  const QMetaObject * QGridLayout::metaObject();
impl /*struct*/ QGridLayout {
  pub fn metaObject<RetType, T: QGridLayout_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QGridLayout_metaObject<RetType> {
  fn metaObject(self , rsthis: & QGridLayout) -> RetType;
}

  // proto:  const QMetaObject * QGridLayout::metaObject();
impl<'a> /*trait*/ QGridLayout_metaObject<QMetaObject> for () {
  fn metaObject(self , rsthis: & QGridLayout) -> QMetaObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QGridLayout10metaObjectEv()};
    let mut ret = unsafe {C_ZNK11QGridLayout10metaObjectEv(rsthis.qclsinst)};
    let mut ret1 = QMetaObject::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  int QGridLayout::verticalSpacing();
impl /*struct*/ QGridLayout {
  pub fn verticalSpacing<RetType, T: QGridLayout_verticalSpacing<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.verticalSpacing(self);
    // return 1;
  }
}

pub trait QGridLayout_verticalSpacing<RetType> {
  fn verticalSpacing(self , rsthis: & QGridLayout) -> RetType;
}

  // proto:  int QGridLayout::verticalSpacing();
impl<'a> /*trait*/ QGridLayout_verticalSpacing<i32> for () {
  fn verticalSpacing(self , rsthis: & QGridLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QGridLayout15verticalSpacingEv()};
    let mut ret = unsafe {C_ZNK11QGridLayout15verticalSpacingEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QGridLayout::QGridLayout(QWidget * parent);
impl /*struct*/ QGridLayout {
  pub fn new<T: QGridLayout_new>(value: T) -> QGridLayout {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QGridLayout_new {
  fn new(self) -> QGridLayout;
}

  // proto:  void QGridLayout::QGridLayout(QWidget * parent);
impl<'a> /*trait*/ QGridLayout_new for (&'a QWidget) {
  fn new(self) -> QGridLayout {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QGridLayoutC2EP7QWidget()};
    let ctysz: c_int = unsafe{QGridLayout_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN11QGridLayoutC2EP7QWidget(arg0)};
    let rsthis = QGridLayout{qbase: QLayout::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  int QGridLayout::horizontalSpacing();
impl /*struct*/ QGridLayout {
  pub fn horizontalSpacing<RetType, T: QGridLayout_horizontalSpacing<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.horizontalSpacing(self);
    // return 1;
  }
}

pub trait QGridLayout_horizontalSpacing<RetType> {
  fn horizontalSpacing(self , rsthis: & QGridLayout) -> RetType;
}

  // proto:  int QGridLayout::horizontalSpacing();
impl<'a> /*trait*/ QGridLayout_horizontalSpacing<i32> for () {
  fn horizontalSpacing(self , rsthis: & QGridLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QGridLayout17horizontalSpacingEv()};
    let mut ret = unsafe {C_ZNK11QGridLayout17horizontalSpacingEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QGridLayout::setColumnMinimumWidth(int column, int minSize);
impl /*struct*/ QGridLayout {
  pub fn setColumnMinimumWidth<RetType, T: QGridLayout_setColumnMinimumWidth<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setColumnMinimumWidth(self);
    // return 1;
  }
}

pub trait QGridLayout_setColumnMinimumWidth<RetType> {
  fn setColumnMinimumWidth(self , rsthis: & QGridLayout) -> RetType;
}

  // proto:  void QGridLayout::setColumnMinimumWidth(int column, int minSize);
impl<'a> /*trait*/ QGridLayout_setColumnMinimumWidth<()> for (i32, i32) {
  fn setColumnMinimumWidth(self , rsthis: & QGridLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QGridLayout21setColumnMinimumWidthEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {C_ZN11QGridLayout21setColumnMinimumWidthEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QGridLayout::QGridLayout();
impl<'a> /*trait*/ QGridLayout_new for () {
  fn new(self) -> QGridLayout {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QGridLayoutC2Ev()};
    let ctysz: c_int = unsafe{QGridLayout_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let qthis: u64 = unsafe {C_ZN11QGridLayoutC2Ev()};
    let rsthis = QGridLayout{qbase: QLayout::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  int QGridLayout::heightForWidth(int );
impl /*struct*/ QGridLayout {
  pub fn heightForWidth<RetType, T: QGridLayout_heightForWidth<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.heightForWidth(self);
    // return 1;
  }
}

pub trait QGridLayout_heightForWidth<RetType> {
  fn heightForWidth(self , rsthis: & QGridLayout) -> RetType;
}

  // proto:  int QGridLayout::heightForWidth(int );
impl<'a> /*trait*/ QGridLayout_heightForWidth<i32> for (i32) {
  fn heightForWidth(self , rsthis: & QGridLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QGridLayout14heightForWidthEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {C_ZNK11QGridLayout14heightForWidthEi(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QGridLayout::~QGridLayout();
impl /*struct*/ QGridLayout {
  pub fn free<RetType, T: QGridLayout_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QGridLayout_free<RetType> {
  fn free(self , rsthis: & QGridLayout) -> RetType;
}

  // proto:  void QGridLayout::~QGridLayout();
impl<'a> /*trait*/ QGridLayout_free<()> for () {
  fn free(self , rsthis: & QGridLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QGridLayoutD2Ev()};
     unsafe {C_ZN11QGridLayoutD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QGridLayout::setSpacing(int spacing);
impl /*struct*/ QGridLayout {
  pub fn setSpacing<RetType, T: QGridLayout_setSpacing<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setSpacing(self);
    // return 1;
  }
}

pub trait QGridLayout_setSpacing<RetType> {
  fn setSpacing(self , rsthis: & QGridLayout) -> RetType;
}

  // proto:  void QGridLayout::setSpacing(int spacing);
impl<'a> /*trait*/ QGridLayout_setSpacing<()> for (i32) {
  fn setSpacing(self , rsthis: & QGridLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QGridLayout10setSpacingEi()};
    let arg0 = self  as c_int;
     unsafe {C_ZN11QGridLayout10setSpacingEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QGridLayout::addWidget(QWidget * w);
impl /*struct*/ QGridLayout {
  pub fn addWidget<RetType, T: QGridLayout_addWidget<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.addWidget(self);
    // return 1;
  }
}

pub trait QGridLayout_addWidget<RetType> {
  fn addWidget(self , rsthis: & QGridLayout) -> RetType;
}

  // proto:  void QGridLayout::addWidget(QWidget * w);
impl<'a> /*trait*/ QGridLayout_addWidget<()> for (&'a QWidget) {
  fn addWidget(self , rsthis: & QGridLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QGridLayout9addWidgetEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN11QGridLayout9addWidgetEP7QWidget(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QLayoutItem * QGridLayout::itemAt(int index);
impl /*struct*/ QGridLayout {
  pub fn itemAt<RetType, T: QGridLayout_itemAt<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.itemAt(self);
    // return 1;
  }
}

pub trait QGridLayout_itemAt<RetType> {
  fn itemAt(self , rsthis: & QGridLayout) -> RetType;
}

  // proto:  QLayoutItem * QGridLayout::itemAt(int index);
impl<'a> /*trait*/ QGridLayout_itemAt<QLayoutItem> for (i32) {
  fn itemAt(self , rsthis: & QGridLayout) -> QLayoutItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QGridLayout6itemAtEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {C_ZNK11QGridLayout6itemAtEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QLayoutItem::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QSize QGridLayout::maximumSize();
impl /*struct*/ QGridLayout {
  pub fn maximumSize<RetType, T: QGridLayout_maximumSize<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.maximumSize(self);
    // return 1;
  }
}

pub trait QGridLayout_maximumSize<RetType> {
  fn maximumSize(self , rsthis: & QGridLayout) -> RetType;
}

  // proto:  QSize QGridLayout::maximumSize();
impl<'a> /*trait*/ QGridLayout_maximumSize<QSize> for () {
  fn maximumSize(self , rsthis: & QGridLayout) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QGridLayout11maximumSizeEv()};
    let mut ret = unsafe {C_ZNK11QGridLayout11maximumSizeEv(rsthis.qclsinst)};
    let mut ret1 = QSize::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

// <= body block end

