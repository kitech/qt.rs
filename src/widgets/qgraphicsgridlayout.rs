// auto generated, do not modify.
// created: Sat Dec 26 12:15:38 2015
// src-file: /QtWidgets/qgraphicsgridlayout.h
// dst-file: /src/widgets/qgraphicsgridlayout.rs
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
use super::qgraphicslayout::QGraphicsLayout; // 773
use std::ops::Deref;
use super::qgraphicslayoutitem::QGraphicsLayoutItem; // 773
use super::super::core::qrect::QRectF; // 771
use super::super::core::qsize::QSizeF; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QGraphicsGridLayout_Class_Size() -> c_int;
  // proto:  void QGraphicsGridLayout::setRowPreferredHeight(int row, qreal height);
  fn _ZN19QGraphicsGridLayout21setRowPreferredHeightEid(qthis: *mut c_void, arg0: c_int, arg1: c_double);
  // proto:  int QGraphicsGridLayout::columnCount();
  fn _ZNK19QGraphicsGridLayout11columnCountEv(qthis: *mut c_void) -> c_int;
  // proto:  QGraphicsLayoutItem * QGraphicsGridLayout::itemAt(int index);
  fn _ZNK19QGraphicsGridLayout6itemAtEi(qthis: *mut c_void, arg0: c_int);
  // proto:  int QGraphicsGridLayout::count();
  fn _ZNK19QGraphicsGridLayout5countEv(qthis: *mut c_void) -> c_int;
  // proto:  void QGraphicsGridLayout::setColumnFixedWidth(int column, qreal width);
  fn _ZN19QGraphicsGridLayout19setColumnFixedWidthEid(qthis: *mut c_void, arg0: c_int, arg1: c_double);
  // proto:  void QGraphicsGridLayout::setColumnMaximumWidth(int column, qreal width);
  fn _ZN19QGraphicsGridLayout21setColumnMaximumWidthEid(qthis: *mut c_void, arg0: c_int, arg1: c_double);
  // proto:  int QGraphicsGridLayout::rowStretchFactor(int row);
  fn _ZNK19QGraphicsGridLayout16rowStretchFactorEi(qthis: *mut c_void, arg0: c_int) -> c_int;
  // proto:  qreal QGraphicsGridLayout::verticalSpacing();
  fn _ZNK19QGraphicsGridLayout15verticalSpacingEv(qthis: *mut c_void) -> c_double;
  // proto:  int QGraphicsGridLayout::columnStretchFactor(int column);
  fn _ZNK19QGraphicsGridLayout19columnStretchFactorEi(qthis: *mut c_void, arg0: c_int) -> c_int;
  // proto:  void QGraphicsGridLayout::setRowMaximumHeight(int row, qreal height);
  fn _ZN19QGraphicsGridLayout19setRowMaximumHeightEid(qthis: *mut c_void, arg0: c_int, arg1: c_double);
  // proto:  void QGraphicsGridLayout::removeItem(QGraphicsLayoutItem * item);
  fn _ZN19QGraphicsGridLayout10removeItemEP19QGraphicsLayoutItem(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QGraphicsGridLayout::~QGraphicsGridLayout();
  fn _ZN19QGraphicsGridLayoutD0Ev(qthis: *mut c_void);
  // proto:  qreal QGraphicsGridLayout::rowMinimumHeight(int row);
  fn _ZNK19QGraphicsGridLayout16rowMinimumHeightEi(qthis: *mut c_void, arg0: c_int) -> c_double;
  // proto:  qreal QGraphicsGridLayout::rowMaximumHeight(int row);
  fn _ZNK19QGraphicsGridLayout16rowMaximumHeightEi(qthis: *mut c_void, arg0: c_int) -> c_double;
  // proto:  void QGraphicsGridLayout::QGraphicsGridLayout(QGraphicsLayoutItem * parent);
  fn dector_ZN19QGraphicsGridLayoutC1EP19QGraphicsLayoutItem(arg0: *mut c_void) -> *mut c_void;
  fn _ZN19QGraphicsGridLayoutC1EP19QGraphicsLayoutItem(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QGraphicsGridLayout::setColumnSpacing(int column, qreal spacing);
  fn _ZN19QGraphicsGridLayout16setColumnSpacingEid(qthis: *mut c_void, arg0: c_int, arg1: c_double);
  // proto:  qreal QGraphicsGridLayout::rowSpacing(int row);
  fn _ZNK19QGraphicsGridLayout10rowSpacingEi(qthis: *mut c_void, arg0: c_int) -> c_double;
  // proto:  qreal QGraphicsGridLayout::columnMaximumWidth(int column);
  fn _ZNK19QGraphicsGridLayout18columnMaximumWidthEi(qthis: *mut c_void, arg0: c_int) -> c_double;
  // proto:  void QGraphicsGridLayout::setRowFixedHeight(int row, qreal height);
  fn _ZN19QGraphicsGridLayout17setRowFixedHeightEid(qthis: *mut c_void, arg0: c_int, arg1: c_double);
  // proto:  qreal QGraphicsGridLayout::rowPreferredHeight(int row);
  fn _ZNK19QGraphicsGridLayout18rowPreferredHeightEi(qthis: *mut c_void, arg0: c_int) -> c_double;
  // proto:  QGraphicsLayoutItem * QGraphicsGridLayout::itemAt(int row, int column);
  fn _ZNK19QGraphicsGridLayout6itemAtEii(qthis: *mut c_void, arg0: c_int, arg1: c_int);
  // proto:  void QGraphicsGridLayout::setVerticalSpacing(qreal spacing);
  fn _ZN19QGraphicsGridLayout18setVerticalSpacingEd(qthis: *mut c_void, arg0: c_double);
  // proto:  void QGraphicsGridLayout::setGeometry(const QRectF & rect);
  fn _ZN19QGraphicsGridLayout11setGeometryERK6QRectF(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  int QGraphicsGridLayout::rowCount();
  fn _ZNK19QGraphicsGridLayout8rowCountEv(qthis: *mut c_void) -> c_int;
  // proto:  void QGraphicsGridLayout::setSpacing(qreal spacing);
  fn _ZN19QGraphicsGridLayout10setSpacingEd(qthis: *mut c_void, arg0: c_double);
  // proto:  void QGraphicsGridLayout::setRowStretchFactor(int row, int stretch);
  fn _ZN19QGraphicsGridLayout19setRowStretchFactorEii(qthis: *mut c_void, arg0: c_int, arg1: c_int);
  // proto:  qreal QGraphicsGridLayout::columnMinimumWidth(int column);
  fn _ZNK19QGraphicsGridLayout18columnMinimumWidthEi(qthis: *mut c_void, arg0: c_int) -> c_double;
  // proto:  void QGraphicsGridLayout::setColumnMinimumWidth(int column, qreal width);
  fn _ZN19QGraphicsGridLayout21setColumnMinimumWidthEid(qthis: *mut c_void, arg0: c_int, arg1: c_double);
  // proto:  void QGraphicsGridLayout::setRowMinimumHeight(int row, qreal height);
  fn _ZN19QGraphicsGridLayout19setRowMinimumHeightEid(qthis: *mut c_void, arg0: c_int, arg1: c_double);
  // proto:  void QGraphicsGridLayout::setHorizontalSpacing(qreal spacing);
  fn _ZN19QGraphicsGridLayout20setHorizontalSpacingEd(qthis: *mut c_void, arg0: c_double);
  // proto:  qreal QGraphicsGridLayout::horizontalSpacing();
  fn _ZNK19QGraphicsGridLayout17horizontalSpacingEv(qthis: *mut c_void) -> c_double;
  // proto:  void QGraphicsGridLayout::setColumnStretchFactor(int column, int stretch);
  fn _ZN19QGraphicsGridLayout22setColumnStretchFactorEii(qthis: *mut c_void, arg0: c_int, arg1: c_int);
  // proto:  void QGraphicsGridLayout::invalidate();
  fn _ZN19QGraphicsGridLayout10invalidateEv(qthis: *mut c_void);
  // proto:  qreal QGraphicsGridLayout::columnPreferredWidth(int column);
  fn _ZNK19QGraphicsGridLayout20columnPreferredWidthEi(qthis: *mut c_void, arg0: c_int) -> c_double;
  // proto:  void QGraphicsGridLayout::setColumnPreferredWidth(int column, qreal width);
  fn _ZN19QGraphicsGridLayout23setColumnPreferredWidthEid(qthis: *mut c_void, arg0: c_int, arg1: c_double);
  // proto:  qreal QGraphicsGridLayout::columnSpacing(int column);
  fn _ZNK19QGraphicsGridLayout13columnSpacingEi(qthis: *mut c_void, arg0: c_int) -> c_double;
  // proto:  void QGraphicsGridLayout::QGraphicsGridLayout(const QGraphicsGridLayout & );
  fn dector_ZN19QGraphicsGridLayoutC1ERKS_(arg0: *mut c_void) -> *mut c_void;
  fn _ZN19QGraphicsGridLayoutC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QGraphicsGridLayout::setRowSpacing(int row, qreal spacing);
  fn _ZN19QGraphicsGridLayout13setRowSpacingEid(qthis: *mut c_void, arg0: c_int, arg1: c_double);
  // proto:  void QGraphicsGridLayout::removeAt(int index);
  fn _ZN19QGraphicsGridLayout8removeAtEi(qthis: *mut c_void, arg0: c_int);
} // <= ext block end

// body block begin =>
// class sizeof(QGraphicsGridLayout)=1
pub struct QGraphicsGridLayout {
  qbase: QGraphicsLayout,
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QGraphicsGridLayout {
  pub fn inheritFrom(qthis: *mut c_void) -> QGraphicsGridLayout {
    return QGraphicsGridLayout{qbase: QGraphicsLayout::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QGraphicsGridLayout {
  type Target = QGraphicsLayout;

  fn deref(&self) -> &QGraphicsLayout {
    return & self.qbase;
  }
}
impl AsRef<QGraphicsLayout> for QGraphicsGridLayout {
  fn as_ref(& self) -> & QGraphicsLayout {
    return & self.qbase;
  }
}
  // proto:  void QGraphicsGridLayout::setRowPreferredHeight(int row, qreal height);
impl /*struct*/ QGraphicsGridLayout {
  pub fn setRowPreferredHeight<RetType, T: QGraphicsGridLayout_setRowPreferredHeight<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setRowPreferredHeight(self);
    // return 1;
  }
}

pub trait QGraphicsGridLayout_setRowPreferredHeight<RetType> {
  fn setRowPreferredHeight(self , rsthis: & QGraphicsGridLayout) -> RetType;
}

  // proto:  void QGraphicsGridLayout::setRowPreferredHeight(int row, qreal height);
impl<'a> /*trait*/ QGraphicsGridLayout_setRowPreferredHeight<()> for (i32, f64) {
  fn setRowPreferredHeight(self , rsthis: & QGraphicsGridLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsGridLayout21setRowPreferredHeightEid()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_double;
     unsafe {_ZN19QGraphicsGridLayout21setRowPreferredHeightEid(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  int QGraphicsGridLayout::columnCount();
impl /*struct*/ QGraphicsGridLayout {
  pub fn columnCount<RetType, T: QGraphicsGridLayout_columnCount<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.columnCount(self);
    // return 1;
  }
}

pub trait QGraphicsGridLayout_columnCount<RetType> {
  fn columnCount(self , rsthis: & QGraphicsGridLayout) -> RetType;
}

  // proto:  int QGraphicsGridLayout::columnCount();
impl<'a> /*trait*/ QGraphicsGridLayout_columnCount<i32> for () {
  fn columnCount(self , rsthis: & QGraphicsGridLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QGraphicsGridLayout11columnCountEv()};
    let mut ret = unsafe {_ZNK19QGraphicsGridLayout11columnCountEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  QGraphicsLayoutItem * QGraphicsGridLayout::itemAt(int index);
impl /*struct*/ QGraphicsGridLayout {
  pub fn itemAt<RetType, T: QGraphicsGridLayout_itemAt<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.itemAt(self);
    // return 1;
  }
}

pub trait QGraphicsGridLayout_itemAt<RetType> {
  fn itemAt(self , rsthis: & QGraphicsGridLayout) -> RetType;
}

  // proto:  QGraphicsLayoutItem * QGraphicsGridLayout::itemAt(int index);
impl<'a> /*trait*/ QGraphicsGridLayout_itemAt<()> for (i32) {
  fn itemAt(self , rsthis: & QGraphicsGridLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QGraphicsGridLayout6itemAtEi()};
    let arg0 = self  as c_int;
     unsafe {_ZNK19QGraphicsGridLayout6itemAtEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QGraphicsGridLayout::count();
impl /*struct*/ QGraphicsGridLayout {
  pub fn count<RetType, T: QGraphicsGridLayout_count<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.count(self);
    // return 1;
  }
}

pub trait QGraphicsGridLayout_count<RetType> {
  fn count(self , rsthis: & QGraphicsGridLayout) -> RetType;
}

  // proto:  int QGraphicsGridLayout::count();
impl<'a> /*trait*/ QGraphicsGridLayout_count<i32> for () {
  fn count(self , rsthis: & QGraphicsGridLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QGraphicsGridLayout5countEv()};
    let mut ret = unsafe {_ZNK19QGraphicsGridLayout5countEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QGraphicsGridLayout::setColumnFixedWidth(int column, qreal width);
impl /*struct*/ QGraphicsGridLayout {
  pub fn setColumnFixedWidth<RetType, T: QGraphicsGridLayout_setColumnFixedWidth<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setColumnFixedWidth(self);
    // return 1;
  }
}

pub trait QGraphicsGridLayout_setColumnFixedWidth<RetType> {
  fn setColumnFixedWidth(self , rsthis: & QGraphicsGridLayout) -> RetType;
}

  // proto:  void QGraphicsGridLayout::setColumnFixedWidth(int column, qreal width);
impl<'a> /*trait*/ QGraphicsGridLayout_setColumnFixedWidth<()> for (i32, f64) {
  fn setColumnFixedWidth(self , rsthis: & QGraphicsGridLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsGridLayout19setColumnFixedWidthEid()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_double;
     unsafe {_ZN19QGraphicsGridLayout19setColumnFixedWidthEid(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QGraphicsGridLayout::setColumnMaximumWidth(int column, qreal width);
impl /*struct*/ QGraphicsGridLayout {
  pub fn setColumnMaximumWidth<RetType, T: QGraphicsGridLayout_setColumnMaximumWidth<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setColumnMaximumWidth(self);
    // return 1;
  }
}

pub trait QGraphicsGridLayout_setColumnMaximumWidth<RetType> {
  fn setColumnMaximumWidth(self , rsthis: & QGraphicsGridLayout) -> RetType;
}

  // proto:  void QGraphicsGridLayout::setColumnMaximumWidth(int column, qreal width);
impl<'a> /*trait*/ QGraphicsGridLayout_setColumnMaximumWidth<()> for (i32, f64) {
  fn setColumnMaximumWidth(self , rsthis: & QGraphicsGridLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsGridLayout21setColumnMaximumWidthEid()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_double;
     unsafe {_ZN19QGraphicsGridLayout21setColumnMaximumWidthEid(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  int QGraphicsGridLayout::rowStretchFactor(int row);
impl /*struct*/ QGraphicsGridLayout {
  pub fn rowStretchFactor<RetType, T: QGraphicsGridLayout_rowStretchFactor<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.rowStretchFactor(self);
    // return 1;
  }
}

pub trait QGraphicsGridLayout_rowStretchFactor<RetType> {
  fn rowStretchFactor(self , rsthis: & QGraphicsGridLayout) -> RetType;
}

  // proto:  int QGraphicsGridLayout::rowStretchFactor(int row);
impl<'a> /*trait*/ QGraphicsGridLayout_rowStretchFactor<i32> for (i32) {
  fn rowStretchFactor(self , rsthis: & QGraphicsGridLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QGraphicsGridLayout16rowStretchFactorEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK19QGraphicsGridLayout16rowStretchFactorEi(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  qreal QGraphicsGridLayout::verticalSpacing();
impl /*struct*/ QGraphicsGridLayout {
  pub fn verticalSpacing<RetType, T: QGraphicsGridLayout_verticalSpacing<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.verticalSpacing(self);
    // return 1;
  }
}

pub trait QGraphicsGridLayout_verticalSpacing<RetType> {
  fn verticalSpacing(self , rsthis: & QGraphicsGridLayout) -> RetType;
}

  // proto:  qreal QGraphicsGridLayout::verticalSpacing();
impl<'a> /*trait*/ QGraphicsGridLayout_verticalSpacing<f64> for () {
  fn verticalSpacing(self , rsthis: & QGraphicsGridLayout) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QGraphicsGridLayout15verticalSpacingEv()};
    let mut ret = unsafe {_ZNK19QGraphicsGridLayout15verticalSpacingEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  int QGraphicsGridLayout::columnStretchFactor(int column);
impl /*struct*/ QGraphicsGridLayout {
  pub fn columnStretchFactor<RetType, T: QGraphicsGridLayout_columnStretchFactor<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.columnStretchFactor(self);
    // return 1;
  }
}

pub trait QGraphicsGridLayout_columnStretchFactor<RetType> {
  fn columnStretchFactor(self , rsthis: & QGraphicsGridLayout) -> RetType;
}

  // proto:  int QGraphicsGridLayout::columnStretchFactor(int column);
impl<'a> /*trait*/ QGraphicsGridLayout_columnStretchFactor<i32> for (i32) {
  fn columnStretchFactor(self , rsthis: & QGraphicsGridLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QGraphicsGridLayout19columnStretchFactorEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK19QGraphicsGridLayout19columnStretchFactorEi(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QGraphicsGridLayout::setRowMaximumHeight(int row, qreal height);
impl /*struct*/ QGraphicsGridLayout {
  pub fn setRowMaximumHeight<RetType, T: QGraphicsGridLayout_setRowMaximumHeight<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setRowMaximumHeight(self);
    // return 1;
  }
}

pub trait QGraphicsGridLayout_setRowMaximumHeight<RetType> {
  fn setRowMaximumHeight(self , rsthis: & QGraphicsGridLayout) -> RetType;
}

  // proto:  void QGraphicsGridLayout::setRowMaximumHeight(int row, qreal height);
impl<'a> /*trait*/ QGraphicsGridLayout_setRowMaximumHeight<()> for (i32, f64) {
  fn setRowMaximumHeight(self , rsthis: & QGraphicsGridLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsGridLayout19setRowMaximumHeightEid()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_double;
     unsafe {_ZN19QGraphicsGridLayout19setRowMaximumHeightEid(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QGraphicsGridLayout::removeItem(QGraphicsLayoutItem * item);
impl /*struct*/ QGraphicsGridLayout {
  pub fn removeItem<RetType, T: QGraphicsGridLayout_removeItem<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.removeItem(self);
    // return 1;
  }
}

pub trait QGraphicsGridLayout_removeItem<RetType> {
  fn removeItem(self , rsthis: & QGraphicsGridLayout) -> RetType;
}

  // proto:  void QGraphicsGridLayout::removeItem(QGraphicsLayoutItem * item);
impl<'a> /*trait*/ QGraphicsGridLayout_removeItem<()> for (&'a QGraphicsLayoutItem) {
  fn removeItem(self , rsthis: & QGraphicsGridLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsGridLayout10removeItemEP19QGraphicsLayoutItem()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN19QGraphicsGridLayout10removeItemEP19QGraphicsLayoutItem(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QGraphicsGridLayout::~QGraphicsGridLayout();
impl /*struct*/ QGraphicsGridLayout {
  pub fn Free<RetType, T: QGraphicsGridLayout_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QGraphicsGridLayout_Free<RetType> {
  fn Free(self , rsthis: & QGraphicsGridLayout) -> RetType;
}

  // proto:  void QGraphicsGridLayout::~QGraphicsGridLayout();
impl<'a> /*trait*/ QGraphicsGridLayout_Free<()> for () {
  fn Free(self , rsthis: & QGraphicsGridLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsGridLayoutD0Ev()};
     unsafe {_ZN19QGraphicsGridLayoutD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  qreal QGraphicsGridLayout::rowMinimumHeight(int row);
impl /*struct*/ QGraphicsGridLayout {
  pub fn rowMinimumHeight<RetType, T: QGraphicsGridLayout_rowMinimumHeight<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.rowMinimumHeight(self);
    // return 1;
  }
}

pub trait QGraphicsGridLayout_rowMinimumHeight<RetType> {
  fn rowMinimumHeight(self , rsthis: & QGraphicsGridLayout) -> RetType;
}

  // proto:  qreal QGraphicsGridLayout::rowMinimumHeight(int row);
impl<'a> /*trait*/ QGraphicsGridLayout_rowMinimumHeight<f64> for (i32) {
  fn rowMinimumHeight(self , rsthis: & QGraphicsGridLayout) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QGraphicsGridLayout16rowMinimumHeightEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK19QGraphicsGridLayout16rowMinimumHeightEi(rsthis.qclsinst, arg0)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  qreal QGraphicsGridLayout::rowMaximumHeight(int row);
impl /*struct*/ QGraphicsGridLayout {
  pub fn rowMaximumHeight<RetType, T: QGraphicsGridLayout_rowMaximumHeight<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.rowMaximumHeight(self);
    // return 1;
  }
}

pub trait QGraphicsGridLayout_rowMaximumHeight<RetType> {
  fn rowMaximumHeight(self , rsthis: & QGraphicsGridLayout) -> RetType;
}

  // proto:  qreal QGraphicsGridLayout::rowMaximumHeight(int row);
impl<'a> /*trait*/ QGraphicsGridLayout_rowMaximumHeight<f64> for (i32) {
  fn rowMaximumHeight(self , rsthis: & QGraphicsGridLayout) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QGraphicsGridLayout16rowMaximumHeightEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK19QGraphicsGridLayout16rowMaximumHeightEi(rsthis.qclsinst, arg0)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  void QGraphicsGridLayout::QGraphicsGridLayout(QGraphicsLayoutItem * parent);
impl /*struct*/ QGraphicsGridLayout {
  pub fn New<T: QGraphicsGridLayout_New>(value: T) -> QGraphicsGridLayout {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QGraphicsGridLayout_New {
  fn New(self) -> QGraphicsGridLayout;
}

  // proto:  void QGraphicsGridLayout::QGraphicsGridLayout(QGraphicsLayoutItem * parent);
impl<'a> /*trait*/ QGraphicsGridLayout_New for (&'a QGraphicsLayoutItem) {
  fn New(self) -> QGraphicsGridLayout {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsGridLayoutC1EP19QGraphicsLayoutItem()};
    let ctysz: c_int = unsafe{QGraphicsGridLayout_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN19QGraphicsGridLayoutC1EP19QGraphicsLayoutItem(qthis, arg0)};
    let qthis: *mut c_void = unsafe {dector_ZN19QGraphicsGridLayoutC1EP19QGraphicsLayoutItem(arg0)};
    let rsthis = QGraphicsGridLayout{/**/qbase: QGraphicsLayout::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QGraphicsGridLayout::setColumnSpacing(int column, qreal spacing);
impl /*struct*/ QGraphicsGridLayout {
  pub fn setColumnSpacing<RetType, T: QGraphicsGridLayout_setColumnSpacing<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setColumnSpacing(self);
    // return 1;
  }
}

pub trait QGraphicsGridLayout_setColumnSpacing<RetType> {
  fn setColumnSpacing(self , rsthis: & QGraphicsGridLayout) -> RetType;
}

  // proto:  void QGraphicsGridLayout::setColumnSpacing(int column, qreal spacing);
impl<'a> /*trait*/ QGraphicsGridLayout_setColumnSpacing<()> for (i32, f64) {
  fn setColumnSpacing(self , rsthis: & QGraphicsGridLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsGridLayout16setColumnSpacingEid()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_double;
     unsafe {_ZN19QGraphicsGridLayout16setColumnSpacingEid(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  qreal QGraphicsGridLayout::rowSpacing(int row);
impl /*struct*/ QGraphicsGridLayout {
  pub fn rowSpacing<RetType, T: QGraphicsGridLayout_rowSpacing<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.rowSpacing(self);
    // return 1;
  }
}

pub trait QGraphicsGridLayout_rowSpacing<RetType> {
  fn rowSpacing(self , rsthis: & QGraphicsGridLayout) -> RetType;
}

  // proto:  qreal QGraphicsGridLayout::rowSpacing(int row);
impl<'a> /*trait*/ QGraphicsGridLayout_rowSpacing<f64> for (i32) {
  fn rowSpacing(self , rsthis: & QGraphicsGridLayout) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QGraphicsGridLayout10rowSpacingEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK19QGraphicsGridLayout10rowSpacingEi(rsthis.qclsinst, arg0)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  qreal QGraphicsGridLayout::columnMaximumWidth(int column);
impl /*struct*/ QGraphicsGridLayout {
  pub fn columnMaximumWidth<RetType, T: QGraphicsGridLayout_columnMaximumWidth<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.columnMaximumWidth(self);
    // return 1;
  }
}

pub trait QGraphicsGridLayout_columnMaximumWidth<RetType> {
  fn columnMaximumWidth(self , rsthis: & QGraphicsGridLayout) -> RetType;
}

  // proto:  qreal QGraphicsGridLayout::columnMaximumWidth(int column);
impl<'a> /*trait*/ QGraphicsGridLayout_columnMaximumWidth<f64> for (i32) {
  fn columnMaximumWidth(self , rsthis: & QGraphicsGridLayout) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QGraphicsGridLayout18columnMaximumWidthEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK19QGraphicsGridLayout18columnMaximumWidthEi(rsthis.qclsinst, arg0)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  void QGraphicsGridLayout::setRowFixedHeight(int row, qreal height);
impl /*struct*/ QGraphicsGridLayout {
  pub fn setRowFixedHeight<RetType, T: QGraphicsGridLayout_setRowFixedHeight<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setRowFixedHeight(self);
    // return 1;
  }
}

pub trait QGraphicsGridLayout_setRowFixedHeight<RetType> {
  fn setRowFixedHeight(self , rsthis: & QGraphicsGridLayout) -> RetType;
}

  // proto:  void QGraphicsGridLayout::setRowFixedHeight(int row, qreal height);
impl<'a> /*trait*/ QGraphicsGridLayout_setRowFixedHeight<()> for (i32, f64) {
  fn setRowFixedHeight(self , rsthis: & QGraphicsGridLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsGridLayout17setRowFixedHeightEid()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_double;
     unsafe {_ZN19QGraphicsGridLayout17setRowFixedHeightEid(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  qreal QGraphicsGridLayout::rowPreferredHeight(int row);
impl /*struct*/ QGraphicsGridLayout {
  pub fn rowPreferredHeight<RetType, T: QGraphicsGridLayout_rowPreferredHeight<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.rowPreferredHeight(self);
    // return 1;
  }
}

pub trait QGraphicsGridLayout_rowPreferredHeight<RetType> {
  fn rowPreferredHeight(self , rsthis: & QGraphicsGridLayout) -> RetType;
}

  // proto:  qreal QGraphicsGridLayout::rowPreferredHeight(int row);
impl<'a> /*trait*/ QGraphicsGridLayout_rowPreferredHeight<f64> for (i32) {
  fn rowPreferredHeight(self , rsthis: & QGraphicsGridLayout) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QGraphicsGridLayout18rowPreferredHeightEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK19QGraphicsGridLayout18rowPreferredHeightEi(rsthis.qclsinst, arg0)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  QGraphicsLayoutItem * QGraphicsGridLayout::itemAt(int row, int column);
impl<'a> /*trait*/ QGraphicsGridLayout_itemAt<()> for (i32, i32) {
  fn itemAt(self , rsthis: & QGraphicsGridLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QGraphicsGridLayout6itemAtEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {_ZNK19QGraphicsGridLayout6itemAtEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QGraphicsGridLayout::setVerticalSpacing(qreal spacing);
impl /*struct*/ QGraphicsGridLayout {
  pub fn setVerticalSpacing<RetType, T: QGraphicsGridLayout_setVerticalSpacing<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setVerticalSpacing(self);
    // return 1;
  }
}

pub trait QGraphicsGridLayout_setVerticalSpacing<RetType> {
  fn setVerticalSpacing(self , rsthis: & QGraphicsGridLayout) -> RetType;
}

  // proto:  void QGraphicsGridLayout::setVerticalSpacing(qreal spacing);
impl<'a> /*trait*/ QGraphicsGridLayout_setVerticalSpacing<()> for (f64) {
  fn setVerticalSpacing(self , rsthis: & QGraphicsGridLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsGridLayout18setVerticalSpacingEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN19QGraphicsGridLayout18setVerticalSpacingEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QGraphicsGridLayout::setGeometry(const QRectF & rect);
impl /*struct*/ QGraphicsGridLayout {
  pub fn setGeometry<RetType, T: QGraphicsGridLayout_setGeometry<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setGeometry(self);
    // return 1;
  }
}

pub trait QGraphicsGridLayout_setGeometry<RetType> {
  fn setGeometry(self , rsthis: & QGraphicsGridLayout) -> RetType;
}

  // proto:  void QGraphicsGridLayout::setGeometry(const QRectF & rect);
impl<'a> /*trait*/ QGraphicsGridLayout_setGeometry<()> for (&'a QRectF) {
  fn setGeometry(self , rsthis: & QGraphicsGridLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsGridLayout11setGeometryERK6QRectF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN19QGraphicsGridLayout11setGeometryERK6QRectF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QGraphicsGridLayout::rowCount();
impl /*struct*/ QGraphicsGridLayout {
  pub fn rowCount<RetType, T: QGraphicsGridLayout_rowCount<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.rowCount(self);
    // return 1;
  }
}

pub trait QGraphicsGridLayout_rowCount<RetType> {
  fn rowCount(self , rsthis: & QGraphicsGridLayout) -> RetType;
}

  // proto:  int QGraphicsGridLayout::rowCount();
impl<'a> /*trait*/ QGraphicsGridLayout_rowCount<i32> for () {
  fn rowCount(self , rsthis: & QGraphicsGridLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QGraphicsGridLayout8rowCountEv()};
    let mut ret = unsafe {_ZNK19QGraphicsGridLayout8rowCountEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QGraphicsGridLayout::setSpacing(qreal spacing);
impl /*struct*/ QGraphicsGridLayout {
  pub fn setSpacing<RetType, T: QGraphicsGridLayout_setSpacing<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setSpacing(self);
    // return 1;
  }
}

pub trait QGraphicsGridLayout_setSpacing<RetType> {
  fn setSpacing(self , rsthis: & QGraphicsGridLayout) -> RetType;
}

  // proto:  void QGraphicsGridLayout::setSpacing(qreal spacing);
impl<'a> /*trait*/ QGraphicsGridLayout_setSpacing<()> for (f64) {
  fn setSpacing(self , rsthis: & QGraphicsGridLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsGridLayout10setSpacingEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN19QGraphicsGridLayout10setSpacingEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QGraphicsGridLayout::setRowStretchFactor(int row, int stretch);
impl /*struct*/ QGraphicsGridLayout {
  pub fn setRowStretchFactor<RetType, T: QGraphicsGridLayout_setRowStretchFactor<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setRowStretchFactor(self);
    // return 1;
  }
}

pub trait QGraphicsGridLayout_setRowStretchFactor<RetType> {
  fn setRowStretchFactor(self , rsthis: & QGraphicsGridLayout) -> RetType;
}

  // proto:  void QGraphicsGridLayout::setRowStretchFactor(int row, int stretch);
impl<'a> /*trait*/ QGraphicsGridLayout_setRowStretchFactor<()> for (i32, i32) {
  fn setRowStretchFactor(self , rsthis: & QGraphicsGridLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsGridLayout19setRowStretchFactorEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {_ZN19QGraphicsGridLayout19setRowStretchFactorEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  qreal QGraphicsGridLayout::columnMinimumWidth(int column);
impl /*struct*/ QGraphicsGridLayout {
  pub fn columnMinimumWidth<RetType, T: QGraphicsGridLayout_columnMinimumWidth<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.columnMinimumWidth(self);
    // return 1;
  }
}

pub trait QGraphicsGridLayout_columnMinimumWidth<RetType> {
  fn columnMinimumWidth(self , rsthis: & QGraphicsGridLayout) -> RetType;
}

  // proto:  qreal QGraphicsGridLayout::columnMinimumWidth(int column);
impl<'a> /*trait*/ QGraphicsGridLayout_columnMinimumWidth<f64> for (i32) {
  fn columnMinimumWidth(self , rsthis: & QGraphicsGridLayout) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QGraphicsGridLayout18columnMinimumWidthEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK19QGraphicsGridLayout18columnMinimumWidthEi(rsthis.qclsinst, arg0)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  void QGraphicsGridLayout::setColumnMinimumWidth(int column, qreal width);
impl /*struct*/ QGraphicsGridLayout {
  pub fn setColumnMinimumWidth<RetType, T: QGraphicsGridLayout_setColumnMinimumWidth<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setColumnMinimumWidth(self);
    // return 1;
  }
}

pub trait QGraphicsGridLayout_setColumnMinimumWidth<RetType> {
  fn setColumnMinimumWidth(self , rsthis: & QGraphicsGridLayout) -> RetType;
}

  // proto:  void QGraphicsGridLayout::setColumnMinimumWidth(int column, qreal width);
impl<'a> /*trait*/ QGraphicsGridLayout_setColumnMinimumWidth<()> for (i32, f64) {
  fn setColumnMinimumWidth(self , rsthis: & QGraphicsGridLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsGridLayout21setColumnMinimumWidthEid()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_double;
     unsafe {_ZN19QGraphicsGridLayout21setColumnMinimumWidthEid(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QGraphicsGridLayout::setRowMinimumHeight(int row, qreal height);
impl /*struct*/ QGraphicsGridLayout {
  pub fn setRowMinimumHeight<RetType, T: QGraphicsGridLayout_setRowMinimumHeight<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setRowMinimumHeight(self);
    // return 1;
  }
}

pub trait QGraphicsGridLayout_setRowMinimumHeight<RetType> {
  fn setRowMinimumHeight(self , rsthis: & QGraphicsGridLayout) -> RetType;
}

  // proto:  void QGraphicsGridLayout::setRowMinimumHeight(int row, qreal height);
impl<'a> /*trait*/ QGraphicsGridLayout_setRowMinimumHeight<()> for (i32, f64) {
  fn setRowMinimumHeight(self , rsthis: & QGraphicsGridLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsGridLayout19setRowMinimumHeightEid()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_double;
     unsafe {_ZN19QGraphicsGridLayout19setRowMinimumHeightEid(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QGraphicsGridLayout::setHorizontalSpacing(qreal spacing);
impl /*struct*/ QGraphicsGridLayout {
  pub fn setHorizontalSpacing<RetType, T: QGraphicsGridLayout_setHorizontalSpacing<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setHorizontalSpacing(self);
    // return 1;
  }
}

pub trait QGraphicsGridLayout_setHorizontalSpacing<RetType> {
  fn setHorizontalSpacing(self , rsthis: & QGraphicsGridLayout) -> RetType;
}

  // proto:  void QGraphicsGridLayout::setHorizontalSpacing(qreal spacing);
impl<'a> /*trait*/ QGraphicsGridLayout_setHorizontalSpacing<()> for (f64) {
  fn setHorizontalSpacing(self , rsthis: & QGraphicsGridLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsGridLayout20setHorizontalSpacingEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN19QGraphicsGridLayout20setHorizontalSpacingEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  qreal QGraphicsGridLayout::horizontalSpacing();
impl /*struct*/ QGraphicsGridLayout {
  pub fn horizontalSpacing<RetType, T: QGraphicsGridLayout_horizontalSpacing<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.horizontalSpacing(self);
    // return 1;
  }
}

pub trait QGraphicsGridLayout_horizontalSpacing<RetType> {
  fn horizontalSpacing(self , rsthis: & QGraphicsGridLayout) -> RetType;
}

  // proto:  qreal QGraphicsGridLayout::horizontalSpacing();
impl<'a> /*trait*/ QGraphicsGridLayout_horizontalSpacing<f64> for () {
  fn horizontalSpacing(self , rsthis: & QGraphicsGridLayout) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QGraphicsGridLayout17horizontalSpacingEv()};
    let mut ret = unsafe {_ZNK19QGraphicsGridLayout17horizontalSpacingEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  void QGraphicsGridLayout::setColumnStretchFactor(int column, int stretch);
impl /*struct*/ QGraphicsGridLayout {
  pub fn setColumnStretchFactor<RetType, T: QGraphicsGridLayout_setColumnStretchFactor<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setColumnStretchFactor(self);
    // return 1;
  }
}

pub trait QGraphicsGridLayout_setColumnStretchFactor<RetType> {
  fn setColumnStretchFactor(self , rsthis: & QGraphicsGridLayout) -> RetType;
}

  // proto:  void QGraphicsGridLayout::setColumnStretchFactor(int column, int stretch);
impl<'a> /*trait*/ QGraphicsGridLayout_setColumnStretchFactor<()> for (i32, i32) {
  fn setColumnStretchFactor(self , rsthis: & QGraphicsGridLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsGridLayout22setColumnStretchFactorEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {_ZN19QGraphicsGridLayout22setColumnStretchFactorEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QGraphicsGridLayout::invalidate();
impl /*struct*/ QGraphicsGridLayout {
  pub fn invalidate<RetType, T: QGraphicsGridLayout_invalidate<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.invalidate(self);
    // return 1;
  }
}

pub trait QGraphicsGridLayout_invalidate<RetType> {
  fn invalidate(self , rsthis: & QGraphicsGridLayout) -> RetType;
}

  // proto:  void QGraphicsGridLayout::invalidate();
impl<'a> /*trait*/ QGraphicsGridLayout_invalidate<()> for () {
  fn invalidate(self , rsthis: & QGraphicsGridLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsGridLayout10invalidateEv()};
     unsafe {_ZN19QGraphicsGridLayout10invalidateEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  qreal QGraphicsGridLayout::columnPreferredWidth(int column);
impl /*struct*/ QGraphicsGridLayout {
  pub fn columnPreferredWidth<RetType, T: QGraphicsGridLayout_columnPreferredWidth<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.columnPreferredWidth(self);
    // return 1;
  }
}

pub trait QGraphicsGridLayout_columnPreferredWidth<RetType> {
  fn columnPreferredWidth(self , rsthis: & QGraphicsGridLayout) -> RetType;
}

  // proto:  qreal QGraphicsGridLayout::columnPreferredWidth(int column);
impl<'a> /*trait*/ QGraphicsGridLayout_columnPreferredWidth<f64> for (i32) {
  fn columnPreferredWidth(self , rsthis: & QGraphicsGridLayout) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QGraphicsGridLayout20columnPreferredWidthEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK19QGraphicsGridLayout20columnPreferredWidthEi(rsthis.qclsinst, arg0)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  void QGraphicsGridLayout::setColumnPreferredWidth(int column, qreal width);
impl /*struct*/ QGraphicsGridLayout {
  pub fn setColumnPreferredWidth<RetType, T: QGraphicsGridLayout_setColumnPreferredWidth<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setColumnPreferredWidth(self);
    // return 1;
  }
}

pub trait QGraphicsGridLayout_setColumnPreferredWidth<RetType> {
  fn setColumnPreferredWidth(self , rsthis: & QGraphicsGridLayout) -> RetType;
}

  // proto:  void QGraphicsGridLayout::setColumnPreferredWidth(int column, qreal width);
impl<'a> /*trait*/ QGraphicsGridLayout_setColumnPreferredWidth<()> for (i32, f64) {
  fn setColumnPreferredWidth(self , rsthis: & QGraphicsGridLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsGridLayout23setColumnPreferredWidthEid()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_double;
     unsafe {_ZN19QGraphicsGridLayout23setColumnPreferredWidthEid(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  qreal QGraphicsGridLayout::columnSpacing(int column);
impl /*struct*/ QGraphicsGridLayout {
  pub fn columnSpacing<RetType, T: QGraphicsGridLayout_columnSpacing<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.columnSpacing(self);
    // return 1;
  }
}

pub trait QGraphicsGridLayout_columnSpacing<RetType> {
  fn columnSpacing(self , rsthis: & QGraphicsGridLayout) -> RetType;
}

  // proto:  qreal QGraphicsGridLayout::columnSpacing(int column);
impl<'a> /*trait*/ QGraphicsGridLayout_columnSpacing<f64> for (i32) {
  fn columnSpacing(self , rsthis: & QGraphicsGridLayout) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QGraphicsGridLayout13columnSpacingEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK19QGraphicsGridLayout13columnSpacingEi(rsthis.qclsinst, arg0)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  void QGraphicsGridLayout::QGraphicsGridLayout(const QGraphicsGridLayout & );
impl<'a> /*trait*/ QGraphicsGridLayout_New for (&'a QGraphicsGridLayout) {
  fn New(self) -> QGraphicsGridLayout {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsGridLayoutC1ERKS_()};
    let ctysz: c_int = unsafe{QGraphicsGridLayout_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN19QGraphicsGridLayoutC1ERKS_(qthis, arg0)};
    let qthis: *mut c_void = unsafe {dector_ZN19QGraphicsGridLayoutC1ERKS_(arg0)};
    let rsthis = QGraphicsGridLayout{/**/qbase: QGraphicsLayout::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QGraphicsGridLayout::setRowSpacing(int row, qreal spacing);
impl /*struct*/ QGraphicsGridLayout {
  pub fn setRowSpacing<RetType, T: QGraphicsGridLayout_setRowSpacing<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setRowSpacing(self);
    // return 1;
  }
}

pub trait QGraphicsGridLayout_setRowSpacing<RetType> {
  fn setRowSpacing(self , rsthis: & QGraphicsGridLayout) -> RetType;
}

  // proto:  void QGraphicsGridLayout::setRowSpacing(int row, qreal spacing);
impl<'a> /*trait*/ QGraphicsGridLayout_setRowSpacing<()> for (i32, f64) {
  fn setRowSpacing(self , rsthis: & QGraphicsGridLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsGridLayout13setRowSpacingEid()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_double;
     unsafe {_ZN19QGraphicsGridLayout13setRowSpacingEid(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QGraphicsGridLayout::removeAt(int index);
impl /*struct*/ QGraphicsGridLayout {
  pub fn removeAt<RetType, T: QGraphicsGridLayout_removeAt<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.removeAt(self);
    // return 1;
  }
}

pub trait QGraphicsGridLayout_removeAt<RetType> {
  fn removeAt(self , rsthis: & QGraphicsGridLayout) -> RetType;
}

  // proto:  void QGraphicsGridLayout::removeAt(int index);
impl<'a> /*trait*/ QGraphicsGridLayout_removeAt<()> for (i32) {
  fn removeAt(self , rsthis: & QGraphicsGridLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsGridLayout8removeAtEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN19QGraphicsGridLayout8removeAtEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// <= body block end

