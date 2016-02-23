// auto generated, do not modify.
// created: Mon Feb 22 23:57:02 2016
// src-file: /QtWidgets/qsplitter.h
// dst-file: /src/widgets/qsplitter.rs
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
use super::qframe::*; // 773
use std::ops::Deref;
use super::qwidget::*; // 773
use super::super::core::qbytearray::*; // 771
use super::super::core::qobjectdefs::*; // 771
use super::super::core::qsize::*; // 771
// use super::qlist::*; // 775
// use super::qsplitter::QSplitterHandle; // 773
// use super::qsplitter::QSplitter; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QSplitter_Class_Size() -> c_int;
  // proto:  void QSplitter::insertWidget(int index, QWidget * widget);
  fn C_ZN9QSplitter12insertWidgetEiP7QWidget(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: *mut c_void);
  // proto:  bool QSplitter::childrenCollapsible();
  fn C_ZNK9QSplitter19childrenCollapsibleEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  int QSplitter::count();
  fn C_ZNK9QSplitter5countEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  QByteArray QSplitter::saveState();
  fn C_ZNK9QSplitter9saveStateEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  const QMetaObject * QSplitter::metaObject();
  fn C_ZNK9QSplitter10metaObjectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  bool QSplitter::opaqueResize();
  fn C_ZNK9QSplitter12opaqueResizeEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QSplitter::addWidget(QWidget * widget);
  fn C_ZN9QSplitter9addWidgetEP7QWidget(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QSplitter::QSplitter(QWidget * parent);
  fn C_ZN9QSplitterC2EP7QWidget(arg0: *mut c_void) -> u64;
  // proto:  void QSplitter::setHandleWidth(int );
  fn C_ZN9QSplitter14setHandleWidthEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  void QSplitter::setStretchFactor(int index, int stretch);
  fn C_ZN9QSplitter16setStretchFactorEii(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_int);
  // proto:  QSize QSplitter::minimumSizeHint();
  fn C_ZNK9QSplitter15minimumSizeHintEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QSplitter::setOpaqueResize(bool opaque);
  fn C_ZN9QSplitter15setOpaqueResizeEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  QWidget * QSplitter::widget(int index);
  fn C_ZNK9QSplitter6widgetEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> *mut c_void;
  // proto:  QList<int> QSplitter::sizes();
  fn C_ZNK9QSplitter5sizesEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  bool QSplitter::isCollapsible(int index);
  fn C_ZNK9QSplitter13isCollapsibleEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> c_char;
  // proto:  void QSplitter::setCollapsible(int index, bool );
  fn C_ZN9QSplitter14setCollapsibleEib(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_char);
  // proto:  bool QSplitter::restoreState(const QByteArray & state);
  fn C_ZN9QSplitter12restoreStateERK10QByteArray(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_char;
  // proto:  void QSplitter::~QSplitter();
  fn C_ZN9QSplitterD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QSplitter::setChildrenCollapsible(bool );
  fn C_ZN9QSplitter22setChildrenCollapsibleEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  int QSplitter::handleWidth();
  fn C_ZNK9QSplitter11handleWidthEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QSplitter::refresh();
  fn C_ZN9QSplitter7refreshEv(qthis: u64 /* *mut c_void*/);
  // proto:  QSize QSplitter::sizeHint();
  fn C_ZNK9QSplitter8sizeHintEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  int QSplitter::indexOf(QWidget * w);
  fn C_ZNK9QSplitter7indexOfEP7QWidget(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_int;
  // proto:  void QSplitter::getRange(int index, int * , int * );
  fn C_ZNK9QSplitter8getRangeEiPiS0_(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: *mut c_int, arg2: *mut c_int);
  // proto:  QSplitterHandle * QSplitter::handle(int index);
  fn C_ZNK9QSplitter6handleEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> *mut c_void;
  fn QSplitterHandle_Class_Size() -> c_int;
  // proto:  void QSplitterHandle::~QSplitterHandle();
  fn C_ZN15QSplitterHandleD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  QSize QSplitterHandle::sizeHint();
  fn C_ZNK15QSplitterHandle8sizeHintEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  bool QSplitterHandle::opaqueResize();
  fn C_ZNK15QSplitterHandle12opaqueResizeEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  QSplitter * QSplitterHandle::splitter();
  fn C_ZNK15QSplitterHandle8splitterEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  const QMetaObject * QSplitterHandle::metaObject();
  fn C_ZNK15QSplitterHandle10metaObjectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  fn QSplitter_SlotProxy_connect__ZN9QSplitter13splitterMovedEii(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QSplitter)=1
#[derive(Default)]
pub struct QSplitter {
  qbase: QFrame,
  pub qclsinst: u64 /* *mut c_void*/,
  pub _splitterMoved: QSplitter_splitterMoved_signal,
}

// class sizeof(QSplitterHandle)=1
#[derive(Default)]
pub struct QSplitterHandle {
  qbase: QWidget,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QSplitter {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QSplitter {
    return QSplitter{qbase: QFrame::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QSplitter {
  type Target = QFrame;

  fn deref(&self) -> &QFrame {
    return & self.qbase;
  }
}
impl AsRef<QFrame> for QSplitter {
  fn as_ref(& self) -> & QFrame {
    return & self.qbase;
  }
}
  // proto:  void QSplitter::insertWidget(int index, QWidget * widget);
impl /*struct*/ QSplitter {
  pub fn insertWidget<RetType, T: QSplitter_insertWidget<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.insertWidget(self);
    // return 1;
  }
}

pub trait QSplitter_insertWidget<RetType> {
  fn insertWidget(self , rsthis: & QSplitter) -> RetType;
}

  // proto:  void QSplitter::insertWidget(int index, QWidget * widget);
impl<'a> /*trait*/ QSplitter_insertWidget<()> for (i32, &'a QWidget) {
  fn insertWidget(self , rsthis: & QSplitter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QSplitter12insertWidgetEiP7QWidget()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {C_ZN9QSplitter12insertWidgetEiP7QWidget(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  bool QSplitter::childrenCollapsible();
impl /*struct*/ QSplitter {
  pub fn childrenCollapsible<RetType, T: QSplitter_childrenCollapsible<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.childrenCollapsible(self);
    // return 1;
  }
}

pub trait QSplitter_childrenCollapsible<RetType> {
  fn childrenCollapsible(self , rsthis: & QSplitter) -> RetType;
}

  // proto:  bool QSplitter::childrenCollapsible();
impl<'a> /*trait*/ QSplitter_childrenCollapsible<i8> for () {
  fn childrenCollapsible(self , rsthis: & QSplitter) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QSplitter19childrenCollapsibleEv()};
    let mut ret = unsafe {C_ZNK9QSplitter19childrenCollapsibleEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  int QSplitter::count();
impl /*struct*/ QSplitter {
  pub fn count<RetType, T: QSplitter_count<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.count(self);
    // return 1;
  }
}

pub trait QSplitter_count<RetType> {
  fn count(self , rsthis: & QSplitter) -> RetType;
}

  // proto:  int QSplitter::count();
impl<'a> /*trait*/ QSplitter_count<i32> for () {
  fn count(self , rsthis: & QSplitter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QSplitter5countEv()};
    let mut ret = unsafe {C_ZNK9QSplitter5countEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  QByteArray QSplitter::saveState();
impl /*struct*/ QSplitter {
  pub fn saveState<RetType, T: QSplitter_saveState<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.saveState(self);
    // return 1;
  }
}

pub trait QSplitter_saveState<RetType> {
  fn saveState(self , rsthis: & QSplitter) -> RetType;
}

  // proto:  QByteArray QSplitter::saveState();
impl<'a> /*trait*/ QSplitter_saveState<QByteArray> for () {
  fn saveState(self , rsthis: & QSplitter) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QSplitter9saveStateEv()};
    let mut ret = unsafe {C_ZNK9QSplitter9saveStateEv(rsthis.qclsinst)};
    let mut ret1 = QByteArray::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  const QMetaObject * QSplitter::metaObject();
impl /*struct*/ QSplitter {
  pub fn metaObject<RetType, T: QSplitter_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QSplitter_metaObject<RetType> {
  fn metaObject(self , rsthis: & QSplitter) -> RetType;
}

  // proto:  const QMetaObject * QSplitter::metaObject();
impl<'a> /*trait*/ QSplitter_metaObject<QMetaObject> for () {
  fn metaObject(self , rsthis: & QSplitter) -> QMetaObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QSplitter10metaObjectEv()};
    let mut ret = unsafe {C_ZNK9QSplitter10metaObjectEv(rsthis.qclsinst)};
    let mut ret1 = QMetaObject::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QSplitter::opaqueResize();
impl /*struct*/ QSplitter {
  pub fn opaqueResize<RetType, T: QSplitter_opaqueResize<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.opaqueResize(self);
    // return 1;
  }
}

pub trait QSplitter_opaqueResize<RetType> {
  fn opaqueResize(self , rsthis: & QSplitter) -> RetType;
}

  // proto:  bool QSplitter::opaqueResize();
impl<'a> /*trait*/ QSplitter_opaqueResize<i8> for () {
  fn opaqueResize(self , rsthis: & QSplitter) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QSplitter12opaqueResizeEv()};
    let mut ret = unsafe {C_ZNK9QSplitter12opaqueResizeEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  void QSplitter::addWidget(QWidget * widget);
impl /*struct*/ QSplitter {
  pub fn addWidget<RetType, T: QSplitter_addWidget<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.addWidget(self);
    // return 1;
  }
}

pub trait QSplitter_addWidget<RetType> {
  fn addWidget(self , rsthis: & QSplitter) -> RetType;
}

  // proto:  void QSplitter::addWidget(QWidget * widget);
impl<'a> /*trait*/ QSplitter_addWidget<()> for (&'a QWidget) {
  fn addWidget(self , rsthis: & QSplitter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QSplitter9addWidgetEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN9QSplitter9addWidgetEP7QWidget(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QSplitter::QSplitter(QWidget * parent);
impl /*struct*/ QSplitter {
  pub fn new<T: QSplitter_new>(value: T) -> QSplitter {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QSplitter_new {
  fn new(self) -> QSplitter;
}

  // proto:  void QSplitter::QSplitter(QWidget * parent);
impl<'a> /*trait*/ QSplitter_new for (Option<&'a QWidget>) {
  fn new(self) -> QSplitter {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QSplitterC2EP7QWidget()};
    let ctysz: c_int = unsafe{QSplitter_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = (if self.is_none() {0} else {self.unwrap().qclsinst})  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN9QSplitterC2EP7QWidget(arg0)};
    let rsthis = QSplitter{qbase: QFrame::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QSplitter::setHandleWidth(int );
impl /*struct*/ QSplitter {
  pub fn setHandleWidth<RetType, T: QSplitter_setHandleWidth<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setHandleWidth(self);
    // return 1;
  }
}

pub trait QSplitter_setHandleWidth<RetType> {
  fn setHandleWidth(self , rsthis: & QSplitter) -> RetType;
}

  // proto:  void QSplitter::setHandleWidth(int );
impl<'a> /*trait*/ QSplitter_setHandleWidth<()> for (i32) {
  fn setHandleWidth(self , rsthis: & QSplitter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QSplitter14setHandleWidthEi()};
    let arg0 = self  as c_int;
     unsafe {C_ZN9QSplitter14setHandleWidthEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QSplitter::setStretchFactor(int index, int stretch);
impl /*struct*/ QSplitter {
  pub fn setStretchFactor<RetType, T: QSplitter_setStretchFactor<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setStretchFactor(self);
    // return 1;
  }
}

pub trait QSplitter_setStretchFactor<RetType> {
  fn setStretchFactor(self , rsthis: & QSplitter) -> RetType;
}

  // proto:  void QSplitter::setStretchFactor(int index, int stretch);
impl<'a> /*trait*/ QSplitter_setStretchFactor<()> for (i32, i32) {
  fn setStretchFactor(self , rsthis: & QSplitter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QSplitter16setStretchFactorEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {C_ZN9QSplitter16setStretchFactorEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  QSize QSplitter::minimumSizeHint();
impl /*struct*/ QSplitter {
  pub fn minimumSizeHint<RetType, T: QSplitter_minimumSizeHint<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.minimumSizeHint(self);
    // return 1;
  }
}

pub trait QSplitter_minimumSizeHint<RetType> {
  fn minimumSizeHint(self , rsthis: & QSplitter) -> RetType;
}

  // proto:  QSize QSplitter::minimumSizeHint();
impl<'a> /*trait*/ QSplitter_minimumSizeHint<QSize> for () {
  fn minimumSizeHint(self , rsthis: & QSplitter) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QSplitter15minimumSizeHintEv()};
    let mut ret = unsafe {C_ZNK9QSplitter15minimumSizeHintEv(rsthis.qclsinst)};
    let mut ret1 = QSize::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QSplitter::setOpaqueResize(bool opaque);
impl /*struct*/ QSplitter {
  pub fn setOpaqueResize<RetType, T: QSplitter_setOpaqueResize<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setOpaqueResize(self);
    // return 1;
  }
}

pub trait QSplitter_setOpaqueResize<RetType> {
  fn setOpaqueResize(self , rsthis: & QSplitter) -> RetType;
}

  // proto:  void QSplitter::setOpaqueResize(bool opaque);
impl<'a> /*trait*/ QSplitter_setOpaqueResize<()> for (Option<i8>) {
  fn setOpaqueResize(self , rsthis: & QSplitter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QSplitter15setOpaqueResizeEb()};
    let arg0 = (if self.is_none() {true as i8} else {self.unwrap()})  as c_char;
     unsafe {C_ZN9QSplitter15setOpaqueResizeEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QWidget * QSplitter::widget(int index);
impl /*struct*/ QSplitter {
  pub fn widget<RetType, T: QSplitter_widget<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.widget(self);
    // return 1;
  }
}

pub trait QSplitter_widget<RetType> {
  fn widget(self , rsthis: & QSplitter) -> RetType;
}

  // proto:  QWidget * QSplitter::widget(int index);
impl<'a> /*trait*/ QSplitter_widget<QWidget> for (i32) {
  fn widget(self , rsthis: & QSplitter) -> QWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QSplitter6widgetEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {C_ZNK9QSplitter6widgetEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QWidget::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QList<int> QSplitter::sizes();
impl /*struct*/ QSplitter {
  pub fn sizes<RetType, T: QSplitter_sizes<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.sizes(self);
    // return 1;
  }
}

pub trait QSplitter_sizes<RetType> {
  fn sizes(self , rsthis: & QSplitter) -> RetType;
}

  // proto:  QList<int> QSplitter::sizes();
impl<'a> /*trait*/ QSplitter_sizes<u64> for () {
  fn sizes(self , rsthis: & QSplitter) -> u64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QSplitter5sizesEv()};
    let mut ret = unsafe {C_ZNK9QSplitter5sizesEv(rsthis.qclsinst)};
    return ret as u64; // 5
    // return 1;
  }
}

  // proto:  bool QSplitter::isCollapsible(int index);
impl /*struct*/ QSplitter {
  pub fn isCollapsible<RetType, T: QSplitter_isCollapsible<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isCollapsible(self);
    // return 1;
  }
}

pub trait QSplitter_isCollapsible<RetType> {
  fn isCollapsible(self , rsthis: & QSplitter) -> RetType;
}

  // proto:  bool QSplitter::isCollapsible(int index);
impl<'a> /*trait*/ QSplitter_isCollapsible<i8> for (i32) {
  fn isCollapsible(self , rsthis: & QSplitter) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QSplitter13isCollapsibleEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {C_ZNK9QSplitter13isCollapsibleEi(rsthis.qclsinst, arg0)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  void QSplitter::setCollapsible(int index, bool );
impl /*struct*/ QSplitter {
  pub fn setCollapsible<RetType, T: QSplitter_setCollapsible<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setCollapsible(self);
    // return 1;
  }
}

pub trait QSplitter_setCollapsible<RetType> {
  fn setCollapsible(self , rsthis: & QSplitter) -> RetType;
}

  // proto:  void QSplitter::setCollapsible(int index, bool );
impl<'a> /*trait*/ QSplitter_setCollapsible<()> for (i32, i8) {
  fn setCollapsible(self , rsthis: & QSplitter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QSplitter14setCollapsibleEib()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_char;
     unsafe {C_ZN9QSplitter14setCollapsibleEib(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  bool QSplitter::restoreState(const QByteArray & state);
impl /*struct*/ QSplitter {
  pub fn restoreState<RetType, T: QSplitter_restoreState<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.restoreState(self);
    // return 1;
  }
}

pub trait QSplitter_restoreState<RetType> {
  fn restoreState(self , rsthis: & QSplitter) -> RetType;
}

  // proto:  bool QSplitter::restoreState(const QByteArray & state);
impl<'a> /*trait*/ QSplitter_restoreState<i8> for (&'a QByteArray) {
  fn restoreState(self , rsthis: & QSplitter) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QSplitter12restoreStateERK10QByteArray()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZN9QSplitter12restoreStateERK10QByteArray(rsthis.qclsinst, arg0)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  void QSplitter::~QSplitter();
impl /*struct*/ QSplitter {
  pub fn free<RetType, T: QSplitter_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QSplitter_free<RetType> {
  fn free(self , rsthis: & QSplitter) -> RetType;
}

  // proto:  void QSplitter::~QSplitter();
impl<'a> /*trait*/ QSplitter_free<()> for () {
  fn free(self , rsthis: & QSplitter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QSplitterD2Ev()};
     unsafe {C_ZN9QSplitterD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QSplitter::setChildrenCollapsible(bool );
impl /*struct*/ QSplitter {
  pub fn setChildrenCollapsible<RetType, T: QSplitter_setChildrenCollapsible<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setChildrenCollapsible(self);
    // return 1;
  }
}

pub trait QSplitter_setChildrenCollapsible<RetType> {
  fn setChildrenCollapsible(self , rsthis: & QSplitter) -> RetType;
}

  // proto:  void QSplitter::setChildrenCollapsible(bool );
impl<'a> /*trait*/ QSplitter_setChildrenCollapsible<()> for (i8) {
  fn setChildrenCollapsible(self , rsthis: & QSplitter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QSplitter22setChildrenCollapsibleEb()};
    let arg0 = self  as c_char;
     unsafe {C_ZN9QSplitter22setChildrenCollapsibleEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QSplitter::handleWidth();
impl /*struct*/ QSplitter {
  pub fn handleWidth<RetType, T: QSplitter_handleWidth<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.handleWidth(self);
    // return 1;
  }
}

pub trait QSplitter_handleWidth<RetType> {
  fn handleWidth(self , rsthis: & QSplitter) -> RetType;
}

  // proto:  int QSplitter::handleWidth();
impl<'a> /*trait*/ QSplitter_handleWidth<i32> for () {
  fn handleWidth(self , rsthis: & QSplitter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QSplitter11handleWidthEv()};
    let mut ret = unsafe {C_ZNK9QSplitter11handleWidthEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  void QSplitter::refresh();
impl /*struct*/ QSplitter {
  pub fn refresh<RetType, T: QSplitter_refresh<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.refresh(self);
    // return 1;
  }
}

pub trait QSplitter_refresh<RetType> {
  fn refresh(self , rsthis: & QSplitter) -> RetType;
}

  // proto:  void QSplitter::refresh();
impl<'a> /*trait*/ QSplitter_refresh<()> for () {
  fn refresh(self , rsthis: & QSplitter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QSplitter7refreshEv()};
     unsafe {C_ZN9QSplitter7refreshEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QSize QSplitter::sizeHint();
impl /*struct*/ QSplitter {
  pub fn sizeHint<RetType, T: QSplitter_sizeHint<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.sizeHint(self);
    // return 1;
  }
}

pub trait QSplitter_sizeHint<RetType> {
  fn sizeHint(self , rsthis: & QSplitter) -> RetType;
}

  // proto:  QSize QSplitter::sizeHint();
impl<'a> /*trait*/ QSplitter_sizeHint<QSize> for () {
  fn sizeHint(self , rsthis: & QSplitter) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QSplitter8sizeHintEv()};
    let mut ret = unsafe {C_ZNK9QSplitter8sizeHintEv(rsthis.qclsinst)};
    let mut ret1 = QSize::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  int QSplitter::indexOf(QWidget * w);
impl /*struct*/ QSplitter {
  pub fn indexOf<RetType, T: QSplitter_indexOf<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.indexOf(self);
    // return 1;
  }
}

pub trait QSplitter_indexOf<RetType> {
  fn indexOf(self , rsthis: & QSplitter) -> RetType;
}

  // proto:  int QSplitter::indexOf(QWidget * w);
impl<'a> /*trait*/ QSplitter_indexOf<i32> for (&'a QWidget) {
  fn indexOf(self , rsthis: & QSplitter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QSplitter7indexOfEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK9QSplitter7indexOfEP7QWidget(rsthis.qclsinst, arg0)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  void QSplitter::getRange(int index, int * , int * );
impl /*struct*/ QSplitter {
  pub fn getRange<RetType, T: QSplitter_getRange<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.getRange(self);
    // return 1;
  }
}

pub trait QSplitter_getRange<RetType> {
  fn getRange(self , rsthis: & QSplitter) -> RetType;
}

  // proto:  void QSplitter::getRange(int index, int * , int * );
impl<'a> /*trait*/ QSplitter_getRange<()> for (i32, &'a mut Vec<i32>, &'a mut Vec<i32>) {
  fn getRange(self , rsthis: & QSplitter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QSplitter8getRangeEiPiS0_()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.as_ptr()  as *mut c_int;
    let arg2 = self.2.as_ptr()  as *mut c_int;
     unsafe {C_ZNK9QSplitter8getRangeEiPiS0_(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  QSplitterHandle * QSplitter::handle(int index);
impl /*struct*/ QSplitter {
  pub fn handle<RetType, T: QSplitter_handle<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.handle(self);
    // return 1;
  }
}

pub trait QSplitter_handle<RetType> {
  fn handle(self , rsthis: & QSplitter) -> RetType;
}

  // proto:  QSplitterHandle * QSplitter::handle(int index);
impl<'a> /*trait*/ QSplitter_handle<QSplitterHandle> for (i32) {
  fn handle(self , rsthis: & QSplitter) -> QSplitterHandle {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QSplitter6handleEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {C_ZNK9QSplitter6handleEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QSplitterHandle::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QSplitterHandle {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QSplitterHandle {
    return QSplitterHandle{qbase: QWidget::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QSplitterHandle {
  type Target = QWidget;

  fn deref(&self) -> &QWidget {
    return & self.qbase;
  }
}
impl AsRef<QWidget> for QSplitterHandle {
  fn as_ref(& self) -> & QWidget {
    return & self.qbase;
  }
}
  // proto:  void QSplitterHandle::~QSplitterHandle();
impl /*struct*/ QSplitterHandle {
  pub fn free<RetType, T: QSplitterHandle_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QSplitterHandle_free<RetType> {
  fn free(self , rsthis: & QSplitterHandle) -> RetType;
}

  // proto:  void QSplitterHandle::~QSplitterHandle();
impl<'a> /*trait*/ QSplitterHandle_free<()> for () {
  fn free(self , rsthis: & QSplitterHandle) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QSplitterHandleD2Ev()};
     unsafe {C_ZN15QSplitterHandleD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QSize QSplitterHandle::sizeHint();
impl /*struct*/ QSplitterHandle {
  pub fn sizeHint<RetType, T: QSplitterHandle_sizeHint<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.sizeHint(self);
    // return 1;
  }
}

pub trait QSplitterHandle_sizeHint<RetType> {
  fn sizeHint(self , rsthis: & QSplitterHandle) -> RetType;
}

  // proto:  QSize QSplitterHandle::sizeHint();
impl<'a> /*trait*/ QSplitterHandle_sizeHint<QSize> for () {
  fn sizeHint(self , rsthis: & QSplitterHandle) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QSplitterHandle8sizeHintEv()};
    let mut ret = unsafe {C_ZNK15QSplitterHandle8sizeHintEv(rsthis.qclsinst)};
    let mut ret1 = QSize::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QSplitterHandle::opaqueResize();
impl /*struct*/ QSplitterHandle {
  pub fn opaqueResize<RetType, T: QSplitterHandle_opaqueResize<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.opaqueResize(self);
    // return 1;
  }
}

pub trait QSplitterHandle_opaqueResize<RetType> {
  fn opaqueResize(self , rsthis: & QSplitterHandle) -> RetType;
}

  // proto:  bool QSplitterHandle::opaqueResize();
impl<'a> /*trait*/ QSplitterHandle_opaqueResize<i8> for () {
  fn opaqueResize(self , rsthis: & QSplitterHandle) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QSplitterHandle12opaqueResizeEv()};
    let mut ret = unsafe {C_ZNK15QSplitterHandle12opaqueResizeEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  QSplitter * QSplitterHandle::splitter();
impl /*struct*/ QSplitterHandle {
  pub fn splitter<RetType, T: QSplitterHandle_splitter<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.splitter(self);
    // return 1;
  }
}

pub trait QSplitterHandle_splitter<RetType> {
  fn splitter(self , rsthis: & QSplitterHandle) -> RetType;
}

  // proto:  QSplitter * QSplitterHandle::splitter();
impl<'a> /*trait*/ QSplitterHandle_splitter<QSplitter> for () {
  fn splitter(self , rsthis: & QSplitterHandle) -> QSplitter {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QSplitterHandle8splitterEv()};
    let mut ret = unsafe {C_ZNK15QSplitterHandle8splitterEv(rsthis.qclsinst)};
    let mut ret1 = QSplitter::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  const QMetaObject * QSplitterHandle::metaObject();
impl /*struct*/ QSplitterHandle {
  pub fn metaObject<RetType, T: QSplitterHandle_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QSplitterHandle_metaObject<RetType> {
  fn metaObject(self , rsthis: & QSplitterHandle) -> RetType;
}

  // proto:  const QMetaObject * QSplitterHandle::metaObject();
impl<'a> /*trait*/ QSplitterHandle_metaObject<QMetaObject> for () {
  fn metaObject(self , rsthis: & QSplitterHandle) -> QMetaObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QSplitterHandle10metaObjectEv()};
    let mut ret = unsafe {C_ZNK15QSplitterHandle10metaObjectEv(rsthis.qclsinst)};
    let mut ret1 = QMetaObject::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

#[derive(Default)] // for QSplitter_splitterMoved
pub struct QSplitter_splitterMoved_signal{poi:u64}
impl /* struct */ QSplitter {
  pub fn splitterMoved(&self) -> QSplitter_splitterMoved_signal {
     return QSplitter_splitterMoved_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QSplitter_splitterMoved_signal {
  pub fn connect<T: QSplitter_splitterMoved_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QSplitter_splitterMoved_signal_connect {
  fn connect(self, sigthis: QSplitter_splitterMoved_signal);
}

// splitterMoved(int, int)
extern fn QSplitter_splitterMoved_signal_connect_cb_0(rsfptr:fn(i32, i32), arg0: c_int, arg1: c_int) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = arg0 as i32;
  let rsarg1 = arg1 as i32;
  rsfptr(rsarg0,rsarg1);
}
extern fn QSplitter_splitterMoved_signal_connect_cb_box_0(rsfptr_raw:*mut Box<Fn(i32, i32)>, arg0: c_int, arg1: c_int) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = arg0 as i32;
  let rsarg1 = arg1 as i32;
  // rsfptr(rsarg0,rsarg1);
  unsafe{(*rsfptr_raw)(rsarg0,rsarg1)};
}
impl /* trait */ QSplitter_splitterMoved_signal_connect for fn(i32, i32) {
  fn connect(self, sigthis: QSplitter_splitterMoved_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QSplitter_splitterMoved_signal_connect_cb_0 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QSplitter_SlotProxy_connect__ZN9QSplitter13splitterMovedEii(arg0, arg1, arg2)};
  }
}
impl /* trait */ QSplitter_splitterMoved_signal_connect for Box<Fn(i32, i32)> {
  fn connect(self, sigthis: QSplitter_splitterMoved_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QSplitter_splitterMoved_signal_connect_cb_box_0 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QSplitter_SlotProxy_connect__ZN9QSplitter13splitterMovedEii(arg0, arg1, arg2)};
  }
}
// <= body block end

