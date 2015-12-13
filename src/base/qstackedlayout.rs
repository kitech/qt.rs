// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qwidget::QWidget;
use super::qlayout::QLayout;
use super::qrect::QRect;
use super::qlayoutitem::QLayoutItem;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: void QStackedLayout::currentChanged(int index);
  fn _ZN14QStackedLayout14currentChangedEi(arg0: c_int) -> i32;
  // proto: int QStackedLayout::insertWidget(int index, QWidget * w);
  fn _ZN14QStackedLayout12insertWidgetEiP7QWidget(arg0: c_int, arg1: *mut c_void) -> i32;
  // proto: void QStackedLayout::NewQStackedLayout(QLayout * parentLayout);
  fn _ZN14QStackedLayoutC1EP7QLayout(qthis: *mut c_void, arg0: *mut c_void) -> i32;
  // proto: void QStackedLayout::NewQStackedLayout(QWidget * parent);
  fn _ZN14QStackedLayoutC1EP7QWidget(qthis: *mut c_void, arg0: *mut c_void) -> i32;
  // proto: void QStackedLayout::setGeometry(const QRect & rect);
  fn _ZN14QStackedLayout11setGeometryERK5QRect(arg0: *const c_void) -> i32;
  // proto: QWidget * QStackedLayout::currentWidget();
  fn _ZNK14QStackedLayout13currentWidgetEv() -> i32;
  // proto: QLayoutItem * QStackedLayout::takeAt(int );
  fn _ZN14QStackedLayout6takeAtEi(arg0: c_int) -> i32;
  // proto: QSize QStackedLayout::minimumSize();
  fn _ZNK14QStackedLayout11minimumSizeEv() -> i32;
  // proto: QSize QStackedLayout::sizeHint();
  fn _ZNK14QStackedLayout8sizeHintEv() -> i32;
  // proto: void QStackedLayout::NewQStackedLayout(const QStackedLayout & );
  fn _ZN14QStackedLayoutC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: int QStackedLayout::currentIndex();
  fn _ZNK14QStackedLayout12currentIndexEv() -> i32;
  // proto: int QStackedLayout::count();
  fn _ZNK14QStackedLayout5countEv() -> i32;
  // proto: void QStackedLayout::addItem(QLayoutItem * item);
  fn _ZN14QStackedLayout7addItemEP11QLayoutItem(arg0: *mut c_void) -> i32;
  // proto: void QStackedLayout::setCurrentWidget(QWidget * w);
  fn _ZN14QStackedLayout16setCurrentWidgetEP7QWidget(arg0: *mut c_void) -> i32;
  // proto: const QMetaObject * QStackedLayout::metaObject();
  fn _ZNK14QStackedLayout10metaObjectEv() -> i32;
  // proto: void QStackedLayout::setCurrentIndex(int index);
  fn _ZN14QStackedLayout15setCurrentIndexEi(arg0: c_int) -> i32;
  // proto: void QStackedLayout::widgetRemoved(int index);
  fn _ZN14QStackedLayout13widgetRemovedEi(arg0: c_int) -> i32;
  // proto: QLayoutItem * QStackedLayout::itemAt(int );
  fn _ZNK14QStackedLayout6itemAtEi(arg0: c_int) -> i32;
  // proto: void QStackedLayout::FreeQStackedLayout();
  fn _ZN14QStackedLayoutD0Ev() -> i32;
  // proto: int QStackedLayout::addWidget(QWidget * w);
  fn _ZN14QStackedLayout9addWidgetEP7QWidget(arg0: *mut c_void) -> i32;
  // proto: int QStackedLayout::heightForWidth(int width);
  fn _ZNK14QStackedLayout14heightForWidthEi(arg0: c_int) -> i32;
  // proto: bool QStackedLayout::hasHeightForWidth();
  fn _ZNK14QStackedLayout17hasHeightForWidthEv() -> i32;
  // proto: void QStackedLayout::NewQStackedLayout();
  fn _ZN14QStackedLayoutC1Ev(qthis: *mut c_void) -> i32;
  // proto: QWidget * QStackedLayout::widget(int );
  fn _ZNK14QStackedLayout6widgetEi(arg0: c_int) -> i32;
}

