// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qlayout::QLayout;
use super::qstring::QString;
use super::qwidget::QWidget;
use super::qrect::QRect;
use super::qlayoutitem::QLayoutItem;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: int QFormLayout::horizontalSpacing();
  fn _ZNK11QFormLayout17horizontalSpacingEv() -> i32;
  // proto: int QFormLayout::rowCount();
  fn _ZNK11QFormLayout8rowCountEv() -> i32;
  // proto: QWidget * QFormLayout::labelForField(QLayout * field);
  fn _ZNK11QFormLayout13labelForFieldEP7QLayout(arg0: *mut c_void) -> i32;
  // proto: void QFormLayout::addRow(const QString & labelText, QLayout * field);
  fn _ZN11QFormLayout6addRowERK7QStringP7QLayout(arg0: *const c_void, arg1: *mut c_void) -> i32;
  // proto: void QFormLayout::insertRow(int row, const QString & labelText, QLayout * field);
  fn _ZN11QFormLayout9insertRowEiRK7QStringP7QLayout(arg0: c_int, arg1: *const c_void, arg2: *mut c_void) -> i32;
  // proto: QWidget * QFormLayout::labelForField(QWidget * field);
  fn _ZNK11QFormLayout13labelForFieldEP7QWidget(arg0: *mut c_void) -> i32;
  // proto: void QFormLayout::insertRow(int row, QWidget * label, QLayout * field);
  fn _ZN11QFormLayout9insertRowEiP7QWidgetP7QLayout(arg0: c_int, arg1: *mut c_void, arg2: *mut c_void) -> i32;
  // proto: int QFormLayout::count();
  fn _ZNK11QFormLayout5countEv() -> i32;
  // proto: int QFormLayout::spacing();
  fn _ZNK11QFormLayout7spacingEv() -> i32;
  // proto: void QFormLayout::NewQFormLayout(QWidget * parent);
  fn _ZN11QFormLayoutC1EP7QWidget(qthis: *mut c_void, arg0: *mut c_void) -> i32;
  // proto: void QFormLayout::insertRow(int row, QLayout * layout);
  fn _ZN11QFormLayout9insertRowEiP7QLayout(arg0: c_int, arg1: *mut c_void) -> i32;
  // proto: void QFormLayout::setGeometry(const QRect & rect);
  fn _ZN11QFormLayout11setGeometryERK5QRect(arg0: *const c_void) -> i32;
  // proto: void QFormLayout::setVerticalSpacing(int spacing);
  fn _ZN11QFormLayout18setVerticalSpacingEi(arg0: c_int) -> i32;
  // proto: void QFormLayout::setHorizontalSpacing(int spacing);
  fn _ZN11QFormLayout20setHorizontalSpacingEi(arg0: c_int) -> i32;
  // proto: void QFormLayout::insertRow(int row, const QString & labelText, QWidget * field);
  fn _ZN11QFormLayout9insertRowEiRK7QStringP7QWidget(arg0: c_int, arg1: *const c_void, arg2: *mut c_void) -> i32;
  // proto: const QMetaObject * QFormLayout::metaObject();
  fn _ZNK11QFormLayout10metaObjectEv() -> i32;
  // proto: void QFormLayout::insertRow(int row, QWidget * label, QWidget * field);
  fn _ZN11QFormLayout9insertRowEiP7QWidgetS1_(arg0: c_int, arg1: *mut c_void, arg2: *mut c_void) -> i32;
  // proto: void QFormLayout::setSpacing(int );
  fn _ZN11QFormLayout10setSpacingEi(arg0: c_int) -> i32;
  // proto: void QFormLayout::FreeQFormLayout();
  fn _ZN11QFormLayoutD0Ev() -> i32;
  // proto: void QFormLayout::addRow(QLayout * layout);
  fn _ZN11QFormLayout6addRowEP7QLayout(arg0: *mut c_void) -> i32;
  // proto: QSize QFormLayout::sizeHint();
  fn _ZNK11QFormLayout8sizeHintEv() -> i32;
  // proto: void QFormLayout::invalidate();
  fn _ZN11QFormLayout10invalidateEv() -> i32;
  // proto: QLayoutItem * QFormLayout::itemAt(int index);
  fn _ZNK11QFormLayout6itemAtEi(arg0: c_int) -> i32;
  // proto: QLayoutItem * QFormLayout::takeAt(int index);
  fn _ZN11QFormLayout6takeAtEi(arg0: c_int) -> i32;
  // proto: void QFormLayout::addRow(const QString & labelText, QWidget * field);
  fn _ZN11QFormLayout6addRowERK7QStringP7QWidget(arg0: *const c_void, arg1: *mut c_void) -> i32;
  // proto: QSize QFormLayout::minimumSize();
  fn _ZNK11QFormLayout11minimumSizeEv() -> i32;
  // proto: void QFormLayout::addRow(QWidget * widget);
  fn _ZN11QFormLayout6addRowEP7QWidget(arg0: *mut c_void) -> i32;
  // proto: void QFormLayout::addRow(QWidget * label, QLayout * field);
  fn _ZN11QFormLayout6addRowEP7QWidgetP7QLayout(arg0: *mut c_void, arg1: *mut c_void) -> i32;
  // proto: int QFormLayout::verticalSpacing();
  fn _ZNK11QFormLayout15verticalSpacingEv() -> i32;
  // proto: int QFormLayout::heightForWidth(int width);
  fn _ZNK11QFormLayout14heightForWidthEi(arg0: c_int) -> i32;
  // proto: void QFormLayout::addItem(QLayoutItem * item);
  fn _ZN11QFormLayout7addItemEP11QLayoutItem(arg0: *mut c_void) -> i32;
  // proto: bool QFormLayout::hasHeightForWidth();
  fn _ZNK11QFormLayout17hasHeightForWidthEv() -> i32;
  // proto: void QFormLayout::insertRow(int row, QWidget * widget);
  fn _ZN11QFormLayout9insertRowEiP7QWidget(arg0: c_int, arg1: *mut c_void) -> i32;
  // proto: void QFormLayout::addRow(QWidget * label, QWidget * field);
  fn _ZN11QFormLayout6addRowEP7QWidgetS1_(arg0: *mut c_void, arg1: *mut c_void) -> i32;
}

