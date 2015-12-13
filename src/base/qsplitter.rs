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

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: void QSplitter::insertWidget(int index, QWidget * widget);
  fn _ZN9QSplitter12insertWidgetEiP7QWidget(arg0: c_int, arg1: *mut c_void) -> i32;
  // proto: bool QSplitter::childrenCollapsible();
  fn _ZNK9QSplitter19childrenCollapsibleEv() -> i32;
  // proto: int QSplitter::count();
  fn _ZNK9QSplitter5countEv() -> i32;
  // proto: void QSplitter::splitterMoved(int pos, int index);
  fn _ZN9QSplitter13splitterMovedEii(arg0: c_int, arg1: c_int) -> i32;
  // proto: QByteArray QSplitter::saveState();
  fn _ZNK9QSplitter9saveStateEv() -> i32;
  // proto: const QMetaObject * QSplitter::metaObject();
  fn _ZNK9QSplitter10metaObjectEv() -> i32;
  // proto: bool QSplitter::opaqueResize();
  fn _ZNK9QSplitter12opaqueResizeEv() -> i32;
  // proto: void QSplitter::addWidget(QWidget * widget);
  fn _ZN9QSplitter9addWidgetEP7QWidget(arg0: *mut c_void) -> i32;
  // proto: void QSplitter::NewQSplitter(const QSplitter & );
  fn _ZN9QSplitterC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: void QSplitter::NewQSplitter(QWidget * parent);
  fn _ZN9QSplitterC1EP7QWidget(qthis: *mut c_void, arg0: *mut c_void) -> i32;
  // proto: void QSplitter::setHandleWidth(int );
  fn _ZN9QSplitter14setHandleWidthEi(arg0: c_int) -> i32;
  // proto: void QSplitter::setStretchFactor(int index, int stretch);
  fn _ZN9QSplitter16setStretchFactorEii(arg0: c_int, arg1: c_int) -> i32;
  // proto: QSize QSplitter::minimumSizeHint();
  fn _ZNK9QSplitter15minimumSizeHintEv() -> i32;
  // proto: void QSplitter::setOpaqueResize(bool opaque);
  fn _ZN9QSplitter15setOpaqueResizeEb(arg0: int8_t) -> i32;
  // proto: QWidget * QSplitter::widget(int index);
  fn _ZNK9QSplitter6widgetEi(arg0: c_int) -> i32;
  // proto: QList<int> QSplitter::sizes();
  fn _ZNK9QSplitter5sizesEv() -> i32;
  // proto: bool QSplitter::isCollapsible(int index);
  fn _ZNK9QSplitter13isCollapsibleEi(arg0: c_int) -> i32;
  // proto: void QSplitter::setCollapsible(int index, bool );
  fn _ZN9QSplitter14setCollapsibleEib(arg0: c_int, arg1: int8_t) -> i32;
  // proto: bool QSplitter::restoreState(const QByteArray & state);
  fn _ZN9QSplitter12restoreStateERK10QByteArray(arg0: *const c_void) -> i32;
  // proto: void QSplitter::FreeQSplitter();
  fn _ZN9QSplitterD0Ev() -> i32;
  // proto: void QSplitter::setChildrenCollapsible(bool );
  fn _ZN9QSplitter22setChildrenCollapsibleEb(arg0: int8_t) -> i32;
  // proto: int QSplitter::handleWidth();
  fn _ZNK9QSplitter11handleWidthEv() -> i32;
  // proto: void QSplitter::refresh();
  fn _ZN9QSplitter7refreshEv() -> i32;
  // proto: QSize QSplitter::sizeHint();
  fn _ZNK9QSplitter8sizeHintEv() -> i32;
  // proto: int QSplitter::indexOf(QWidget * w);
  fn _ZNK9QSplitter7indexOfEP7QWidget(arg0: *mut c_void) -> i32;
  // proto: void QSplitter::getRange(int index, int * , int * );
  fn _ZNK9QSplitter8getRangeEiPiS0_(arg0: c_int, arg1: *mut c_int, arg2: *mut c_int) -> i32;
  // proto: QSplitterHandle * QSplitter::handle(int index);
  fn _ZNK9QSplitter6handleEi(arg0: c_int) -> i32;
}