// body block begin
// class sizeof(QStackedLayout)=1
pub struct QStackedLayout {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QStackedLayout {
  pub fn currentChanged<T: QStackedLayout_currentChanged>(&mut self, value: T) -> i32 {
    value.currentChanged(self);
    return 1;
  }
}

pub trait QStackedLayout_currentChanged {
  fn currentChanged(self, this: &mut QStackedLayout) -> i32;
}

// proto: void QStackedLayout::currentChanged(int index);
impl<'a> /*trait*/ QStackedLayout_currentChanged for (i32) {
  fn currentChanged(self, this: &mut QStackedLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QStackedLayout14currentChangedEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN14QStackedLayout14currentChangedEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QStackedLayout {
  pub fn insertWidget<T: QStackedLayout_insertWidget>(&mut self, value: T) -> i32 {
    value.insertWidget(self);
    return 1;
  }
}

pub trait QStackedLayout_insertWidget {
  fn insertWidget(self, this: &mut QStackedLayout) -> i32;
}

// proto: int QStackedLayout::insertWidget(int index, QWidget * w);
impl<'a> /*trait*/ QStackedLayout_insertWidget for (i32, &'a mut QWidget) {
  fn insertWidget(self, this: &mut QStackedLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QStackedLayout12insertWidgetEiP7QWidget()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN14QStackedLayout12insertWidgetEiP7QWidget(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QStackedLayout {
  pub fn NewQStackedLayout<T: QStackedLayout_NewQStackedLayout>(value: T) -> QStackedLayout {
    let rsthis = value.NewQStackedLayout();
    return rsthis;
    // return 1;
  }
}

pub trait QStackedLayout_NewQStackedLayout {
  fn NewQStackedLayout(self) -> QStackedLayout;
}

// proto: void QStackedLayout::NewQStackedLayout(QLayout * parentLayout);
impl<'a> /*trait*/ QStackedLayout_NewQStackedLayout for (&'a mut QLayout) {
  fn NewQStackedLayout(self) -> QStackedLayout {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QStackedLayoutC1EP7QLayout()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN14QStackedLayoutC1EP7QLayout(qthis, arg0)};
    let rsthis = QStackedLayout{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: void QStackedLayout::NewQStackedLayout(QWidget * parent);
impl<'a> /*trait*/ QStackedLayout_NewQStackedLayout for (&'a mut QWidget) {
  fn NewQStackedLayout(self) -> QStackedLayout {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QStackedLayoutC1EP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN14QStackedLayoutC1EP7QWidget(qthis, arg0)};
    let rsthis = QStackedLayout{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QStackedLayout {
  pub fn setGeometry<T: QStackedLayout_setGeometry>(&mut self, value: T) -> i32 {
    value.setGeometry(self);
    return 1;
  }
}

pub trait QStackedLayout_setGeometry {
  fn setGeometry(self, this: &mut QStackedLayout) -> i32;
}

// proto: void QStackedLayout::setGeometry(const QRect & rect);
impl<'a> /*trait*/ QStackedLayout_setGeometry for (&'a  QRect) {
  fn setGeometry(self, this: &mut QStackedLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QStackedLayout11setGeometryERK5QRect()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN14QStackedLayout11setGeometryERK5QRect(arg0)};
    return 1;
  }
}

impl /*struct*/ QStackedLayout {
  pub fn currentWidget<T: QStackedLayout_currentWidget>(&mut self, value: T) -> i32 {
    value.currentWidget(self);
    return 1;
  }
}

pub trait QStackedLayout_currentWidget {
  fn currentWidget(self, this: &mut QStackedLayout) -> i32;
}

// proto: QWidget * QStackedLayout::currentWidget();
impl<'a> /*trait*/ QStackedLayout_currentWidget for () {
  fn currentWidget(self, this: &mut QStackedLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QStackedLayout13currentWidgetEv()};
    unsafe {_ZNK14QStackedLayout13currentWidgetEv()};
    return 1;
  }
}

impl /*struct*/ QStackedLayout {
  pub fn takeAt<T: QStackedLayout_takeAt>(&mut self, value: T) -> i32 {
    value.takeAt(self);
    return 1;
  }
}

pub trait QStackedLayout_takeAt {
  fn takeAt(self, this: &mut QStackedLayout) -> i32;
}

// proto: QLayoutItem * QStackedLayout::takeAt(int );
impl<'a> /*trait*/ QStackedLayout_takeAt for (i32) {
  fn takeAt(self, this: &mut QStackedLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QStackedLayout6takeAtEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN14QStackedLayout6takeAtEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QStackedLayout {
  pub fn minimumSize<T: QStackedLayout_minimumSize>(&mut self, value: T) -> i32 {
    value.minimumSize(self);
    return 1;
  }
}

pub trait QStackedLayout_minimumSize {
  fn minimumSize(self, this: &mut QStackedLayout) -> i32;
}

// proto: QSize QStackedLayout::minimumSize();
impl<'a> /*trait*/ QStackedLayout_minimumSize for () {
  fn minimumSize(self, this: &mut QStackedLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QStackedLayout11minimumSizeEv()};
    unsafe {_ZNK14QStackedLayout11minimumSizeEv()};
    return 1;
  }
}

impl /*struct*/ QStackedLayout {
  pub fn sizeHint<T: QStackedLayout_sizeHint>(&mut self, value: T) -> i32 {
    value.sizeHint(self);
    return 1;
  }
}

pub trait QStackedLayout_sizeHint {
  fn sizeHint(self, this: &mut QStackedLayout) -> i32;
}

// proto: QSize QStackedLayout::sizeHint();
impl<'a> /*trait*/ QStackedLayout_sizeHint for () {
  fn sizeHint(self, this: &mut QStackedLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QStackedLayout8sizeHintEv()};
    unsafe {_ZNK14QStackedLayout8sizeHintEv()};
    return 1;
  }
}

