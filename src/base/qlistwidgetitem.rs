// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qvariant::QVariant;
use super::qbrush::QBrush;
use super::qlistwidget::QListWidget;
use super::qdatastream::QDataStream;
use super::qicon::QIcon;
use super::qstring::QString;
use super::qfont::QFont;
use super::qcolor::QColor;
use super::qsize::QSize;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: void QListWidgetItem::FreeQListWidgetItem();
  fn _ZN15QListWidgetItemD0Ev() -> i32;
  // proto: bool QListWidgetItem::isHidden();
  fn _ZNK15QListWidgetItem8isHiddenEv() -> i32;
  // proto: void QListWidgetItem::setData(int role, const QVariant & value);
  fn _ZN15QListWidgetItem7setDataEiRK8QVariant(arg0: c_int, arg1: *const c_void) -> i32;
  // proto: void QListWidgetItem::setBackground(const QBrush & brush);
  fn _ZN15QListWidgetItem13setBackgroundERK6QBrush(arg0: *const c_void) -> i32;
  // proto: void QListWidgetItem::setSelected(bool select);
  fn _ZN15QListWidgetItem11setSelectedEb(arg0: int8_t) -> i32;
  // proto: QFont QListWidgetItem::font();
  fn _ZNK15QListWidgetItem4fontEv() -> i32;
  // proto: void QListWidgetItem::setTextAlignment(int alignment);
  fn _ZN15QListWidgetItem16setTextAlignmentEi(arg0: c_int) -> i32;
  // proto: void QListWidgetItem::NewQListWidgetItem(QListWidget * view, int type);
  fn _ZN15QListWidgetItemC1EP11QListWidgeti(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int) -> i32;
  // proto: void QListWidgetItem::write(QDataStream & out);
  fn _ZNK15QListWidgetItem5writeER11QDataStream(arg0: *mut c_void) -> i32;
  // proto: QString QListWidgetItem::whatsThis();
  fn _ZNK15QListWidgetItem9whatsThisEv() -> i32;
  // proto: int QListWidgetItem::type_();
  fn _ZNK15QListWidgetItem4typeEv() -> i32;
  // proto: void QListWidgetItem::NewQListWidgetItem(const QIcon & icon, const QString & text, QListWidget * view, int type);
  fn _ZN15QListWidgetItemC1ERK5QIconRK7QStringP11QListWidgeti(qthis: *mut c_void, arg0: *const c_void, arg1: *const c_void, arg2: *mut c_void, arg3: c_int) -> i32;
  // proto: QIcon QListWidgetItem::icon();
  fn _ZNK15QListWidgetItem4iconEv() -> i32;
  // proto: QColor QListWidgetItem::textColor();
  fn _ZNK15QListWidgetItem9textColorEv() -> i32;
  // proto: QBrush QListWidgetItem::foreground();
  fn _ZNK15QListWidgetItem10foregroundEv() -> i32;
  // proto: QBrush QListWidgetItem::background();
  fn _ZNK15QListWidgetItem10backgroundEv() -> i32;
  // proto: void QListWidgetItem::setStatusTip(const QString & statusTip);
  fn _ZN15QListWidgetItem12setStatusTipERK7QString(arg0: *const c_void) -> i32;
  // proto: QString QListWidgetItem::text();
  fn _ZNK15QListWidgetItem4textEv() -> i32;
  // proto: QColor QListWidgetItem::backgroundColor();
  fn _ZNK15QListWidgetItem15backgroundColorEv() -> i32;
  // proto: bool QListWidgetItem::isSelected();
  fn _ZNK15QListWidgetItem10isSelectedEv() -> i32;
  // proto: void QListWidgetItem::setFont(const QFont & font);
  fn _ZN15QListWidgetItem7setFontERK5QFont(arg0: *const c_void) -> i32;
  // proto: void QListWidgetItem::setText(const QString & text);
  fn _ZN15QListWidgetItem7setTextERK7QString(arg0: *const c_void) -> i32;
  // proto: void QListWidgetItem::NewQListWidgetItem(const QString & text, QListWidget * view, int type);
  fn _ZN15QListWidgetItemC1ERK7QStringP11QListWidgeti(qthis: *mut c_void, arg0: *const c_void, arg1: *mut c_void, arg2: c_int) -> i32;
  // proto: QVariant QListWidgetItem::data(int role);
  fn _ZNK15QListWidgetItem4dataEi(arg0: c_int) -> i32;
  // proto: QSize QListWidgetItem::sizeHint();
  fn _ZNK15QListWidgetItem8sizeHintEv() -> i32;
  // proto: void QListWidgetItem::setWhatsThis(const QString & whatsThis);
  fn _ZN15QListWidgetItem12setWhatsThisERK7QString(arg0: *const c_void) -> i32;
  // proto: void QListWidgetItem::read(QDataStream & in);
  fn _ZN15QListWidgetItem4readER11QDataStream(arg0: *mut c_void) -> i32;
  // proto: void QListWidgetItem::setTextColor(const QColor & color);
  fn _ZN15QListWidgetItem12setTextColorERK6QColor(arg0: *const c_void) -> i32;
  // proto: void QListWidgetItem::setSizeHint(const QSize & size);
  fn _ZN15QListWidgetItem11setSizeHintERK5QSize(arg0: *const c_void) -> i32;
  // proto: QListWidget * QListWidgetItem::listWidget();
  fn _ZNK15QListWidgetItem10listWidgetEv() -> i32;
  // proto: void QListWidgetItem::setIcon(const QIcon & icon);
  fn _ZN15QListWidgetItem7setIconERK5QIcon(arg0: *const c_void) -> i32;
  // proto: QListWidgetItem * QListWidgetItem::clone();
  fn _ZNK15QListWidgetItem5cloneEv() -> i32;
  // proto: void QListWidgetItem::setBackgroundColor(const QColor & color);
  fn _ZN15QListWidgetItem18setBackgroundColorERK6QColor(arg0: *const c_void) -> i32;
  // proto: void QListWidgetItem::setForeground(const QBrush & brush);
  fn _ZN15QListWidgetItem13setForegroundERK6QBrush(arg0: *const c_void) -> i32;
  // proto: void QListWidgetItem::NewQListWidgetItem(const QListWidgetItem & other);
  fn _ZN15QListWidgetItemC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: void QListWidgetItem::setHidden(bool hide);
  fn _ZN15QListWidgetItem9setHiddenEb(arg0: int8_t) -> i32;
  // proto: QString QListWidgetItem::toolTip();
  fn _ZNK15QListWidgetItem7toolTipEv() -> i32;
  // proto: int QListWidgetItem::textAlignment();
  fn _ZNK15QListWidgetItem13textAlignmentEv() -> i32;
  // proto: QString QListWidgetItem::statusTip();
  fn _ZNK15QListWidgetItem9statusTipEv() -> i32;
  // proto: void QListWidgetItem::setToolTip(const QString & toolTip);
  fn _ZN15QListWidgetItem10setToolTipERK7QString(arg0: *const c_void) -> i32;
}

