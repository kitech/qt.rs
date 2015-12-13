// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qstring::QString;
use super::qsize::QSize;
use super::qdatastream::QDataStream;
use super::qicon::QIcon;
use super::qbrush::QBrush;
use super::qvariant::QVariant;
use super::qcolor::QColor;
use super::qfont::QFont;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: QColor QTableWidgetItem::backgroundColor();
  fn _ZNK16QTableWidgetItem15backgroundColorEv() -> i32;
  // proto: QVariant QTableWidgetItem::data(int role);
  fn _ZNK16QTableWidgetItem4dataEi(arg0: c_int) -> i32;
  // proto: void QTableWidgetItem::setSelected(bool select);
  fn _ZN16QTableWidgetItem11setSelectedEb(arg0: int8_t) -> i32;
  // proto: void QTableWidgetItem::setStatusTip(const QString & statusTip);
  fn _ZN16QTableWidgetItem12setStatusTipERK7QString(arg0: *const c_void) -> i32;
  // proto: QColor QTableWidgetItem::textColor();
  fn _ZNK16QTableWidgetItem9textColorEv() -> i32;
  // proto: void QTableWidgetItem::FreeQTableWidgetItem();
  fn _ZN16QTableWidgetItemD0Ev() -> i32;
  // proto: QString QTableWidgetItem::text();
  fn _ZNK16QTableWidgetItem4textEv() -> i32;
  // proto: void QTableWidgetItem::setSizeHint(const QSize & size);
  fn _ZN16QTableWidgetItem11setSizeHintERK5QSize(arg0: *const c_void) -> i32;
  // proto: QBrush QTableWidgetItem::foreground();
  fn _ZNK16QTableWidgetItem10foregroundEv() -> i32;
  // proto: int QTableWidgetItem::type_();
  fn _ZNK16QTableWidgetItem4typeEv() -> i32;
  // proto: int QTableWidgetItem::column();
  fn _ZNK16QTableWidgetItem6columnEv() -> i32;
  // proto: void QTableWidgetItem::setTextAlignment(int alignment);
  fn _ZN16QTableWidgetItem16setTextAlignmentEi(arg0: c_int) -> i32;
  // proto: QFont QTableWidgetItem::font();
  fn _ZNK16QTableWidgetItem4fontEv() -> i32;
  // proto: QIcon QTableWidgetItem::icon();
  fn _ZNK16QTableWidgetItem4iconEv() -> i32;
  // proto: void QTableWidgetItem::write(QDataStream & out);
  fn _ZNK16QTableWidgetItem5writeER11QDataStream(arg0: *mut c_void) -> i32;
  // proto: void QTableWidgetItem::NewQTableWidgetItem(const QTableWidgetItem & other);
  fn _ZN16QTableWidgetItemC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: QBrush QTableWidgetItem::background();
  fn _ZNK16QTableWidgetItem10backgroundEv() -> i32;
  // proto: void QTableWidgetItem::setIcon(const QIcon & icon);
  fn _ZN16QTableWidgetItem7setIconERK5QIcon(arg0: *const c_void) -> i32;
  // proto: void QTableWidgetItem::NewQTableWidgetItem(const QString & text, int type);
  fn _ZN16QTableWidgetItemC1ERK7QStringi(qthis: *mut c_void, arg0: *const c_void, arg1: c_int) -> i32;
  // proto: QString QTableWidgetItem::statusTip();
  fn _ZNK16QTableWidgetItem9statusTipEv() -> i32;
  // proto: QTableWidgetItem * QTableWidgetItem::clone();
  fn _ZNK16QTableWidgetItem5cloneEv() -> i32;
  // proto: void QTableWidgetItem::NewQTableWidgetItem(int type);
  fn _ZN16QTableWidgetItemC1Ei(qthis: *mut c_void, arg0: c_int) -> i32;
  // proto: void QTableWidgetItem::setWhatsThis(const QString & whatsThis);
  fn _ZN16QTableWidgetItem12setWhatsThisERK7QString(arg0: *const c_void) -> i32;
  // proto: QSize QTableWidgetItem::sizeHint();
  fn _ZNK16QTableWidgetItem8sizeHintEv() -> i32;
  // proto: void QTableWidgetItem::setForeground(const QBrush & brush);
  fn _ZN16QTableWidgetItem13setForegroundERK6QBrush(arg0: *const c_void) -> i32;
  // proto: int QTableWidgetItem::row();
  fn _ZNK16QTableWidgetItem3rowEv() -> i32;
  // proto: void QTableWidgetItem::setData(int role, const QVariant & value);
  fn _ZN16QTableWidgetItem7setDataEiRK8QVariant(arg0: c_int, arg1: *const c_void) -> i32;
  // proto: QTableWidget * QTableWidgetItem::tableWidget();
  fn _ZNK16QTableWidgetItem11tableWidgetEv() -> i32;
  // proto: void QTableWidgetItem::NewQTableWidgetItem(const QIcon & icon, const QString & text, int type);
  fn _ZN16QTableWidgetItemC1ERK5QIconRK7QStringi(qthis: *mut c_void, arg0: *const c_void, arg1: *const c_void, arg2: c_int) -> i32;
  // proto: int QTableWidgetItem::textAlignment();
  fn _ZNK16QTableWidgetItem13textAlignmentEv() -> i32;
  // proto: void QTableWidgetItem::read(QDataStream & in);
  fn _ZN16QTableWidgetItem4readER11QDataStream(arg0: *mut c_void) -> i32;
  // proto: QString QTableWidgetItem::toolTip();
  fn _ZNK16QTableWidgetItem7toolTipEv() -> i32;
  // proto: bool QTableWidgetItem::isSelected();
  fn _ZNK16QTableWidgetItem10isSelectedEv() -> i32;
  // proto: void QTableWidgetItem::setBackgroundColor(const QColor & color);
  fn _ZN16QTableWidgetItem18setBackgroundColorERK6QColor(arg0: *const c_void) -> i32;
  // proto: void QTableWidgetItem::setBackground(const QBrush & brush);
  fn _ZN16QTableWidgetItem13setBackgroundERK6QBrush(arg0: *const c_void) -> i32;
  // proto: void QTableWidgetItem::setFont(const QFont & font);
  fn _ZN16QTableWidgetItem7setFontERK5QFont(arg0: *const c_void) -> i32;
  // proto: void QTableWidgetItem::setTextColor(const QColor & color);
  fn _ZN16QTableWidgetItem12setTextColorERK6QColor(arg0: *const c_void) -> i32;
  // proto: void QTableWidgetItem::setText(const QString & text);
  fn _ZN16QTableWidgetItem7setTextERK7QString(arg0: *const c_void) -> i32;
  // proto: QString QTableWidgetItem::whatsThis();
  fn _ZNK16QTableWidgetItem9whatsThisEv() -> i32;
  // proto: void QTableWidgetItem::setToolTip(const QString & toolTip);
  fn _ZN16QTableWidgetItem10setToolTipERK7QString(arg0: *const c_void) -> i32;
}