// body block begin
// class sizeof(QSplitter)=1
pub struct QSplitter {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QSplitter {
  pub fn insertWidget<T: QSplitter_insertWidget>(&mut self, value: T) -> i32 {
    value.insertWidget(self);
    return 1;
  }
}

pub trait QSplitter_insertWidget {
  fn insertWidget(self, this: &mut QSplitter) -> i32;
}

// proto: void QSplitter::insertWidget(int index, QWidget * widget);
impl<'a> /*trait*/ QSplitter_insertWidget for (i32, &'a mut QWidget) {
  fn insertWidget(self, this: &mut QSplitter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QSplitter12insertWidgetEiP7QWidget()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN9QSplitter12insertWidgetEiP7QWidget(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QSplitter {
  pub fn childrenCollapsible<T: QSplitter_childrenCollapsible>(&mut self, value: T) -> i32 {
    value.childrenCollapsible(self);
    return 1;
  }
}

pub trait QSplitter_childrenCollapsible {
  fn childrenCollapsible(self, this: &mut QSplitter) -> i32;
}

// proto: bool QSplitter::childrenCollapsible();
impl<'a> /*trait*/ QSplitter_childrenCollapsible for () {
  fn childrenCollapsible(self, this: &mut QSplitter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QSplitter19childrenCollapsibleEv()};
    unsafe {_ZNK9QSplitter19childrenCollapsibleEv()};
    return 1;
  }
}

impl /*struct*/ QSplitter {
  pub fn count<T: QSplitter_count>(&mut self, value: T) -> i32 {
    value.count(self);
    return 1;
  }
}

pub trait QSplitter_count {
  fn count(self, this: &mut QSplitter) -> i32;
}

// proto: int QSplitter::count();
impl<'a> /*trait*/ QSplitter_count for () {
  fn count(self, this: &mut QSplitter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QSplitter5countEv()};
    unsafe {_ZNK9QSplitter5countEv()};
    return 1;
  }
}

impl /*struct*/ QSplitter {
  pub fn splitterMoved<T: QSplitter_splitterMoved>(&mut self, value: T) -> i32 {
    value.splitterMoved(self);
    return 1;
  }
}

pub trait QSplitter_splitterMoved {
  fn splitterMoved(self, this: &mut QSplitter) -> i32;
}

// proto: void QSplitter::splitterMoved(int pos, int index);
impl<'a> /*trait*/ QSplitter_splitterMoved for (i32, i32) {
  fn splitterMoved(self, this: &mut QSplitter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QSplitter13splitterMovedEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    unsafe {_ZN9QSplitter13splitterMovedEii(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QSplitter {
  pub fn saveState<T: QSplitter_saveState>(&mut self, value: T) -> i32 {
    value.saveState(self);
    return 1;
  }
}

pub trait QSplitter_saveState {
  fn saveState(self, this: &mut QSplitter) -> i32;
}

// proto: QByteArray QSplitter::saveState();
impl<'a> /*trait*/ QSplitter_saveState for () {
  fn saveState(self, this: &mut QSplitter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QSplitter9saveStateEv()};
    unsafe {_ZNK9QSplitter9saveStateEv()};
    return 1;
  }
}

impl /*struct*/ QSplitter {
  pub fn metaObject<T: QSplitter_metaObject>(&mut self, value: T) -> i32 {
    value.metaObject(self);
    return 1;
  }
}

pub trait QSplitter_metaObject {
  fn metaObject(self, this: &mut QSplitter) -> i32;
}

// proto: const QMetaObject * QSplitter::metaObject();
impl<'a> /*trait*/ QSplitter_metaObject for () {
  fn metaObject(self, this: &mut QSplitter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QSplitter10metaObjectEv()};
    unsafe {_ZNK9QSplitter10metaObjectEv()};
    return 1;
  }
}