// body block begin
// class sizeof(QListWidgetItem)=1
pub struct QListWidgetItem {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QListWidgetItem {
  pub fn FreeQListWidgetItem<T: QListWidgetItem_FreeQListWidgetItem>(&mut self, value: T) -> i32 {
    value.FreeQListWidgetItem(self);
    return 1;
  }
}

pub trait QListWidgetItem_FreeQListWidgetItem {
  fn FreeQListWidgetItem(self, this: &mut QListWidgetItem) -> i32;
}

// proto: void QListWidgetItem::FreeQListWidgetItem();
impl<'a> /*trait*/ QListWidgetItem_FreeQListWidgetItem for () {
  fn FreeQListWidgetItem(self, this: &mut QListWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QListWidgetItemD0Ev()};
    unsafe {_ZN15QListWidgetItemD0Ev()};
    return 1;
  }
}

impl /*struct*/ QListWidgetItem {
  pub fn isHidden<T: QListWidgetItem_isHidden>(&mut self, value: T) -> i32 {
    value.isHidden(self);
    return 1;
  }
}

pub trait QListWidgetItem_isHidden {
  fn isHidden(self, this: &mut QListWidgetItem) -> i32;
}

// proto: bool QListWidgetItem::isHidden();
impl<'a> /*trait*/ QListWidgetItem_isHidden for () {
  fn isHidden(self, this: &mut QListWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QListWidgetItem8isHiddenEv()};
    unsafe {_ZNK15QListWidgetItem8isHiddenEv()};
    return 1;
  }
}