// body block begin
// class sizeof(QFormLayout)=1
pub struct QFormLayout {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QFormLayout {
  pub fn horizontalSpacing<T: QFormLayout_horizontalSpacing>(&mut self, value: T) -> i32 {
    value.horizontalSpacing(self);
    return 1;
  }
}

pub trait QFormLayout_horizontalSpacing {
  fn horizontalSpacing(self, this: &mut QFormLayout) -> i32;
}

// proto: int QFormLayout::horizontalSpacing();
impl<'a> /*trait*/ QFormLayout_horizontalSpacing for () {
  fn horizontalSpacing(self, this: &mut QFormLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QFormLayout17horizontalSpacingEv()};
    unsafe {_ZNK11QFormLayout17horizontalSpacingEv()};
    return 1;
  }
}

impl /*struct*/ QFormLayout {
  pub fn rowCount<T: QFormLayout_rowCount>(&mut self, value: T) -> i32 {
    value.rowCount(self);
    return 1;
  }
}

pub trait QFormLayout_rowCount {
  fn rowCount(self, this: &mut QFormLayout) -> i32;
}

// proto: int QFormLayout::rowCount();
impl<'a> /*trait*/ QFormLayout_rowCount for () {
  fn rowCount(self, this: &mut QFormLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QFormLayout8rowCountEv()};
    unsafe {_ZNK11QFormLayout8rowCountEv()};
    return 1;
  }
}

impl /*struct*/ QFormLayout {
  pub fn labelForField<T: QFormLayout_labelForField>(&mut self, value: T) -> i32 {
    value.labelForField(self);
    return 1;
  }
}

pub trait QFormLayout_labelForField {
  fn labelForField(self, this: &mut QFormLayout) -> i32;
}

// proto: QWidget * QFormLayout::labelForField(QLayout * field);
impl<'a> /*trait*/ QFormLayout_labelForField for (&'a mut QLayout) {
  fn labelForField(self, this: &mut QFormLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QFormLayout13labelForFieldEP7QLayout()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZNK11QFormLayout13labelForFieldEP7QLayout(arg0)};
    return 1;
  }
}