impl /*struct*/ QSplitter {
  pub fn opaqueResize<T: QSplitter_opaqueResize>(&mut self, value: T) -> i32 {
    value.opaqueResize(self);
    return 1;
  }
}

pub trait QSplitter_opaqueResize {
  fn opaqueResize(self, this: &mut QSplitter) -> i32;
}

// proto: bool QSplitter::opaqueResize();
impl<'a> /*trait*/ QSplitter_opaqueResize for () {
  fn opaqueResize(self, this: &mut QSplitter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QSplitter12opaqueResizeEv()};
    unsafe {_ZNK9QSplitter12opaqueResizeEv()};
    return 1;
  }
}

impl /*struct*/ QSplitter {
  pub fn addWidget<T: QSplitter_addWidget>(&mut self, value: T) -> i32 {
    value.addWidget(self);
    return 1;
  }
}

pub trait QSplitter_addWidget {
  fn addWidget(self, this: &mut QSplitter) -> i32;
}

// proto: void QSplitter::addWidget(QWidget * widget);
impl<'a> /*trait*/ QSplitter_addWidget for (&'a mut QWidget) {
  fn addWidget(self, this: &mut QSplitter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QSplitter9addWidgetEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN9QSplitter9addWidgetEP7QWidget(arg0)};
    return 1;
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
    let arg0 = self.qclsinst  as *const c_void;
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
  pub fn setHandleWidth<T: QSplitter_setHandleWidth>(&mut self, value: T) -> i32 {
    value.setHandleWidth(self);
    return 1;
  }
}

pub trait QSplitter_setHandleWidth {
  fn setHandleWidth(self, this: &mut QSplitter) -> i32;
}