// body block begin
// class sizeof(QTableWidgetItem)=1
pub struct QTableWidgetItem {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QTableWidgetItem {
  pub fn backgroundColor<T: QTableWidgetItem_backgroundColor>(&mut self, value: T) -> i32 {
    value.backgroundColor(self);
    return 1;
  }
}

pub trait QTableWidgetItem_backgroundColor {
  fn backgroundColor(self, this: &mut QTableWidgetItem) -> i32;
}

// proto: QColor QTableWidgetItem::backgroundColor();
impl<'a> /*trait*/ QTableWidgetItem_backgroundColor for () {
  fn backgroundColor(self, this: &mut QTableWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTableWidgetItem15backgroundColorEv()};
    unsafe {_ZNK16QTableWidgetItem15backgroundColorEv()};
    return 1;
  }
}

impl /*struct*/ QTableWidgetItem {
  pub fn data<T: QTableWidgetItem_data>(&mut self, value: T) -> i32 {
    value.data(self);
    return 1;
  }
}

pub trait QTableWidgetItem_data {
  fn data(self, this: &mut QTableWidgetItem) -> i32;
}

// proto: QVariant QTableWidgetItem::data(int role);
impl<'a> /*trait*/ QTableWidgetItem_data for (i32) {
  fn data(self, this: &mut QTableWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTableWidgetItem4dataEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK16QTableWidgetItem4dataEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QTableWidgetItem {
  pub fn setSelected<T: QTableWidgetItem_setSelected>(&mut self, value: T) -> i32 {
    value.setSelected(self);
    return 1;
  }
}

pub trait QTableWidgetItem_setSelected {
  fn setSelected(self, this: &mut QTableWidgetItem) -> i32;
}

// proto: void QTableWidgetItem::setSelected(bool select);
impl<'a> /*trait*/ QTableWidgetItem_setSelected for (i8) {
  fn setSelected(self, this: &mut QTableWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTableWidgetItem11setSelectedEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN16QTableWidgetItem11setSelectedEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QTableWidgetItem {
  pub fn setStatusTip<T: QTableWidgetItem_setStatusTip>(&mut self, value: T) -> i32 {
    value.setStatusTip(self);
    return 1;
  }
}

pub trait QTableWidgetItem_setStatusTip {
  fn setStatusTip(self, this: &mut QTableWidgetItem) -> i32;
}

// proto: void QTableWidgetItem::setStatusTip(const QString & statusTip);
impl<'a> /*trait*/ QTableWidgetItem_setStatusTip for (&'a  QString) {
  fn setStatusTip(self, this: &mut QTableWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTableWidgetItem12setStatusTipERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN16QTableWidgetItem12setStatusTipERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QTableWidgetItem {
  pub fn textColor<T: QTableWidgetItem_textColor>(&mut self, value: T) -> i32 {
    value.textColor(self);
    return 1;
  }
}

pub trait QTableWidgetItem_textColor {
  fn textColor(self, this: &mut QTableWidgetItem) -> i32;
}

// proto: QColor QTableWidgetItem::textColor();
impl<'a> /*trait*/ QTableWidgetItem_textColor for () {
  fn textColor(self, this: &mut QTableWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTableWidgetItem9textColorEv()};
    unsafe {_ZNK16QTableWidgetItem9textColorEv()};
    return 1;
  }
}

impl /*struct*/ QTableWidgetItem {
  pub fn FreeQTableWidgetItem<T: QTableWidgetItem_FreeQTableWidgetItem>(&mut self, value: T) -> i32 {
    value.FreeQTableWidgetItem(self);
    return 1;
  }
}

pub trait QTableWidgetItem_FreeQTableWidgetItem {
  fn FreeQTableWidgetItem(self, this: &mut QTableWidgetItem) -> i32;
}

// proto: void QTableWidgetItem::FreeQTableWidgetItem();
impl<'a> /*trait*/ QTableWidgetItem_FreeQTableWidgetItem for () {
  fn FreeQTableWidgetItem(self, this: &mut QTableWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTableWidgetItemD0Ev()};
    unsafe {_ZN16QTableWidgetItemD0Ev()};
    return 1;
  }
}