// proto: void QStackedLayout::NewQStackedLayout(const QStackedLayout & );
impl<'a> /*trait*/ QStackedLayout_NewQStackedLayout for (&'a  QStackedLayout) {
  fn NewQStackedLayout(self) -> QStackedLayout {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QStackedLayoutC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN14QStackedLayoutC1ERKS_(qthis, arg0)};
    let rsthis = QStackedLayout{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QStackedLayout {
  pub fn currentIndex<T: QStackedLayout_currentIndex>(&mut self, value: T) -> i32 {
    value.currentIndex(self);
    return 1;
  }
}

pub trait QStackedLayout_currentIndex {
  fn currentIndex(self, this: &mut QStackedLayout) -> i32;
}

// proto: int QStackedLayout::currentIndex();
impl<'a> /*trait*/ QStackedLayout_currentIndex for () {
  fn currentIndex(self, this: &mut QStackedLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QStackedLayout12currentIndexEv()};
    unsafe {_ZNK14QStackedLayout12currentIndexEv()};
    return 1;
  }
}

impl /*struct*/ QStackedLayout {
  pub fn count<T: QStackedLayout_count>(&mut self, value: T) -> i32 {
    value.count(self);
    return 1;
  }
}

pub trait QStackedLayout_count {
  fn count(self, this: &mut QStackedLayout) -> i32;
}

// proto: int QStackedLayout::count();
impl<'a> /*trait*/ QStackedLayout_count for () {
  fn count(self, this: &mut QStackedLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QStackedLayout5countEv()};
    unsafe {_ZNK14QStackedLayout5countEv()};
    return 1;
  }
}