impl /*struct*/ QListWidgetItem {
  pub fn setData<T: QListWidgetItem_setData>(&mut self, value: T) -> i32 {
    value.setData(self);
    return 1;
  }
}

pub trait QListWidgetItem_setData {
  fn setData(self, this: &mut QListWidgetItem) -> i32;
}

// proto: void QListWidgetItem::setData(int role, const QVariant & value);
impl<'a> /*trait*/ QListWidgetItem_setData for (i32, &'a  QVariant) {
  fn setData(self, this: &mut QListWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QListWidgetItem7setDataEiRK8QVariant()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN15QListWidgetItem7setDataEiRK8QVariant(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QListWidgetItem {
  pub fn setBackground<T: QListWidgetItem_setBackground>(&mut self, value: T) -> i32 {
    value.setBackground(self);
    return 1;
  }
}

pub trait QListWidgetItem_setBackground {
  fn setBackground(self, this: &mut QListWidgetItem) -> i32;
}

// proto: void QListWidgetItem::setBackground(const QBrush & brush);
impl<'a> /*trait*/ QListWidgetItem_setBackground for (&'a  QBrush) {
  fn setBackground(self, this: &mut QListWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QListWidgetItem13setBackgroundERK6QBrush()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN15QListWidgetItem13setBackgroundERK6QBrush(arg0)};
    return 1;
  }
}

impl /*struct*/ QListWidgetItem {
  pub fn setSelected<T: QListWidgetItem_setSelected>(&mut self, value: T) -> i32 {
    value.setSelected(self);
    return 1;
  }
}

pub trait QListWidgetItem_setSelected {
  fn setSelected(self, this: &mut QListWidgetItem) -> i32;
}

// proto: void QListWidgetItem::setSelected(bool select);
impl<'a> /*trait*/ QListWidgetItem_setSelected for (i8) {
  fn setSelected(self, this: &mut QListWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QListWidgetItem11setSelectedEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN15QListWidgetItem11setSelectedEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QListWidgetItem {
  pub fn font<T: QListWidgetItem_font>(&mut self, value: T) -> i32 {
    value.font(self);
    return 1;
  }
}

pub trait QListWidgetItem_font {
  fn font(self, this: &mut QListWidgetItem) -> i32;
}

// proto: QFont QListWidgetItem::font();
impl<'a> /*trait*/ QListWidgetItem_font for () {
  fn font(self, this: &mut QListWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QListWidgetItem4fontEv()};
    unsafe {_ZNK15QListWidgetItem4fontEv()};
    return 1;
  }
}

impl /*struct*/ QListWidgetItem {
  pub fn setTextAlignment<T: QListWidgetItem_setTextAlignment>(&mut self, value: T) -> i32 {
    value.setTextAlignment(self);
    return 1;
  }
}

pub trait QListWidgetItem_setTextAlignment {
  fn setTextAlignment(self, this: &mut QListWidgetItem) -> i32;
}

