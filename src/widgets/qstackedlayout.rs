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
use super::qsize::QSize;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  void QStackedLayout::currentChanged(int index);
  fn _ZN14QStackedLayout14currentChangedEi(qthis: *mut c_void, arg0: c_int);
  // proto:  int QStackedLayout::insertWidget(int index, QWidget * w);
  fn _ZN14QStackedLayout12insertWidgetEiP7QWidget(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void) -> c_int;
  // proto:  void QStackedLayout::QStackedLayout(QLayout * parentLayout);
  fn _ZN14QStackedLayoutC1EP7QLayout(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QStackedLayout::QStackedLayout(QWidget * parent);
  fn _ZN14QStackedLayoutC1EP7QWidget(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QStackedLayout::setGeometry(const QRect & rect);
  fn _ZN14QStackedLayout11setGeometryERK5QRect(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QWidget * QStackedLayout::currentWidget();
  fn _ZNK14QStackedLayout13currentWidgetEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QLayoutItem * QStackedLayout::takeAt(int );
  fn _ZN14QStackedLayout6takeAtEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  QSize QStackedLayout::minimumSize();
  fn _ZNK14QStackedLayout11minimumSizeEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QSize QStackedLayout::sizeHint();
  fn _ZNK14QStackedLayout8sizeHintEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QStackedLayout::QStackedLayout(const QStackedLayout & );
  fn _ZN14QStackedLayoutC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  int QStackedLayout::currentIndex();
  fn _ZNK14QStackedLayout12currentIndexEv(qthis: *mut c_void) -> c_int;
  // proto:  int QStackedLayout::count();
  fn _ZNK14QStackedLayout5countEv(qthis: *mut c_void) -> c_int;
  // proto:  void QStackedLayout::addItem(QLayoutItem * item);
  fn _ZN14QStackedLayout7addItemEP11QLayoutItem(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QStackedLayout::setCurrentWidget(QWidget * w);
  fn _ZN14QStackedLayout16setCurrentWidgetEP7QWidget(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  const QMetaObject * QStackedLayout::metaObject();
  fn _ZNK14QStackedLayout10metaObjectEv(qthis: *mut c_void);
  // proto:  void QStackedLayout::setCurrentIndex(int index);
  fn _ZN14QStackedLayout15setCurrentIndexEi(qthis: *mut c_void, arg0: c_int);
  // proto:  void QStackedLayout::widgetRemoved(int index);
  fn _ZN14QStackedLayout13widgetRemovedEi(qthis: *mut c_void, arg0: c_int);
  // proto:  QLayoutItem * QStackedLayout::itemAt(int );
  fn _ZNK14QStackedLayout6itemAtEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  void QStackedLayout::~QStackedLayout();
  fn _ZN14QStackedLayoutD0Ev(qthis: *mut c_void);
  // proto:  int QStackedLayout::addWidget(QWidget * w);
  fn _ZN14QStackedLayout9addWidgetEP7QWidget(qthis: *mut c_void, arg0: *mut c_void) -> c_int;
  // proto:  int QStackedLayout::heightForWidth(int width);
  fn _ZNK14QStackedLayout14heightForWidthEi(qthis: *mut c_void, arg0: c_int) -> c_int;
  // proto:  bool QStackedLayout::hasHeightForWidth();
  fn _ZNK14QStackedLayout17hasHeightForWidthEv(qthis: *mut c_void) -> c_char;
  // proto:  void QStackedLayout::QStackedLayout();
  fn _ZN14QStackedLayoutC1Ev(qthis: *mut c_void);
  // proto:  QWidget * QStackedLayout::widget(int );
  fn _ZNK14QStackedLayout6widgetEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
}

// body block begin
// class sizeof(QStackedLayout)=1
pub struct QStackedLayout {
  pub qclsinst: *mut c_void,
}

  // proto:  void QStackedLayout::currentChanged(int index);
impl /*struct*/ QStackedLayout {
  pub fn currentChanged<RetType, T: QStackedLayout_currentChanged<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.currentChanged(self);
    // return 1;
  }
}

pub trait QStackedLayout_currentChanged<RetType> {
  fn currentChanged(self , rsthis: &mut QStackedLayout) -> RetType;
}

  // proto:  void QStackedLayout::currentChanged(int index);
impl<'a> /*trait*/ QStackedLayout_currentChanged<()> for (i32) {
  fn currentChanged(self , rsthis: &mut QStackedLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QStackedLayout14currentChangedEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN14QStackedLayout14currentChangedEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QStackedLayout::insertWidget(int index, QWidget * w);
impl /*struct*/ QStackedLayout {
  pub fn insertWidget<RetType, T: QStackedLayout_insertWidget<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.insertWidget(self);
    // return 1;
  }
}

pub trait QStackedLayout_insertWidget<RetType> {
  fn insertWidget(self , rsthis: &mut QStackedLayout) -> RetType;
}

  // proto:  int QStackedLayout::insertWidget(int index, QWidget * w);
impl<'a> /*trait*/ QStackedLayout_insertWidget<i32> for (i32, QWidget) {
  fn insertWidget(self , rsthis: &mut QStackedLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QStackedLayout12insertWidgetEiP7QWidget()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN14QStackedLayout12insertWidgetEiP7QWidget(rsthis.qclsinst, arg0, arg1)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QStackedLayout::QStackedLayout(QLayout * parentLayout);
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

  // proto:  void QStackedLayout::QStackedLayout(QLayout * parentLayout);
impl<'a> /*trait*/ QStackedLayout_NewQStackedLayout for (QLayout) {
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

  // proto:  void QStackedLayout::QStackedLayout(QWidget * parent);
impl<'a> /*trait*/ QStackedLayout_NewQStackedLayout for (QWidget) {
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

  // proto:  void QStackedLayout::setGeometry(const QRect & rect);
impl /*struct*/ QStackedLayout {
  pub fn setGeometry<RetType, T: QStackedLayout_setGeometry<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setGeometry(self);
    // return 1;
  }
}

pub trait QStackedLayout_setGeometry<RetType> {
  fn setGeometry(self , rsthis: &mut QStackedLayout) -> RetType;
}

  // proto:  void QStackedLayout::setGeometry(const QRect & rect);
impl<'a> /*trait*/ QStackedLayout_setGeometry<()> for (QRect) {
  fn setGeometry(self , rsthis: &mut QStackedLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QStackedLayout11setGeometryERK5QRect()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN14QStackedLayout11setGeometryERK5QRect(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QWidget * QStackedLayout::currentWidget();
impl /*struct*/ QStackedLayout {
  pub fn currentWidget<RetType, T: QStackedLayout_currentWidget<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.currentWidget(self);
    // return 1;
  }
}

pub trait QStackedLayout_currentWidget<RetType> {
  fn currentWidget(self , rsthis: &mut QStackedLayout) -> RetType;
}

  // proto:  QWidget * QStackedLayout::currentWidget();
impl<'a> /*trait*/ QStackedLayout_currentWidget<QWidget> for () {
  fn currentWidget(self , rsthis: &mut QStackedLayout) -> QWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QStackedLayout13currentWidgetEv()};
    let mut ret = unsafe {_ZNK14QStackedLayout13currentWidgetEv(rsthis.qclsinst)};
    let mut ret1 = QWidget{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QLayoutItem * QStackedLayout::takeAt(int );
impl /*struct*/ QStackedLayout {
  pub fn takeAt<RetType, T: QStackedLayout_takeAt<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.takeAt(self);
    // return 1;
  }
}

pub trait QStackedLayout_takeAt<RetType> {
  fn takeAt(self , rsthis: &mut QStackedLayout) -> RetType;
}

  // proto:  QLayoutItem * QStackedLayout::takeAt(int );
impl<'a> /*trait*/ QStackedLayout_takeAt<QLayoutItem> for (i32) {
  fn takeAt(self , rsthis: &mut QStackedLayout) -> QLayoutItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QStackedLayout6takeAtEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZN14QStackedLayout6takeAtEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QLayoutItem{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QSize QStackedLayout::minimumSize();
impl /*struct*/ QStackedLayout {
  pub fn minimumSize<RetType, T: QStackedLayout_minimumSize<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.minimumSize(self);
    // return 1;
  }
}

pub trait QStackedLayout_minimumSize<RetType> {
  fn minimumSize(self , rsthis: &mut QStackedLayout) -> RetType;
}

  // proto:  QSize QStackedLayout::minimumSize();
impl<'a> /*trait*/ QStackedLayout_minimumSize<QSize> for () {
  fn minimumSize(self , rsthis: &mut QStackedLayout) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QStackedLayout11minimumSizeEv()};
    let mut ret = unsafe {_ZNK14QStackedLayout11minimumSizeEv(rsthis.qclsinst)};
    let mut ret1 = QSize{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QSize QStackedLayout::sizeHint();
impl /*struct*/ QStackedLayout {
  pub fn sizeHint<RetType, T: QStackedLayout_sizeHint<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.sizeHint(self);
    // return 1;
  }
}

pub trait QStackedLayout_sizeHint<RetType> {
  fn sizeHint(self , rsthis: &mut QStackedLayout) -> RetType;
}

  // proto:  QSize QStackedLayout::sizeHint();
impl<'a> /*trait*/ QStackedLayout_sizeHint<QSize> for () {
  fn sizeHint(self , rsthis: &mut QStackedLayout) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QStackedLayout8sizeHintEv()};
    let mut ret = unsafe {_ZNK14QStackedLayout8sizeHintEv(rsthis.qclsinst)};
    let mut ret1 = QSize{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QStackedLayout::QStackedLayout(const QStackedLayout & );
impl<'a> /*trait*/ QStackedLayout_NewQStackedLayout for (QStackedLayout) {
  fn NewQStackedLayout(self) -> QStackedLayout {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QStackedLayoutC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN14QStackedLayoutC1ERKS_(qthis, arg0)};
    let rsthis = QStackedLayout{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  int QStackedLayout::currentIndex();
impl /*struct*/ QStackedLayout {
  pub fn currentIndex<RetType, T: QStackedLayout_currentIndex<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.currentIndex(self);
    // return 1;
  }
}

pub trait QStackedLayout_currentIndex<RetType> {
  fn currentIndex(self , rsthis: &mut QStackedLayout) -> RetType;
}

  // proto:  int QStackedLayout::currentIndex();
impl<'a> /*trait*/ QStackedLayout_currentIndex<i32> for () {
  fn currentIndex(self , rsthis: &mut QStackedLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QStackedLayout12currentIndexEv()};
    let mut ret = unsafe {_ZNK14QStackedLayout12currentIndexEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  int QStackedLayout::count();
impl /*struct*/ QStackedLayout {
  pub fn count<RetType, T: QStackedLayout_count<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.count(self);
    // return 1;
  }
}

pub trait QStackedLayout_count<RetType> {
  fn count(self , rsthis: &mut QStackedLayout) -> RetType;
}

  // proto:  int QStackedLayout::count();
impl<'a> /*trait*/ QStackedLayout_count<i32> for () {
  fn count(self , rsthis: &mut QStackedLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QStackedLayout5countEv()};
    let mut ret = unsafe {_ZNK14QStackedLayout5countEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QStackedLayout::addItem(QLayoutItem * item);
impl /*struct*/ QStackedLayout {
  pub fn addItem<RetType, T: QStackedLayout_addItem<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.addItem(self);
    // return 1;
  }
}

pub trait QStackedLayout_addItem<RetType> {
  fn addItem(self , rsthis: &mut QStackedLayout) -> RetType;
}

  // proto:  void QStackedLayout::addItem(QLayoutItem * item);
impl<'a> /*trait*/ QStackedLayout_addItem<()> for (QLayoutItem) {
  fn addItem(self , rsthis: &mut QStackedLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QStackedLayout7addItemEP11QLayoutItem()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN14QStackedLayout7addItemEP11QLayoutItem(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QStackedLayout::setCurrentWidget(QWidget * w);
impl /*struct*/ QStackedLayout {
  pub fn setCurrentWidget<RetType, T: QStackedLayout_setCurrentWidget<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setCurrentWidget(self);
    // return 1;
  }
}

pub trait QStackedLayout_setCurrentWidget<RetType> {
  fn setCurrentWidget(self , rsthis: &mut QStackedLayout) -> RetType;
}

  // proto:  void QStackedLayout::setCurrentWidget(QWidget * w);
impl<'a> /*trait*/ QStackedLayout_setCurrentWidget<()> for (QWidget) {
  fn setCurrentWidget(self , rsthis: &mut QStackedLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QStackedLayout16setCurrentWidgetEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN14QStackedLayout16setCurrentWidgetEP7QWidget(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  const QMetaObject * QStackedLayout::metaObject();
impl /*struct*/ QStackedLayout {
  pub fn metaObject<RetType, T: QStackedLayout_metaObject<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QStackedLayout_metaObject<RetType> {
  fn metaObject(self , rsthis: &mut QStackedLayout) -> RetType;
}

  // proto:  const QMetaObject * QStackedLayout::metaObject();
impl<'a> /*trait*/ QStackedLayout_metaObject<()> for () {
  fn metaObject(self , rsthis: &mut QStackedLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QStackedLayout10metaObjectEv()};
     unsafe {_ZNK14QStackedLayout10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QStackedLayout::setCurrentIndex(int index);
impl /*struct*/ QStackedLayout {
  pub fn setCurrentIndex<RetType, T: QStackedLayout_setCurrentIndex<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setCurrentIndex(self);
    // return 1;
  }
}

pub trait QStackedLayout_setCurrentIndex<RetType> {
  fn setCurrentIndex(self , rsthis: &mut QStackedLayout) -> RetType;
}

  // proto:  void QStackedLayout::setCurrentIndex(int index);
impl<'a> /*trait*/ QStackedLayout_setCurrentIndex<()> for (i32) {
  fn setCurrentIndex(self , rsthis: &mut QStackedLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QStackedLayout15setCurrentIndexEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN14QStackedLayout15setCurrentIndexEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QStackedLayout::widgetRemoved(int index);
impl /*struct*/ QStackedLayout {
  pub fn widgetRemoved<RetType, T: QStackedLayout_widgetRemoved<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.widgetRemoved(self);
    // return 1;
  }
}

pub trait QStackedLayout_widgetRemoved<RetType> {
  fn widgetRemoved(self , rsthis: &mut QStackedLayout) -> RetType;
}

  // proto:  void QStackedLayout::widgetRemoved(int index);
impl<'a> /*trait*/ QStackedLayout_widgetRemoved<()> for (i32) {
  fn widgetRemoved(self , rsthis: &mut QStackedLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QStackedLayout13widgetRemovedEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN14QStackedLayout13widgetRemovedEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QLayoutItem * QStackedLayout::itemAt(int );
impl /*struct*/ QStackedLayout {
  pub fn itemAt<RetType, T: QStackedLayout_itemAt<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.itemAt(self);
    // return 1;
  }
}

pub trait QStackedLayout_itemAt<RetType> {
  fn itemAt(self , rsthis: &mut QStackedLayout) -> RetType;
}

  // proto:  QLayoutItem * QStackedLayout::itemAt(int );
impl<'a> /*trait*/ QStackedLayout_itemAt<QLayoutItem> for (i32) {
  fn itemAt(self , rsthis: &mut QStackedLayout) -> QLayoutItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QStackedLayout6itemAtEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK14QStackedLayout6itemAtEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QLayoutItem{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QStackedLayout::~QStackedLayout();
impl /*struct*/ QStackedLayout {
  pub fn FreeQStackedLayout<RetType, T: QStackedLayout_FreeQStackedLayout<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQStackedLayout(self);
    // return 1;
  }
}

pub trait QStackedLayout_FreeQStackedLayout<RetType> {
  fn FreeQStackedLayout(self , rsthis: &mut QStackedLayout) -> RetType;
}

  // proto:  void QStackedLayout::~QStackedLayout();
impl<'a> /*trait*/ QStackedLayout_FreeQStackedLayout<()> for () {
  fn FreeQStackedLayout(self , rsthis: &mut QStackedLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QStackedLayoutD0Ev()};
     unsafe {_ZN14QStackedLayoutD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  int QStackedLayout::addWidget(QWidget * w);
impl /*struct*/ QStackedLayout {
  pub fn addWidget<RetType, T: QStackedLayout_addWidget<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.addWidget(self);
    // return 1;
  }
}

pub trait QStackedLayout_addWidget<RetType> {
  fn addWidget(self , rsthis: &mut QStackedLayout) -> RetType;
}

  // proto:  int QStackedLayout::addWidget(QWidget * w);
impl<'a> /*trait*/ QStackedLayout_addWidget<i32> for (QWidget) {
  fn addWidget(self , rsthis: &mut QStackedLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QStackedLayout9addWidgetEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN14QStackedLayout9addWidgetEP7QWidget(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  int QStackedLayout::heightForWidth(int width);
impl /*struct*/ QStackedLayout {
  pub fn heightForWidth<RetType, T: QStackedLayout_heightForWidth<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.heightForWidth(self);
    // return 1;
  }
}

pub trait QStackedLayout_heightForWidth<RetType> {
  fn heightForWidth(self , rsthis: &mut QStackedLayout) -> RetType;
}

  // proto:  int QStackedLayout::heightForWidth(int width);
impl<'a> /*trait*/ QStackedLayout_heightForWidth<i32> for (i32) {
  fn heightForWidth(self , rsthis: &mut QStackedLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QStackedLayout14heightForWidthEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK14QStackedLayout14heightForWidthEi(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  bool QStackedLayout::hasHeightForWidth();
impl /*struct*/ QStackedLayout {
  pub fn hasHeightForWidth<RetType, T: QStackedLayout_hasHeightForWidth<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.hasHeightForWidth(self);
    // return 1;
  }
}

pub trait QStackedLayout_hasHeightForWidth<RetType> {
  fn hasHeightForWidth(self , rsthis: &mut QStackedLayout) -> RetType;
}

  // proto:  bool QStackedLayout::hasHeightForWidth();
impl<'a> /*trait*/ QStackedLayout_hasHeightForWidth<i8> for () {
  fn hasHeightForWidth(self , rsthis: &mut QStackedLayout) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QStackedLayout17hasHeightForWidthEv()};
    let mut ret = unsafe {_ZNK14QStackedLayout17hasHeightForWidthEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QStackedLayout::QStackedLayout();
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

  // proto:  QWidget * QStackedLayout::widget(int );
impl /*struct*/ QStackedLayout {
  pub fn widget<RetType, T: QStackedLayout_widget<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.widget(self);
    // return 1;
  }
}

pub trait QStackedLayout_widget<RetType> {
  fn widget(self , rsthis: &mut QStackedLayout) -> RetType;
}

  // proto:  QWidget * QStackedLayout::widget(int );
impl<'a> /*trait*/ QStackedLayout_widget<QWidget> for (i32) {
  fn widget(self , rsthis: &mut QStackedLayout) -> QWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QStackedLayout6widgetEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK14QStackedLayout6widgetEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QWidget{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