impl /*struct*/ QTableWidgetItem {
  pub fn text<T: QTableWidgetItem_text>(&mut self, value: T) -> i32 {
    value.text(self);
    return 1;
  }
}

pub trait QTableWidgetItem_text {
  fn text(self, this: &mut QTableWidgetItem) -> i32;
}

// proto: QString QTableWidgetItem::text();
impl<'a> /*trait*/ QTableWidgetItem_text for () {
  fn text(self, this: &mut QTableWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTableWidgetItem4textEv()};
    unsafe {_ZNK16QTableWidgetItem4textEv()};
    return 1;
  }
}

impl /*struct*/ QTableWidgetItem {
  pub fn setSizeHint<T: QTableWidgetItem_setSizeHint>(&mut self, value: T) -> i32 {
    value.setSizeHint(self);
    return 1;
  }
}

pub trait QTableWidgetItem_setSizeHint {
  fn setSizeHint(self, this: &mut QTableWidgetItem) -> i32;
}

// proto: void QTableWidgetItem::setSizeHint(const QSize & size);
impl<'a> /*trait*/ QTableWidgetItem_setSizeHint for (&'a  QSize) {
  fn setSizeHint(self, this: &mut QTableWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTableWidgetItem11setSizeHintERK5QSize()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN16QTableWidgetItem11setSizeHintERK5QSize(arg0)};
    return 1;
  }
}

impl /*struct*/ QTableWidgetItem {
  pub fn foreground<T: QTableWidgetItem_foreground>(&mut self, value: T) -> i32 {
    value.foreground(self);
    return 1;
  }
}