impl /*struct*/ QStackedLayout {
  pub fn addItem<T: QStackedLayout_addItem>(&mut self, value: T) -> i32 {
    value.addItem(self);
    return 1;
  }
}

pub trait QStackedLayout_addItem {
  fn addItem(self, this: &mut QStackedLayout) -> i32;
}

// proto: void QStackedLayout::addItem(QLayoutItem * item);
impl<'a> /*trait*/ QStackedLayout_addItem for (&'a mut QLayoutItem) {
  fn addItem(self, this: &mut QStackedLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QStackedLayout7addItemEP11QLayoutItem()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN14QStackedLayout7addItemEP11QLayoutItem(arg0)};
    return 1;
  }
}

impl /*struct*/ QStackedLayout {
  pub fn setCurrentWidget<T: QStackedLayout_setCurrentWidget>(&mut self, value: T) -> i32 {
    value.setCurrentWidget(self);
    return 1;
  }
}

pub trait QStackedLayout_setCurrentWidget {
  fn setCurrentWidget(self, this: &mut QStackedLayout) -> i32;
}

// proto: void QStackedLayout::setCurrentWidget(QWidget * w);
impl<'a> /*trait*/ QStackedLayout_setCurrentWidget for (&'a mut QWidget) {
  fn setCurrentWidget(self, this: &mut QStackedLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QStackedLayout16setCurrentWidgetEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN14QStackedLayout16setCurrentWidgetEP7QWidget(arg0)};
    return 1;
  }
}

impl /*struct*/ QStackedLayout {
  pub fn metaObject<T: QStackedLayout_metaObject>(&mut self, value: T) -> i32 {
    value.metaObject(self);
    return 1;
  }
}

pub trait QStackedLayout_metaObject {
  fn metaObject(self, this: &mut QStackedLayout) -> i32;
}

// proto: const QMetaObject * QStackedLayout::metaObject();
impl<'a> /*trait*/ QStackedLayout_metaObject for () {
  fn metaObject(self, this: &mut QStackedLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QStackedLayout10metaObjectEv()};
    unsafe {_ZNK14QStackedLayout10metaObjectEv()};
    return 1;
  }
}

impl /*struct*/ QStackedLayout {
  pub fn setCurrentIndex<T: QStackedLayout_setCurrentIndex>(&mut self, value: T) -> i32 {
    value.setCurrentIndex(self);
    return 1;
  }
}

pub trait QStackedLayout_setCurrentIndex {
  fn setCurrentIndex(self, this: &mut QStackedLayout) -> i32;
}