// proto: void QListWidgetItem::setTextAlignment(int alignment);
impl<'a> /*trait*/ QListWidgetItem_setTextAlignment for (i32) {
  fn setTextAlignment(self, this: &mut QListWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QListWidgetItem16setTextAlignmentEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN15QListWidgetItem16setTextAlignmentEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QListWidgetItem {
  pub fn NewQListWidgetItem<T: QListWidgetItem_NewQListWidgetItem>(value: T) -> QListWidgetItem {
    let rsthis = value.NewQListWidgetItem();
    return rsthis;
    // return 1;
  }
}

pub trait QListWidgetItem_NewQListWidgetItem {
  fn NewQListWidgetItem(self) -> QListWidgetItem;
}

// proto: void QListWidgetItem::NewQListWidgetItem(QListWidget * view, int type);
impl<'a> /*trait*/ QListWidgetItem_NewQListWidgetItem for (&'a mut QListWidget, i32) {
  fn NewQListWidgetItem(self) -> QListWidgetItem {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QListWidgetItemC1EP11QListWidgeti()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    unsafe {_ZN15QListWidgetItemC1EP11QListWidgeti(qthis, arg0, arg1)};
    let rsthis = QListWidgetItem{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QListWidgetItem {
  pub fn write<T: QListWidgetItem_write>(&mut self, value: T) -> i32 {
    value.write(self);
    return 1;
  }
}

pub trait QListWidgetItem_write {
  fn write(self, this: &mut QListWidgetItem) -> i32;
}

// proto: void QListWidgetItem::write(QDataStream & out);
impl<'a> /*trait*/ QListWidgetItem_write for (&'a mut QDataStream) {
  fn write(self, this: &mut QListWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QListWidgetItem5writeER11QDataStream()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZNK15QListWidgetItem5writeER11QDataStream(arg0)};
    return 1;
  }
}

impl /*struct*/ QListWidgetItem {
  pub fn whatsThis<T: QListWidgetItem_whatsThis>(&mut self, value: T) -> i32 {
    value.whatsThis(self);
    return 1;
  }
}

pub trait QListWidgetItem_whatsThis {
  fn whatsThis(self, this: &mut QListWidgetItem) -> i32;
}

// proto: QString QListWidgetItem::whatsThis();
impl<'a> /*trait*/ QListWidgetItem_whatsThis for () {
  fn whatsThis(self, this: &mut QListWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QListWidgetItem9whatsThisEv()};
    unsafe {_ZNK15QListWidgetItem9whatsThisEv()};
    return 1;
  }
}

impl /*struct*/ QListWidgetItem {
  pub fn type_<T: QListWidgetItem_type_>(&mut self, value: T) -> i32 {
    value.type_(self);
    return 1;
  }
}

pub trait QListWidgetItem_type_ {
  fn type_(self, this: &mut QListWidgetItem) -> i32;
}

// proto: int QListWidgetItem::type_();
impl<'a> /*trait*/ QListWidgetItem_type_ for () {
  fn type_(self, this: &mut QListWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QListWidgetItem4typeEv()};
    unsafe {_ZNK15QListWidgetItem4typeEv()};
    return 1;
  }
}

// proto: void QListWidgetItem::NewQListWidgetItem(const QIcon & icon, const QString & text, QListWidget * view, int type);
impl<'a> /*trait*/ QListWidgetItem_NewQListWidgetItem for (&'a  QIcon, &'a  QString, &'a mut QListWidget, i32) {
  fn NewQListWidgetItem(self) -> QListWidgetItem {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QListWidgetItemC1ERK5QIconRK7QStringP11QListWidgeti()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let arg3 = self.3  as c_int;
    unsafe {_ZN15QListWidgetItemC1ERK5QIconRK7QStringP11QListWidgeti(qthis, arg0, arg1, arg2, arg3)};
    let rsthis = QListWidgetItem{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QListWidgetItem {
  pub fn icon<T: QListWidgetItem_icon>(&mut self, value: T) -> i32 {
    value.icon(self);
    return 1;
  }
}

pub trait QListWidgetItem_icon {
  fn icon(self, this: &mut QListWidgetItem) -> i32;
}

// proto: QIcon QListWidgetItem::icon();
impl<'a> /*trait*/ QListWidgetItem_icon for () {
  fn icon(self, this: &mut QListWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QListWidgetItem4iconEv()};
    unsafe {_ZNK15QListWidgetItem4iconEv()};
    return 1;
  }
}

impl /*struct*/ QListWidgetItem {
  pub fn textColor<T: QListWidgetItem_textColor>(&mut self, value: T) -> i32 {
    value.textColor(self);
    return 1;
  }
}

pub trait QListWidgetItem_textColor {
  fn textColor(self, this: &mut QListWidgetItem) -> i32;
}

// proto: QColor QListWidgetItem::textColor();
impl<'a> /*trait*/ QListWidgetItem_textColor for () {
  fn textColor(self, this: &mut QListWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QListWidgetItem9textColorEv()};
    unsafe {_ZNK15QListWidgetItem9textColorEv()};
    return 1;
  }
}

impl /*struct*/ QListWidgetItem {
  pub fn foreground<T: QListWidgetItem_foreground>(&mut self, value: T) -> i32 {
    value.foreground(self);
    return 1;
  }
}

