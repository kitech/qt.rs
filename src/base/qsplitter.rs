// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qwidget::QWidget;
use super::qbytearray::QByteArray;
use super::qsize::QSize;
use super::qsplitterhandle::QSplitterHandle;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  void QSplitter::insertWidget(int index, QWidget * widget);
  fn _ZN9QSplitter12insertWidgetEiP7QWidget(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void) ;
  // proto:  bool QSplitter::childrenCollapsible();
  fn _ZNK9QSplitter19childrenCollapsibleEv(qthis: *mut c_void) -> int8_t;
  // proto:  int QSplitter::count();
  fn _ZNK9QSplitter5countEv(qthis: *mut c_void) -> c_int;
  // proto:  void QSplitter::splitterMoved(int pos, int index);
  fn _ZN9QSplitter13splitterMovedEii(qthis: *mut c_void, arg0: c_int, arg1: c_int) ;
  // proto:  QByteArray QSplitter::saveState();
  fn _ZNK9QSplitter9saveStateEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  const QMetaObject * QSplitter::metaObject();
  fn _ZNK9QSplitter10metaObjectEv(qthis: *mut c_void) ;
  // proto:  bool QSplitter::opaqueResize();
  fn _ZNK9QSplitter12opaqueResizeEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QSplitter::addWidget(QWidget * widget);
  fn _ZN9QSplitter9addWidgetEP7QWidget(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QSplitter::NewQSplitter(const QSplitter & );
  fn _ZN9QSplitterC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QSplitter::NewQSplitter(QWidget * parent);
  fn _ZN9QSplitterC1EP7QWidget(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QSplitter::setHandleWidth(int );
  fn _ZN9QSplitter14setHandleWidthEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QSplitter::setStretchFactor(int index, int stretch);
  fn _ZN9QSplitter16setStretchFactorEii(qthis: *mut c_void, arg0: c_int, arg1: c_int) ;
  // proto:  QSize QSplitter::minimumSizeHint();
  fn _ZNK9QSplitter15minimumSizeHintEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QSplitter::setOpaqueResize(bool opaque);
  fn _ZN9QSplitter15setOpaqueResizeEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  QWidget * QSplitter::widget(int index);
  fn _ZNK9QSplitter6widgetEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  QList<int> QSplitter::sizes();
  fn _ZNK9QSplitter5sizesEv(qthis: *mut c_void) ;
  // proto:  bool QSplitter::isCollapsible(int index);
  fn _ZNK9QSplitter13isCollapsibleEi(qthis: *mut c_void, arg0: c_int) -> int8_t;
  // proto:  void QSplitter::setCollapsible(int index, bool );
  fn _ZN9QSplitter14setCollapsibleEib(qthis: *mut c_void, arg0: c_int, arg1: int8_t) ;
  // proto:  bool QSplitter::restoreState(const QByteArray & state);
  fn _ZN9QSplitter12restoreStateERK10QByteArray(qthis: *mut c_void, arg0: *mut c_void) -> int8_t;
  // proto:  void QSplitter::FreeQSplitter();
  fn _ZN9QSplitterD0Ev(qthis: *mut c_void) ;
  // proto:  void QSplitter::setChildrenCollapsible(bool );
  fn _ZN9QSplitter22setChildrenCollapsibleEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  int QSplitter::handleWidth();
  fn _ZNK9QSplitter11handleWidthEv(qthis: *mut c_void) -> c_int;
  // proto:  void QSplitter::refresh();
  fn _ZN9QSplitter7refreshEv(qthis: *mut c_void) ;
  // proto:  QSize QSplitter::sizeHint();
  fn _ZNK9QSplitter8sizeHintEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  int QSplitter::indexOf(QWidget * w);
  fn _ZNK9QSplitter7indexOfEP7QWidget(qthis: *mut c_void, arg0: *mut c_void) -> c_int;
  // proto:  void QSplitter::getRange(int index, int * , int * );
  fn _ZNK9QSplitter8getRangeEiPiS0_(qthis: *mut c_void, arg0: c_int, arg1: *mut c_int, arg2: *mut c_int) ;
  // proto:  QSplitterHandle * QSplitter::handle(int index);
  fn _ZNK9QSplitter6handleEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
}

// body block begin
// class sizeof(QSplitter)=1
pub struct QSplitter {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QSplitter {
  pub fn insertWidget<RetType, T: QSplitter_insertWidget<RetType>>(&mut self, value: T) -> RetType {
    return value.insertWidget(self);
    // return 1;
  }
}

pub trait QSplitter_insertWidget<RetType> {
  fn insertWidget(self, rsthis: &mut QSplitter) -> RetType;
}

// proto:  void QSplitter::insertWidget(int index, QWidget * widget);
impl<'a> /*trait*/ QSplitter_insertWidget<()> for (i32, &'a mut QWidget) {
  fn insertWidget(self, rsthis: &mut QSplitter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QSplitter12insertWidgetEiP7QWidget()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN9QSplitter12insertWidgetEiP7QWidget(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QSplitter {
  pub fn childrenCollapsible<RetType, T: QSplitter_childrenCollapsible<RetType>>(&mut self, value: T) -> RetType {
    return value.childrenCollapsible(self);
    // return 1;
  }
}

pub trait QSplitter_childrenCollapsible<RetType> {
  fn childrenCollapsible(self, rsthis: &mut QSplitter) -> RetType;
}

// proto:  bool QSplitter::childrenCollapsible();
impl<'a> /*trait*/ QSplitter_childrenCollapsible<i8> for () {
  fn childrenCollapsible(self, rsthis: &mut QSplitter) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QSplitter19childrenCollapsibleEv()};
    let mut ret = unsafe {_ZNK9QSplitter19childrenCollapsibleEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QSplitter {
  pub fn count<RetType, T: QSplitter_count<RetType>>(&mut self, value: T) -> RetType {
    return value.count(self);
    // return 1;
  }
}

pub trait QSplitter_count<RetType> {
  fn count(self, rsthis: &mut QSplitter) -> RetType;
}

// proto:  int QSplitter::count();
impl<'a> /*trait*/ QSplitter_count<i32> for () {
  fn count(self, rsthis: &mut QSplitter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QSplitter5countEv()};
    let mut ret = unsafe {_ZNK9QSplitter5countEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QSplitter {
  pub fn splitterMoved<RetType, T: QSplitter_splitterMoved<RetType>>(&mut self, value: T) -> RetType {
    return value.splitterMoved(self);
    // return 1;
  }
}

pub trait QSplitter_splitterMoved<RetType> {
  fn splitterMoved(self, rsthis: &mut QSplitter) -> RetType;
}

// proto:  void QSplitter::splitterMoved(int pos, int index);
impl<'a> /*trait*/ QSplitter_splitterMoved<()> for (i32, i32) {
  fn splitterMoved(self, rsthis: &mut QSplitter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QSplitter13splitterMovedEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {_ZN9QSplitter13splitterMovedEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QSplitter {
  pub fn saveState<RetType, T: QSplitter_saveState<RetType>>(&mut self, value: T) -> RetType {
    return value.saveState(self);
    // return 1;
  }
}

pub trait QSplitter_saveState<RetType> {
  fn saveState(self, rsthis: &mut QSplitter) -> RetType;
}

// proto:  QByteArray QSplitter::saveState();
impl<'a> /*trait*/ QSplitter_saveState<QByteArray> for () {
  fn saveState(self, rsthis: &mut QSplitter) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QSplitter9saveStateEv()};
    let mut ret = unsafe {_ZNK9QSplitter9saveStateEv(rsthis.qclsinst)};
    let mut ret1 = QByteArray{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QSplitter {
  pub fn metaObject<RetType, T: QSplitter_metaObject<RetType>>(&mut self, value: T) -> RetType {
    return value.metaObject(self);
    // return 1;
  }
}

pub trait QSplitter_metaObject<RetType> {
  fn metaObject(self, rsthis: &mut QSplitter) -> RetType;
}

// proto:  const QMetaObject * QSplitter::metaObject();
impl<'a> /*trait*/ QSplitter_metaObject<()> for () {
  fn metaObject(self, rsthis: &mut QSplitter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QSplitter10metaObjectEv()};
     unsafe {_ZNK9QSplitter10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QSplitter {
  pub fn opaqueResize<RetType, T: QSplitter_opaqueResize<RetType>>(&mut self, value: T) -> RetType {
    return value.opaqueResize(self);
    // return 1;
  }
}

pub trait QSplitter_opaqueResize<RetType> {
  fn opaqueResize(self, rsthis: &mut QSplitter) -> RetType;
}

// proto:  bool QSplitter::opaqueResize();
impl<'a> /*trait*/ QSplitter_opaqueResize<i8> for () {
  fn opaqueResize(self, rsthis: &mut QSplitter) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QSplitter12opaqueResizeEv()};
    let mut ret = unsafe {_ZNK9QSplitter12opaqueResizeEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QSplitter {
  pub fn addWidget<RetType, T: QSplitter_addWidget<RetType>>(&mut self, value: T) -> RetType {
    return value.addWidget(self);
    // return 1;
  }
}

pub trait QSplitter_addWidget<RetType> {
  fn addWidget(self, rsthis: &mut QSplitter) -> RetType;
}

// proto:  void QSplitter::addWidget(QWidget * widget);
impl<'a> /*trait*/ QSplitter_addWidget<()> for (&'a mut QWidget) {
  fn addWidget(self, rsthis: &mut QSplitter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QSplitter9addWidgetEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QSplitter9addWidgetEP7QWidget(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QSplitter {
  pub fn NewQSplitter<T: QSplitter_NewQSplitter>(value: T) -> QSplitter {
    let rsthis = value.NewQSplitter();
    return rsthis;
    // return 1;
  }
}

pub trait QSplitter_NewQSplitter {
  fn NewQSplitter(self) -> QSplitter;
}

// proto: void QSplitter::NewQSplitter(const QSplitter & );
impl<'a> /*trait*/ QSplitter_NewQSplitter for (&'a  QSplitter) {
  fn NewQSplitter(self) -> QSplitter {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QSplitterC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN9QSplitterC1ERKS_(qthis, arg0)};
    let rsthis = QSplitter{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: void QSplitter::NewQSplitter(QWidget * parent);
impl<'a> /*trait*/ QSplitter_NewQSplitter for (&'a mut QWidget) {
  fn NewQSplitter(self) -> QSplitter {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QSplitterC1EP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN9QSplitterC1EP7QWidget(qthis, arg0)};
    let rsthis = QSplitter{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QSplitter {
  pub fn setHandleWidth<RetType, T: QSplitter_setHandleWidth<RetType>>(&mut self, value: T) -> RetType {
    return value.setHandleWidth(self);
    // return 1;
  }
}

pub trait QSplitter_setHandleWidth<RetType> {
  fn setHandleWidth(self, rsthis: &mut QSplitter) -> RetType;
}

// proto:  void QSplitter::setHandleWidth(int );
impl<'a> /*trait*/ QSplitter_setHandleWidth<()> for (i32) {
  fn setHandleWidth(self, rsthis: &mut QSplitter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QSplitter14setHandleWidthEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN9QSplitter14setHandleWidthEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QSplitter {
  pub fn setStretchFactor<RetType, T: QSplitter_setStretchFactor<RetType>>(&mut self, value: T) -> RetType {
    return value.setStretchFactor(self);
    // return 1;
  }
}

pub trait QSplitter_setStretchFactor<RetType> {
  fn setStretchFactor(self, rsthis: &mut QSplitter) -> RetType;
}

// proto:  void QSplitter::setStretchFactor(int index, int stretch);
impl<'a> /*trait*/ QSplitter_setStretchFactor<()> for (i32, i32) {
  fn setStretchFactor(self, rsthis: &mut QSplitter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QSplitter16setStretchFactorEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {_ZN9QSplitter16setStretchFactorEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QSplitter {
  pub fn minimumSizeHint<RetType, T: QSplitter_minimumSizeHint<RetType>>(&mut self, value: T) -> RetType {
    return value.minimumSizeHint(self);
    // return 1;
  }
}

pub trait QSplitter_minimumSizeHint<RetType> {
  fn minimumSizeHint(self, rsthis: &mut QSplitter) -> RetType;
}

// proto:  QSize QSplitter::minimumSizeHint();
impl<'a> /*trait*/ QSplitter_minimumSizeHint<QSize> for () {
  fn minimumSizeHint(self, rsthis: &mut QSplitter) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QSplitter15minimumSizeHintEv()};
    let mut ret = unsafe {_ZNK9QSplitter15minimumSizeHintEv(rsthis.qclsinst)};
    let mut ret1 = QSize{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QSplitter {
  pub fn setOpaqueResize<RetType, T: QSplitter_setOpaqueResize<RetType>>(&mut self, value: T) -> RetType {
    return value.setOpaqueResize(self);
    // return 1;
  }
}

pub trait QSplitter_setOpaqueResize<RetType> {
  fn setOpaqueResize(self, rsthis: &mut QSplitter) -> RetType;
}

// proto:  void QSplitter::setOpaqueResize(bool opaque);
impl<'a> /*trait*/ QSplitter_setOpaqueResize<()> for (i8) {
  fn setOpaqueResize(self, rsthis: &mut QSplitter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QSplitter15setOpaqueResizeEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN9QSplitter15setOpaqueResizeEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QSplitter {
  pub fn widget<RetType, T: QSplitter_widget<RetType>>(&mut self, value: T) -> RetType {
    return value.widget(self);
    // return 1;
  }
}

pub trait QSplitter_widget<RetType> {
  fn widget(self, rsthis: &mut QSplitter) -> RetType;
}

// proto:  QWidget * QSplitter::widget(int index);
impl<'a> /*trait*/ QSplitter_widget<QWidget> for (i32) {
  fn widget(self, rsthis: &mut QSplitter) -> QWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QSplitter6widgetEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK9QSplitter6widgetEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QWidget{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QSplitter {
  pub fn sizes<RetType, T: QSplitter_sizes<RetType>>(&mut self, value: T) -> RetType {
    return value.sizes(self);
    // return 1;
  }
}

pub trait QSplitter_sizes<RetType> {
  fn sizes(self, rsthis: &mut QSplitter) -> RetType;
}

// proto:  QList<int> QSplitter::sizes();
impl<'a> /*trait*/ QSplitter_sizes<()> for () {
  fn sizes(self, rsthis: &mut QSplitter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QSplitter5sizesEv()};
     unsafe {_ZNK9QSplitter5sizesEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QSplitter {
  pub fn isCollapsible<RetType, T: QSplitter_isCollapsible<RetType>>(&mut self, value: T) -> RetType {
    return value.isCollapsible(self);
    // return 1;
  }
}

pub trait QSplitter_isCollapsible<RetType> {
  fn isCollapsible(self, rsthis: &mut QSplitter) -> RetType;
}

// proto:  bool QSplitter::isCollapsible(int index);
impl<'a> /*trait*/ QSplitter_isCollapsible<i8> for (i32) {
  fn isCollapsible(self, rsthis: &mut QSplitter) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QSplitter13isCollapsibleEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK9QSplitter13isCollapsibleEi(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QSplitter {
  pub fn setCollapsible<RetType, T: QSplitter_setCollapsible<RetType>>(&mut self, value: T) -> RetType {
    return value.setCollapsible(self);
    // return 1;
  }
}

pub trait QSplitter_setCollapsible<RetType> {
  fn setCollapsible(self, rsthis: &mut QSplitter) -> RetType;
}

// proto:  void QSplitter::setCollapsible(int index, bool );
impl<'a> /*trait*/ QSplitter_setCollapsible<()> for (i32, i8) {
  fn setCollapsible(self, rsthis: &mut QSplitter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QSplitter14setCollapsibleEib()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as int8_t;
     unsafe {_ZN9QSplitter14setCollapsibleEib(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QSplitter {
  pub fn restoreState<RetType, T: QSplitter_restoreState<RetType>>(&mut self, value: T) -> RetType {
    return value.restoreState(self);
    // return 1;
  }
}

pub trait QSplitter_restoreState<RetType> {
  fn restoreState(self, rsthis: &mut QSplitter) -> RetType;
}

// proto:  bool QSplitter::restoreState(const QByteArray & state);
impl<'a> /*trait*/ QSplitter_restoreState<i8> for (&'a  QByteArray) {
  fn restoreState(self, rsthis: &mut QSplitter) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QSplitter12restoreStateERK10QByteArray()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN9QSplitter12restoreStateERK10QByteArray(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QSplitter {
  pub fn FreeQSplitter<RetType, T: QSplitter_FreeQSplitter<RetType>>(&mut self, value: T) -> RetType {
    return value.FreeQSplitter(self);
    // return 1;
  }
}

pub trait QSplitter_FreeQSplitter<RetType> {
  fn FreeQSplitter(self, rsthis: &mut QSplitter) -> RetType;
}

// proto:  void QSplitter::FreeQSplitter();
impl<'a> /*trait*/ QSplitter_FreeQSplitter<()> for () {
  fn FreeQSplitter(self, rsthis: &mut QSplitter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QSplitterD0Ev()};
     unsafe {_ZN9QSplitterD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QSplitter {
  pub fn setChildrenCollapsible<RetType, T: QSplitter_setChildrenCollapsible<RetType>>(&mut self, value: T) -> RetType {
    return value.setChildrenCollapsible(self);
    // return 1;
  }
}

pub trait QSplitter_setChildrenCollapsible<RetType> {
  fn setChildrenCollapsible(self, rsthis: &mut QSplitter) -> RetType;
}

// proto:  void QSplitter::setChildrenCollapsible(bool );
impl<'a> /*trait*/ QSplitter_setChildrenCollapsible<()> for (i8) {
  fn setChildrenCollapsible(self, rsthis: &mut QSplitter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QSplitter22setChildrenCollapsibleEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN9QSplitter22setChildrenCollapsibleEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QSplitter {
  pub fn handleWidth<RetType, T: QSplitter_handleWidth<RetType>>(&mut self, value: T) -> RetType {
    return value.handleWidth(self);
    // return 1;
  }
}

pub trait QSplitter_handleWidth<RetType> {
  fn handleWidth(self, rsthis: &mut QSplitter) -> RetType;
}

// proto:  int QSplitter::handleWidth();
impl<'a> /*trait*/ QSplitter_handleWidth<i32> for () {
  fn handleWidth(self, rsthis: &mut QSplitter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QSplitter11handleWidthEv()};
    let mut ret = unsafe {_ZNK9QSplitter11handleWidthEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QSplitter {
  pub fn refresh<RetType, T: QSplitter_refresh<RetType>>(&mut self, value: T) -> RetType {
    return value.refresh(self);
    // return 1;
  }
}

pub trait QSplitter_refresh<RetType> {
  fn refresh(self, rsthis: &mut QSplitter) -> RetType;
}

// proto:  void QSplitter::refresh();
impl<'a> /*trait*/ QSplitter_refresh<()> for () {
  fn refresh(self, rsthis: &mut QSplitter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QSplitter7refreshEv()};
     unsafe {_ZN9QSplitter7refreshEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QSplitter {
  pub fn sizeHint<RetType, T: QSplitter_sizeHint<RetType>>(&mut self, value: T) -> RetType {
    return value.sizeHint(self);
    // return 1;
  }
}

pub trait QSplitter_sizeHint<RetType> {
  fn sizeHint(self, rsthis: &mut QSplitter) -> RetType;
}

// proto:  QSize QSplitter::sizeHint();
impl<'a> /*trait*/ QSplitter_sizeHint<QSize> for () {
  fn sizeHint(self, rsthis: &mut QSplitter) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QSplitter8sizeHintEv()};
    let mut ret = unsafe {_ZNK9QSplitter8sizeHintEv(rsthis.qclsinst)};
    let mut ret1 = QSize{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QSplitter {
  pub fn indexOf<RetType, T: QSplitter_indexOf<RetType>>(&mut self, value: T) -> RetType {
    return value.indexOf(self);
    // return 1;
  }
}

pub trait QSplitter_indexOf<RetType> {
  fn indexOf(self, rsthis: &mut QSplitter) -> RetType;
}

// proto:  int QSplitter::indexOf(QWidget * w);
impl<'a> /*trait*/ QSplitter_indexOf<i32> for (&'a mut QWidget) {
  fn indexOf(self, rsthis: &mut QSplitter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QSplitter7indexOfEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK9QSplitter7indexOfEP7QWidget(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QSplitter {
  pub fn getRange<RetType, T: QSplitter_getRange<RetType>>(&mut self, value: T) -> RetType {
    return value.getRange(self);
    // return 1;
  }
}

pub trait QSplitter_getRange<RetType> {
  fn getRange(self, rsthis: &mut QSplitter) -> RetType;
}

// proto:  void QSplitter::getRange(int index, int * , int * );
impl<'a> /*trait*/ QSplitter_getRange<()> for (i32, &'a mut i32, &'a mut i32) {
  fn getRange(self, rsthis: &mut QSplitter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QSplitter8getRangeEiPiS0_()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as *mut c_int;
    let arg2 = self.2  as *mut c_int;
     unsafe {_ZNK9QSplitter8getRangeEiPiS0_(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

impl /*struct*/ QSplitter {
  pub fn handle<RetType, T: QSplitter_handle<RetType>>(&mut self, value: T) -> RetType {
    return value.handle(self);
    // return 1;
  }
}

pub trait QSplitter_handle<RetType> {
  fn handle(self, rsthis: &mut QSplitter) -> RetType;
}

// proto:  QSplitterHandle * QSplitter::handle(int index);
impl<'a> /*trait*/ QSplitter_handle<QSplitterHandle> for (i32) {
  fn handle(self, rsthis: &mut QSplitter) -> QSplitterHandle {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QSplitter6handleEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK9QSplitter6handleEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QSplitterHandle{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