// proto: void QStackedLayout::setCurrentIndex(int index);
impl<'a> /*trait*/ QStackedLayout_setCurrentIndex for (i32) {
  fn setCurrentIndex(self, this: &mut QStackedLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QStackedLayout15setCurrentIndexEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN14QStackedLayout15setCurrentIndexEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QStackedLayout {
  pub fn widgetRemoved<T: QStackedLayout_widgetRemoved>(&mut self, value: T) -> i32 {
    value.widgetRemoved(self);
    return 1;
  }
}

pub trait QStackedLayout_widgetRemoved {
  fn widgetRemoved(self, this: &mut QStackedLayout) -> i32;
}

// proto: void QStackedLayout::widgetRemoved(int index);
impl<'a> /*trait*/ QStackedLayout_widgetRemoved for (i32) {
  fn widgetRemoved(self, this: &mut QStackedLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QStackedLayout13widgetRemovedEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN14QStackedLayout13widgetRemovedEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QStackedLayout {
  pub fn itemAt<T: QStackedLayout_itemAt>(&mut self, value: T) -> i32 {
    value.itemAt(self);
    return 1;
  }
}

pub trait QStackedLayout_itemAt {
  fn itemAt(self, this: &mut QStackedLayout) -> i32;
}

// proto: QLayoutItem * QStackedLayout::itemAt(int );
impl<'a> /*trait*/ QStackedLayout_itemAt for (i32) {
  fn itemAt(self, this: &mut QStackedLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QStackedLayout6itemAtEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK14QStackedLayout6itemAtEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QStackedLayout {
  pub fn FreeQStackedLayout<T: QStackedLayout_FreeQStackedLayout>(&mut self, value: T) -> i32 {
    value.FreeQStackedLayout(self);
    return 1;
  }
}

pub trait QStackedLayout_FreeQStackedLayout {
  fn FreeQStackedLayout(self, this: &mut QStackedLayout) -> i32;
}

// proto: void QStackedLayout::FreeQStackedLayout();
impl<'a> /*trait*/ QStackedLayout_FreeQStackedLayout for () {
  fn FreeQStackedLayout(self, this: &mut QStackedLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QStackedLayoutD0Ev()};
    unsafe {_ZN14QStackedLayoutD0Ev()};
    return 1;
  }
}

impl /*struct*/ QStackedLayout {
  pub fn addWidget<T: QStackedLayout_addWidget>(&mut self, value: T) -> i32 {
    value.addWidget(self);
    return 1;
  }
}

pub trait QStackedLayout_addWidget {
  fn addWidget(self, this: &mut QStackedLayout) -> i32;
}

// proto: int QStackedLayout::addWidget(QWidget * w);
impl<'a> /*trait*/ QStackedLayout_addWidget for (&'a mut QWidget) {
  fn addWidget(self, this: &mut QStackedLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QStackedLayout9addWidgetEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN14QStackedLayout9addWidgetEP7QWidget(arg0)};
    return 1;
  }
}

impl /*struct*/ QStackedLayout {
  pub fn heightForWidth<T: QStackedLayout_heightForWidth>(&mut self, value: T) -> i32 {
    value.heightForWidth(self);
    return 1;
  }
}

pub trait QStackedLayout_heightForWidth {
  fn heightForWidth(self, this: &mut QStackedLayout) -> i32;
}

// proto: int QStackedLayout::heightForWidth(int width);
impl<'a> /*trait*/ QStackedLayout_heightForWidth for (i32) {
  fn heightForWidth(self, this: &mut QStackedLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QStackedLayout14heightForWidthEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK14QStackedLayout14heightForWidthEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QStackedLayout {
  pub fn hasHeightForWidth<T: QStackedLayout_hasHeightForWidth>(&mut self, value: T) -> i32 {
    value.hasHeightForWidth(self);
    return 1;
  }
}

pub trait QStackedLayout_hasHeightForWidth {
  fn hasHeightForWidth(self, this: &mut QStackedLayout) -> i32;
}

// proto: bool QStackedLayout::hasHeightForWidth();
impl<'a> /*trait*/ QStackedLayout_hasHeightForWidth for () {
  fn hasHeightForWidth(self, this: &mut QStackedLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QStackedLayout17hasHeightForWidthEv()};
    unsafe {_ZNK14QStackedLayout17hasHeightForWidthEv()};
    return 1;
  }
}

// proto: void QStackedLayout::NewQStackedLayout();
impl<'a> /*trait*/ QStackedLayout_NewQStackedLayout for () {
  fn NewQStackedLayout(self) -> QStackedLayout {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QStackedLayoutC1Ev()};
    unsafe {_ZN14QStackedLayoutC1Ev(qthis)};
    let rsthis = QStackedLayout{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QStackedLayout {
  pub fn widget<T: QStackedLayout_widget>(&mut self, value: T) -> i32 {
    value.widget(self);
    return 1;
  }
}

pub trait QStackedLayout_widget {
  fn widget(self, this: &mut QStackedLayout) -> i32;
}

// proto: QWidget * QStackedLayout::widget(int );
impl<'a> /*trait*/ QStackedLayout_widget for (i32) {
  fn widget(self, this: &mut QStackedLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QStackedLayout6widgetEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK14QStackedLayout6widgetEi(arg0)};
    return 1;
  }
}