impl /*struct*/ QFormLayout {
  pub fn addRow<T: QFormLayout_addRow>(&mut self, value: T) -> i32 {
    value.addRow(self);
    return 1;
  }
}

pub trait QFormLayout_addRow {
  fn addRow(self, this: &mut QFormLayout) -> i32;
}

// proto: void QFormLayout::addRow(const QString & labelText, QLayout * field);
impl<'a> /*trait*/ QFormLayout_addRow for (&'a  QString, &'a mut QLayout) {
  fn addRow(self, this: &mut QFormLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFormLayout6addRowERK7QStringP7QLayout()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN11QFormLayout6addRowERK7QStringP7QLayout(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QFormLayout {
  pub fn insertRow<T: QFormLayout_insertRow>(&mut self, value: T) -> i32 {
    value.insertRow(self);
    return 1;
  }
}

pub trait QFormLayout_insertRow {
  fn insertRow(self, this: &mut QFormLayout) -> i32;
}

// proto: void QFormLayout::insertRow(int row, const QString & labelText, QLayout * field);
impl<'a> /*trait*/ QFormLayout_insertRow for (i32, &'a  QString, &'a mut QLayout) {
  fn insertRow(self, this: &mut QFormLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFormLayout9insertRowEiRK7QStringP7QLayout()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *const c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    unsafe {_ZN11QFormLayout9insertRowEiRK7QStringP7QLayout(arg0, arg1, arg2)};
    return 1;
  }
}

// proto: QWidget * QFormLayout::labelForField(QWidget * field);
impl<'a> /*trait*/ QFormLayout_labelForField for (&'a mut QWidget) {
  fn labelForField(self, this: &mut QFormLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QFormLayout13labelForFieldEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZNK11QFormLayout13labelForFieldEP7QWidget(arg0)};
    return 1;
  }
}

// proto: void QFormLayout::insertRow(int row, QWidget * label, QLayout * field);
impl<'a> /*trait*/ QFormLayout_insertRow for (i32, &'a mut QWidget, &'a mut QLayout) {
  fn insertRow(self, this: &mut QFormLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFormLayout9insertRowEiP7QWidgetP7QLayout()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    unsafe {_ZN11QFormLayout9insertRowEiP7QWidgetP7QLayout(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QFormLayout {
  pub fn count<T: QFormLayout_count>(&mut self, value: T) -> i32 {
    value.count(self);
    return 1;
  }
}

pub trait QFormLayout_count {
  fn count(self, this: &mut QFormLayout) -> i32;
}

// proto: int QFormLayout::count();
impl<'a> /*trait*/ QFormLayout_count for () {
  fn count(self, this: &mut QFormLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QFormLayout5countEv()};
    unsafe {_ZNK11QFormLayout5countEv()};
    return 1;
  }
}

impl /*struct*/ QFormLayout {
  pub fn spacing<T: QFormLayout_spacing>(&mut self, value: T) -> i32 {
    value.spacing(self);
    return 1;
  }
}

pub trait QFormLayout_spacing {
  fn spacing(self, this: &mut QFormLayout) -> i32;
}

// proto: int QFormLayout::spacing();
impl<'a> /*trait*/ QFormLayout_spacing for () {
  fn spacing(self, this: &mut QFormLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QFormLayout7spacingEv()};
    unsafe {_ZNK11QFormLayout7spacingEv()};
    return 1;
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

// proto: void QFormLayout::insertRow(int row, QLayout * layout);
impl<'a> /*trait*/ QFormLayout_insertRow for (i32, &'a mut QLayout) {
  fn insertRow(self, this: &mut QFormLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFormLayout9insertRowEiP7QLayout()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN11QFormLayout9insertRowEiP7QLayout(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QFormLayout {
  pub fn setGeometry<T: QFormLayout_setGeometry>(&mut self, value: T) -> i32 {
    value.setGeometry(self);
    return 1;
  }
}

pub trait QFormLayout_setGeometry {
  fn setGeometry(self, this: &mut QFormLayout) -> i32;
}

// proto: void QFormLayout::setGeometry(const QRect & rect);
impl<'a> /*trait*/ QFormLayout_setGeometry for (&'a  QRect) {
  fn setGeometry(self, this: &mut QFormLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFormLayout11setGeometryERK5QRect()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN11QFormLayout11setGeometryERK5QRect(arg0)};
    return 1;
  }
}

impl /*struct*/ QFormLayout {
  pub fn setVerticalSpacing<T: QFormLayout_setVerticalSpacing>(&mut self, value: T) -> i32 {
    value.setVerticalSpacing(self);
    return 1;
  }
}

pub trait QFormLayout_setVerticalSpacing {
  fn setVerticalSpacing(self, this: &mut QFormLayout) -> i32;
}

// proto: void QFormLayout::setVerticalSpacing(int spacing);
impl<'a> /*trait*/ QFormLayout_setVerticalSpacing for (i32) {
  fn setVerticalSpacing(self, this: &mut QFormLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFormLayout18setVerticalSpacingEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN11QFormLayout18setVerticalSpacingEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QFormLayout {
  pub fn setHorizontalSpacing<T: QFormLayout_setHorizontalSpacing>(&mut self, value: T) -> i32 {
    value.setHorizontalSpacing(self);
    return 1;
  }
}

pub trait QFormLayout_setHorizontalSpacing {
  fn setHorizontalSpacing(self, this: &mut QFormLayout) -> i32;
}

// proto: void QFormLayout::setHorizontalSpacing(int spacing);
impl<'a> /*trait*/ QFormLayout_setHorizontalSpacing for (i32) {
  fn setHorizontalSpacing(self, this: &mut QFormLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFormLayout20setHorizontalSpacingEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN11QFormLayout20setHorizontalSpacingEi(arg0)};
    return 1;
  }
}