// proto: void QSplitter::setHandleWidth(int );
impl<'a> /*trait*/ QSplitter_setHandleWidth for (i32) {
  fn setHandleWidth(self, this: &mut QSplitter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QSplitter14setHandleWidthEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN9QSplitter14setHandleWidthEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QSplitter {
  pub fn setStretchFactor<T: QSplitter_setStretchFactor>(&mut self, value: T) -> i32 {
    value.setStretchFactor(self);
    return 1;
  }
}

pub trait QSplitter_setStretchFactor {
  fn setStretchFactor(self, this: &mut QSplitter) -> i32;
}

// proto: void QSplitter::setStretchFactor(int index, int stretch);
impl<'a> /*trait*/ QSplitter_setStretchFactor for (i32, i32) {
  fn setStretchFactor(self, this: &mut QSplitter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QSplitter16setStretchFactorEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    unsafe {_ZN9QSplitter16setStretchFactorEii(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QSplitter {
  pub fn minimumSizeHint<T: QSplitter_minimumSizeHint>(&mut self, value: T) -> i32 {
    value.minimumSizeHint(self);
    return 1;
  }
}

pub trait QSplitter_minimumSizeHint {
  fn minimumSizeHint(self, this: &mut QSplitter) -> i32;
}

// proto: QSize QSplitter::minimumSizeHint();
impl<'a> /*trait*/ QSplitter_minimumSizeHint for () {
  fn minimumSizeHint(self, this: &mut QSplitter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QSplitter15minimumSizeHintEv()};
    unsafe {_ZNK9QSplitter15minimumSizeHintEv()};
    return 1;
  }
}

impl /*struct*/ QSplitter {
  pub fn setOpaqueResize<T: QSplitter_setOpaqueResize>(&mut self, value: T) -> i32 {
    value.setOpaqueResize(self);
    return 1;
  }
}

pub trait QSplitter_setOpaqueResize {
  fn setOpaqueResize(self, this: &mut QSplitter) -> i32;
}

// proto: void QSplitter::setOpaqueResize(bool opaque);
impl<'a> /*trait*/ QSplitter_setOpaqueResize for (i8) {
  fn setOpaqueResize(self, this: &mut QSplitter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QSplitter15setOpaqueResizeEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN9QSplitter15setOpaqueResizeEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QSplitter {
  pub fn widget<T: QSplitter_widget>(&mut self, value: T) -> i32 {
    value.widget(self);
    return 1;
  }
}

pub trait QSplitter_widget {
  fn widget(self, this: &mut QSplitter) -> i32;
}

// proto: QWidget * QSplitter::widget(int index);
impl<'a> /*trait*/ QSplitter_widget for (i32) {
  fn widget(self, this: &mut QSplitter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QSplitter6widgetEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK9QSplitter6widgetEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QSplitter {
  pub fn sizes<T: QSplitter_sizes>(&mut self, value: T) -> i32 {
    value.sizes(self);
    return 1;
  }
}

pub trait QSplitter_sizes {
  fn sizes(self, this: &mut QSplitter) -> i32;
}

// proto: QList<int> QSplitter::sizes();
impl<'a> /*trait*/ QSplitter_sizes for () {
  fn sizes(self, this: &mut QSplitter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QSplitter5sizesEv()};
    unsafe {_ZNK9QSplitter5sizesEv()};
    return 1;
  }
}

impl /*struct*/ QSplitter {
  pub fn isCollapsible<T: QSplitter_isCollapsible>(&mut self, value: T) -> i32 {
    value.isCollapsible(self);
    return 1;
  }
}

pub trait QSplitter_isCollapsible {
  fn isCollapsible(self, this: &mut QSplitter) -> i32;
}

// proto: bool QSplitter::isCollapsible(int index);
impl<'a> /*trait*/ QSplitter_isCollapsible for (i32) {
  fn isCollapsible(self, this: &mut QSplitter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QSplitter13isCollapsibleEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK9QSplitter13isCollapsibleEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QSplitter {
  pub fn setCollapsible<T: QSplitter_setCollapsible>(&mut self, value: T) -> i32 {
    value.setCollapsible(self);
    return 1;
  }
}

pub trait QSplitter_setCollapsible {
  fn setCollapsible(self, this: &mut QSplitter) -> i32;
}

// proto: void QSplitter::setCollapsible(int index, bool );
impl<'a> /*trait*/ QSplitter_setCollapsible for (i32, i8) {
  fn setCollapsible(self, this: &mut QSplitter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QSplitter14setCollapsibleEib()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as int8_t;
    unsafe {_ZN9QSplitter14setCollapsibleEib(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QSplitter {
  pub fn restoreState<T: QSplitter_restoreState>(&mut self, value: T) -> i32 {
    value.restoreState(self);
    return 1;
  }
}

pub trait QSplitter_restoreState {
  fn restoreState(self, this: &mut QSplitter) -> i32;
}

// proto: bool QSplitter::restoreState(const QByteArray & state);
impl<'a> /*trait*/ QSplitter_restoreState for (&'a  QByteArray) {
  fn restoreState(self, this: &mut QSplitter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QSplitter12restoreStateERK10QByteArray()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN9QSplitter12restoreStateERK10QByteArray(arg0)};
    return 1;
  }
}

impl /*struct*/ QSplitter {
  pub fn FreeQSplitter<T: QSplitter_FreeQSplitter>(&mut self, value: T) -> i32 {
    value.FreeQSplitter(self);
    return 1;
  }
}

pub trait QSplitter_FreeQSplitter {
  fn FreeQSplitter(self, this: &mut QSplitter) -> i32;
}

// proto: void QSplitter::FreeQSplitter();
impl<'a> /*trait*/ QSplitter_FreeQSplitter for () {
  fn FreeQSplitter(self, this: &mut QSplitter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QSplitterD0Ev()};
    unsafe {_ZN9QSplitterD0Ev()};
    return 1;
  }
}

impl /*struct*/ QSplitter {
  pub fn setChildrenCollapsible<T: QSplitter_setChildrenCollapsible>(&mut self, value: T) -> i32 {
    value.setChildrenCollapsible(self);
    return 1;
  }
}

pub trait QSplitter_setChildrenCollapsible {
  fn setChildrenCollapsible(self, this: &mut QSplitter) -> i32;
}

// proto: void QSplitter::setChildrenCollapsible(bool );
impl<'a> /*trait*/ QSplitter_setChildrenCollapsible for (i8) {
  fn setChildrenCollapsible(self, this: &mut QSplitter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QSplitter22setChildrenCollapsibleEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN9QSplitter22setChildrenCollapsibleEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QSplitter {
  pub fn handleWidth<T: QSplitter_handleWidth>(&mut self, value: T) -> i32 {
    value.handleWidth(self);
    return 1;
  }
}

pub trait QSplitter_handleWidth {
  fn handleWidth(self, this: &mut QSplitter) -> i32;
}

// proto: int QSplitter::handleWidth();
impl<'a> /*trait*/ QSplitter_handleWidth for () {
  fn handleWidth(self, this: &mut QSplitter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QSplitter11handleWidthEv()};
    unsafe {_ZNK9QSplitter11handleWidthEv()};
    return 1;
  }
}

impl /*struct*/ QSplitter {
  pub fn refresh<T: QSplitter_refresh>(&mut self, value: T) -> i32 {
    value.refresh(self);
    return 1;
  }
}

pub trait QSplitter_refresh {
  fn refresh(self, this: &mut QSplitter) -> i32;
}

// proto: void QSplitter::refresh();
impl<'a> /*trait*/ QSplitter_refresh for () {
  fn refresh(self, this: &mut QSplitter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QSplitter7refreshEv()};
    unsafe {_ZN9QSplitter7refreshEv()};
    return 1;
  }
}