pub trait QTableWidgetItem_foreground {
  fn foreground(self, this: &mut QTableWidgetItem) -> i32;
}

// proto: QBrush QTableWidgetItem::foreground();
impl<'a> /*trait*/ QTableWidgetItem_foreground for () {
  fn foreground(self, this: &mut QTableWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTableWidgetItem10foregroundEv()};
    unsafe {_ZNK16QTableWidgetItem10foregroundEv()};
    return 1;
  }
}

impl /*struct*/ QTableWidgetItem {
  pub fn type_<T: QTableWidgetItem_type_>(&mut self, value: T) -> i32 {
    value.type_(self);
    return 1;
  }
}

pub trait QTableWidgetItem_type_ {
  fn type_(self, this: &mut QTableWidgetItem) -> i32;
}

// proto: int QTableWidgetItem::type_();
impl<'a> /*trait*/ QTableWidgetItem_type_ for () {
  fn type_(self, this: &mut QTableWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTableWidgetItem4typeEv()};
    unsafe {_ZNK16QTableWidgetItem4typeEv()};
    return 1;
  }
}

impl /*struct*/ QTableWidgetItem {
  pub fn column<T: QTableWidgetItem_column>(&mut self, value: T) -> i32 {
    value.column(self);
    return 1;
  }
}

pub trait QTableWidgetItem_column {
  fn column(self, this: &mut QTableWidgetItem) -> i32;
}

// proto: int QTableWidgetItem::column();
impl<'a> /*trait*/ QTableWidgetItem_column for () {
  fn column(self, this: &mut QTableWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTableWidgetItem6columnEv()};
    unsafe {_ZNK16QTableWidgetItem6columnEv()};
    return 1;
  }
}

impl /*struct*/ QTableWidgetItem {
  pub fn setTextAlignment<T: QTableWidgetItem_setTextAlignment>(&mut self, value: T) -> i32 {
    value.setTextAlignment(self);
    return 1;
  }
}

pub trait QTableWidgetItem_setTextAlignment {
  fn setTextAlignment(self, this: &mut QTableWidgetItem) -> i32;
}

// proto: void QTableWidgetItem::setTextAlignment(int alignment);
impl<'a> /*trait*/ QTableWidgetItem_setTextAlignment for (i32) {
  fn setTextAlignment(self, this: &mut QTableWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTableWidgetItem16setTextAlignmentEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN16QTableWidgetItem16setTextAlignmentEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QTableWidgetItem {
  pub fn font<T: QTableWidgetItem_font>(&mut self, value: T) -> i32 {
    value.font(self);
    return 1;
  }
}

pub trait QTableWidgetItem_font {
  fn font(self, this: &mut QTableWidgetItem) -> i32;
}

// proto: QFont QTableWidgetItem::font();
impl<'a> /*trait*/ QTableWidgetItem_font for () {
  fn font(self, this: &mut QTableWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTableWidgetItem4fontEv()};
    unsafe {_ZNK16QTableWidgetItem4fontEv()};
    return 1;
  }
}

impl /*struct*/ QTableWidgetItem {
  pub fn icon<T: QTableWidgetItem_icon>(&mut self, value: T) -> i32 {
    value.icon(self);
    return 1;
  }
}

pub trait QTableWidgetItem_icon {
  fn icon(self, this: &mut QTableWidgetItem) -> i32;
}

// proto: QIcon QTableWidgetItem::icon();
impl<'a> /*trait*/ QTableWidgetItem_icon for () {
  fn icon(self, this: &mut QTableWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTableWidgetItem4iconEv()};
    unsafe {_ZNK16QTableWidgetItem4iconEv()};
    return 1;
  }
}

impl /*struct*/ QTableWidgetItem {
  pub fn write<T: QTableWidgetItem_write>(&mut self, value: T) -> i32 {
    value.write(self);
    return 1;
  }
}

pub trait QTableWidgetItem_write {
  fn write(self, this: &mut QTableWidgetItem) -> i32;
}

// proto: void QTableWidgetItem::write(QDataStream & out);
impl<'a> /*trait*/ QTableWidgetItem_write for (&'a mut QDataStream) {
  fn write(self, this: &mut QTableWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTableWidgetItem5writeER11QDataStream()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZNK16QTableWidgetItem5writeER11QDataStream(arg0)};
    return 1;
  }
}