// proto: void QFormLayout::insertRow(int row, const QString & labelText, QWidget * field);
impl<'a> /*trait*/ QFormLayout_insertRow for (i32, &'a  QString, &'a mut QWidget) {
  fn insertRow(self, this: &mut QFormLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFormLayout9insertRowEiRK7QStringP7QWidget()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *const c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    unsafe {_ZN11QFormLayout9insertRowEiRK7QStringP7QWidget(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QFormLayout {
  pub fn metaObject<T: QFormLayout_metaObject>(&mut self, value: T) -> i32 {
    value.metaObject(self);
    return 1;
  }
}

pub trait QFormLayout_metaObject {
  fn metaObject(self, this: &mut QFormLayout) -> i32;
}

// proto: const QMetaObject * QFormLayout::metaObject();
impl<'a> /*trait*/ QFormLayout_metaObject for () {
  fn metaObject(self, this: &mut QFormLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QFormLayout10metaObjectEv()};
    unsafe {_ZNK11QFormLayout10metaObjectEv()};
    return 1;
  }
}

// proto: void QFormLayout::insertRow(int row, QWidget * label, QWidget * field);
impl<'a> /*trait*/ QFormLayout_insertRow for (i32, &'a mut QWidget, &'a mut QWidget) {
  fn insertRow(self, this: &mut QFormLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFormLayout9insertRowEiP7QWidgetS1_()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    unsafe {_ZN11QFormLayout9insertRowEiP7QWidgetS1_(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QFormLayout {
  pub fn setSpacing<T: QFormLayout_setSpacing>(&mut self, value: T) -> i32 {
    value.setSpacing(self);
    return 1;
  }
}

pub trait QFormLayout_setSpacing {
  fn setSpacing(self, this: &mut QFormLayout) -> i32;
}

// proto: void QFormLayout::setSpacing(int );
impl<'a> /*trait*/ QFormLayout_setSpacing for (i32) {
  fn setSpacing(self, this: &mut QFormLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFormLayout10setSpacingEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN11QFormLayout10setSpacingEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QFormLayout {
  pub fn FreeQFormLayout<T: QFormLayout_FreeQFormLayout>(&mut self, value: T) -> i32 {
    value.FreeQFormLayout(self);
    return 1;
  }
}

pub trait QFormLayout_FreeQFormLayout {
  fn FreeQFormLayout(self, this: &mut QFormLayout) -> i32;
}

// proto: void QFormLayout::FreeQFormLayout();
impl<'a> /*trait*/ QFormLayout_FreeQFormLayout for () {
  fn FreeQFormLayout(self, this: &mut QFormLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFormLayoutD0Ev()};
    unsafe {_ZN11QFormLayoutD0Ev()};
    return 1;
  }
}

// proto: void QFormLayout::addRow(QLayout * layout);
impl<'a> /*trait*/ QFormLayout_addRow for (&'a mut QLayout) {
  fn addRow(self, this: &mut QFormLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFormLayout6addRowEP7QLayout()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN11QFormLayout6addRowEP7QLayout(arg0)};
    return 1;
  }
}

impl /*struct*/ QFormLayout {
  pub fn sizeHint<T: QFormLayout_sizeHint>(&mut self, value: T) -> i32 {
    value.sizeHint(self);
    return 1;
  }
}

pub trait QFormLayout_sizeHint {
  fn sizeHint(self, this: &mut QFormLayout) -> i32;
}

// proto: QSize QFormLayout::sizeHint();
impl<'a> /*trait*/ QFormLayout_sizeHint for () {
  fn sizeHint(self, this: &mut QFormLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QFormLayout8sizeHintEv()};
    unsafe {_ZNK11QFormLayout8sizeHintEv()};
    return 1;
  }
}