pub trait QListWidgetItem_foreground {
  fn foreground(self, this: &mut QListWidgetItem) -> i32;
}

// proto: QBrush QListWidgetItem::foreground();
impl<'a> /*trait*/ QListWidgetItem_foreground for () {
  fn foreground(self, this: &mut QListWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QListWidgetItem10foregroundEv()};
    unsafe {_ZNK15QListWidgetItem10foregroundEv()};
    return 1;
  }
}

impl /*struct*/ QListWidgetItem {
  pub fn background<T: QListWidgetItem_background>(&mut self, value: T) -> i32 {
    value.background(self);
    return 1;
  }
}

pub trait QListWidgetItem_background {
  fn background(self, this: &mut QListWidgetItem) -> i32;
}

// proto: QBrush QListWidgetItem::background();
impl<'a> /*trait*/ QListWidgetItem_background for () {
  fn background(self, this: &mut QListWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QListWidgetItem10backgroundEv()};
    unsafe {_ZNK15QListWidgetItem10backgroundEv()};
    return 1;
  }
}

impl /*struct*/ QListWidgetItem {
  pub fn setStatusTip<T: QListWidgetItem_setStatusTip>(&mut self, value: T) -> i32 {
    value.setStatusTip(self);
    return 1;
  }
}

pub trait QListWidgetItem_setStatusTip {
  fn setStatusTip(self, this: &mut QListWidgetItem) -> i32;
}

// proto: void QListWidgetItem::setStatusTip(const QString & statusTip);
impl<'a> /*trait*/ QListWidgetItem_setStatusTip for (&'a  QString) {
  fn setStatusTip(self, this: &mut QListWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QListWidgetItem12setStatusTipERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN15QListWidgetItem12setStatusTipERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QListWidgetItem {
  pub fn text<T: QListWidgetItem_text>(&mut self, value: T) -> i32 {
    value.text(self);
    return 1;
  }
}

pub trait QListWidgetItem_text {
  fn text(self, this: &mut QListWidgetItem) -> i32;
}

// proto: QString QListWidgetItem::text();
impl<'a> /*trait*/ QListWidgetItem_text for () {
  fn text(self, this: &mut QListWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QListWidgetItem4textEv()};
    unsafe {_ZNK15QListWidgetItem4textEv()};
    return 1;
  }
}

impl /*struct*/ QListWidgetItem {
  pub fn backgroundColor<T: QListWidgetItem_backgroundColor>(&mut self, value: T) -> i32 {
    value.backgroundColor(self);
    return 1;
  }
}

pub trait QListWidgetItem_backgroundColor {
  fn backgroundColor(self, this: &mut QListWidgetItem) -> i32;
}

// proto: QColor QListWidgetItem::backgroundColor();
impl<'a> /*trait*/ QListWidgetItem_backgroundColor for () {
  fn backgroundColor(self, this: &mut QListWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QListWidgetItem15backgroundColorEv()};
    unsafe {_ZNK15QListWidgetItem15backgroundColorEv()};
    return 1;
  }
}

impl /*struct*/ QListWidgetItem {
  pub fn isSelected<T: QListWidgetItem_isSelected>(&mut self, value: T) -> i32 {
    value.isSelected(self);
    return 1;
  }
}

pub trait QListWidgetItem_isSelected {
  fn isSelected(self, this: &mut QListWidgetItem) -> i32;
}

// proto: bool QListWidgetItem::isSelected();
impl<'a> /*trait*/ QListWidgetItem_isSelected for () {
  fn isSelected(self, this: &mut QListWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QListWidgetItem10isSelectedEv()};
    unsafe {_ZNK15QListWidgetItem10isSelectedEv()};
    return 1;
  }
}

impl /*struct*/ QListWidgetItem {
  pub fn setFont<T: QListWidgetItem_setFont>(&mut self, value: T) -> i32 {
    value.setFont(self);
    return 1;
  }
}

pub trait QListWidgetItem_setFont {
  fn setFont(self, this: &mut QListWidgetItem) -> i32;
}

// proto: void QListWidgetItem::setFont(const QFont & font);
impl<'a> /*trait*/ QListWidgetItem_setFont for (&'a  QFont) {
  fn setFont(self, this: &mut QListWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QListWidgetItem7setFontERK5QFont()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN15QListWidgetItem7setFontERK5QFont(arg0)};
    return 1;
  }
}

