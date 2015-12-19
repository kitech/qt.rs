// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qcolor::QColor;
use super::qvariant::QVariant;
use super::qstring::QString;
use super::qsize::QSize;
use super::qbrush::QBrush;
use super::qfont::QFont;
use super::qicon::QIcon;
use super::qdatastream::QDataStream;
use super::qtablewidget::QTableWidget;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  QColor QTableWidgetItem::backgroundColor();
  fn _ZNK16QTableWidgetItem15backgroundColorEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QVariant QTableWidgetItem::data(int role);
  fn _ZNK16QTableWidgetItem4dataEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  void QTableWidgetItem::setSelected(bool select);
  fn _ZN16QTableWidgetItem11setSelectedEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  void QTableWidgetItem::setStatusTip(const QString & statusTip);
  fn _ZN16QTableWidgetItem12setStatusTipERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QColor QTableWidgetItem::textColor();
  fn _ZNK16QTableWidgetItem9textColorEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QTableWidgetItem::FreeQTableWidgetItem();
  fn _ZN16QTableWidgetItemD0Ev(qthis: *mut c_void) ;
  // proto:  QString QTableWidgetItem::text();
  fn _ZNK16QTableWidgetItem4textEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QTableWidgetItem::setSizeHint(const QSize & size);
  fn _ZN16QTableWidgetItem11setSizeHintERK5QSize(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QBrush QTableWidgetItem::foreground();
  fn _ZNK16QTableWidgetItem10foregroundEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  int QTableWidgetItem::type_();
  fn _ZNK16QTableWidgetItem4typeEv(qthis: *mut c_void) -> c_int;
  // proto:  int QTableWidgetItem::column();
  fn _ZNK16QTableWidgetItem6columnEv(qthis: *mut c_void) -> c_int;
  // proto:  void QTableWidgetItem::setTextAlignment(int alignment);
  fn _ZN16QTableWidgetItem16setTextAlignmentEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  QFont QTableWidgetItem::font();
  fn _ZNK16QTableWidgetItem4fontEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QIcon QTableWidgetItem::icon();
  fn _ZNK16QTableWidgetItem4iconEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QTableWidgetItem::write(QDataStream & out);
  fn _ZNK16QTableWidgetItem5writeER11QDataStream(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QTableWidgetItem::NewQTableWidgetItem(const QTableWidgetItem & other);
  fn _ZN16QTableWidgetItemC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QBrush QTableWidgetItem::background();
  fn _ZNK16QTableWidgetItem10backgroundEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QTableWidgetItem::setIcon(const QIcon & icon);
  fn _ZN16QTableWidgetItem7setIconERK5QIcon(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QTableWidgetItem::NewQTableWidgetItem(const QString & text, int type);
  fn _ZN16QTableWidgetItemC1ERK7QStringi(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int) ;
  // proto:  QString QTableWidgetItem::statusTip();
  fn _ZNK16QTableWidgetItem9statusTipEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QTableWidgetItem * QTableWidgetItem::clone();
  fn _ZNK16QTableWidgetItem5cloneEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QTableWidgetItem::NewQTableWidgetItem(int type);
  fn _ZN16QTableWidgetItemC1Ei(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QTableWidgetItem::setWhatsThis(const QString & whatsThis);
  fn _ZN16QTableWidgetItem12setWhatsThisERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QSize QTableWidgetItem::sizeHint();
  fn _ZNK16QTableWidgetItem8sizeHintEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QTableWidgetItem::setForeground(const QBrush & brush);
  fn _ZN16QTableWidgetItem13setForegroundERK6QBrush(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  int QTableWidgetItem::row();
  fn _ZNK16QTableWidgetItem3rowEv(qthis: *mut c_void) -> c_int;
  // proto:  void QTableWidgetItem::setData(int role, const QVariant & value);
  fn _ZN16QTableWidgetItem7setDataEiRK8QVariant(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void) ;
  // proto:  QTableWidget * QTableWidgetItem::tableWidget();
  fn _ZNK16QTableWidgetItem11tableWidgetEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QTableWidgetItem::NewQTableWidgetItem(const QIcon & icon, const QString & text, int type);
  fn _ZN16QTableWidgetItemC1ERK5QIconRK7QStringi(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: c_int) ;
  // proto:  int QTableWidgetItem::textAlignment();
  fn _ZNK16QTableWidgetItem13textAlignmentEv(qthis: *mut c_void) -> c_int;
  // proto:  void QTableWidgetItem::read(QDataStream & in);
  fn _ZN16QTableWidgetItem4readER11QDataStream(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QString QTableWidgetItem::toolTip();
  fn _ZNK16QTableWidgetItem7toolTipEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QTableWidgetItem::isSelected();
  fn _ZNK16QTableWidgetItem10isSelectedEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QTableWidgetItem::setBackgroundColor(const QColor & color);
  fn _ZN16QTableWidgetItem18setBackgroundColorERK6QColor(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QTableWidgetItem::setBackground(const QBrush & brush);
  fn _ZN16QTableWidgetItem13setBackgroundERK6QBrush(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QTableWidgetItem::setFont(const QFont & font);
  fn _ZN16QTableWidgetItem7setFontERK5QFont(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QTableWidgetItem::setTextColor(const QColor & color);
  fn _ZN16QTableWidgetItem12setTextColorERK6QColor(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QTableWidgetItem::setText(const QString & text);
  fn _ZN16QTableWidgetItem7setTextERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QString QTableWidgetItem::whatsThis();
  fn _ZNK16QTableWidgetItem9whatsThisEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QTableWidgetItem::setToolTip(const QString & toolTip);
  fn _ZN16QTableWidgetItem10setToolTipERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
}

// body block begin
// class sizeof(QTableWidgetItem)=1
pub struct QTableWidgetItem {
  pub qclsinst: *mut c_void,
}

// proto:  QColor QTableWidgetItem::backgroundColor();
impl /*struct*/ QTableWidgetItem {
  pub fn backgroundColor<RetType, T: QTableWidgetItem_backgroundColor<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.backgroundColor(self);
    // return 1;
  }
}

pub trait QTableWidgetItem_backgroundColor<RetType> {
  fn backgroundColor(self , rsthis: &mut QTableWidgetItem) -> RetType;
}

// proto:  QColor QTableWidgetItem::backgroundColor();
impl<'a> /*trait*/ QTableWidgetItem_backgroundColor<QColor> for () {
  fn backgroundColor(self , rsthis: &mut QTableWidgetItem) -> QColor {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTableWidgetItem15backgroundColorEv()};
    let mut ret = unsafe {_ZNK16QTableWidgetItem15backgroundColorEv(rsthis.qclsinst)};
    let mut ret1 = QColor{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  QVariant QTableWidgetItem::data(int role);
impl /*struct*/ QTableWidgetItem {
  pub fn data<RetType, T: QTableWidgetItem_data<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.data(self);
    // return 1;
  }
}

pub trait QTableWidgetItem_data<RetType> {
  fn data(self , rsthis: &mut QTableWidgetItem) -> RetType;
}

// proto:  QVariant QTableWidgetItem::data(int role);
impl<'a> /*trait*/ QTableWidgetItem_data<QVariant> for (i32) {
  fn data(self , rsthis: &mut QTableWidgetItem) -> QVariant {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTableWidgetItem4dataEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK16QTableWidgetItem4dataEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QVariant{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  void QTableWidgetItem::setSelected(bool select);
impl /*struct*/ QTableWidgetItem {
  pub fn setSelected<RetType, T: QTableWidgetItem_setSelected<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setSelected(self);
    // return 1;
  }
}

pub trait QTableWidgetItem_setSelected<RetType> {
  fn setSelected(self , rsthis: &mut QTableWidgetItem) -> RetType;
}

// proto:  void QTableWidgetItem::setSelected(bool select);
impl<'a> /*trait*/ QTableWidgetItem_setSelected<()> for (i8) {
  fn setSelected(self , rsthis: &mut QTableWidgetItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTableWidgetItem11setSelectedEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN16QTableWidgetItem11setSelectedEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  void QTableWidgetItem::setStatusTip(const QString & statusTip);
impl /*struct*/ QTableWidgetItem {
  pub fn setStatusTip<RetType, T: QTableWidgetItem_setStatusTip<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setStatusTip(self);
    // return 1;
  }
}

pub trait QTableWidgetItem_setStatusTip<RetType> {
  fn setStatusTip(self , rsthis: &mut QTableWidgetItem) -> RetType;
}

// proto:  void QTableWidgetItem::setStatusTip(const QString & statusTip);
impl<'a> /*trait*/ QTableWidgetItem_setStatusTip<()> for (&'a  QString) {
  fn setStatusTip(self , rsthis: &mut QTableWidgetItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTableWidgetItem12setStatusTipERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN16QTableWidgetItem12setStatusTipERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  QColor QTableWidgetItem::textColor();
impl /*struct*/ QTableWidgetItem {
  pub fn textColor<RetType, T: QTableWidgetItem_textColor<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.textColor(self);
    // return 1;
  }
}

pub trait QTableWidgetItem_textColor<RetType> {
  fn textColor(self , rsthis: &mut QTableWidgetItem) -> RetType;
}

// proto:  QColor QTableWidgetItem::textColor();
impl<'a> /*trait*/ QTableWidgetItem_textColor<QColor> for () {
  fn textColor(self , rsthis: &mut QTableWidgetItem) -> QColor {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTableWidgetItem9textColorEv()};
    let mut ret = unsafe {_ZNK16QTableWidgetItem9textColorEv(rsthis.qclsinst)};
    let mut ret1 = QColor{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  void QTableWidgetItem::FreeQTableWidgetItem();
impl /*struct*/ QTableWidgetItem {
  pub fn FreeQTableWidgetItem<RetType, T: QTableWidgetItem_FreeQTableWidgetItem<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.FreeQTableWidgetItem(self);
    // return 1;
  }
}

pub trait QTableWidgetItem_FreeQTableWidgetItem<RetType> {
  fn FreeQTableWidgetItem(self , rsthis: &mut QTableWidgetItem) -> RetType;
}

// proto:  void QTableWidgetItem::FreeQTableWidgetItem();
impl<'a> /*trait*/ QTableWidgetItem_FreeQTableWidgetItem<()> for () {
  fn FreeQTableWidgetItem(self , rsthis: &mut QTableWidgetItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTableWidgetItemD0Ev()};
     unsafe {_ZN16QTableWidgetItemD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

// proto:  QString QTableWidgetItem::text();
impl /*struct*/ QTableWidgetItem {
  pub fn text<RetType, T: QTableWidgetItem_text<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.text(self);
    // return 1;
  }
}

pub trait QTableWidgetItem_text<RetType> {
  fn text(self , rsthis: &mut QTableWidgetItem) -> RetType;
}

// proto:  QString QTableWidgetItem::text();
impl<'a> /*trait*/ QTableWidgetItem_text<QString> for () {
  fn text(self , rsthis: &mut QTableWidgetItem) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTableWidgetItem4textEv()};
    let mut ret = unsafe {_ZNK16QTableWidgetItem4textEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  void QTableWidgetItem::setSizeHint(const QSize & size);
impl /*struct*/ QTableWidgetItem {
  pub fn setSizeHint<RetType, T: QTableWidgetItem_setSizeHint<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setSizeHint(self);
    // return 1;
  }
}

pub trait QTableWidgetItem_setSizeHint<RetType> {
  fn setSizeHint(self , rsthis: &mut QTableWidgetItem) -> RetType;
}

// proto:  void QTableWidgetItem::setSizeHint(const QSize & size);
impl<'a> /*trait*/ QTableWidgetItem_setSizeHint<()> for (&'a  QSize) {
  fn setSizeHint(self , rsthis: &mut QTableWidgetItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTableWidgetItem11setSizeHintERK5QSize()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN16QTableWidgetItem11setSizeHintERK5QSize(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  QBrush QTableWidgetItem::foreground();
impl /*struct*/ QTableWidgetItem {
  pub fn foreground<RetType, T: QTableWidgetItem_foreground<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.foreground(self);
    // return 1;
  }
}

pub trait QTableWidgetItem_foreground<RetType> {
  fn foreground(self , rsthis: &mut QTableWidgetItem) -> RetType;
}

// proto:  QBrush QTableWidgetItem::foreground();
impl<'a> /*trait*/ QTableWidgetItem_foreground<QBrush> for () {
  fn foreground(self , rsthis: &mut QTableWidgetItem) -> QBrush {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTableWidgetItem10foregroundEv()};
    let mut ret = unsafe {_ZNK16QTableWidgetItem10foregroundEv(rsthis.qclsinst)};
    let mut ret1 = QBrush{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  int QTableWidgetItem::type_();
impl /*struct*/ QTableWidgetItem {
  pub fn type_<RetType, T: QTableWidgetItem_type_<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.type_(self);
    // return 1;
  }
}

pub trait QTableWidgetItem_type_<RetType> {
  fn type_(self , rsthis: &mut QTableWidgetItem) -> RetType;
}

// proto:  int QTableWidgetItem::type_();
impl<'a> /*trait*/ QTableWidgetItem_type_<i32> for () {
  fn type_(self , rsthis: &mut QTableWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTableWidgetItem4typeEv()};
    let mut ret = unsafe {_ZNK16QTableWidgetItem4typeEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

// proto:  int QTableWidgetItem::column();
impl /*struct*/ QTableWidgetItem {
  pub fn column<RetType, T: QTableWidgetItem_column<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.column(self);
    // return 1;
  }
}

pub trait QTableWidgetItem_column<RetType> {
  fn column(self , rsthis: &mut QTableWidgetItem) -> RetType;
}

// proto:  int QTableWidgetItem::column();
impl<'a> /*trait*/ QTableWidgetItem_column<i32> for () {
  fn column(self , rsthis: &mut QTableWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTableWidgetItem6columnEv()};
    let mut ret = unsafe {_ZNK16QTableWidgetItem6columnEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

// proto:  void QTableWidgetItem::setTextAlignment(int alignment);
impl /*struct*/ QTableWidgetItem {
  pub fn setTextAlignment<RetType, T: QTableWidgetItem_setTextAlignment<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setTextAlignment(self);
    // return 1;
  }
}

pub trait QTableWidgetItem_setTextAlignment<RetType> {
  fn setTextAlignment(self , rsthis: &mut QTableWidgetItem) -> RetType;
}

// proto:  void QTableWidgetItem::setTextAlignment(int alignment);
impl<'a> /*trait*/ QTableWidgetItem_setTextAlignment<()> for (i32) {
  fn setTextAlignment(self , rsthis: &mut QTableWidgetItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTableWidgetItem16setTextAlignmentEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN16QTableWidgetItem16setTextAlignmentEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  QFont QTableWidgetItem::font();
impl /*struct*/ QTableWidgetItem {
  pub fn font<RetType, T: QTableWidgetItem_font<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.font(self);
    // return 1;
  }
}

pub trait QTableWidgetItem_font<RetType> {
  fn font(self , rsthis: &mut QTableWidgetItem) -> RetType;
}

// proto:  QFont QTableWidgetItem::font();
impl<'a> /*trait*/ QTableWidgetItem_font<QFont> for () {
  fn font(self , rsthis: &mut QTableWidgetItem) -> QFont {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTableWidgetItem4fontEv()};
    let mut ret = unsafe {_ZNK16QTableWidgetItem4fontEv(rsthis.qclsinst)};
    let mut ret1 = QFont{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  QIcon QTableWidgetItem::icon();
impl /*struct*/ QTableWidgetItem {
  pub fn icon<RetType, T: QTableWidgetItem_icon<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.icon(self);
    // return 1;
  }
}

pub trait QTableWidgetItem_icon<RetType> {
  fn icon(self , rsthis: &mut QTableWidgetItem) -> RetType;
}

// proto:  QIcon QTableWidgetItem::icon();
impl<'a> /*trait*/ QTableWidgetItem_icon<QIcon> for () {
  fn icon(self , rsthis: &mut QTableWidgetItem) -> QIcon {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTableWidgetItem4iconEv()};
    let mut ret = unsafe {_ZNK16QTableWidgetItem4iconEv(rsthis.qclsinst)};
    let mut ret1 = QIcon{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  void QTableWidgetItem::write(QDataStream & out);
impl /*struct*/ QTableWidgetItem {
  pub fn write<RetType, T: QTableWidgetItem_write<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.write(self);
    // return 1;
  }
}

pub trait QTableWidgetItem_write<RetType> {
  fn write(self , rsthis: &mut QTableWidgetItem) -> RetType;
}

// proto:  void QTableWidgetItem::write(QDataStream & out);
impl<'a> /*trait*/ QTableWidgetItem_write<()> for (&'a mut QDataStream) {
  fn write(self , rsthis: &mut QTableWidgetItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTableWidgetItem5writeER11QDataStream()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZNK16QTableWidgetItem5writeER11QDataStream(rsthis.qclsinst, arg0)};
    // return 1;
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
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN16QTableWidgetItemC1ERKS_(qthis, arg0)};
    let rsthis = QTableWidgetItem{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto:  QBrush QTableWidgetItem::background();
impl /*struct*/ QTableWidgetItem {
  pub fn background<RetType, T: QTableWidgetItem_background<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.background(self);
    // return 1;
  }
}

pub trait QTableWidgetItem_background<RetType> {
  fn background(self , rsthis: &mut QTableWidgetItem) -> RetType;
}

// proto:  QBrush QTableWidgetItem::background();
impl<'a> /*trait*/ QTableWidgetItem_background<QBrush> for () {
  fn background(self , rsthis: &mut QTableWidgetItem) -> QBrush {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTableWidgetItem10backgroundEv()};
    let mut ret = unsafe {_ZNK16QTableWidgetItem10backgroundEv(rsthis.qclsinst)};
    let mut ret1 = QBrush{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  void QTableWidgetItem::setIcon(const QIcon & icon);
impl /*struct*/ QTableWidgetItem {
  pub fn setIcon<RetType, T: QTableWidgetItem_setIcon<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setIcon(self);
    // return 1;
  }
}

pub trait QTableWidgetItem_setIcon<RetType> {
  fn setIcon(self , rsthis: &mut QTableWidgetItem) -> RetType;
}

// proto:  void QTableWidgetItem::setIcon(const QIcon & icon);
impl<'a> /*trait*/ QTableWidgetItem_setIcon<()> for (&'a  QIcon) {
  fn setIcon(self , rsthis: &mut QTableWidgetItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTableWidgetItem7setIconERK5QIcon()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN16QTableWidgetItem7setIconERK5QIcon(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto: void QTableWidgetItem::NewQTableWidgetItem(const QString & text, int type);
impl<'a> /*trait*/ QTableWidgetItem_NewQTableWidgetItem for (&'a  QString, i32) {
  fn NewQTableWidgetItem(self) -> QTableWidgetItem {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTableWidgetItemC1ERK7QStringi()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    unsafe {_ZN16QTableWidgetItemC1ERK7QStringi(qthis, arg0, arg1)};
    let rsthis = QTableWidgetItem{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto:  QString QTableWidgetItem::statusTip();
impl /*struct*/ QTableWidgetItem {
  pub fn statusTip<RetType, T: QTableWidgetItem_statusTip<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.statusTip(self);
    // return 1;
  }
}

pub trait QTableWidgetItem_statusTip<RetType> {
  fn statusTip(self , rsthis: &mut QTableWidgetItem) -> RetType;
}

// proto:  QString QTableWidgetItem::statusTip();
impl<'a> /*trait*/ QTableWidgetItem_statusTip<QString> for () {
  fn statusTip(self , rsthis: &mut QTableWidgetItem) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTableWidgetItem9statusTipEv()};
    let mut ret = unsafe {_ZNK16QTableWidgetItem9statusTipEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  QTableWidgetItem * QTableWidgetItem::clone();
impl /*struct*/ QTableWidgetItem {
  pub fn clone<RetType, T: QTableWidgetItem_clone<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.clone(self);
    // return 1;
  }
}

pub trait QTableWidgetItem_clone<RetType> {
  fn clone(self , rsthis: &mut QTableWidgetItem) -> RetType;
}

// proto:  QTableWidgetItem * QTableWidgetItem::clone();
impl<'a> /*trait*/ QTableWidgetItem_clone<QTableWidgetItem> for () {
  fn clone(self , rsthis: &mut QTableWidgetItem) -> QTableWidgetItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTableWidgetItem5cloneEv()};
    let mut ret = unsafe {_ZNK16QTableWidgetItem5cloneEv(rsthis.qclsinst)};
    let mut ret1 = QTableWidgetItem{qclsinst: ret};
    return ret1;
    // return 1;
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

// proto:  void QTableWidgetItem::setWhatsThis(const QString & whatsThis);
impl /*struct*/ QTableWidgetItem {
  pub fn setWhatsThis<RetType, T: QTableWidgetItem_setWhatsThis<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setWhatsThis(self);
    // return 1;
  }
}

pub trait QTableWidgetItem_setWhatsThis<RetType> {
  fn setWhatsThis(self , rsthis: &mut QTableWidgetItem) -> RetType;
}

// proto:  void QTableWidgetItem::setWhatsThis(const QString & whatsThis);
impl<'a> /*trait*/ QTableWidgetItem_setWhatsThis<()> for (&'a  QString) {
  fn setWhatsThis(self , rsthis: &mut QTableWidgetItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTableWidgetItem12setWhatsThisERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN16QTableWidgetItem12setWhatsThisERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  QSize QTableWidgetItem::sizeHint();
impl /*struct*/ QTableWidgetItem {
  pub fn sizeHint<RetType, T: QTableWidgetItem_sizeHint<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.sizeHint(self);
    // return 1;
  }
}

pub trait QTableWidgetItem_sizeHint<RetType> {
  fn sizeHint(self , rsthis: &mut QTableWidgetItem) -> RetType;
}

// proto:  QSize QTableWidgetItem::sizeHint();
impl<'a> /*trait*/ QTableWidgetItem_sizeHint<QSize> for () {
  fn sizeHint(self , rsthis: &mut QTableWidgetItem) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTableWidgetItem8sizeHintEv()};
    let mut ret = unsafe {_ZNK16QTableWidgetItem8sizeHintEv(rsthis.qclsinst)};
    let mut ret1 = QSize{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  void QTableWidgetItem::setForeground(const QBrush & brush);
impl /*struct*/ QTableWidgetItem {
  pub fn setForeground<RetType, T: QTableWidgetItem_setForeground<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setForeground(self);
    // return 1;
  }
}

pub trait QTableWidgetItem_setForeground<RetType> {
  fn setForeground(self , rsthis: &mut QTableWidgetItem) -> RetType;
}

// proto:  void QTableWidgetItem::setForeground(const QBrush & brush);
impl<'a> /*trait*/ QTableWidgetItem_setForeground<()> for (&'a  QBrush) {
  fn setForeground(self , rsthis: &mut QTableWidgetItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTableWidgetItem13setForegroundERK6QBrush()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN16QTableWidgetItem13setForegroundERK6QBrush(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  int QTableWidgetItem::row();
impl /*struct*/ QTableWidgetItem {
  pub fn row<RetType, T: QTableWidgetItem_row<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.row(self);
    // return 1;
  }
}

pub trait QTableWidgetItem_row<RetType> {
  fn row(self , rsthis: &mut QTableWidgetItem) -> RetType;
}

// proto:  int QTableWidgetItem::row();
impl<'a> /*trait*/ QTableWidgetItem_row<i32> for () {
  fn row(self , rsthis: &mut QTableWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTableWidgetItem3rowEv()};
    let mut ret = unsafe {_ZNK16QTableWidgetItem3rowEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

// proto:  void QTableWidgetItem::setData(int role, const QVariant & value);
impl /*struct*/ QTableWidgetItem {
  pub fn setData<RetType, T: QTableWidgetItem_setData<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setData(self);
    // return 1;
  }
}

pub trait QTableWidgetItem_setData<RetType> {
  fn setData(self , rsthis: &mut QTableWidgetItem) -> RetType;
}

// proto:  void QTableWidgetItem::setData(int role, const QVariant & value);
impl<'a> /*trait*/ QTableWidgetItem_setData<()> for (i32, &'a  QVariant) {
  fn setData(self , rsthis: &mut QTableWidgetItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTableWidgetItem7setDataEiRK8QVariant()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN16QTableWidgetItem7setDataEiRK8QVariant(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

// proto:  QTableWidget * QTableWidgetItem::tableWidget();
impl /*struct*/ QTableWidgetItem {
  pub fn tableWidget<RetType, T: QTableWidgetItem_tableWidget<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.tableWidget(self);
    // return 1;
  }
}

pub trait QTableWidgetItem_tableWidget<RetType> {
  fn tableWidget(self , rsthis: &mut QTableWidgetItem) -> RetType;
}

// proto:  QTableWidget * QTableWidgetItem::tableWidget();
impl<'a> /*trait*/ QTableWidgetItem_tableWidget<QTableWidget> for () {
  fn tableWidget(self , rsthis: &mut QTableWidgetItem) -> QTableWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTableWidgetItem11tableWidgetEv()};
    let mut ret = unsafe {_ZNK16QTableWidgetItem11tableWidgetEv(rsthis.qclsinst)};
    let mut ret1 = QTableWidget{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto: void QTableWidgetItem::NewQTableWidgetItem(const QIcon & icon, const QString & text, int type);
impl<'a> /*trait*/ QTableWidgetItem_NewQTableWidgetItem for (&'a  QIcon, &'a  QString, i32) {
  fn NewQTableWidgetItem(self) -> QTableWidgetItem {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTableWidgetItemC1ERK5QIconRK7QStringi()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2  as c_int;
    unsafe {_ZN16QTableWidgetItemC1ERK5QIconRK7QStringi(qthis, arg0, arg1, arg2)};
    let rsthis = QTableWidgetItem{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto:  int QTableWidgetItem::textAlignment();
impl /*struct*/ QTableWidgetItem {
  pub fn textAlignment<RetType, T: QTableWidgetItem_textAlignment<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.textAlignment(self);
    // return 1;
  }
}

pub trait QTableWidgetItem_textAlignment<RetType> {
  fn textAlignment(self , rsthis: &mut QTableWidgetItem) -> RetType;
}

// proto:  int QTableWidgetItem::textAlignment();
impl<'a> /*trait*/ QTableWidgetItem_textAlignment<i32> for () {
  fn textAlignment(self , rsthis: &mut QTableWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTableWidgetItem13textAlignmentEv()};
    let mut ret = unsafe {_ZNK16QTableWidgetItem13textAlignmentEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

// proto:  void QTableWidgetItem::read(QDataStream & in);
impl /*struct*/ QTableWidgetItem {
  pub fn read<RetType, T: QTableWidgetItem_read<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.read(self);
    // return 1;
  }
}

pub trait QTableWidgetItem_read<RetType> {
  fn read(self , rsthis: &mut QTableWidgetItem) -> RetType;
}

// proto:  void QTableWidgetItem::read(QDataStream & in);
impl<'a> /*trait*/ QTableWidgetItem_read<()> for (&'a mut QDataStream) {
  fn read(self , rsthis: &mut QTableWidgetItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTableWidgetItem4readER11QDataStream()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN16QTableWidgetItem4readER11QDataStream(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  QString QTableWidgetItem::toolTip();
impl /*struct*/ QTableWidgetItem {
  pub fn toolTip<RetType, T: QTableWidgetItem_toolTip<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.toolTip(self);
    // return 1;
  }
}

pub trait QTableWidgetItem_toolTip<RetType> {
  fn toolTip(self , rsthis: &mut QTableWidgetItem) -> RetType;
}

// proto:  QString QTableWidgetItem::toolTip();
impl<'a> /*trait*/ QTableWidgetItem_toolTip<QString> for () {
  fn toolTip(self , rsthis: &mut QTableWidgetItem) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTableWidgetItem7toolTipEv()};
    let mut ret = unsafe {_ZNK16QTableWidgetItem7toolTipEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  bool QTableWidgetItem::isSelected();
impl /*struct*/ QTableWidgetItem {
  pub fn isSelected<RetType, T: QTableWidgetItem_isSelected<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.isSelected(self);
    // return 1;
  }
}

pub trait QTableWidgetItem_isSelected<RetType> {
  fn isSelected(self , rsthis: &mut QTableWidgetItem) -> RetType;
}

// proto:  bool QTableWidgetItem::isSelected();
impl<'a> /*trait*/ QTableWidgetItem_isSelected<i8> for () {
  fn isSelected(self , rsthis: &mut QTableWidgetItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTableWidgetItem10isSelectedEv()};
    let mut ret = unsafe {_ZNK16QTableWidgetItem10isSelectedEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

// proto:  void QTableWidgetItem::setBackgroundColor(const QColor & color);
impl /*struct*/ QTableWidgetItem {
  pub fn setBackgroundColor<RetType, T: QTableWidgetItem_setBackgroundColor<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setBackgroundColor(self);
    // return 1;
  }
}

pub trait QTableWidgetItem_setBackgroundColor<RetType> {
  fn setBackgroundColor(self , rsthis: &mut QTableWidgetItem) -> RetType;
}

// proto:  void QTableWidgetItem::setBackgroundColor(const QColor & color);
impl<'a> /*trait*/ QTableWidgetItem_setBackgroundColor<()> for (&'a  QColor) {
  fn setBackgroundColor(self , rsthis: &mut QTableWidgetItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTableWidgetItem18setBackgroundColorERK6QColor()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN16QTableWidgetItem18setBackgroundColorERK6QColor(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  void QTableWidgetItem::setBackground(const QBrush & brush);
impl /*struct*/ QTableWidgetItem {
  pub fn setBackground<RetType, T: QTableWidgetItem_setBackground<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setBackground(self);
    // return 1;
  }
}

pub trait QTableWidgetItem_setBackground<RetType> {
  fn setBackground(self , rsthis: &mut QTableWidgetItem) -> RetType;
}

// proto:  void QTableWidgetItem::setBackground(const QBrush & brush);
impl<'a> /*trait*/ QTableWidgetItem_setBackground<()> for (&'a  QBrush) {
  fn setBackground(self , rsthis: &mut QTableWidgetItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTableWidgetItem13setBackgroundERK6QBrush()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN16QTableWidgetItem13setBackgroundERK6QBrush(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  void QTableWidgetItem::setFont(const QFont & font);
impl /*struct*/ QTableWidgetItem {
  pub fn setFont<RetType, T: QTableWidgetItem_setFont<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setFont(self);
    // return 1;
  }
}

pub trait QTableWidgetItem_setFont<RetType> {
  fn setFont(self , rsthis: &mut QTableWidgetItem) -> RetType;
}

// proto:  void QTableWidgetItem::setFont(const QFont & font);
impl<'a> /*trait*/ QTableWidgetItem_setFont<()> for (&'a  QFont) {
  fn setFont(self , rsthis: &mut QTableWidgetItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTableWidgetItem7setFontERK5QFont()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN16QTableWidgetItem7setFontERK5QFont(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  void QTableWidgetItem::setTextColor(const QColor & color);
impl /*struct*/ QTableWidgetItem {
  pub fn setTextColor<RetType, T: QTableWidgetItem_setTextColor<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setTextColor(self);
    // return 1;
  }
}

pub trait QTableWidgetItem_setTextColor<RetType> {
  fn setTextColor(self , rsthis: &mut QTableWidgetItem) -> RetType;
}

// proto:  void QTableWidgetItem::setTextColor(const QColor & color);
impl<'a> /*trait*/ QTableWidgetItem_setTextColor<()> for (&'a  QColor) {
  fn setTextColor(self , rsthis: &mut QTableWidgetItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTableWidgetItem12setTextColorERK6QColor()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN16QTableWidgetItem12setTextColorERK6QColor(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  void QTableWidgetItem::setText(const QString & text);
impl /*struct*/ QTableWidgetItem {
  pub fn setText<RetType, T: QTableWidgetItem_setText<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setText(self);
    // return 1;
  }
}

pub trait QTableWidgetItem_setText<RetType> {
  fn setText(self , rsthis: &mut QTableWidgetItem) -> RetType;
}

// proto:  void QTableWidgetItem::setText(const QString & text);
impl<'a> /*trait*/ QTableWidgetItem_setText<()> for (&'a  QString) {
  fn setText(self , rsthis: &mut QTableWidgetItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTableWidgetItem7setTextERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN16QTableWidgetItem7setTextERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  QString QTableWidgetItem::whatsThis();
impl /*struct*/ QTableWidgetItem {
  pub fn whatsThis<RetType, T: QTableWidgetItem_whatsThis<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.whatsThis(self);
    // return 1;
  }
}

pub trait QTableWidgetItem_whatsThis<RetType> {
  fn whatsThis(self , rsthis: &mut QTableWidgetItem) -> RetType;
}

// proto:  QString QTableWidgetItem::whatsThis();
impl<'a> /*trait*/ QTableWidgetItem_whatsThis<QString> for () {
  fn whatsThis(self , rsthis: &mut QTableWidgetItem) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTableWidgetItem9whatsThisEv()};
    let mut ret = unsafe {_ZNK16QTableWidgetItem9whatsThisEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  void QTableWidgetItem::setToolTip(const QString & toolTip);
impl /*struct*/ QTableWidgetItem {
  pub fn setToolTip<RetType, T: QTableWidgetItem_setToolTip<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setToolTip(self);
    // return 1;
  }
}

pub trait QTableWidgetItem_setToolTip<RetType> {
  fn setToolTip(self , rsthis: &mut QTableWidgetItem) -> RetType;
}

// proto:  void QTableWidgetItem::setToolTip(const QString & toolTip);
impl<'a> /*trait*/ QTableWidgetItem_setToolTip<()> for (&'a  QString) {
  fn setToolTip(self , rsthis: &mut QTableWidgetItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTableWidgetItem10setToolTipERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN16QTableWidgetItem10setToolTipERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