impl /*struct*/ QFormLayout {
  pub fn invalidate<T: QFormLayout_invalidate>(&mut self, value: T) -> i32 {
    value.invalidate(self);
    return 1;
  }
}

pub trait QFormLayout_invalidate {
  fn invalidate(self, this: &mut QFormLayout) -> i32;
}

// proto: void QFormLayout::invalidate();
impl<'a> /*trait*/ QFormLayout_invalidate for () {
  fn invalidate(self, this: &mut QFormLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFormLayout10invalidateEv()};
    unsafe {_ZN11QFormLayout10invalidateEv()};
    return 1;
  }
}

impl /*struct*/ QFormLayout {
  pub fn itemAt<T: QFormLayout_itemAt>(&mut self, value: T) -> i32 {
    value.itemAt(self);
    return 1;
  }
}

pub trait QFormLayout_itemAt {
  fn itemAt(self, this: &mut QFormLayout) -> i32;
}

// proto: QLayoutItem * QFormLayout::itemAt(int index);
impl<'a> /*trait*/ QFormLayout_itemAt for (i32) {
  fn itemAt(self, this: &mut QFormLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QFormLayout6itemAtEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK11QFormLayout6itemAtEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QFormLayout {
  pub fn takeAt<T: QFormLayout_takeAt>(&mut self, value: T) -> i32 {
    value.takeAt(self);
    return 1;
  }
}

pub trait QFormLayout_takeAt {
  fn takeAt(self, this: &mut QFormLayout) -> i32;
}

// proto: QLayoutItem * QFormLayout::takeAt(int index);
impl<'a> /*trait*/ QFormLayout_takeAt for (i32) {
  fn takeAt(self, this: &mut QFormLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFormLayout6takeAtEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN11QFormLayout6takeAtEi(arg0)};
    return 1;
  }
}

// proto: void QFormLayout::addRow(const QString & labelText, QWidget * field);
impl<'a> /*trait*/ QFormLayout_addRow for (&'a  QString, &'a mut QWidget) {
  fn addRow(self, this: &mut QFormLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFormLayout6addRowERK7QStringP7QWidget()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN11QFormLayout6addRowERK7QStringP7QWidget(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QFormLayout {
  pub fn minimumSize<T: QFormLayout_minimumSize>(&mut self, value: T) -> i32 {
    value.minimumSize(self);
    return 1;
  }
}

pub trait QFormLayout_minimumSize {
  fn minimumSize(self, this: &mut QFormLayout) -> i32;
}

// proto: QSize QFormLayout::minimumSize();
impl<'a> /*trait*/ QFormLayout_minimumSize for () {
  fn minimumSize(self, this: &mut QFormLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QFormLayout11minimumSizeEv()};
    unsafe {_ZNK11QFormLayout11minimumSizeEv()};
    return 1;
  }
}

