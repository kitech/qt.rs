// auto generated, do not modify.
// created: Mon Dec 21 22:54:38 2015
// src-file: /QtWidgets/qboxlayout.h
// dst-file: /src/widgets/qboxlayout.rs
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
use super::qlayoutitem::QLayoutItem; // 773
use super::super::core::qsize::QSize; // 771
use super::qlayout::QLayout; // 773
use super::super::core::qrect::QRect; // 771
use super::qlayoutitem::QSpacerItem; // 773
// <= use block end

// ext block begin =>
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]

extern {
  // proto:  void QHBoxLayout::QHBoxLayout(QWidget * parent);
  fn _ZN11QHBoxLayoutC1EP7QWidget(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  const QMetaObject * QHBoxLayout::metaObject();
  fn _ZNK11QHBoxLayout10metaObjectEv(qthis: *mut c_void);
  // proto:  void QHBoxLayout::~QHBoxLayout();
  fn _ZN11QHBoxLayoutD0Ev(qthis: *mut c_void);
  // proto:  void QHBoxLayout::QHBoxLayout(const QHBoxLayout & );
  fn _ZN11QHBoxLayoutC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QHBoxLayout::QHBoxLayout();
  fn _ZN11QHBoxLayoutC1Ev(qthis: *mut c_void);
  // proto:  int QBoxLayout::spacing();
  fn _ZNK10QBoxLayout7spacingEv(qthis: *mut c_void) -> c_int;
  // proto:  bool QBoxLayout::hasHeightForWidth();
  fn _ZNK10QBoxLayout17hasHeightForWidthEv(qthis: *mut c_void) -> c_char;
  // proto:  void QBoxLayout::addItem(QLayoutItem * );
  fn _ZN10QBoxLayout7addItemEP11QLayoutItem(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QSize QBoxLayout::sizeHint();
  fn _ZNK10QBoxLayout8sizeHintEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QBoxLayout::~QBoxLayout();
  fn _ZN10QBoxLayoutD0Ev(qthis: *mut c_void);
  // proto:  void QBoxLayout::insertSpacing(int index, int size);
  fn _ZN10QBoxLayout13insertSpacingEii(qthis: *mut c_void, arg0: c_int, arg1: c_int);
  // proto:  void QBoxLayout::setStretch(int index, int stretch);
  fn _ZN10QBoxLayout10setStretchEii(qthis: *mut c_void, arg0: c_int, arg1: c_int);
  // proto:  void QBoxLayout::QBoxLayout(const QBoxLayout & );
  fn _ZN10QBoxLayoutC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QBoxLayout::insertStretch(int index, int stretch);
  fn _ZN10QBoxLayout13insertStretchEii(qthis: *mut c_void, arg0: c_int, arg1: c_int);
  // proto:  void QBoxLayout::addLayout(QLayout * layout, int stretch);
  fn _ZN10QBoxLayout9addLayoutEP7QLayouti(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int);
  // proto:  bool QBoxLayout::setStretchFactor(QWidget * w, int stretch);
  fn _ZN10QBoxLayout16setStretchFactorEP7QWidgeti(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int) -> c_char;
  // proto:  void QBoxLayout::invalidate();
  fn _ZN10QBoxLayout10invalidateEv(qthis: *mut c_void);
  // proto:  void QBoxLayout::setGeometry(const QRect & );
  fn _ZN10QBoxLayout11setGeometryERK5QRect(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QBoxLayout::addStretch(int stretch);
  fn _ZN10QBoxLayout10addStretchEi(qthis: *mut c_void, arg0: c_int);
  // proto:  void QBoxLayout::insertLayout(int index, QLayout * layout, int stretch);
  fn _ZN10QBoxLayout12insertLayoutEiP7QLayouti(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void, arg2: c_int);
  // proto:  bool QBoxLayout::setStretchFactor(QLayout * l, int stretch);
  fn _ZN10QBoxLayout16setStretchFactorEP7QLayouti(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int) -> c_char;
  // proto:  int QBoxLayout::count();
  fn _ZNK10QBoxLayout5countEv(qthis: *mut c_void) -> c_int;
  // proto:  QLayoutItem * QBoxLayout::itemAt(int );
  fn _ZNK10QBoxLayout6itemAtEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  const QMetaObject * QBoxLayout::metaObject();
  fn _ZNK10QBoxLayout10metaObjectEv(qthis: *mut c_void);
  // proto:  void QBoxLayout::insertSpacerItem(int index, QSpacerItem * spacerItem);
  fn _ZN10QBoxLayout16insertSpacerItemEiP11QSpacerItem(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void);
  // proto:  int QBoxLayout::heightForWidth(int );
  fn _ZNK10QBoxLayout14heightForWidthEi(qthis: *mut c_void, arg0: c_int) -> c_int;
  // proto:  void QBoxLayout::addStrut(int );
  fn _ZN10QBoxLayout8addStrutEi(qthis: *mut c_void, arg0: c_int);
  // proto:  QSize QBoxLayout::maximumSize();
  fn _ZNK10QBoxLayout11maximumSizeEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  int QBoxLayout::stretch(int index);
  fn _ZNK10QBoxLayout7stretchEi(qthis: *mut c_void, arg0: c_int) -> c_int;
  // proto:  void QBoxLayout::addSpacerItem(QSpacerItem * spacerItem);
  fn _ZN10QBoxLayout13addSpacerItemEP11QSpacerItem(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  int QBoxLayout::minimumHeightForWidth(int );
  fn _ZNK10QBoxLayout21minimumHeightForWidthEi(qthis: *mut c_void, arg0: c_int) -> c_int;
  // proto:  QSize QBoxLayout::minimumSize();
  fn _ZNK10QBoxLayout11minimumSizeEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QBoxLayout::setSpacing(int spacing);
  fn _ZN10QBoxLayout10setSpacingEi(qthis: *mut c_void, arg0: c_int);
  // proto:  QLayoutItem * QBoxLayout::takeAt(int );
  fn _ZN10QBoxLayout6takeAtEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  void QBoxLayout::insertItem(int index, QLayoutItem * );
  fn _ZN10QBoxLayout10insertItemEiP11QLayoutItem(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void);
  // proto:  void QBoxLayout::addSpacing(int size);
  fn _ZN10QBoxLayout10addSpacingEi(qthis: *mut c_void, arg0: c_int);
  // proto:  void QVBoxLayout::QVBoxLayout();
  fn _ZN11QVBoxLayoutC1Ev(qthis: *mut c_void);
  // proto:  const QMetaObject * QVBoxLayout::metaObject();
  fn _ZNK11QVBoxLayout10metaObjectEv(qthis: *mut c_void);
  // proto:  void QVBoxLayout::QVBoxLayout(const QVBoxLayout & );
  fn _ZN11QVBoxLayoutC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QVBoxLayout::QVBoxLayout(QWidget * parent);
  fn _ZN11QVBoxLayoutC1EP7QWidget(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QVBoxLayout::~QVBoxLayout();
  fn _ZN11QVBoxLayoutD0Ev(qthis: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QHBoxLayout)=1
pub struct QHBoxLayout {
  pub qclsinst: *mut c_void,
}

// class sizeof(QBoxLayout)=1
pub struct QBoxLayout {
  pub qclsinst: *mut c_void,
}

// class sizeof(QVBoxLayout)=1
pub struct QVBoxLayout {
  pub qclsinst: *mut c_void,
}

  // proto:  void QHBoxLayout::QHBoxLayout(QWidget * parent);
impl /*struct*/ QHBoxLayout {
  pub fn NewQHBoxLayout<T: QHBoxLayout_NewQHBoxLayout>(value: T) -> QHBoxLayout {
    let rsthis = value.NewQHBoxLayout();
    return rsthis;
    // return 1;
  }
}

pub trait QHBoxLayout_NewQHBoxLayout {
  fn NewQHBoxLayout(self) -> QHBoxLayout;
}

  // proto:  void QHBoxLayout::QHBoxLayout(QWidget * parent);
impl<'a> /*trait*/ QHBoxLayout_NewQHBoxLayout for (QWidget) {
  fn NewQHBoxLayout(self) -> QHBoxLayout {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QHBoxLayoutC1EP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN11QHBoxLayoutC1EP7QWidget(qthis, arg0)};
    let rsthis = QHBoxLayout{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  const QMetaObject * QHBoxLayout::metaObject();
impl /*struct*/ QHBoxLayout {
  pub fn metaObject<RetType, T: QHBoxLayout_metaObject<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QHBoxLayout_metaObject<RetType> {
  fn metaObject(self , rsthis: &mut QHBoxLayout) -> RetType;
}

  // proto:  const QMetaObject * QHBoxLayout::metaObject();
impl<'a> /*trait*/ QHBoxLayout_metaObject<()> for () {
  fn metaObject(self , rsthis: &mut QHBoxLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QHBoxLayout10metaObjectEv()};
     unsafe {_ZNK11QHBoxLayout10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QHBoxLayout::~QHBoxLayout();
impl /*struct*/ QHBoxLayout {
  pub fn FreeQHBoxLayout<RetType, T: QHBoxLayout_FreeQHBoxLayout<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQHBoxLayout(self);
    // return 1;
  }
}

pub trait QHBoxLayout_FreeQHBoxLayout<RetType> {
  fn FreeQHBoxLayout(self , rsthis: &mut QHBoxLayout) -> RetType;
}

  // proto:  void QHBoxLayout::~QHBoxLayout();
impl<'a> /*trait*/ QHBoxLayout_FreeQHBoxLayout<()> for () {
  fn FreeQHBoxLayout(self , rsthis: &mut QHBoxLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QHBoxLayoutD0Ev()};
     unsafe {_ZN11QHBoxLayoutD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QHBoxLayout::QHBoxLayout(const QHBoxLayout & );
impl<'a> /*trait*/ QHBoxLayout_NewQHBoxLayout for (QHBoxLayout) {
  fn NewQHBoxLayout(self) -> QHBoxLayout {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QHBoxLayoutC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN11QHBoxLayoutC1ERKS_(qthis, arg0)};
    let rsthis = QHBoxLayout{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QHBoxLayout::QHBoxLayout();
impl<'a> /*trait*/ QHBoxLayout_NewQHBoxLayout for () {
  fn NewQHBoxLayout(self) -> QHBoxLayout {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QHBoxLayoutC1Ev()};
    unsafe {_ZN11QHBoxLayoutC1Ev(qthis)};
    let rsthis = QHBoxLayout{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  int QBoxLayout::spacing();
impl /*struct*/ QBoxLayout {
  pub fn spacing<RetType, T: QBoxLayout_spacing<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.spacing(self);
    // return 1;
  }
}

pub trait QBoxLayout_spacing<RetType> {
  fn spacing(self , rsthis: &mut QBoxLayout) -> RetType;
}

  // proto:  int QBoxLayout::spacing();
impl<'a> /*trait*/ QBoxLayout_spacing<i32> for () {
  fn spacing(self , rsthis: &mut QBoxLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QBoxLayout7spacingEv()};
    let mut ret = unsafe {_ZNK10QBoxLayout7spacingEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  bool QBoxLayout::hasHeightForWidth();
impl /*struct*/ QBoxLayout {
  pub fn hasHeightForWidth<RetType, T: QBoxLayout_hasHeightForWidth<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.hasHeightForWidth(self);
    // return 1;
  }
}

pub trait QBoxLayout_hasHeightForWidth<RetType> {
  fn hasHeightForWidth(self , rsthis: &mut QBoxLayout) -> RetType;
}

  // proto:  bool QBoxLayout::hasHeightForWidth();
impl<'a> /*trait*/ QBoxLayout_hasHeightForWidth<i8> for () {
  fn hasHeightForWidth(self , rsthis: &mut QBoxLayout) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QBoxLayout17hasHeightForWidthEv()};
    let mut ret = unsafe {_ZNK10QBoxLayout17hasHeightForWidthEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QBoxLayout::addItem(QLayoutItem * );
impl /*struct*/ QBoxLayout {
  pub fn addItem<RetType, T: QBoxLayout_addItem<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.addItem(self);
    // return 1;
  }
}

pub trait QBoxLayout_addItem<RetType> {
  fn addItem(self , rsthis: &mut QBoxLayout) -> RetType;
}

  // proto:  void QBoxLayout::addItem(QLayoutItem * );
impl<'a> /*trait*/ QBoxLayout_addItem<()> for (QLayoutItem) {
  fn addItem(self , rsthis: &mut QBoxLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QBoxLayout7addItemEP11QLayoutItem()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN10QBoxLayout7addItemEP11QLayoutItem(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QSize QBoxLayout::sizeHint();
impl /*struct*/ QBoxLayout {
  pub fn sizeHint<RetType, T: QBoxLayout_sizeHint<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.sizeHint(self);
    // return 1;
  }
}

pub trait QBoxLayout_sizeHint<RetType> {
  fn sizeHint(self , rsthis: &mut QBoxLayout) -> RetType;
}

  // proto:  QSize QBoxLayout::sizeHint();
impl<'a> /*trait*/ QBoxLayout_sizeHint<QSize> for () {
  fn sizeHint(self , rsthis: &mut QBoxLayout) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QBoxLayout8sizeHintEv()};
    let mut ret = unsafe {_ZNK10QBoxLayout8sizeHintEv(rsthis.qclsinst)};
    let mut ret1 = QSize{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QBoxLayout::~QBoxLayout();
impl /*struct*/ QBoxLayout {
  pub fn FreeQBoxLayout<RetType, T: QBoxLayout_FreeQBoxLayout<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQBoxLayout(self);
    // return 1;
  }
}

pub trait QBoxLayout_FreeQBoxLayout<RetType> {
  fn FreeQBoxLayout(self , rsthis: &mut QBoxLayout) -> RetType;
}

  // proto:  void QBoxLayout::~QBoxLayout();
impl<'a> /*trait*/ QBoxLayout_FreeQBoxLayout<()> for () {
  fn FreeQBoxLayout(self , rsthis: &mut QBoxLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QBoxLayoutD0Ev()};
     unsafe {_ZN10QBoxLayoutD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QBoxLayout::insertSpacing(int index, int size);
impl /*struct*/ QBoxLayout {
  pub fn insertSpacing<RetType, T: QBoxLayout_insertSpacing<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.insertSpacing(self);
    // return 1;
  }
}

pub trait QBoxLayout_insertSpacing<RetType> {
  fn insertSpacing(self , rsthis: &mut QBoxLayout) -> RetType;
}

  // proto:  void QBoxLayout::insertSpacing(int index, int size);
impl<'a> /*trait*/ QBoxLayout_insertSpacing<()> for (i32, i32) {
  fn insertSpacing(self , rsthis: &mut QBoxLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QBoxLayout13insertSpacingEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {_ZN10QBoxLayout13insertSpacingEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QBoxLayout::setStretch(int index, int stretch);
impl /*struct*/ QBoxLayout {
  pub fn setStretch<RetType, T: QBoxLayout_setStretch<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setStretch(self);
    // return 1;
  }
}

pub trait QBoxLayout_setStretch<RetType> {
  fn setStretch(self , rsthis: &mut QBoxLayout) -> RetType;
}

  // proto:  void QBoxLayout::setStretch(int index, int stretch);
impl<'a> /*trait*/ QBoxLayout_setStretch<()> for (i32, i32) {
  fn setStretch(self , rsthis: &mut QBoxLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QBoxLayout10setStretchEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {_ZN10QBoxLayout10setStretchEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QBoxLayout::QBoxLayout(const QBoxLayout & );
impl /*struct*/ QBoxLayout {
  pub fn NewQBoxLayout<T: QBoxLayout_NewQBoxLayout>(value: T) -> QBoxLayout {
    let rsthis = value.NewQBoxLayout();
    return rsthis;
    // return 1;
  }
}

pub trait QBoxLayout_NewQBoxLayout {
  fn NewQBoxLayout(self) -> QBoxLayout;
}

  // proto:  void QBoxLayout::QBoxLayout(const QBoxLayout & );
impl<'a> /*trait*/ QBoxLayout_NewQBoxLayout for (QBoxLayout) {
  fn NewQBoxLayout(self) -> QBoxLayout {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QBoxLayoutC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN10QBoxLayoutC1ERKS_(qthis, arg0)};
    let rsthis = QBoxLayout{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QBoxLayout::insertStretch(int index, int stretch);
impl /*struct*/ QBoxLayout {
  pub fn insertStretch<RetType, T: QBoxLayout_insertStretch<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.insertStretch(self);
    // return 1;
  }
}

pub trait QBoxLayout_insertStretch<RetType> {
  fn insertStretch(self , rsthis: &mut QBoxLayout) -> RetType;
}

  // proto:  void QBoxLayout::insertStretch(int index, int stretch);
impl<'a> /*trait*/ QBoxLayout_insertStretch<()> for (i32, i32) {
  fn insertStretch(self , rsthis: &mut QBoxLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QBoxLayout13insertStretchEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {_ZN10QBoxLayout13insertStretchEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QBoxLayout::addLayout(QLayout * layout, int stretch);
impl /*struct*/ QBoxLayout {
  pub fn addLayout<RetType, T: QBoxLayout_addLayout<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.addLayout(self);
    // return 1;
  }
}

pub trait QBoxLayout_addLayout<RetType> {
  fn addLayout(self , rsthis: &mut QBoxLayout) -> RetType;
}

  // proto:  void QBoxLayout::addLayout(QLayout * layout, int stretch);
impl<'a> /*trait*/ QBoxLayout_addLayout<()> for (QLayout, i32) {
  fn addLayout(self , rsthis: &mut QBoxLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QBoxLayout9addLayoutEP7QLayouti()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
     unsafe {_ZN10QBoxLayout9addLayoutEP7QLayouti(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  bool QBoxLayout::setStretchFactor(QWidget * w, int stretch);
impl /*struct*/ QBoxLayout {
  pub fn setStretchFactor<RetType, T: QBoxLayout_setStretchFactor<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setStretchFactor(self);
    // return 1;
  }
}

pub trait QBoxLayout_setStretchFactor<RetType> {
  fn setStretchFactor(self , rsthis: &mut QBoxLayout) -> RetType;
}

  // proto:  bool QBoxLayout::setStretchFactor(QWidget * w, int stretch);
impl<'a> /*trait*/ QBoxLayout_setStretchFactor<i8> for (QWidget, i32) {
  fn setStretchFactor(self , rsthis: &mut QBoxLayout) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QBoxLayout16setStretchFactorEP7QWidgeti()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZN10QBoxLayout16setStretchFactorEP7QWidgeti(rsthis.qclsinst, arg0, arg1)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QBoxLayout::invalidate();
impl /*struct*/ QBoxLayout {
  pub fn invalidate<RetType, T: QBoxLayout_invalidate<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.invalidate(self);
    // return 1;
  }
}

pub trait QBoxLayout_invalidate<RetType> {
  fn invalidate(self , rsthis: &mut QBoxLayout) -> RetType;
}

  // proto:  void QBoxLayout::invalidate();
impl<'a> /*trait*/ QBoxLayout_invalidate<()> for () {
  fn invalidate(self , rsthis: &mut QBoxLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QBoxLayout10invalidateEv()};
     unsafe {_ZN10QBoxLayout10invalidateEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QBoxLayout::setGeometry(const QRect & );
impl /*struct*/ QBoxLayout {
  pub fn setGeometry<RetType, T: QBoxLayout_setGeometry<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setGeometry(self);
    // return 1;
  }
}

pub trait QBoxLayout_setGeometry<RetType> {
  fn setGeometry(self , rsthis: &mut QBoxLayout) -> RetType;
}

  // proto:  void QBoxLayout::setGeometry(const QRect & );
impl<'a> /*trait*/ QBoxLayout_setGeometry<()> for (QRect) {
  fn setGeometry(self , rsthis: &mut QBoxLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QBoxLayout11setGeometryERK5QRect()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN10QBoxLayout11setGeometryERK5QRect(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QBoxLayout::addStretch(int stretch);
impl /*struct*/ QBoxLayout {
  pub fn addStretch<RetType, T: QBoxLayout_addStretch<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.addStretch(self);
    // return 1;
  }
}

pub trait QBoxLayout_addStretch<RetType> {
  fn addStretch(self , rsthis: &mut QBoxLayout) -> RetType;
}

  // proto:  void QBoxLayout::addStretch(int stretch);
impl<'a> /*trait*/ QBoxLayout_addStretch<()> for (i32) {
  fn addStretch(self , rsthis: &mut QBoxLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QBoxLayout10addStretchEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN10QBoxLayout10addStretchEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QBoxLayout::insertLayout(int index, QLayout * layout, int stretch);
impl /*struct*/ QBoxLayout {
  pub fn insertLayout<RetType, T: QBoxLayout_insertLayout<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.insertLayout(self);
    // return 1;
  }
}

pub trait QBoxLayout_insertLayout<RetType> {
  fn insertLayout(self , rsthis: &mut QBoxLayout) -> RetType;
}

  // proto:  void QBoxLayout::insertLayout(int index, QLayout * layout, int stretch);
impl<'a> /*trait*/ QBoxLayout_insertLayout<()> for (i32, QLayout, i32) {
  fn insertLayout(self , rsthis: &mut QBoxLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QBoxLayout12insertLayoutEiP7QLayouti()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2  as c_int;
     unsafe {_ZN10QBoxLayout12insertLayoutEiP7QLayouti(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  bool QBoxLayout::setStretchFactor(QLayout * l, int stretch);
impl<'a> /*trait*/ QBoxLayout_setStretchFactor<i8> for (QLayout, i32) {
  fn setStretchFactor(self , rsthis: &mut QBoxLayout) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QBoxLayout16setStretchFactorEP7QLayouti()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZN10QBoxLayout16setStretchFactorEP7QLayouti(rsthis.qclsinst, arg0, arg1)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  int QBoxLayout::count();
impl /*struct*/ QBoxLayout {
  pub fn count<RetType, T: QBoxLayout_count<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.count(self);
    // return 1;
  }
}

pub trait QBoxLayout_count<RetType> {
  fn count(self , rsthis: &mut QBoxLayout) -> RetType;
}

  // proto:  int QBoxLayout::count();
impl<'a> /*trait*/ QBoxLayout_count<i32> for () {
  fn count(self , rsthis: &mut QBoxLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QBoxLayout5countEv()};
    let mut ret = unsafe {_ZNK10QBoxLayout5countEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  QLayoutItem * QBoxLayout::itemAt(int );
impl /*struct*/ QBoxLayout {
  pub fn itemAt<RetType, T: QBoxLayout_itemAt<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.itemAt(self);
    // return 1;
  }
}

pub trait QBoxLayout_itemAt<RetType> {
  fn itemAt(self , rsthis: &mut QBoxLayout) -> RetType;
}

  // proto:  QLayoutItem * QBoxLayout::itemAt(int );
impl<'a> /*trait*/ QBoxLayout_itemAt<QLayoutItem> for (i32) {
  fn itemAt(self , rsthis: &mut QBoxLayout) -> QLayoutItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QBoxLayout6itemAtEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK10QBoxLayout6itemAtEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QLayoutItem{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  const QMetaObject * QBoxLayout::metaObject();
impl /*struct*/ QBoxLayout {
  pub fn metaObject<RetType, T: QBoxLayout_metaObject<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QBoxLayout_metaObject<RetType> {
  fn metaObject(self , rsthis: &mut QBoxLayout) -> RetType;
}

  // proto:  const QMetaObject * QBoxLayout::metaObject();
impl<'a> /*trait*/ QBoxLayout_metaObject<()> for () {
  fn metaObject(self , rsthis: &mut QBoxLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QBoxLayout10metaObjectEv()};
     unsafe {_ZNK10QBoxLayout10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QBoxLayout::insertSpacerItem(int index, QSpacerItem * spacerItem);
impl /*struct*/ QBoxLayout {
  pub fn insertSpacerItem<RetType, T: QBoxLayout_insertSpacerItem<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.insertSpacerItem(self);
    // return 1;
  }
}

pub trait QBoxLayout_insertSpacerItem<RetType> {
  fn insertSpacerItem(self , rsthis: &mut QBoxLayout) -> RetType;
}

  // proto:  void QBoxLayout::insertSpacerItem(int index, QSpacerItem * spacerItem);
impl<'a> /*trait*/ QBoxLayout_insertSpacerItem<()> for (i32, QSpacerItem) {
  fn insertSpacerItem(self , rsthis: &mut QBoxLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QBoxLayout16insertSpacerItemEiP11QSpacerItem()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN10QBoxLayout16insertSpacerItemEiP11QSpacerItem(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  int QBoxLayout::heightForWidth(int );
impl /*struct*/ QBoxLayout {
  pub fn heightForWidth<RetType, T: QBoxLayout_heightForWidth<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.heightForWidth(self);
    // return 1;
  }
}

pub trait QBoxLayout_heightForWidth<RetType> {
  fn heightForWidth(self , rsthis: &mut QBoxLayout) -> RetType;
}

  // proto:  int QBoxLayout::heightForWidth(int );
impl<'a> /*trait*/ QBoxLayout_heightForWidth<i32> for (i32) {
  fn heightForWidth(self , rsthis: &mut QBoxLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QBoxLayout14heightForWidthEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK10QBoxLayout14heightForWidthEi(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QBoxLayout::addStrut(int );
impl /*struct*/ QBoxLayout {
  pub fn addStrut<RetType, T: QBoxLayout_addStrut<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.addStrut(self);
    // return 1;
  }
}

pub trait QBoxLayout_addStrut<RetType> {
  fn addStrut(self , rsthis: &mut QBoxLayout) -> RetType;
}

  // proto:  void QBoxLayout::addStrut(int );
impl<'a> /*trait*/ QBoxLayout_addStrut<()> for (i32) {
  fn addStrut(self , rsthis: &mut QBoxLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QBoxLayout8addStrutEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN10QBoxLayout8addStrutEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QSize QBoxLayout::maximumSize();
impl /*struct*/ QBoxLayout {
  pub fn maximumSize<RetType, T: QBoxLayout_maximumSize<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.maximumSize(self);
    // return 1;
  }
}

pub trait QBoxLayout_maximumSize<RetType> {
  fn maximumSize(self , rsthis: &mut QBoxLayout) -> RetType;
}

  // proto:  QSize QBoxLayout::maximumSize();
impl<'a> /*trait*/ QBoxLayout_maximumSize<QSize> for () {
  fn maximumSize(self , rsthis: &mut QBoxLayout) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QBoxLayout11maximumSizeEv()};
    let mut ret = unsafe {_ZNK10QBoxLayout11maximumSizeEv(rsthis.qclsinst)};
    let mut ret1 = QSize{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  int QBoxLayout::stretch(int index);
impl /*struct*/ QBoxLayout {
  pub fn stretch<RetType, T: QBoxLayout_stretch<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.stretch(self);
    // return 1;
  }
}

pub trait QBoxLayout_stretch<RetType> {
  fn stretch(self , rsthis: &mut QBoxLayout) -> RetType;
}

  // proto:  int QBoxLayout::stretch(int index);
impl<'a> /*trait*/ QBoxLayout_stretch<i32> for (i32) {
  fn stretch(self , rsthis: &mut QBoxLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QBoxLayout7stretchEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK10QBoxLayout7stretchEi(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QBoxLayout::addSpacerItem(QSpacerItem * spacerItem);
impl /*struct*/ QBoxLayout {
  pub fn addSpacerItem<RetType, T: QBoxLayout_addSpacerItem<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.addSpacerItem(self);
    // return 1;
  }
}

pub trait QBoxLayout_addSpacerItem<RetType> {
  fn addSpacerItem(self , rsthis: &mut QBoxLayout) -> RetType;
}

  // proto:  void QBoxLayout::addSpacerItem(QSpacerItem * spacerItem);
impl<'a> /*trait*/ QBoxLayout_addSpacerItem<()> for (QSpacerItem) {
  fn addSpacerItem(self , rsthis: &mut QBoxLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QBoxLayout13addSpacerItemEP11QSpacerItem()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN10QBoxLayout13addSpacerItemEP11QSpacerItem(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QBoxLayout::minimumHeightForWidth(int );
impl /*struct*/ QBoxLayout {
  pub fn minimumHeightForWidth<RetType, T: QBoxLayout_minimumHeightForWidth<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.minimumHeightForWidth(self);
    // return 1;
  }
}

pub trait QBoxLayout_minimumHeightForWidth<RetType> {
  fn minimumHeightForWidth(self , rsthis: &mut QBoxLayout) -> RetType;
}

  // proto:  int QBoxLayout::minimumHeightForWidth(int );
impl<'a> /*trait*/ QBoxLayout_minimumHeightForWidth<i32> for (i32) {
  fn minimumHeightForWidth(self , rsthis: &mut QBoxLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QBoxLayout21minimumHeightForWidthEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK10QBoxLayout21minimumHeightForWidthEi(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  QSize QBoxLayout::minimumSize();
impl /*struct*/ QBoxLayout {
  pub fn minimumSize<RetType, T: QBoxLayout_minimumSize<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.minimumSize(self);
    // return 1;
  }
}

pub trait QBoxLayout_minimumSize<RetType> {
  fn minimumSize(self , rsthis: &mut QBoxLayout) -> RetType;
}

  // proto:  QSize QBoxLayout::minimumSize();
impl<'a> /*trait*/ QBoxLayout_minimumSize<QSize> for () {
  fn minimumSize(self , rsthis: &mut QBoxLayout) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QBoxLayout11minimumSizeEv()};
    let mut ret = unsafe {_ZNK10QBoxLayout11minimumSizeEv(rsthis.qclsinst)};
    let mut ret1 = QSize{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QBoxLayout::setSpacing(int spacing);
impl /*struct*/ QBoxLayout {
  pub fn setSpacing<RetType, T: QBoxLayout_setSpacing<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setSpacing(self);
    // return 1;
  }
}

pub trait QBoxLayout_setSpacing<RetType> {
  fn setSpacing(self , rsthis: &mut QBoxLayout) -> RetType;
}

  // proto:  void QBoxLayout::setSpacing(int spacing);
impl<'a> /*trait*/ QBoxLayout_setSpacing<()> for (i32) {
  fn setSpacing(self , rsthis: &mut QBoxLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QBoxLayout10setSpacingEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN10QBoxLayout10setSpacingEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QLayoutItem * QBoxLayout::takeAt(int );
impl /*struct*/ QBoxLayout {
  pub fn takeAt<RetType, T: QBoxLayout_takeAt<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.takeAt(self);
    // return 1;
  }
}

pub trait QBoxLayout_takeAt<RetType> {
  fn takeAt(self , rsthis: &mut QBoxLayout) -> RetType;
}

  // proto:  QLayoutItem * QBoxLayout::takeAt(int );
impl<'a> /*trait*/ QBoxLayout_takeAt<QLayoutItem> for (i32) {
  fn takeAt(self , rsthis: &mut QBoxLayout) -> QLayoutItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QBoxLayout6takeAtEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZN10QBoxLayout6takeAtEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QLayoutItem{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QBoxLayout::insertItem(int index, QLayoutItem * );
impl /*struct*/ QBoxLayout {
  pub fn insertItem<RetType, T: QBoxLayout_insertItem<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.insertItem(self);
    // return 1;
  }
}

pub trait QBoxLayout_insertItem<RetType> {
  fn insertItem(self , rsthis: &mut QBoxLayout) -> RetType;
}

  // proto:  void QBoxLayout::insertItem(int index, QLayoutItem * );
impl<'a> /*trait*/ QBoxLayout_insertItem<()> for (i32, QLayoutItem) {
  fn insertItem(self , rsthis: &mut QBoxLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QBoxLayout10insertItemEiP11QLayoutItem()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN10QBoxLayout10insertItemEiP11QLayoutItem(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QBoxLayout::addSpacing(int size);
impl /*struct*/ QBoxLayout {
  pub fn addSpacing<RetType, T: QBoxLayout_addSpacing<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.addSpacing(self);
    // return 1;
  }
}

pub trait QBoxLayout_addSpacing<RetType> {
  fn addSpacing(self , rsthis: &mut QBoxLayout) -> RetType;
}

  // proto:  void QBoxLayout::addSpacing(int size);
impl<'a> /*trait*/ QBoxLayout_addSpacing<()> for (i32) {
  fn addSpacing(self , rsthis: &mut QBoxLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QBoxLayout10addSpacingEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN10QBoxLayout10addSpacingEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QVBoxLayout::QVBoxLayout();
impl /*struct*/ QVBoxLayout {
  pub fn NewQVBoxLayout<T: QVBoxLayout_NewQVBoxLayout>(value: T) -> QVBoxLayout {
    let rsthis = value.NewQVBoxLayout();
    return rsthis;
    // return 1;
  }
}

pub trait QVBoxLayout_NewQVBoxLayout {
  fn NewQVBoxLayout(self) -> QVBoxLayout;
}

  // proto:  void QVBoxLayout::QVBoxLayout();
impl<'a> /*trait*/ QVBoxLayout_NewQVBoxLayout for () {
  fn NewQVBoxLayout(self) -> QVBoxLayout {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QVBoxLayoutC1Ev()};
    unsafe {_ZN11QVBoxLayoutC1Ev(qthis)};
    let rsthis = QVBoxLayout{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  const QMetaObject * QVBoxLayout::metaObject();
impl /*struct*/ QVBoxLayout {
  pub fn metaObject<RetType, T: QVBoxLayout_metaObject<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QVBoxLayout_metaObject<RetType> {
  fn metaObject(self , rsthis: &mut QVBoxLayout) -> RetType;
}

  // proto:  const QMetaObject * QVBoxLayout::metaObject();
impl<'a> /*trait*/ QVBoxLayout_metaObject<()> for () {
  fn metaObject(self , rsthis: &mut QVBoxLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QVBoxLayout10metaObjectEv()};
     unsafe {_ZNK11QVBoxLayout10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QVBoxLayout::QVBoxLayout(const QVBoxLayout & );
impl<'a> /*trait*/ QVBoxLayout_NewQVBoxLayout for (QVBoxLayout) {
  fn NewQVBoxLayout(self) -> QVBoxLayout {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QVBoxLayoutC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN11QVBoxLayoutC1ERKS_(qthis, arg0)};
    let rsthis = QVBoxLayout{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QVBoxLayout::QVBoxLayout(QWidget * parent);
impl<'a> /*trait*/ QVBoxLayout_NewQVBoxLayout for (QWidget) {
  fn NewQVBoxLayout(self) -> QVBoxLayout {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QVBoxLayoutC1EP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN11QVBoxLayoutC1EP7QWidget(qthis, arg0)};
    let rsthis = QVBoxLayout{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QVBoxLayout::~QVBoxLayout();
impl /*struct*/ QVBoxLayout {
  pub fn FreeQVBoxLayout<RetType, T: QVBoxLayout_FreeQVBoxLayout<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQVBoxLayout(self);
    // return 1;
  }
}

pub trait QVBoxLayout_FreeQVBoxLayout<RetType> {
  fn FreeQVBoxLayout(self , rsthis: &mut QVBoxLayout) -> RetType;
}

  // proto:  void QVBoxLayout::~QVBoxLayout();
impl<'a> /*trait*/ QVBoxLayout_FreeQVBoxLayout<()> for () {
  fn FreeQVBoxLayout(self , rsthis: &mut QVBoxLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QVBoxLayoutD0Ev()};
     unsafe {_ZN11QVBoxLayoutD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

// <= body block end