impl /*struct*/ QTableWidgetItem {
  pub fn NewQTableWidgetItem<T: QTableWidgetItem_NewQTableWidgetItem>(value: T) -> QTableWidgetItem {
    let rsthis = value.NewQTableWidgetItem();
    return rsthis;
    // return 1;
  }
}

pub trait QTableWidgetItem_NewQTableWidgetItem {
  fn NewQTableWidgetItem(self) -> QTableWidgetItem;
}

// proto: void QTableWidgetItem::NewQTableWidgetItem(const QTableWidgetItem & other);
impl<'a> /*trait*/ QTableWidgetItem_NewQTableWidgetItem for (&'a  QTableWidgetItem) {
  fn NewQTableWidgetItem(self) -> QTableWidgetItem {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTableWidgetItemC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN16QTableWidgetItemC1ERKS_(qthis, arg0)};
    let rsthis = QTableWidgetItem{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QTableWidgetItem {
  pub fn background<T: QTableWidgetItem_background>(&mut self, value: T) -> i32 {
    value.background(self);
    return 1;
  }
}

pub trait QTableWidgetItem_background {
  fn background(self, this: &mut QTableWidgetItem) -> i32;
}

// proto: QBrush QTableWidgetItem::background();
impl<'a> /*trait*/ QTableWidgetItem_background for () {
  fn background(self, this: &mut QTableWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTableWidgetItem10backgroundEv()};
    unsafe {_ZNK16QTableWidgetItem10backgroundEv()};
    return 1;
  }
}

impl /*struct*/ QTableWidgetItem {
  pub fn setIcon<T: QTableWidgetItem_setIcon>(&mut self, value: T) -> i32 {
    value.setIcon(self);
    return 1;
  }
}

pub trait QTableWidgetItem_setIcon {
  fn setIcon(self, this: &mut QTableWidgetItem) -> i32;
}

// proto: void QTableWidgetItem::setIcon(const QIcon & icon);
impl<'a> /*trait*/ QTableWidgetItem_setIcon for (&'a  QIcon) {
  fn setIcon(self, this: &mut QTableWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTableWidgetItem7setIconERK5QIcon()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN16QTableWidgetItem7setIconERK5QIcon(arg0)};
    return 1;
  }
}

// proto: void QTableWidgetItem::NewQTableWidgetItem(const QString & text, int type);
impl<'a> /*trait*/ QTableWidgetItem_NewQTableWidgetItem for (&'a  QString, i32) {
  fn NewQTableWidgetItem(self) -> QTableWidgetItem {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTableWidgetItemC1ERK7QStringi()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1  as c_int;
    unsafe {_ZN16QTableWidgetItemC1ERK7QStringi(qthis, arg0, arg1)};
    let rsthis = QTableWidgetItem{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QTableWidgetItem {
  pub fn statusTip<T: QTableWidgetItem_statusTip>(&mut self, value: T) -> i32 {
    value.statusTip(self);
    return 1;
  }
}

pub trait QTableWidgetItem_statusTip {
  fn statusTip(self, this: &mut QTableWidgetItem) -> i32;
}

// proto: QString QTableWidgetItem::statusTip();
impl<'a> /*trait*/ QTableWidgetItem_statusTip for () {
  fn statusTip(self, this: &mut QTableWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTableWidgetItem9statusTipEv()};
    unsafe {_ZNK16QTableWidgetItem9statusTipEv()};
    return 1;
  }
}

impl /*struct*/ QTableWidgetItem {
  pub fn clone<T: QTableWidgetItem_clone>(&mut self, value: T) -> i32 {
    value.clone(self);
    return 1;
  }
}

pub trait QTableWidgetItem_clone {
  fn clone(self, this: &mut QTableWidgetItem) -> i32;
}

// proto: QTableWidgetItem * QTableWidgetItem::clone();
impl<'a> /*trait*/ QTableWidgetItem_clone for () {
  fn clone(self, this: &mut QTableWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTableWidgetItem5cloneEv()};
    unsafe {_ZNK16QTableWidgetItem5cloneEv()};
    return 1;
  }
}