impl /*struct*/ QListWidgetItem {
  pub fn setText<T: QListWidgetItem_setText>(&mut self, value: T) -> i32 {
    value.setText(self);
    return 1;
  }
}

pub trait QListWidgetItem_setText {
  fn setText(self, this: &mut QListWidgetItem) -> i32;
}

// proto: void QListWidgetItem::setText(const QString & text);
impl<'a> /*trait*/ QListWidgetItem_setText for (&'a  QString) {
  fn setText(self, this: &mut QListWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QListWidgetItem7setTextERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN15QListWidgetItem7setTextERK7QString(arg0)};
    return 1;
  }
}

// proto: void QListWidgetItem::NewQListWidgetItem(const QString & text, QListWidget * view, int type);
impl<'a> /*trait*/ QListWidgetItem_NewQListWidgetItem for (&'a  QString, &'a mut QListWidget, i32) {
  fn NewQListWidgetItem(self) -> QListWidgetItem {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QListWidgetItemC1ERK7QStringP11QListWidgeti()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2  as c_int;
    unsafe {_ZN15QListWidgetItemC1ERK7QStringP11QListWidgeti(qthis, arg0, arg1, arg2)};
    let rsthis = QListWidgetItem{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QListWidgetItem {
  pub fn data<T: QListWidgetItem_data>(&mut self, value: T) -> i32 {
    value.data(self);
    return 1;
  }
}

pub trait QListWidgetItem_data {
  fn data(self, this: &mut QListWidgetItem) -> i32;
}

// proto: QVariant QListWidgetItem::data(int role);
impl<'a> /*trait*/ QListWidgetItem_data for (i32) {
  fn data(self, this: &mut QListWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QListWidgetItem4dataEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK15QListWidgetItem4dataEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QListWidgetItem {
  pub fn sizeHint<T: QListWidgetItem_sizeHint>(&mut self, value: T) -> i32 {
    value.sizeHint(self);
    return 1;
  }
}

pub trait QListWidgetItem_sizeHint {
  fn sizeHint(self, this: &mut QListWidgetItem) -> i32;
}

// proto: QSize QListWidgetItem::sizeHint();
impl<'a> /*trait*/ QListWidgetItem_sizeHint for () {
  fn sizeHint(self, this: &mut QListWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QListWidgetItem8sizeHintEv()};
    unsafe {_ZNK15QListWidgetItem8sizeHintEv()};
    return 1;
  }
}

impl /*struct*/ QListWidgetItem {
  pub fn setWhatsThis<T: QListWidgetItem_setWhatsThis>(&mut self, value: T) -> i32 {
    value.setWhatsThis(self);
    return 1;
  }
}

pub trait QListWidgetItem_setWhatsThis {
  fn setWhatsThis(self, this: &mut QListWidgetItem) -> i32;
}

// proto: void QListWidgetItem::setWhatsThis(const QString & whatsThis);
impl<'a> /*trait*/ QListWidgetItem_setWhatsThis for (&'a  QString) {
  fn setWhatsThis(self, this: &mut QListWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QListWidgetItem12setWhatsThisERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN15QListWidgetItem12setWhatsThisERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QListWidgetItem {
  pub fn read<T: QListWidgetItem_read>(&mut self, value: T) -> i32 {
    value.read(self);
    return 1;
  }
}

pub trait QListWidgetItem_read {
  fn read(self, this: &mut QListWidgetItem) -> i32;
}

// proto: void QListWidgetItem::read(QDataStream & in);
impl<'a> /*trait*/ QListWidgetItem_read for (&'a mut QDataStream) {
  fn read(self, this: &mut QListWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QListWidgetItem4readER11QDataStream()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN15QListWidgetItem4readER11QDataStream(arg0)};
    return 1;
  }
}

impl /*struct*/ QListWidgetItem {
  pub fn setTextColor<T: QListWidgetItem_setTextColor>(&mut self, value: T) -> i32 {
    value.setTextColor(self);
    return 1;
  }
}

pub trait QListWidgetItem_setTextColor {
  fn setTextColor(self, this: &mut QListWidgetItem) -> i32;
}

