// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qlayout::QLayout;
use super::qwidget::QWidget;
use super::qstring::QString;
use super::qrect::QRect;
use super::qsize::QSize;
use super::qlayoutitem::QLayoutItem;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  int QFormLayout::horizontalSpacing();
  fn _ZNK11QFormLayout17horizontalSpacingEv(qthis: *mut c_void) -> c_int;
  // proto:  int QFormLayout::rowCount();
  fn _ZNK11QFormLayout8rowCountEv(qthis: *mut c_void) -> c_int;
  // proto:  QWidget * QFormLayout::labelForField(QLayout * field);
  fn _ZNK11QFormLayout13labelForFieldEP7QLayout(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QFormLayout::addRow(const QString & labelText, QLayout * field);
  fn _ZN11QFormLayout6addRowERK7QStringP7QLayout(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) ;
  // proto:  void QFormLayout::insertRow(int row, const QString & labelText, QLayout * field);
  fn _ZN11QFormLayout9insertRowEiRK7QStringP7QLayout(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void, arg2: *mut c_void) ;
  // proto:  QWidget * QFormLayout::labelForField(QWidget * field);
  fn _ZNK11QFormLayout13labelForFieldEP7QWidget(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QFormLayout::insertRow(int row, QWidget * label, QLayout * field);
  fn _ZN11QFormLayout9insertRowEiP7QWidgetP7QLayout(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void, arg2: *mut c_void) ;
  // proto:  int QFormLayout::count();
  fn _ZNK11QFormLayout5countEv(qthis: *mut c_void) -> c_int;
  // proto:  int QFormLayout::spacing();
  fn _ZNK11QFormLayout7spacingEv(qthis: *mut c_void) -> c_int;
  // proto:  void QFormLayout::NewQFormLayout(QWidget * parent);
  fn _ZN11QFormLayoutC1EP7QWidget(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QFormLayout::insertRow(int row, QLayout * layout);
  fn _ZN11QFormLayout9insertRowEiP7QLayout(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void) ;
  // proto:  void QFormLayout::setGeometry(const QRect & rect);
  fn _ZN11QFormLayout11setGeometryERK5QRect(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QFormLayout::setVerticalSpacing(int spacing);
  fn _ZN11QFormLayout18setVerticalSpacingEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QFormLayout::setHorizontalSpacing(int spacing);
  fn _ZN11QFormLayout20setHorizontalSpacingEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QFormLayout::insertRow(int row, const QString & labelText, QWidget * field);
  fn _ZN11QFormLayout9insertRowEiRK7QStringP7QWidget(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void, arg2: *mut c_void) ;
  // proto:  const QMetaObject * QFormLayout::metaObject();
  fn _ZNK11QFormLayout10metaObjectEv(qthis: *mut c_void) ;
  // proto:  void QFormLayout::insertRow(int row, QWidget * label, QWidget * field);
  fn _ZN11QFormLayout9insertRowEiP7QWidgetS1_(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void, arg2: *mut c_void) ;
  // proto:  void QFormLayout::setSpacing(int );
  fn _ZN11QFormLayout10setSpacingEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QFormLayout::FreeQFormLayout();
  fn _ZN11QFormLayoutD0Ev(qthis: *mut c_void) ;
  // proto:  void QFormLayout::addRow(QLayout * layout);
  fn _ZN11QFormLayout6addRowEP7QLayout(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QSize QFormLayout::sizeHint();
  fn _ZNK11QFormLayout8sizeHintEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QFormLayout::invalidate();
  fn _ZN11QFormLayout10invalidateEv(qthis: *mut c_void) ;
  // proto:  QLayoutItem * QFormLayout::itemAt(int index);
  fn _ZNK11QFormLayout6itemAtEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  QLayoutItem * QFormLayout::takeAt(int index);
  fn _ZN11QFormLayout6takeAtEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  void QFormLayout::addRow(const QString & labelText, QWidget * field);
  fn _ZN11QFormLayout6addRowERK7QStringP7QWidget(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) ;
  // proto:  QSize QFormLayout::minimumSize();
  fn _ZNK11QFormLayout11minimumSizeEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QFormLayout::addRow(QWidget * widget);
  fn _ZN11QFormLayout6addRowEP7QWidget(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QFormLayout::addRow(QWidget * label, QLayout * field);
  fn _ZN11QFormLayout6addRowEP7QWidgetP7QLayout(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) ;
  // proto:  int QFormLayout::verticalSpacing();
  fn _ZNK11QFormLayout15verticalSpacingEv(qthis: *mut c_void) -> c_int;
  // proto:  int QFormLayout::heightForWidth(int width);
  fn _ZNK11QFormLayout14heightForWidthEi(qthis: *mut c_void, arg0: c_int) -> c_int;
  // proto:  void QFormLayout::addItem(QLayoutItem * item);
  fn _ZN11QFormLayout7addItemEP11QLayoutItem(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  bool QFormLayout::hasHeightForWidth();
  fn _ZNK11QFormLayout17hasHeightForWidthEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QFormLayout::insertRow(int row, QWidget * widget);
  fn _ZN11QFormLayout9insertRowEiP7QWidget(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void) ;
  // proto:  void QFormLayout::addRow(QWidget * label, QWidget * field);
  fn _ZN11QFormLayout6addRowEP7QWidgetS1_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) ;
}

// body block begin
// class sizeof(QFormLayout)=1
pub struct QFormLayout {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QFormLayout {
  pub fn horizontalSpacing<T: QFormLayout_horizontalSpacing>(&mut self, value: T) -> i32 {
    return value.horizontalSpacing(self);
    // return 1;
  }
}

pub trait QFormLayout_horizontalSpacing {
  fn horizontalSpacing(self, rsthis: &mut QFormLayout) -> i32;
}

// proto:  int QFormLayout::horizontalSpacing();
impl<'a> /*trait*/ QFormLayout_horizontalSpacing for () {
  fn horizontalSpacing(self, rsthis: &mut QFormLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QFormLayout17horizontalSpacingEv()};
    let mut ret = unsafe {_ZNK11QFormLayout17horizontalSpacingEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QFormLayout {
  pub fn rowCount<T: QFormLayout_rowCount>(&mut self, value: T) -> i32 {
    return value.rowCount(self);
    // return 1;
  }
}

pub trait QFormLayout_rowCount {
  fn rowCount(self, rsthis: &mut QFormLayout) -> i32;
}

// proto:  int QFormLayout::rowCount();
impl<'a> /*trait*/ QFormLayout_rowCount for () {
  fn rowCount(self, rsthis: &mut QFormLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QFormLayout8rowCountEv()};
    let mut ret = unsafe {_ZNK11QFormLayout8rowCountEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QFormLayout {
  pub fn labelForField<T: QFormLayout_labelForField>(&mut self, value: T) -> QWidget {
    return value.labelForField(self);
    // return 1;
  }
}

pub trait QFormLayout_labelForField {
  fn labelForField(self, rsthis: &mut QFormLayout) -> QWidget;
}

// proto:  QWidget * QFormLayout::labelForField(QLayout * field);
impl<'a> /*trait*/ QFormLayout_labelForField for (&'a mut QLayout) {
  fn labelForField(self, rsthis: &mut QFormLayout) -> QWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QFormLayout13labelForFieldEP7QLayout()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK11QFormLayout13labelForFieldEP7QLayout(rsthis.qclsinst, arg0)};
    let mut ret1 = QWidget{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QFormLayout {
  pub fn addRow<T: QFormLayout_addRow>(&mut self, value: T)  {
     value.addRow(self);
    // return 1;
  }
}

pub trait QFormLayout_addRow {
  fn addRow(self, rsthis: &mut QFormLayout) ;
}

// proto:  void QFormLayout::addRow(const QString & labelText, QLayout * field);
impl<'a> /*trait*/ QFormLayout_addRow for (&'a  QString, &'a mut QLayout) {
  fn addRow(self, rsthis: &mut QFormLayout)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFormLayout6addRowERK7QStringP7QLayout()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN11QFormLayout6addRowERK7QStringP7QLayout(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QFormLayout {
  pub fn insertRow<T: QFormLayout_insertRow>(&mut self, value: T)  {
     value.insertRow(self);
    // return 1;
  }
}

pub trait QFormLayout_insertRow {
  fn insertRow(self, rsthis: &mut QFormLayout) ;
}

// proto:  void QFormLayout::insertRow(int row, const QString & labelText, QLayout * field);
impl<'a> /*trait*/ QFormLayout_insertRow for (i32, &'a  QString, &'a mut QLayout) {
  fn insertRow(self, rsthis: &mut QFormLayout)  {
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
impl<'a> /*trait*/ QFormLayout_labelForField for (&'a mut QWidget) {
  fn labelForField(self, rsthis: &mut QFormLayout) -> QWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QFormLayout13labelForFieldEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK11QFormLayout13labelForFieldEP7QWidget(rsthis.qclsinst, arg0)};
    let mut ret1 = QWidget{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  void QFormLayout::insertRow(int row, QWidget * label, QLayout * field);
impl<'a> /*trait*/ QFormLayout_insertRow for (i32, &'a mut QWidget, &'a mut QLayout) {
  fn insertRow(self, rsthis: &mut QFormLayout)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFormLayout9insertRowEiP7QWidgetP7QLayout()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
     unsafe {_ZN11QFormLayout9insertRowEiP7QWidgetP7QLayout(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

impl /*struct*/ QFormLayout {
  pub fn count<T: QFormLayout_count>(&mut self, value: T) -> i32 {
    return value.count(self);
    // return 1;
  }
}

pub trait QFormLayout_count {
  fn count(self, rsthis: &mut QFormLayout) -> i32;
}

// proto:  int QFormLayout::count();
impl<'a> /*trait*/ QFormLayout_count for () {
  fn count(self, rsthis: &mut QFormLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QFormLayout5countEv()};
    let mut ret = unsafe {_ZNK11QFormLayout5countEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QFormLayout {
  pub fn spacing<T: QFormLayout_spacing>(&mut self, value: T) -> i32 {
    return value.spacing(self);
    // return 1;
  }
}

pub trait QFormLayout_spacing {
  fn spacing(self, rsthis: &mut QFormLayout) -> i32;
}

// proto:  int QFormLayout::spacing();
impl<'a> /*trait*/ QFormLayout_spacing for () {
  fn spacing(self, rsthis: &mut QFormLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QFormLayout7spacingEv()};
    let mut ret = unsafe {_ZNK11QFormLayout7spacingEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QFormLayout {
  pub fn NewQFormLayout<T: QFormLayout_NewQFormLayout>(value: T) -> QFormLayout {
    let rsthis = value.NewQFormLayout();
    return rsthis;
    // return 1;
  }
}

pub trait QFormLayout_NewQFormLayout {
  fn NewQFormLayout(self) -> QFormLayout;
}

// proto: void QFormLayout::NewQFormLayout(QWidget * parent);
impl<'a> /*trait*/ QFormLayout_NewQFormLayout for (&'a mut QWidget) {
  fn NewQFormLayout(self) -> QFormLayout {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFormLayoutC1EP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN11QFormLayoutC1EP7QWidget(qthis, arg0)};
    let rsthis = QFormLayout{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto:  void QFormLayout::insertRow(int row, QLayout * layout);
impl<'a> /*trait*/ QFormLayout_insertRow for (i32, &'a mut QLayout) {
  fn insertRow(self, rsthis: &mut QFormLayout)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFormLayout9insertRowEiP7QLayout()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN11QFormLayout9insertRowEiP7QLayout(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QFormLayout {
  pub fn setGeometry<T: QFormLayout_setGeometry>(&mut self, value: T)  {
     value.setGeometry(self);
    // return 1;
  }
}

pub trait QFormLayout_setGeometry {
  fn setGeometry(self, rsthis: &mut QFormLayout) ;
}

// proto:  void QFormLayout::setGeometry(const QRect & rect);
impl<'a> /*trait*/ QFormLayout_setGeometry for (&'a  QRect) {
  fn setGeometry(self, rsthis: &mut QFormLayout)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFormLayout11setGeometryERK5QRect()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QFormLayout11setGeometryERK5QRect(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QFormLayout {
  pub fn setVerticalSpacing<T: QFormLayout_setVerticalSpacing>(&mut self, value: T)  {
     value.setVerticalSpacing(self);
    // return 1;
  }
}

pub trait QFormLayout_setVerticalSpacing {
  fn setVerticalSpacing(self, rsthis: &mut QFormLayout) ;
}

// proto:  void QFormLayout::setVerticalSpacing(int spacing);
impl<'a> /*trait*/ QFormLayout_setVerticalSpacing for (i32) {
  fn setVerticalSpacing(self, rsthis: &mut QFormLayout)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFormLayout18setVerticalSpacingEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN11QFormLayout18setVerticalSpacingEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QFormLayout {
  pub fn setHorizontalSpacing<T: QFormLayout_setHorizontalSpacing>(&mut self, value: T)  {
     value.setHorizontalSpacing(self);
    // return 1;
  }
}

pub trait QFormLayout_setHorizontalSpacing {
  fn setHorizontalSpacing(self, rsthis: &mut QFormLayout) ;
}

// proto:  void QFormLayout::setHorizontalSpacing(int spacing);
impl<'a> /*trait*/ QFormLayout_setHorizontalSpacing for (i32) {
  fn setHorizontalSpacing(self, rsthis: &mut QFormLayout)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFormLayout20setHorizontalSpacingEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN11QFormLayout20setHorizontalSpacingEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  void QFormLayout::insertRow(int row, const QString & labelText, QWidget * field);
impl<'a> /*trait*/ QFormLayout_insertRow for (i32, &'a  QString, &'a mut QWidget) {
  fn insertRow(self, rsthis: &mut QFormLayout)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFormLayout9insertRowEiRK7QStringP7QWidget()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
     unsafe {_ZN11QFormLayout9insertRowEiRK7QStringP7QWidget(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

impl /*struct*/ QFormLayout {
  pub fn metaObject<T: QFormLayout_metaObject>(&mut self, value: T)  {
     value.metaObject(self);
    // return 1;
  }
}

pub trait QFormLayout_metaObject {
  fn metaObject(self, rsthis: &mut QFormLayout) ;
}

// proto:  const QMetaObject * QFormLayout::metaObject();
impl<'a> /*trait*/ QFormLayout_metaObject for () {
  fn metaObject(self, rsthis: &mut QFormLayout)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QFormLayout10metaObjectEv()};
     unsafe {_ZNK11QFormLayout10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

// proto:  void QFormLayout::insertRow(int row, QWidget * label, QWidget * field);
impl<'a> /*trait*/ QFormLayout_insertRow for (i32, &'a mut QWidget, &'a mut QWidget) {
  fn insertRow(self, rsthis: &mut QFormLayout)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFormLayout9insertRowEiP7QWidgetS1_()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
     unsafe {_ZN11QFormLayout9insertRowEiP7QWidgetS1_(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

impl /*struct*/ QFormLayout {
  pub fn setSpacing<T: QFormLayout_setSpacing>(&mut self, value: T)  {
     value.setSpacing(self);
    // return 1;
  }
}

pub trait QFormLayout_setSpacing {
  fn setSpacing(self, rsthis: &mut QFormLayout) ;
}

// proto:  void QFormLayout::setSpacing(int );
impl<'a> /*trait*/ QFormLayout_setSpacing for (i32) {
  fn setSpacing(self, rsthis: &mut QFormLayout)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFormLayout10setSpacingEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN11QFormLayout10setSpacingEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QFormLayout {
  pub fn FreeQFormLayout<T: QFormLayout_FreeQFormLayout>(&mut self, value: T)  {
     value.FreeQFormLayout(self);
    // return 1;
  }
}

pub trait QFormLayout_FreeQFormLayout {
  fn FreeQFormLayout(self, rsthis: &mut QFormLayout) ;
}

// proto:  void QFormLayout::FreeQFormLayout();
impl<'a> /*trait*/ QFormLayout_FreeQFormLayout for () {
  fn FreeQFormLayout(self, rsthis: &mut QFormLayout)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFormLayoutD0Ev()};
     unsafe {_ZN11QFormLayoutD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

// proto:  void QFormLayout::addRow(QLayout * layout);
impl<'a> /*trait*/ QFormLayout_addRow for (&'a mut QLayout) {
  fn addRow(self, rsthis: &mut QFormLayout)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFormLayout6addRowEP7QLayout()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QFormLayout6addRowEP7QLayout(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QFormLayout {
  pub fn sizeHint<T: QFormLayout_sizeHint>(&mut self, value: T) -> QSize {
    return value.sizeHint(self);
    // return 1;
  }
}

pub trait QFormLayout_sizeHint {
  fn sizeHint(self, rsthis: &mut QFormLayout) -> QSize;
}

// proto:  QSize QFormLayout::sizeHint();
impl<'a> /*trait*/ QFormLayout_sizeHint for () {
  fn sizeHint(self, rsthis: &mut QFormLayout) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QFormLayout8sizeHintEv()};
    let mut ret = unsafe {_ZNK11QFormLayout8sizeHintEv(rsthis.qclsinst)};
    let mut ret1 = QSize{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QFormLayout {
  pub fn invalidate<T: QFormLayout_invalidate>(&mut self, value: T)  {
     value.invalidate(self);
    // return 1;
  }
}

pub trait QFormLayout_invalidate {
  fn invalidate(self, rsthis: &mut QFormLayout) ;
}

// proto:  void QFormLayout::invalidate();
impl<'a> /*trait*/ QFormLayout_invalidate for () {
  fn invalidate(self, rsthis: &mut QFormLayout)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFormLayout10invalidateEv()};
     unsafe {_ZN11QFormLayout10invalidateEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QFormLayout {
  pub fn itemAt<T: QFormLayout_itemAt>(&mut self, value: T) -> QLayoutItem {
    return value.itemAt(self);
    // return 1;
  }
}

pub trait QFormLayout_itemAt {
  fn itemAt(self, rsthis: &mut QFormLayout) -> QLayoutItem;
}

// proto:  QLayoutItem * QFormLayout::itemAt(int index);
impl<'a> /*trait*/ QFormLayout_itemAt for (i32) {
  fn itemAt(self, rsthis: &mut QFormLayout) -> QLayoutItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QFormLayout6itemAtEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK11QFormLayout6itemAtEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QLayoutItem{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QFormLayout {
  pub fn takeAt<T: QFormLayout_takeAt>(&mut self, value: T) -> QLayoutItem {
    return value.takeAt(self);
    // return 1;
  }
}

pub trait QFormLayout_takeAt {
  fn takeAt(self, rsthis: &mut QFormLayout) -> QLayoutItem;
}

// proto:  QLayoutItem * QFormLayout::takeAt(int index);
impl<'a> /*trait*/ QFormLayout_takeAt for (i32) {
  fn takeAt(self, rsthis: &mut QFormLayout) -> QLayoutItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFormLayout6takeAtEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZN11QFormLayout6takeAtEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QLayoutItem{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  void QFormLayout::addRow(const QString & labelText, QWidget * field);
impl<'a> /*trait*/ QFormLayout_addRow for (&'a  QString, &'a mut QWidget) {
  fn addRow(self, rsthis: &mut QFormLayout)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFormLayout6addRowERK7QStringP7QWidget()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN11QFormLayout6addRowERK7QStringP7QWidget(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QFormLayout {
  pub fn minimumSize<T: QFormLayout_minimumSize>(&mut self, value: T) -> QSize {
    return value.minimumSize(self);
    // return 1;
  }
}

pub trait QFormLayout_minimumSize {
  fn minimumSize(self, rsthis: &mut QFormLayout) -> QSize;
}

// proto:  QSize QFormLayout::minimumSize();
impl<'a> /*trait*/ QFormLayout_minimumSize for () {
  fn minimumSize(self, rsthis: &mut QFormLayout) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QFormLayout11minimumSizeEv()};
    let mut ret = unsafe {_ZNK11QFormLayout11minimumSizeEv(rsthis.qclsinst)};
    let mut ret1 = QSize{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  void QFormLayout::addRow(QWidget * widget);
impl<'a> /*trait*/ QFormLayout_addRow for (&'a mut QWidget) {
  fn addRow(self, rsthis: &mut QFormLayout)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFormLayout6addRowEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QFormLayout6addRowEP7QWidget(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  void QFormLayout::addRow(QWidget * label, QLayout * field);
impl<'a> /*trait*/ QFormLayout_addRow for (&'a mut QWidget, &'a mut QLayout) {
  fn addRow(self, rsthis: &mut QFormLayout)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFormLayout6addRowEP7QWidgetP7QLayout()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN11QFormLayout6addRowEP7QWidgetP7QLayout(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QFormLayout {
  pub fn verticalSpacing<T: QFormLayout_verticalSpacing>(&mut self, value: T) -> i32 {
    return value.verticalSpacing(self);
    // return 1;
  }
}

pub trait QFormLayout_verticalSpacing {
  fn verticalSpacing(self, rsthis: &mut QFormLayout) -> i32;
}

// proto:  int QFormLayout::verticalSpacing();
impl<'a> /*trait*/ QFormLayout_verticalSpacing for () {
  fn verticalSpacing(self, rsthis: &mut QFormLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QFormLayout15verticalSpacingEv()};
    let mut ret = unsafe {_ZNK11QFormLayout15verticalSpacingEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QFormLayout {
  pub fn heightForWidth<T: QFormLayout_heightForWidth>(&mut self, value: T) -> i32 {
    return value.heightForWidth(self);
    // return 1;
  }
}

pub trait QFormLayout_heightForWidth {
  fn heightForWidth(self, rsthis: &mut QFormLayout) -> i32;
}

// proto:  int QFormLayout::heightForWidth(int width);
impl<'a> /*trait*/ QFormLayout_heightForWidth for (i32) {
  fn heightForWidth(self, rsthis: &mut QFormLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QFormLayout14heightForWidthEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK11QFormLayout14heightForWidthEi(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QFormLayout {
  pub fn addItem<T: QFormLayout_addItem>(&mut self, value: T)  {
     value.addItem(self);
    // return 1;
  }
}

pub trait QFormLayout_addItem {
  fn addItem(self, rsthis: &mut QFormLayout) ;
}

// proto:  void QFormLayout::addItem(QLayoutItem * item);
impl<'a> /*trait*/ QFormLayout_addItem for (&'a mut QLayoutItem) {
  fn addItem(self, rsthis: &mut QFormLayout)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFormLayout7addItemEP11QLayoutItem()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QFormLayout7addItemEP11QLayoutItem(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QFormLayout {
  pub fn hasHeightForWidth<T: QFormLayout_hasHeightForWidth>(&mut self, value: T) -> i8 {
    return value.hasHeightForWidth(self);
    // return 1;
  }
}

pub trait QFormLayout_hasHeightForWidth {
  fn hasHeightForWidth(self, rsthis: &mut QFormLayout) -> i8;
}

// proto:  bool QFormLayout::hasHeightForWidth();
impl<'a> /*trait*/ QFormLayout_hasHeightForWidth for () {
  fn hasHeightForWidth(self, rsthis: &mut QFormLayout) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QFormLayout17hasHeightForWidthEv()};
    let mut ret = unsafe {_ZNK11QFormLayout17hasHeightForWidthEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

// proto:  void QFormLayout::insertRow(int row, QWidget * widget);
impl<'a> /*trait*/ QFormLayout_insertRow for (i32, &'a mut QWidget) {
  fn insertRow(self, rsthis: &mut QFormLayout)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFormLayout9insertRowEiP7QWidget()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN11QFormLayout9insertRowEiP7QWidget(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

// proto:  void QFormLayout::addRow(QWidget * label, QWidget * field);
impl<'a> /*trait*/ QFormLayout_addRow for (&'a mut QWidget, &'a mut QWidget) {
  fn addRow(self, rsthis: &mut QFormLayout)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFormLayout6addRowEP7QWidgetS1_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN11QFormLayout6addRowEP7QWidgetS1_(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