impl /*struct*/ QSplitter {
  pub fn sizeHint<T: QSplitter_sizeHint>(&mut self, value: T) -> i32 {
    value.sizeHint(self);
    return 1;
  }
}

pub trait QSplitter_sizeHint {
  fn sizeHint(self, this: &mut QSplitter) -> i32;
}

// proto: QSize QSplitter::sizeHint();
impl<'a> /*trait*/ QSplitter_sizeHint for () {
  fn sizeHint(self, this: &mut QSplitter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QSplitter8sizeHintEv()};
    unsafe {_ZNK9QSplitter8sizeHintEv()};
    return 1;
  }
}

impl /*struct*/ QSplitter {
  pub fn indexOf<T: QSplitter_indexOf>(&mut self, value: T) -> i32 {
    value.indexOf(self);
    return 1;
  }
}

pub trait QSplitter_indexOf {
  fn indexOf(self, this: &mut QSplitter) -> i32;
}

// proto: int QSplitter::indexOf(QWidget * w);
impl<'a> /*trait*/ QSplitter_indexOf for (&'a mut QWidget) {
  fn indexOf(self, this: &mut QSplitter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QSplitter7indexOfEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZNK9QSplitter7indexOfEP7QWidget(arg0)};
    return 1;
  }
}

impl /*struct*/ QSplitter {
  pub fn getRange<T: QSplitter_getRange>(&mut self, value: T) -> i32 {
    value.getRange(self);
    return 1;
  }
}

pub trait QSplitter_getRange {
  fn getRange(self, this: &mut QSplitter) -> i32;
}

// proto: void QSplitter::getRange(int index, int * , int * );
impl<'a> /*trait*/ QSplitter_getRange for (i32, &'a mut i32, &'a mut i32) {
  fn getRange(self, this: &mut QSplitter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QSplitter8getRangeEiPiS0_()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as *mut c_int;
    let arg2 = self.2  as *mut c_int;
    unsafe {_ZNK9QSplitter8getRangeEiPiS0_(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QSplitter {
  pub fn handle<T: QSplitter_handle>(&mut self, value: T) -> i32 {
    value.handle(self);
    return 1;
  }
}

pub trait QSplitter_handle {
  fn handle(self, this: &mut QSplitter) -> i32;
}

// proto: QSplitterHandle * QSplitter::handle(int index);
impl<'a> /*trait*/ QSplitter_handle for (i32) {
  fn handle(self, this: &mut QSplitter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QSplitter6handleEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK9QSplitter6handleEi(arg0)};
    return 1;
  }
}