// proto: void QTableWidgetItem::NewQTableWidgetItem(int type);
impl<'a> /*trait*/ QTableWidgetItem_NewQTableWidgetItem for (i32) {
  fn NewQTableWidgetItem(self) -> QTableWidgetItem {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTableWidgetItemC1Ei()};
    let arg0 = self  as c_int;
    unsafe {_ZN16QTableWidgetItemC1Ei(qthis, arg0)};
    let rsthis = QTableWidgetItem{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QTableWidgetItem {
  pub fn setWhatsThis<T: QTableWidgetItem_setWhatsThis>(&mut self, value: T) -> i32 {
    value.setWhatsThis(self);
    return 1;
  }
}

pub trait QTableWidgetItem_setWhatsThis {
  fn setWhatsThis(self, this: &mut QTableWidgetItem) -> i32;
}

// proto: void QTableWidgetItem::setWhatsThis(const QString & whatsThis);
impl<'a> /*trait*/ QTableWidgetItem_setWhatsThis for (&'a  QString) {
  fn setWhatsThis(self, this: &mut QTableWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTableWidgetItem12setWhatsThisERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN16QTableWidgetItem12setWhatsThisERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QTableWidgetItem {
  pub fn sizeHint<T: QTableWidgetItem_sizeHint>(&mut self, value: T) -> i32 {
    value.sizeHint(self);
    return 1;
  }
}

pub trait QTableWidgetItem_sizeHint {
  fn sizeHint(self, this: &mut QTableWidgetItem) -> i32;
}

// proto: QSize QTableWidgetItem::sizeHint();
impl<'a> /*trait*/ QTableWidgetItem_sizeHint for () {
  fn sizeHint(self, this: &mut QTableWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTableWidgetItem8sizeHintEv()};
    unsafe {_ZNK16QTableWidgetItem8sizeHintEv()};
    return 1;
  }
}

impl /*struct*/ QTableWidgetItem {
  pub fn setForeground<T: QTableWidgetItem_setForeground>(&mut self, value: T) -> i32 {
    value.setForeground(self);
    return 1;
  }
}

pub trait QTableWidgetItem_setForeground {
  fn setForeground(self, this: &mut QTableWidgetItem) -> i32;
}

// proto: void QTableWidgetItem::setForeground(const QBrush & brush);
impl<'a> /*trait*/ QTableWidgetItem_setForeground for (&'a  QBrush) {
  fn setForeground(self, this: &mut QTableWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTableWidgetItem13setForegroundERK6QBrush()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN16QTableWidgetItem13setForegroundERK6QBrush(arg0)};
    return 1;
  }
}

impl /*struct*/ QTableWidgetItem {
  pub fn row<T: QTableWidgetItem_row>(&mut self, value: T) -> i32 {
    value.row(self);
    return 1;
  }
}

pub trait QTableWidgetItem_row {
  fn row(self, this: &mut QTableWidgetItem) -> i32;
}

// proto: int QTableWidgetItem::row();
impl<'a> /*trait*/ QTableWidgetItem_row for () {
  fn row(self, this: &mut QTableWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTableWidgetItem3rowEv()};
    unsafe {_ZNK16QTableWidgetItem3rowEv()};
    return 1;
  }
}

impl /*struct*/ QTableWidgetItem {
  pub fn setData<T: QTableWidgetItem_setData>(&mut self, value: T) -> i32 {
    value.setData(self);
    return 1;
  }
}

pub trait QTableWidgetItem_setData {
  fn setData(self, this: &mut QTableWidgetItem) -> i32;
}

