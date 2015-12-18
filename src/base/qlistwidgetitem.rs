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
use super::qfont::QFont;
use super::qlistwidget::QListWidget;
use super::qdatastream::QDataStream;
use super::qstring::QString;
use super::qicon::QIcon;
use super::qcolor::QColor;
use super::qsize::QSize;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  void QListWidgetItem::FreeQListWidgetItem();
  fn _ZN15QListWidgetItemD0Ev(qthis: *mut c_void) ;
  // proto:  bool QListWidgetItem::isHidden();
  fn _ZNK15QListWidgetItem8isHiddenEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QListWidgetItem::setData(int role, const QVariant & value);
  fn _ZN15QListWidgetItem7setDataEiRK8QVariant(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void) ;
  // proto:  void QListWidgetItem::setBackground(const QBrush & brush);
  fn _ZN15QListWidgetItem13setBackgroundERK6QBrush(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QListWidgetItem::setSelected(bool select);
  fn _ZN15QListWidgetItem11setSelectedEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  QFont QListWidgetItem::font();
  fn _ZNK15QListWidgetItem4fontEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QListWidgetItem::setTextAlignment(int alignment);
  fn _ZN15QListWidgetItem16setTextAlignmentEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QListWidgetItem::NewQListWidgetItem(QListWidget * view, int type);
  fn _ZN15QListWidgetItemC1EP11QListWidgeti(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int) ;
  // proto:  void QListWidgetItem::write(QDataStream & out);
  fn _ZNK15QListWidgetItem5writeER11QDataStream(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QString QListWidgetItem::whatsThis();
  fn _ZNK15QListWidgetItem9whatsThisEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  int QListWidgetItem::type_();
  fn _ZNK15QListWidgetItem4typeEv(qthis: *mut c_void) -> c_int;
  // proto:  void QListWidgetItem::NewQListWidgetItem(const QIcon & icon, const QString & text, QListWidget * view, int type);
  fn _ZN15QListWidgetItemC1ERK5QIconRK7QStringP11QListWidgeti(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void, arg3: c_int) ;
  // proto:  QIcon QListWidgetItem::icon();
  fn _ZNK15QListWidgetItem4iconEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QColor QListWidgetItem::textColor();
  fn _ZNK15QListWidgetItem9textColorEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QBrush QListWidgetItem::foreground();
  fn _ZNK15QListWidgetItem10foregroundEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QBrush QListWidgetItem::background();
  fn _ZNK15QListWidgetItem10backgroundEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QListWidgetItem::setStatusTip(const QString & statusTip);
  fn _ZN15QListWidgetItem12setStatusTipERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QString QListWidgetItem::text();
  fn _ZNK15QListWidgetItem4textEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QColor QListWidgetItem::backgroundColor();
  fn _ZNK15QListWidgetItem15backgroundColorEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QListWidgetItem::isSelected();
  fn _ZNK15QListWidgetItem10isSelectedEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QListWidgetItem::setFont(const QFont & font);
  fn _ZN15QListWidgetItem7setFontERK5QFont(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QListWidgetItem::setText(const QString & text);
  fn _ZN15QListWidgetItem7setTextERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QListWidgetItem::NewQListWidgetItem(const QString & text, QListWidget * view, int type);
  fn _ZN15QListWidgetItemC1ERK7QStringP11QListWidgeti(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: c_int) ;
  // proto:  QVariant QListWidgetItem::data(int role);
  fn _ZNK15QListWidgetItem4dataEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  QSize QListWidgetItem::sizeHint();
  fn _ZNK15QListWidgetItem8sizeHintEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QListWidgetItem::setWhatsThis(const QString & whatsThis);
  fn _ZN15QListWidgetItem12setWhatsThisERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QListWidgetItem::read(QDataStream & in);
  fn _ZN15QListWidgetItem4readER11QDataStream(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QListWidgetItem::setTextColor(const QColor & color);
  fn _ZN15QListWidgetItem12setTextColorERK6QColor(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QListWidgetItem::setSizeHint(const QSize & size);
  fn _ZN15QListWidgetItem11setSizeHintERK5QSize(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QListWidget * QListWidgetItem::listWidget();
  fn _ZNK15QListWidgetItem10listWidgetEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QListWidgetItem::setIcon(const QIcon & icon);
  fn _ZN15QListWidgetItem7setIconERK5QIcon(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QListWidgetItem * QListWidgetItem::clone();
  fn _ZNK15QListWidgetItem5cloneEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QListWidgetItem::setBackgroundColor(const QColor & color);
  fn _ZN15QListWidgetItem18setBackgroundColorERK6QColor(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QListWidgetItem::setForeground(const QBrush & brush);
  fn _ZN15QListWidgetItem13setForegroundERK6QBrush(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QListWidgetItem::NewQListWidgetItem(const QListWidgetItem & other);
  fn _ZN15QListWidgetItemC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QListWidgetItem::setHidden(bool hide);
  fn _ZN15QListWidgetItem9setHiddenEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  QString QListWidgetItem::toolTip();
  fn _ZNK15QListWidgetItem7toolTipEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  int QListWidgetItem::textAlignment();
  fn _ZNK15QListWidgetItem13textAlignmentEv(qthis: *mut c_void) -> c_int;
  // proto:  QString QListWidgetItem::statusTip();
  fn _ZNK15QListWidgetItem9statusTipEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QListWidgetItem::setToolTip(const QString & toolTip);
  fn _ZN15QListWidgetItem10setToolTipERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
}

// body block begin
// class sizeof(QListWidgetItem)=1
pub struct QListWidgetItem {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QListWidgetItem {
  pub fn FreeQListWidgetItem<RetType, T: QListWidgetItem_FreeQListWidgetItem<RetType>>(&mut self, value: T) -> RetType {
    return value.FreeQListWidgetItem(self);
    // return 1;
  }
}

pub trait QListWidgetItem_FreeQListWidgetItem<RetType> {
  fn FreeQListWidgetItem(self, rsthis: &mut QListWidgetItem) -> RetType;
}

// proto:  void QListWidgetItem::FreeQListWidgetItem();
impl<'a> /*trait*/ QListWidgetItem_FreeQListWidgetItem<()> for () {
  fn FreeQListWidgetItem(self, rsthis: &mut QListWidgetItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QListWidgetItemD0Ev()};
     unsafe {_ZN15QListWidgetItemD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QListWidgetItem {
  pub fn isHidden<RetType, T: QListWidgetItem_isHidden<RetType>>(&mut self, value: T) -> RetType {
    return value.isHidden(self);
    // return 1;
  }
}

pub trait QListWidgetItem_isHidden<RetType> {
  fn isHidden(self, rsthis: &mut QListWidgetItem) -> RetType;
}

// proto:  bool QListWidgetItem::isHidden();
impl<'a> /*trait*/ QListWidgetItem_isHidden<i8> for () {
  fn isHidden(self, rsthis: &mut QListWidgetItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QListWidgetItem8isHiddenEv()};
    let mut ret = unsafe {_ZNK15QListWidgetItem8isHiddenEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QListWidgetItem {
  pub fn setData<RetType, T: QListWidgetItem_setData<RetType>>(&mut self, value: T) -> RetType {
    return value.setData(self);
    // return 1;
  }
}

pub trait QListWidgetItem_setData<RetType> {
  fn setData(self, rsthis: &mut QListWidgetItem) -> RetType;
}

// proto:  void QListWidgetItem::setData(int role, const QVariant & value);
impl<'a> /*trait*/ QListWidgetItem_setData<()> for (i32, &'a  QVariant) {
  fn setData(self, rsthis: &mut QListWidgetItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QListWidgetItem7setDataEiRK8QVariant()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN15QListWidgetItem7setDataEiRK8QVariant(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QListWidgetItem {
  pub fn setBackground<RetType, T: QListWidgetItem_setBackground<RetType>>(&mut self, value: T) -> RetType {
    return value.setBackground(self);
    // return 1;
  }
}

pub trait QListWidgetItem_setBackground<RetType> {
  fn setBackground(self, rsthis: &mut QListWidgetItem) -> RetType;
}

// proto:  void QListWidgetItem::setBackground(const QBrush & brush);
impl<'a> /*trait*/ QListWidgetItem_setBackground<()> for (&'a  QBrush) {
  fn setBackground(self, rsthis: &mut QListWidgetItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QListWidgetItem13setBackgroundERK6QBrush()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QListWidgetItem13setBackgroundERK6QBrush(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QListWidgetItem {
  pub fn setSelected<RetType, T: QListWidgetItem_setSelected<RetType>>(&mut self, value: T) -> RetType {
    return value.setSelected(self);
    // return 1;
  }
}

pub trait QListWidgetItem_setSelected<RetType> {
  fn setSelected(self, rsthis: &mut QListWidgetItem) -> RetType;
}

// proto:  void QListWidgetItem::setSelected(bool select);
impl<'a> /*trait*/ QListWidgetItem_setSelected<()> for (i8) {
  fn setSelected(self, rsthis: &mut QListWidgetItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QListWidgetItem11setSelectedEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN15QListWidgetItem11setSelectedEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QListWidgetItem {
  pub fn font<RetType, T: QListWidgetItem_font<RetType>>(&mut self, value: T) -> RetType {
    return value.font(self);
    // return 1;
  }
}

pub trait QListWidgetItem_font<RetType> {
  fn font(self, rsthis: &mut QListWidgetItem) -> RetType;
}

// proto:  QFont QListWidgetItem::font();
impl<'a> /*trait*/ QListWidgetItem_font<QFont> for () {
  fn font(self, rsthis: &mut QListWidgetItem) -> QFont {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QListWidgetItem4fontEv()};
    let mut ret = unsafe {_ZNK15QListWidgetItem4fontEv(rsthis.qclsinst)};
    let mut ret1 = QFont{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QListWidgetItem {
  pub fn setTextAlignment<RetType, T: QListWidgetItem_setTextAlignment<RetType>>(&mut self, value: T) -> RetType {
    return value.setTextAlignment(self);
    // return 1;
  }
}

pub trait QListWidgetItem_setTextAlignment<RetType> {
  fn setTextAlignment(self, rsthis: &mut QListWidgetItem) -> RetType;
}

// proto:  void QListWidgetItem::setTextAlignment(int alignment);
impl<'a> /*trait*/ QListWidgetItem_setTextAlignment<()> for (i32) {
  fn setTextAlignment(self, rsthis: &mut QListWidgetItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QListWidgetItem16setTextAlignmentEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN15QListWidgetItem16setTextAlignmentEi(rsthis.qclsinst, arg0)};
    // return 1;
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
  pub fn write<RetType, T: QListWidgetItem_write<RetType>>(&mut self, value: T) -> RetType {
    return value.write(self);
    // return 1;
  }
}

pub trait QListWidgetItem_write<RetType> {
  fn write(self, rsthis: &mut QListWidgetItem) -> RetType;
}

// proto:  void QListWidgetItem::write(QDataStream & out);
impl<'a> /*trait*/ QListWidgetItem_write<()> for (&'a mut QDataStream) {
  fn write(self, rsthis: &mut QListWidgetItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QListWidgetItem5writeER11QDataStream()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZNK15QListWidgetItem5writeER11QDataStream(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QListWidgetItem {
  pub fn whatsThis<RetType, T: QListWidgetItem_whatsThis<RetType>>(&mut self, value: T) -> RetType {
    return value.whatsThis(self);
    // return 1;
  }
}

pub trait QListWidgetItem_whatsThis<RetType> {
  fn whatsThis(self, rsthis: &mut QListWidgetItem) -> RetType;
}

// proto:  QString QListWidgetItem::whatsThis();
impl<'a> /*trait*/ QListWidgetItem_whatsThis<QString> for () {
  fn whatsThis(self, rsthis: &mut QListWidgetItem) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QListWidgetItem9whatsThisEv()};
    let mut ret = unsafe {_ZNK15QListWidgetItem9whatsThisEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QListWidgetItem {
  pub fn type_<RetType, T: QListWidgetItem_type_<RetType>>(&mut self, value: T) -> RetType {
    return value.type_(self);
    // return 1;
  }
}

pub trait QListWidgetItem_type_<RetType> {
  fn type_(self, rsthis: &mut QListWidgetItem) -> RetType;
}

// proto:  int QListWidgetItem::type_();
impl<'a> /*trait*/ QListWidgetItem_type_<i32> for () {
  fn type_(self, rsthis: &mut QListWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QListWidgetItem4typeEv()};
    let mut ret = unsafe {_ZNK15QListWidgetItem4typeEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

// proto: void QListWidgetItem::NewQListWidgetItem(const QIcon & icon, const QString & text, QListWidget * view, int type);
impl<'a> /*trait*/ QListWidgetItem_NewQListWidgetItem for (&'a  QIcon, &'a  QString, &'a mut QListWidget, i32) {
  fn NewQListWidgetItem(self) -> QListWidgetItem {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QListWidgetItemC1ERK5QIconRK7QStringP11QListWidgeti()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let arg3 = self.3  as c_int;
    unsafe {_ZN15QListWidgetItemC1ERK5QIconRK7QStringP11QListWidgeti(qthis, arg0, arg1, arg2, arg3)};
    let rsthis = QListWidgetItem{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QListWidgetItem {
  pub fn icon<RetType, T: QListWidgetItem_icon<RetType>>(&mut self, value: T) -> RetType {
    return value.icon(self);
    // return 1;
  }
}

pub trait QListWidgetItem_icon<RetType> {
  fn icon(self, rsthis: &mut QListWidgetItem) -> RetType;
}

// proto:  QIcon QListWidgetItem::icon();
impl<'a> /*trait*/ QListWidgetItem_icon<QIcon> for () {
  fn icon(self, rsthis: &mut QListWidgetItem) -> QIcon {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QListWidgetItem4iconEv()};
    let mut ret = unsafe {_ZNK15QListWidgetItem4iconEv(rsthis.qclsinst)};
    let mut ret1 = QIcon{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QListWidgetItem {
  pub fn textColor<RetType, T: QListWidgetItem_textColor<RetType>>(&mut self, value: T) -> RetType {
    return value.textColor(self);
    // return 1;
  }
}

pub trait QListWidgetItem_textColor<RetType> {
  fn textColor(self, rsthis: &mut QListWidgetItem) -> RetType;
}

// proto:  QColor QListWidgetItem::textColor();
impl<'a> /*trait*/ QListWidgetItem_textColor<QColor> for () {
  fn textColor(self, rsthis: &mut QListWidgetItem) -> QColor {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QListWidgetItem9textColorEv()};
    let mut ret = unsafe {_ZNK15QListWidgetItem9textColorEv(rsthis.qclsinst)};
    let mut ret1 = QColor{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QListWidgetItem {
  pub fn foreground<RetType, T: QListWidgetItem_foreground<RetType>>(&mut self, value: T) -> RetType {
    return value.foreground(self);
    // return 1;
  }
}

pub trait QListWidgetItem_foreground<RetType> {
  fn foreground(self, rsthis: &mut QListWidgetItem) -> RetType;
}

// proto:  QBrush QListWidgetItem::foreground();
impl<'a> /*trait*/ QListWidgetItem_foreground<QBrush> for () {
  fn foreground(self, rsthis: &mut QListWidgetItem) -> QBrush {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QListWidgetItem10foregroundEv()};
    let mut ret = unsafe {_ZNK15QListWidgetItem10foregroundEv(rsthis.qclsinst)};
    let mut ret1 = QBrush{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QListWidgetItem {
  pub fn background<RetType, T: QListWidgetItem_background<RetType>>(&mut self, value: T) -> RetType {
    return value.background(self);
    // return 1;
  }
}

pub trait QListWidgetItem_background<RetType> {
  fn background(self, rsthis: &mut QListWidgetItem) -> RetType;
}

// proto:  QBrush QListWidgetItem::background();
impl<'a> /*trait*/ QListWidgetItem_background<QBrush> for () {
  fn background(self, rsthis: &mut QListWidgetItem) -> QBrush {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QListWidgetItem10backgroundEv()};
    let mut ret = unsafe {_ZNK15QListWidgetItem10backgroundEv(rsthis.qclsinst)};
    let mut ret1 = QBrush{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QListWidgetItem {
  pub fn setStatusTip<RetType, T: QListWidgetItem_setStatusTip<RetType>>(&mut self, value: T) -> RetType {
    return value.setStatusTip(self);
    // return 1;
  }
}

pub trait QListWidgetItem_setStatusTip<RetType> {
  fn setStatusTip(self, rsthis: &mut QListWidgetItem) -> RetType;
}

// proto:  void QListWidgetItem::setStatusTip(const QString & statusTip);
impl<'a> /*trait*/ QListWidgetItem_setStatusTip<()> for (&'a  QString) {
  fn setStatusTip(self, rsthis: &mut QListWidgetItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QListWidgetItem12setStatusTipERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QListWidgetItem12setStatusTipERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QListWidgetItem {
  pub fn text<RetType, T: QListWidgetItem_text<RetType>>(&mut self, value: T) -> RetType {
    return value.text(self);
    // return 1;
  }
}

pub trait QListWidgetItem_text<RetType> {
  fn text(self, rsthis: &mut QListWidgetItem) -> RetType;
}

// proto:  QString QListWidgetItem::text();
impl<'a> /*trait*/ QListWidgetItem_text<QString> for () {
  fn text(self, rsthis: &mut QListWidgetItem) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QListWidgetItem4textEv()};
    let mut ret = unsafe {_ZNK15QListWidgetItem4textEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QListWidgetItem {
  pub fn backgroundColor<RetType, T: QListWidgetItem_backgroundColor<RetType>>(&mut self, value: T) -> RetType {
    return value.backgroundColor(self);
    // return 1;
  }
}

pub trait QListWidgetItem_backgroundColor<RetType> {
  fn backgroundColor(self, rsthis: &mut QListWidgetItem) -> RetType;
}

// proto:  QColor QListWidgetItem::backgroundColor();
impl<'a> /*trait*/ QListWidgetItem_backgroundColor<QColor> for () {
  fn backgroundColor(self, rsthis: &mut QListWidgetItem) -> QColor {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QListWidgetItem15backgroundColorEv()};
    let mut ret = unsafe {_ZNK15QListWidgetItem15backgroundColorEv(rsthis.qclsinst)};
    let mut ret1 = QColor{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QListWidgetItem {
  pub fn isSelected<RetType, T: QListWidgetItem_isSelected<RetType>>(&mut self, value: T) -> RetType {
    return value.isSelected(self);
    // return 1;
  }
}

pub trait QListWidgetItem_isSelected<RetType> {
  fn isSelected(self, rsthis: &mut QListWidgetItem) -> RetType;
}

// proto:  bool QListWidgetItem::isSelected();
impl<'a> /*trait*/ QListWidgetItem_isSelected<i8> for () {
  fn isSelected(self, rsthis: &mut QListWidgetItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QListWidgetItem10isSelectedEv()};
    let mut ret = unsafe {_ZNK15QListWidgetItem10isSelectedEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QListWidgetItem {
  pub fn setFont<RetType, T: QListWidgetItem_setFont<RetType>>(&mut self, value: T) -> RetType {
    return value.setFont(self);
    // return 1;
  }
}

pub trait QListWidgetItem_setFont<RetType> {
  fn setFont(self, rsthis: &mut QListWidgetItem) -> RetType;
}

// proto:  void QListWidgetItem::setFont(const QFont & font);
impl<'a> /*trait*/ QListWidgetItem_setFont<()> for (&'a  QFont) {
  fn setFont(self, rsthis: &mut QListWidgetItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QListWidgetItem7setFontERK5QFont()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QListWidgetItem7setFontERK5QFont(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QListWidgetItem {
  pub fn setText<RetType, T: QListWidgetItem_setText<RetType>>(&mut self, value: T) -> RetType {
    return value.setText(self);
    // return 1;
  }
}

pub trait QListWidgetItem_setText<RetType> {
  fn setText(self, rsthis: &mut QListWidgetItem) -> RetType;
}

// proto:  void QListWidgetItem::setText(const QString & text);
impl<'a> /*trait*/ QListWidgetItem_setText<()> for (&'a  QString) {
  fn setText(self, rsthis: &mut QListWidgetItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QListWidgetItem7setTextERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QListWidgetItem7setTextERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto: void QListWidgetItem::NewQListWidgetItem(const QString & text, QListWidget * view, int type);
impl<'a> /*trait*/ QListWidgetItem_NewQListWidgetItem for (&'a  QString, &'a mut QListWidget, i32) {
  fn NewQListWidgetItem(self) -> QListWidgetItem {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QListWidgetItemC1ERK7QStringP11QListWidgeti()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2  as c_int;
    unsafe {_ZN15QListWidgetItemC1ERK7QStringP11QListWidgeti(qthis, arg0, arg1, arg2)};
    let rsthis = QListWidgetItem{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QListWidgetItem {
  pub fn data<RetType, T: QListWidgetItem_data<RetType>>(&mut self, value: T) -> RetType {
    return value.data(self);
    // return 1;
  }
}

pub trait QListWidgetItem_data<RetType> {
  fn data(self, rsthis: &mut QListWidgetItem) -> RetType;
}

// proto:  QVariant QListWidgetItem::data(int role);
impl<'a> /*trait*/ QListWidgetItem_data<QVariant> for (i32) {
  fn data(self, rsthis: &mut QListWidgetItem) -> QVariant {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QListWidgetItem4dataEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK15QListWidgetItem4dataEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QVariant{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QListWidgetItem {
  pub fn sizeHint<RetType, T: QListWidgetItem_sizeHint<RetType>>(&mut self, value: T) -> RetType {
    return value.sizeHint(self);
    // return 1;
  }
}

pub trait QListWidgetItem_sizeHint<RetType> {
  fn sizeHint(self, rsthis: &mut QListWidgetItem) -> RetType;
}

// proto:  QSize QListWidgetItem::sizeHint();
impl<'a> /*trait*/ QListWidgetItem_sizeHint<QSize> for () {
  fn sizeHint(self, rsthis: &mut QListWidgetItem) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QListWidgetItem8sizeHintEv()};
    let mut ret = unsafe {_ZNK15QListWidgetItem8sizeHintEv(rsthis.qclsinst)};
    let mut ret1 = QSize{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QListWidgetItem {
  pub fn setWhatsThis<RetType, T: QListWidgetItem_setWhatsThis<RetType>>(&mut self, value: T) -> RetType {
    return value.setWhatsThis(self);
    // return 1;
  }
}

pub trait QListWidgetItem_setWhatsThis<RetType> {
  fn setWhatsThis(self, rsthis: &mut QListWidgetItem) -> RetType;
}

// proto:  void QListWidgetItem::setWhatsThis(const QString & whatsThis);
impl<'a> /*trait*/ QListWidgetItem_setWhatsThis<()> for (&'a  QString) {
  fn setWhatsThis(self, rsthis: &mut QListWidgetItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QListWidgetItem12setWhatsThisERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QListWidgetItem12setWhatsThisERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QListWidgetItem {
  pub fn read<RetType, T: QListWidgetItem_read<RetType>>(&mut self, value: T) -> RetType {
    return value.read(self);
    // return 1;
  }
}

pub trait QListWidgetItem_read<RetType> {
  fn read(self, rsthis: &mut QListWidgetItem) -> RetType;
}

// proto:  void QListWidgetItem::read(QDataStream & in);
impl<'a> /*trait*/ QListWidgetItem_read<()> for (&'a mut QDataStream) {
  fn read(self, rsthis: &mut QListWidgetItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QListWidgetItem4readER11QDataStream()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QListWidgetItem4readER11QDataStream(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QListWidgetItem {
  pub fn setTextColor<RetType, T: QListWidgetItem_setTextColor<RetType>>(&mut self, value: T) -> RetType {
    return value.setTextColor(self);
    // return 1;
  }
}

pub trait QListWidgetItem_setTextColor<RetType> {
  fn setTextColor(self, rsthis: &mut QListWidgetItem) -> RetType;
}

// proto:  void QListWidgetItem::setTextColor(const QColor & color);
impl<'a> /*trait*/ QListWidgetItem_setTextColor<()> for (&'a  QColor) {
  fn setTextColor(self, rsthis: &mut QListWidgetItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QListWidgetItem12setTextColorERK6QColor()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QListWidgetItem12setTextColorERK6QColor(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QListWidgetItem {
  pub fn setSizeHint<RetType, T: QListWidgetItem_setSizeHint<RetType>>(&mut self, value: T) -> RetType {
    return value.setSizeHint(self);
    // return 1;
  }
}

pub trait QListWidgetItem_setSizeHint<RetType> {
  fn setSizeHint(self, rsthis: &mut QListWidgetItem) -> RetType;
}

// proto:  void QListWidgetItem::setSizeHint(const QSize & size);
impl<'a> /*trait*/ QListWidgetItem_setSizeHint<()> for (&'a  QSize) {
  fn setSizeHint(self, rsthis: &mut QListWidgetItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QListWidgetItem11setSizeHintERK5QSize()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QListWidgetItem11setSizeHintERK5QSize(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QListWidgetItem {
  pub fn listWidget<RetType, T: QListWidgetItem_listWidget<RetType>>(&mut self, value: T) -> RetType {
    return value.listWidget(self);
    // return 1;
  }
}

pub trait QListWidgetItem_listWidget<RetType> {
  fn listWidget(self, rsthis: &mut QListWidgetItem) -> RetType;
}

// proto:  QListWidget * QListWidgetItem::listWidget();
impl<'a> /*trait*/ QListWidgetItem_listWidget<QListWidget> for () {
  fn listWidget(self, rsthis: &mut QListWidgetItem) -> QListWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QListWidgetItem10listWidgetEv()};
    let mut ret = unsafe {_ZNK15QListWidgetItem10listWidgetEv(rsthis.qclsinst)};
    let mut ret1 = QListWidget{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QListWidgetItem {
  pub fn setIcon<RetType, T: QListWidgetItem_setIcon<RetType>>(&mut self, value: T) -> RetType {
    return value.setIcon(self);
    // return 1;
  }
}

pub trait QListWidgetItem_setIcon<RetType> {
  fn setIcon(self, rsthis: &mut QListWidgetItem) -> RetType;
}

// proto:  void QListWidgetItem::setIcon(const QIcon & icon);
impl<'a> /*trait*/ QListWidgetItem_setIcon<()> for (&'a  QIcon) {
  fn setIcon(self, rsthis: &mut QListWidgetItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QListWidgetItem7setIconERK5QIcon()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QListWidgetItem7setIconERK5QIcon(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QListWidgetItem {
  pub fn clone<RetType, T: QListWidgetItem_clone<RetType>>(&mut self, value: T) -> RetType {
    return value.clone(self);
    // return 1;
  }
}

pub trait QListWidgetItem_clone<RetType> {
  fn clone(self, rsthis: &mut QListWidgetItem) -> RetType;
}

// proto:  QListWidgetItem * QListWidgetItem::clone();
impl<'a> /*trait*/ QListWidgetItem_clone<QListWidgetItem> for () {
  fn clone(self, rsthis: &mut QListWidgetItem) -> QListWidgetItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QListWidgetItem5cloneEv()};
    let mut ret = unsafe {_ZNK15QListWidgetItem5cloneEv(rsthis.qclsinst)};
    let mut ret1 = QListWidgetItem{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QListWidgetItem {
  pub fn setBackgroundColor<RetType, T: QListWidgetItem_setBackgroundColor<RetType>>(&mut self, value: T) -> RetType {
    return value.setBackgroundColor(self);
    // return 1;
  }
}

pub trait QListWidgetItem_setBackgroundColor<RetType> {
  fn setBackgroundColor(self, rsthis: &mut QListWidgetItem) -> RetType;
}

// proto:  void QListWidgetItem::setBackgroundColor(const QColor & color);
impl<'a> /*trait*/ QListWidgetItem_setBackgroundColor<()> for (&'a  QColor) {
  fn setBackgroundColor(self, rsthis: &mut QListWidgetItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QListWidgetItem18setBackgroundColorERK6QColor()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QListWidgetItem18setBackgroundColorERK6QColor(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QListWidgetItem {
  pub fn setForeground<RetType, T: QListWidgetItem_setForeground<RetType>>(&mut self, value: T) -> RetType {
    return value.setForeground(self);
    // return 1;
  }
}

pub trait QListWidgetItem_setForeground<RetType> {
  fn setForeground(self, rsthis: &mut QListWidgetItem) -> RetType;
}

// proto:  void QListWidgetItem::setForeground(const QBrush & brush);
impl<'a> /*trait*/ QListWidgetItem_setForeground<()> for (&'a  QBrush) {
  fn setForeground(self, rsthis: &mut QListWidgetItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QListWidgetItem13setForegroundERK6QBrush()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QListWidgetItem13setForegroundERK6QBrush(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto: void QListWidgetItem::NewQListWidgetItem(const QListWidgetItem & other);
impl<'a> /*trait*/ QListWidgetItem_NewQListWidgetItem for (&'a  QListWidgetItem) {
  fn NewQListWidgetItem(self) -> QListWidgetItem {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QListWidgetItemC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN15QListWidgetItemC1ERKS_(qthis, arg0)};
    let rsthis = QListWidgetItem{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QListWidgetItem {
  pub fn setHidden<RetType, T: QListWidgetItem_setHidden<RetType>>(&mut self, value: T) -> RetType {
    return value.setHidden(self);
    // return 1;
  }
}

pub trait QListWidgetItem_setHidden<RetType> {
  fn setHidden(self, rsthis: &mut QListWidgetItem) -> RetType;
}

// proto:  void QListWidgetItem::setHidden(bool hide);
impl<'a> /*trait*/ QListWidgetItem_setHidden<()> for (i8) {
  fn setHidden(self, rsthis: &mut QListWidgetItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QListWidgetItem9setHiddenEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN15QListWidgetItem9setHiddenEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QListWidgetItem {
  pub fn toolTip<RetType, T: QListWidgetItem_toolTip<RetType>>(&mut self, value: T) -> RetType {
    return value.toolTip(self);
    // return 1;
  }
}

pub trait QListWidgetItem_toolTip<RetType> {
  fn toolTip(self, rsthis: &mut QListWidgetItem) -> RetType;
}

// proto:  QString QListWidgetItem::toolTip();
impl<'a> /*trait*/ QListWidgetItem_toolTip<QString> for () {
  fn toolTip(self, rsthis: &mut QListWidgetItem) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QListWidgetItem7toolTipEv()};
    let mut ret = unsafe {_ZNK15QListWidgetItem7toolTipEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QListWidgetItem {
  pub fn textAlignment<RetType, T: QListWidgetItem_textAlignment<RetType>>(&mut self, value: T) -> RetType {
    return value.textAlignment(self);
    // return 1;
  }
}

pub trait QListWidgetItem_textAlignment<RetType> {
  fn textAlignment(self, rsthis: &mut QListWidgetItem) -> RetType;
}

// proto:  int QListWidgetItem::textAlignment();
impl<'a> /*trait*/ QListWidgetItem_textAlignment<i32> for () {
  fn textAlignment(self, rsthis: &mut QListWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QListWidgetItem13textAlignmentEv()};
    let mut ret = unsafe {_ZNK15QListWidgetItem13textAlignmentEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QListWidgetItem {
  pub fn statusTip<RetType, T: QListWidgetItem_statusTip<RetType>>(&mut self, value: T) -> RetType {
    return value.statusTip(self);
    // return 1;
  }
}

pub trait QListWidgetItem_statusTip<RetType> {
  fn statusTip(self, rsthis: &mut QListWidgetItem) -> RetType;
}

// proto:  QString QListWidgetItem::statusTip();
impl<'a> /*trait*/ QListWidgetItem_statusTip<QString> for () {
  fn statusTip(self, rsthis: &mut QListWidgetItem) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QListWidgetItem9statusTipEv()};
    let mut ret = unsafe {_ZNK15QListWidgetItem9statusTipEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QListWidgetItem {
  pub fn setToolTip<RetType, T: QListWidgetItem_setToolTip<RetType>>(&mut self, value: T) -> RetType {
    return value.setToolTip(self);
    // return 1;
  }
}

pub trait QListWidgetItem_setToolTip<RetType> {
  fn setToolTip(self, rsthis: &mut QListWidgetItem) -> RetType;
}

// proto:  void QListWidgetItem::setToolTip(const QString & toolTip);
impl<'a> /*trait*/ QListWidgetItem_setToolTip<()> for (&'a  QString) {
  fn setToolTip(self, rsthis: &mut QListWidgetItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QListWidgetItem10setToolTipERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QListWidgetItem10setToolTipERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