// proto: void QListWidgetItem::setTextColor(const QColor & color);
impl<'a> /*trait*/ QListWidgetItem_setTextColor for (&'a  QColor) {
  fn setTextColor(self, this: &mut QListWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QListWidgetItem12setTextColorERK6QColor()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN15QListWidgetItem12setTextColorERK6QColor(arg0)};
    return 1;
  }
}

impl /*struct*/ QListWidgetItem {
  pub fn setSizeHint<T: QListWidgetItem_setSizeHint>(&mut self, value: T) -> i32 {
    value.setSizeHint(self);
    return 1;
  }
}

pub trait QListWidgetItem_setSizeHint {
  fn setSizeHint(self, this: &mut QListWidgetItem) -> i32;
}

// proto: void QListWidgetItem::setSizeHint(const QSize & size);
impl<'a> /*trait*/ QListWidgetItem_setSizeHint for (&'a  QSize) {
  fn setSizeHint(self, this: &mut QListWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QListWidgetItem11setSizeHintERK5QSize()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN15QListWidgetItem11setSizeHintERK5QSize(arg0)};
    return 1;
  }
}

impl /*struct*/ QListWidgetItem {
  pub fn listWidget<T: QListWidgetItem_listWidget>(&mut self, value: T) -> i32 {
    value.listWidget(self);
    return 1;
  }
}

pub trait QListWidgetItem_listWidget {
  fn listWidget(self, this: &mut QListWidgetItem) -> i32;
}

// proto: QListWidget * QListWidgetItem::listWidget();
impl<'a> /*trait*/ QListWidgetItem_listWidget for () {
  fn listWidget(self, this: &mut QListWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QListWidgetItem10listWidgetEv()};
    unsafe {_ZNK15QListWidgetItem10listWidgetEv()};
    return 1;
  }
}

impl /*struct*/ QListWidgetItem {
  pub fn setIcon<T: QListWidgetItem_setIcon>(&mut self, value: T) -> i32 {
    value.setIcon(self);
    return 1;
  }
}

pub trait QListWidgetItem_setIcon {
  fn setIcon(self, this: &mut QListWidgetItem) -> i32;
}

// proto: void QListWidgetItem::setIcon(const QIcon & icon);
impl<'a> /*trait*/ QListWidgetItem_setIcon for (&'a  QIcon) {
  fn setIcon(self, this: &mut QListWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QListWidgetItem7setIconERK5QIcon()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN15QListWidgetItem7setIconERK5QIcon(arg0)};
    return 1;
  }
}

impl /*struct*/ QListWidgetItem {
  pub fn clone<T: QListWidgetItem_clone>(&mut self, value: T) -> i32 {
    value.clone(self);
    return 1;
  }
}

pub trait QListWidgetItem_clone {
  fn clone(self, this: &mut QListWidgetItem) -> i32;
}

// proto: QListWidgetItem * QListWidgetItem::clone();
impl<'a> /*trait*/ QListWidgetItem_clone for () {
  fn clone(self, this: &mut QListWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QListWidgetItem5cloneEv()};
    unsafe {_ZNK15QListWidgetItem5cloneEv()};
    return 1;
  }
}

impl /*struct*/ QListWidgetItem {
  pub fn setBackgroundColor<T: QListWidgetItem_setBackgroundColor>(&mut self, value: T) -> i32 {
    value.setBackgroundColor(self);
    return 1;
  }
}

pub trait QListWidgetItem_setBackgroundColor {
  fn setBackgroundColor(self, this: &mut QListWidgetItem) -> i32;
}

// proto: void QListWidgetItem::setBackgroundColor(const QColor & color);
impl<'a> /*trait*/ QListWidgetItem_setBackgroundColor for (&'a  QColor) {
  fn setBackgroundColor(self, this: &mut QListWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QListWidgetItem18setBackgroundColorERK6QColor()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN15QListWidgetItem18setBackgroundColorERK6QColor(arg0)};
    return 1;
  }
}

impl /*struct*/ QListWidgetItem {
  pub fn setForeground<T: QListWidgetItem_setForeground>(&mut self, value: T) -> i32 {
    value.setForeground(self);
    return 1;
  }
}