// proto: void QTableWidgetItem::setData(int role, const QVariant & value);
impl<'a> /*trait*/ QTableWidgetItem_setData for (i32, &'a  QVariant) {
  fn setData(self, this: &mut QTableWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTableWidgetItem7setDataEiRK8QVariant()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN16QTableWidgetItem7setDataEiRK8QVariant(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QTableWidgetItem {
  pub fn tableWidget<T: QTableWidgetItem_tableWidget>(&mut self, value: T) -> i32 {
    value.tableWidget(self);
    return 1;
  }
}

pub trait QTableWidgetItem_tableWidget {
  fn tableWidget(self, this: &mut QTableWidgetItem) -> i32;
}

// proto: QTableWidget * QTableWidgetItem::tableWidget();
impl<'a> /*trait*/ QTableWidgetItem_tableWidget for () {
  fn tableWidget(self, this: &mut QTableWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTableWidgetItem11tableWidgetEv()};
    unsafe {_ZNK16QTableWidgetItem11tableWidgetEv()};
    return 1;
  }
}

// proto: void QTableWidgetItem::NewQTableWidgetItem(const QIcon & icon, const QString & text, int type);
impl<'a> /*trait*/ QTableWidgetItem_NewQTableWidgetItem for (&'a  QIcon, &'a  QString, i32) {
  fn NewQTableWidgetItem(self) -> QTableWidgetItem {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTableWidgetItemC1ERK5QIconRK7QStringi()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    let arg2 = self.2  as c_int;
    unsafe {_ZN16QTableWidgetItemC1ERK5QIconRK7QStringi(qthis, arg0, arg1, arg2)};
    let rsthis = QTableWidgetItem{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QTableWidgetItem {
  pub fn textAlignment<T: QTableWidgetItem_textAlignment>(&mut self, value: T) -> i32 {
    value.textAlignment(self);
    return 1;
  }
}

pub trait QTableWidgetItem_textAlignment {
  fn textAlignment(self, this: &mut QTableWidgetItem) -> i32;
}

// proto: int QTableWidgetItem::textAlignment();
impl<'a> /*trait*/ QTableWidgetItem_textAlignment for () {
  fn textAlignment(self, this: &mut QTableWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTableWidgetItem13textAlignmentEv()};
    unsafe {_ZNK16QTableWidgetItem13textAlignmentEv()};
    return 1;
  }
}

impl /*struct*/ QTableWidgetItem {
  pub fn read<T: QTableWidgetItem_read>(&mut self, value: T) -> i32 {
    value.read(self);
    return 1;
  }
}

pub trait QTableWidgetItem_read {
  fn read(self, this: &mut QTableWidgetItem) -> i32;
}

// proto: void QTableWidgetItem::read(QDataStream & in);
impl<'a> /*trait*/ QTableWidgetItem_read for (&'a mut QDataStream) {
  fn read(self, this: &mut QTableWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTableWidgetItem4readER11QDataStream()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN16QTableWidgetItem4readER11QDataStream(arg0)};
    return 1;
  }
}

impl /*struct*/ QTableWidgetItem {
  pub fn toolTip<T: QTableWidgetItem_toolTip>(&mut self, value: T) -> i32 {
    value.toolTip(self);
    return 1;
  }
}

pub trait QTableWidgetItem_toolTip {
  fn toolTip(self, this: &mut QTableWidgetItem) -> i32;
}

// proto: QString QTableWidgetItem::toolTip();
impl<'a> /*trait*/ QTableWidgetItem_toolTip for () {
  fn toolTip(self, this: &mut QTableWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTableWidgetItem7toolTipEv()};
    unsafe {_ZNK16QTableWidgetItem7toolTipEv()};
    return 1;
  }
}

impl /*struct*/ QTableWidgetItem {
  pub fn isSelected<T: QTableWidgetItem_isSelected>(&mut self, value: T) -> i32 {
    value.isSelected(self);
    return 1;
  }
}

pub trait QTableWidgetItem_isSelected {
  fn isSelected(self, this: &mut QTableWidgetItem) -> i32;
}

// proto: bool QTableWidgetItem::isSelected();
impl<'a> /*trait*/ QTableWidgetItem_isSelected for () {
  fn isSelected(self, this: &mut QTableWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTableWidgetItem10isSelectedEv()};
    unsafe {_ZNK16QTableWidgetItem10isSelectedEv()};
    return 1;
  }
}

impl /*struct*/ QTableWidgetItem {
  pub fn setBackgroundColor<T: QTableWidgetItem_setBackgroundColor>(&mut self, value: T) -> i32 {
    value.setBackgroundColor(self);
    return 1;
  }
}

pub trait QTableWidgetItem_setBackgroundColor {
  fn setBackgroundColor(self, this: &mut QTableWidgetItem) -> i32;
}

// proto: void QTableWidgetItem::setBackgroundColor(const QColor & color);
impl<'a> /*trait*/ QTableWidgetItem_setBackgroundColor for (&'a  QColor) {
  fn setBackgroundColor(self, this: &mut QTableWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTableWidgetItem18setBackgroundColorERK6QColor()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN16QTableWidgetItem18setBackgroundColorERK6QColor(arg0)};
    return 1;
  }
}