// proto: void QFormLayout::addRow(QWidget * widget);
impl<'a> /*trait*/ QFormLayout_addRow for (&'a mut QWidget) {
  fn addRow(self, this: &mut QFormLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFormLayout6addRowEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN11QFormLayout6addRowEP7QWidget(arg0)};
    return 1;
  }
}

// proto: void QFormLayout::addRow(QWidget * label, QLayout * field);
impl<'a> /*trait*/ QFormLayout_addRow for (&'a mut QWidget, &'a mut QLayout) {
  fn addRow(self, this: &mut QFormLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFormLayout6addRowEP7QWidgetP7QLayout()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN11QFormLayout6addRowEP7QWidgetP7QLayout(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QFormLayout {
  pub fn verticalSpacing<T: QFormLayout_verticalSpacing>(&mut self, value: T) -> i32 {
    value.verticalSpacing(self);
    return 1;
  }
}

pub trait QFormLayout_verticalSpacing {
  fn verticalSpacing(self, this: &mut QFormLayout) -> i32;
}

// proto: int QFormLayout::verticalSpacing();
impl<'a> /*trait*/ QFormLayout_verticalSpacing for () {
  fn verticalSpacing(self, this: &mut QFormLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QFormLayout15verticalSpacingEv()};
    unsafe {_ZNK11QFormLayout15verticalSpacingEv()};
    return 1;
  }
}

impl /*struct*/ QFormLayout {
  pub fn heightForWidth<T: QFormLayout_heightForWidth>(&mut self, value: T) -> i32 {
    value.heightForWidth(self);
    return 1;
  }
}

pub trait QFormLayout_heightForWidth {
  fn heightForWidth(self, this: &mut QFormLayout) -> i32;
}

// proto: int QFormLayout::heightForWidth(int width);
impl<'a> /*trait*/ QFormLayout_heightForWidth for (i32) {
  fn heightForWidth(self, this: &mut QFormLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QFormLayout14heightForWidthEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK11QFormLayout14heightForWidthEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QFormLayout {
  pub fn addItem<T: QFormLayout_addItem>(&mut self, value: T) -> i32 {
    value.addItem(self);
    return 1;
  }
}

pub trait QFormLayout_addItem {
  fn addItem(self, this: &mut QFormLayout) -> i32;
}

// proto: void QFormLayout::addItem(QLayoutItem * item);
impl<'a> /*trait*/ QFormLayout_addItem for (&'a mut QLayoutItem) {
  fn addItem(self, this: &mut QFormLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFormLayout7addItemEP11QLayoutItem()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN11QFormLayout7addItemEP11QLayoutItem(arg0)};
    return 1;
  }
}

impl /*struct*/ QFormLayout {
  pub fn hasHeightForWidth<T: QFormLayout_hasHeightForWidth>(&mut self, value: T) -> i32 {
    value.hasHeightForWidth(self);
    return 1;
  }
}

pub trait QFormLayout_hasHeightForWidth {
  fn hasHeightForWidth(self, this: &mut QFormLayout) -> i32;
}

// proto: bool QFormLayout::hasHeightForWidth();
impl<'a> /*trait*/ QFormLayout_hasHeightForWidth for () {
  fn hasHeightForWidth(self, this: &mut QFormLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QFormLayout17hasHeightForWidthEv()};
    unsafe {_ZNK11QFormLayout17hasHeightForWidthEv()};
    return 1;
  }
}

// proto: void QFormLayout::insertRow(int row, QWidget * widget);
impl<'a> /*trait*/ QFormLayout_insertRow for (i32, &'a mut QWidget) {
  fn insertRow(self, this: &mut QFormLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFormLayout9insertRowEiP7QWidget()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN11QFormLayout9insertRowEiP7QWidget(arg0, arg1)};
    return 1;
  }
}

// proto: void QFormLayout::addRow(QWidget * label, QWidget * field);
impl<'a> /*trait*/ QFormLayout_addRow for (&'a mut QWidget, &'a mut QWidget) {
  fn addRow(self, this: &mut QFormLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFormLayout6addRowEP7QWidgetS1_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN11QFormLayout6addRowEP7QWidgetS1_(arg0, arg1)};
    return 1;
  }
}