pub trait QListWidgetItem_setForeground {
  fn setForeground(self, this: &mut QListWidgetItem) -> i32;
}

// proto: void QListWidgetItem::setForeground(const QBrush & brush);
impl<'a> /*trait*/ QListWidgetItem_setForeground for (&'a  QBrush) {
  fn setForeground(self, this: &mut QListWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QListWidgetItem13setForegroundERK6QBrush()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN15QListWidgetItem13setForegroundERK6QBrush(arg0)};
    return 1;
  }
}

// proto: void QListWidgetItem::NewQListWidgetItem(const QListWidgetItem & other);
impl<'a> /*trait*/ QListWidgetItem_NewQListWidgetItem for (&'a  QListWidgetItem) {
  fn NewQListWidgetItem(self) -> QListWidgetItem {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QListWidgetItemC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN15QListWidgetItemC1ERKS_(qthis, arg0)};
    let rsthis = QListWidgetItem{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QListWidgetItem {
  pub fn setHidden<T: QListWidgetItem_setHidden>(&mut self, value: T) -> i32 {
    value.setHidden(self);
    return 1;
  }
}

pub trait QListWidgetItem_setHidden {
  fn setHidden(self, this: &mut QListWidgetItem) -> i32;
}

// proto: void QListWidgetItem::setHidden(bool hide);
impl<'a> /*trait*/ QListWidgetItem_setHidden for (i8) {
  fn setHidden(self, this: &mut QListWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QListWidgetItem9setHiddenEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN15QListWidgetItem9setHiddenEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QListWidgetItem {
  pub fn toolTip<T: QListWidgetItem_toolTip>(&mut self, value: T) -> i32 {
    value.toolTip(self);
    return 1;
  }
}

pub trait QListWidgetItem_toolTip {
  fn toolTip(self, this: &mut QListWidgetItem) -> i32;
}

// proto: QString QListWidgetItem::toolTip();
impl<'a> /*trait*/ QListWidgetItem_toolTip for () {
  fn toolTip(self, this: &mut QListWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QListWidgetItem7toolTipEv()};
    unsafe {_ZNK15QListWidgetItem7toolTipEv()};
    return 1;
  }
}

impl /*struct*/ QListWidgetItem {
  pub fn textAlignment<T: QListWidgetItem_textAlignment>(&mut self, value: T) -> i32 {
    value.textAlignment(self);
    return 1;
  }
}

pub trait QListWidgetItem_textAlignment {
  fn textAlignment(self, this: &mut QListWidgetItem) -> i32;
}

// proto: int QListWidgetItem::textAlignment();
impl<'a> /*trait*/ QListWidgetItem_textAlignment for () {
  fn textAlignment(self, this: &mut QListWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QListWidgetItem13textAlignmentEv()};
    unsafe {_ZNK15QListWidgetItem13textAlignmentEv()};
    return 1;
  }
}

impl /*struct*/ QListWidgetItem {
  pub fn statusTip<T: QListWidgetItem_statusTip>(&mut self, value: T) -> i32 {
    value.statusTip(self);
    return 1;
  }
}

pub trait QListWidgetItem_statusTip {
  fn statusTip(self, this: &mut QListWidgetItem) -> i32;
}

// proto: QString QListWidgetItem::statusTip();
impl<'a> /*trait*/ QListWidgetItem_statusTip for () {
  fn statusTip(self, this: &mut QListWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QListWidgetItem9statusTipEv()};
    unsafe {_ZNK15QListWidgetItem9statusTipEv()};
    return 1;
  }
}

impl /*struct*/ QListWidgetItem {
  pub fn setToolTip<T: QListWidgetItem_setToolTip>(&mut self, value: T) -> i32 {
    value.setToolTip(self);
    return 1;
  }
}

pub trait QListWidgetItem_setToolTip {
  fn setToolTip(self, this: &mut QListWidgetItem) -> i32;
}

// proto: void QListWidgetItem::setToolTip(const QString & toolTip);
impl<'a> /*trait*/ QListWidgetItem_setToolTip for (&'a  QString) {
  fn setToolTip(self, this: &mut QListWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QListWidgetItem10setToolTipERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN15QListWidgetItem10setToolTipERK7QString(arg0)};
    return 1;
  }
}