impl /*struct*/ QTableWidgetItem {
  pub fn setBackground<T: QTableWidgetItem_setBackground>(&mut self, value: T) -> i32 {
    value.setBackground(self);
    return 1;
  }
}

pub trait QTableWidgetItem_setBackground {
  fn setBackground(self, this: &mut QTableWidgetItem) -> i32;
}

// proto: void QTableWidgetItem::setBackground(const QBrush & brush);
impl<'a> /*trait*/ QTableWidgetItem_setBackground for (&'a  QBrush) {
  fn setBackground(self, this: &mut QTableWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTableWidgetItem13setBackgroundERK6QBrush()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN16QTableWidgetItem13setBackgroundERK6QBrush(arg0)};
    return 1;
  }
}

impl /*struct*/ QTableWidgetItem {
  pub fn setFont<T: QTableWidgetItem_setFont>(&mut self, value: T) -> i32 {
    value.setFont(self);
    return 1;
  }
}

pub trait QTableWidgetItem_setFont {
  fn setFont(self, this: &mut QTableWidgetItem) -> i32;
}

// proto: void QTableWidgetItem::setFont(const QFont & font);
impl<'a> /*trait*/ QTableWidgetItem_setFont for (&'a  QFont) {
  fn setFont(self, this: &mut QTableWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTableWidgetItem7setFontERK5QFont()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN16QTableWidgetItem7setFontERK5QFont(arg0)};
    return 1;
  }
}

impl /*struct*/ QTableWidgetItem {
  pub fn setTextColor<T: QTableWidgetItem_setTextColor>(&mut self, value: T) -> i32 {
    value.setTextColor(self);
    return 1;
  }
}

pub trait QTableWidgetItem_setTextColor {
  fn setTextColor(self, this: &mut QTableWidgetItem) -> i32;
}

// proto: void QTableWidgetItem::setTextColor(const QColor & color);
impl<'a> /*trait*/ QTableWidgetItem_setTextColor for (&'a  QColor) {
  fn setTextColor(self, this: &mut QTableWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTableWidgetItem12setTextColorERK6QColor()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN16QTableWidgetItem12setTextColorERK6QColor(arg0)};
    return 1;
  }
}

impl /*struct*/ QTableWidgetItem {
  pub fn setText<T: QTableWidgetItem_setText>(&mut self, value: T) -> i32 {
    value.setText(self);
    return 1;
  }
}

pub trait QTableWidgetItem_setText {
  fn setText(self, this: &mut QTableWidgetItem) -> i32;
}

// proto: void QTableWidgetItem::setText(const QString & text);
impl<'a> /*trait*/ QTableWidgetItem_setText for (&'a  QString) {
  fn setText(self, this: &mut QTableWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTableWidgetItem7setTextERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN16QTableWidgetItem7setTextERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QTableWidgetItem {
  pub fn whatsThis<T: QTableWidgetItem_whatsThis>(&mut self, value: T) -> i32 {
    value.whatsThis(self);
    return 1;
  }
}

pub trait QTableWidgetItem_whatsThis {
  fn whatsThis(self, this: &mut QTableWidgetItem) -> i32;
}

// proto: QString QTableWidgetItem::whatsThis();
impl<'a> /*trait*/ QTableWidgetItem_whatsThis for () {
  fn whatsThis(self, this: &mut QTableWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTableWidgetItem9whatsThisEv()};
    unsafe {_ZNK16QTableWidgetItem9whatsThisEv()};
    return 1;
  }
}

impl /*struct*/ QTableWidgetItem {
  pub fn setToolTip<T: QTableWidgetItem_setToolTip>(&mut self, value: T) -> i32 {
    value.setToolTip(self);
    return 1;
  }
}

pub trait QTableWidgetItem_setToolTip {
  fn setToolTip(self, this: &mut QTableWidgetItem) -> i32;
}

// proto: void QTableWidgetItem::setToolTip(const QString & toolTip);
impl<'a> /*trait*/ QTableWidgetItem_setToolTip for (&'a  QString) {
  fn setToolTip(self, this: &mut QTableWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTableWidgetItem10setToolTipERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN16QTableWidgetItem10setToolTipERK7QString(arg0)};
    return 1;
  }
}

