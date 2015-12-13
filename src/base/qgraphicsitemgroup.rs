// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qgraphicsitem::QGraphicsItem;
use super::qrectf::QRectF;
use super::qpainter::QPainter;
use super::qstyleoptiongraphicsitem::QStyleOptionGraphicsItem;
use super::qwidget::QWidget;
use super::qpainterpath::QPainterPath;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  bool QGraphicsItemGroup::isObscuredBy(const QGraphicsItem * item);
  fn _ZNK18QGraphicsItemGroup12isObscuredByEPK13QGraphicsItem(qthis: *mut c_void, arg0: *mut c_void) -> int8_t;
  // proto:  void QGraphicsItemGroup::FreeQGraphicsItemGroup();
  fn _ZN18QGraphicsItemGroupD0Ev(qthis: *mut c_void) ;
  // proto:  void QGraphicsItemGroup::NewQGraphicsItemGroup(QGraphicsItem * parent);
  fn _ZN18QGraphicsItemGroupC1EP13QGraphicsItem(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  int QGraphicsItemGroup::type_();
  fn _ZNK18QGraphicsItemGroup4typeEv(qthis: *mut c_void) -> c_int;
  // proto:  QRectF QGraphicsItemGroup::boundingRect();
  fn _ZNK18QGraphicsItemGroup12boundingRectEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QGraphicsItemGroup::paint(QPainter * painter, const QStyleOptionGraphicsItem * option, QWidget * widget);
  fn _ZN18QGraphicsItemGroup5paintEP8QPainterPK24QStyleOptionGraphicsItemP7QWidget(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void) ;
  // proto:  void QGraphicsItemGroup::removeFromGroup(QGraphicsItem * item);
  fn _ZN18QGraphicsItemGroup15removeFromGroupEP13QGraphicsItem(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QGraphicsItemGroup::addToGroup(QGraphicsItem * item);
  fn _ZN18QGraphicsItemGroup10addToGroupEP13QGraphicsItem(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QPainterPath QGraphicsItemGroup::opaqueArea();
  fn _ZNK18QGraphicsItemGroup10opaqueAreaEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QGraphicsItemGroup::NewQGraphicsItemGroup(const QGraphicsItemGroup & );
  fn _ZN18QGraphicsItemGroupC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
}

// body block begin
// class sizeof(QGraphicsItemGroup)=1
pub struct QGraphicsItemGroup {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QGraphicsItemGroup {
  pub fn isObscuredBy<T: QGraphicsItemGroup_isObscuredBy>(&mut self, value: T) -> i8 {
    return value.isObscuredBy(self);
    // return 1;
  }
}

pub trait QGraphicsItemGroup_isObscuredBy {
  fn isObscuredBy(self, rsthis: &mut QGraphicsItemGroup) -> i8;
}

// proto:  bool QGraphicsItemGroup::isObscuredBy(const QGraphicsItem * item);
impl<'a> /*trait*/ QGraphicsItemGroup_isObscuredBy for (&'a  QGraphicsItem) {
  fn isObscuredBy(self, rsthis: &mut QGraphicsItemGroup) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QGraphicsItemGroup12isObscuredByEPK13QGraphicsItem()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK18QGraphicsItemGroup12isObscuredByEPK13QGraphicsItem(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QGraphicsItemGroup {
  pub fn FreeQGraphicsItemGroup<T: QGraphicsItemGroup_FreeQGraphicsItemGroup>(&mut self, value: T)  {
     value.FreeQGraphicsItemGroup(self);
    // return 1;
  }
}

pub trait QGraphicsItemGroup_FreeQGraphicsItemGroup {
  fn FreeQGraphicsItemGroup(self, rsthis: &mut QGraphicsItemGroup) ;
}

// proto:  void QGraphicsItemGroup::FreeQGraphicsItemGroup();
impl<'a> /*trait*/ QGraphicsItemGroup_FreeQGraphicsItemGroup for () {
  fn FreeQGraphicsItemGroup(self, rsthis: &mut QGraphicsItemGroup)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QGraphicsItemGroupD0Ev()};
     unsafe {_ZN18QGraphicsItemGroupD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsItemGroup {
  pub fn NewQGraphicsItemGroup<T: QGraphicsItemGroup_NewQGraphicsItemGroup>(value: T) -> QGraphicsItemGroup {
    let rsthis = value.NewQGraphicsItemGroup();
    return rsthis;
    // return 1;
  }
}

pub trait QGraphicsItemGroup_NewQGraphicsItemGroup {
  fn NewQGraphicsItemGroup(self) -> QGraphicsItemGroup;
}

// proto: void QGraphicsItemGroup::NewQGraphicsItemGroup(QGraphicsItem * parent);
impl<'a> /*trait*/ QGraphicsItemGroup_NewQGraphicsItemGroup for (&'a mut QGraphicsItem) {
  fn NewQGraphicsItemGroup(self) -> QGraphicsItemGroup {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QGraphicsItemGroupC1EP13QGraphicsItem()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN18QGraphicsItemGroupC1EP13QGraphicsItem(qthis, arg0)};
    let rsthis = QGraphicsItemGroup{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QGraphicsItemGroup {
  pub fn type_<T: QGraphicsItemGroup_type_>(&mut self, value: T) -> i32 {
    return value.type_(self);
    // return 1;
  }
}

pub trait QGraphicsItemGroup_type_ {
  fn type_(self, rsthis: &mut QGraphicsItemGroup) -> i32;
}

// proto:  int QGraphicsItemGroup::type_();
impl<'a> /*trait*/ QGraphicsItemGroup_type_ for () {
  fn type_(self, rsthis: &mut QGraphicsItemGroup) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QGraphicsItemGroup4typeEv()};
    let mut ret = unsafe {_ZNK18QGraphicsItemGroup4typeEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QGraphicsItemGroup {
  pub fn boundingRect<T: QGraphicsItemGroup_boundingRect>(&mut self, value: T) -> QRectF {
    return value.boundingRect(self);
    // return 1;
  }
}

pub trait QGraphicsItemGroup_boundingRect {
  fn boundingRect(self, rsthis: &mut QGraphicsItemGroup) -> QRectF;
}

// proto:  QRectF QGraphicsItemGroup::boundingRect();
impl<'a> /*trait*/ QGraphicsItemGroup_boundingRect for () {
  fn boundingRect(self, rsthis: &mut QGraphicsItemGroup) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QGraphicsItemGroup12boundingRectEv()};
    let mut ret = unsafe {_ZNK18QGraphicsItemGroup12boundingRectEv(rsthis.qclsinst)};
    let mut ret1 = QRectF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QGraphicsItemGroup {
  pub fn paint<T: QGraphicsItemGroup_paint>(&mut self, value: T)  {
     value.paint(self);
    // return 1;
  }
}

pub trait QGraphicsItemGroup_paint {
  fn paint(self, rsthis: &mut QGraphicsItemGroup) ;
}

// proto:  void QGraphicsItemGroup::paint(QPainter * painter, const QStyleOptionGraphicsItem * option, QWidget * widget);
impl<'a> /*trait*/ QGraphicsItemGroup_paint for (&'a mut QPainter, &'a  QStyleOptionGraphicsItem, &'a mut QWidget) {
  fn paint(self, rsthis: &mut QGraphicsItemGroup)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QGraphicsItemGroup5paintEP8QPainterPK24QStyleOptionGraphicsItemP7QWidget()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
     unsafe {_ZN18QGraphicsItemGroup5paintEP8QPainterPK24QStyleOptionGraphicsItemP7QWidget(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsItemGroup {
  pub fn removeFromGroup<T: QGraphicsItemGroup_removeFromGroup>(&mut self, value: T)  {
     value.removeFromGroup(self);
    // return 1;
  }
}

pub trait QGraphicsItemGroup_removeFromGroup {
  fn removeFromGroup(self, rsthis: &mut QGraphicsItemGroup) ;
}

// proto:  void QGraphicsItemGroup::removeFromGroup(QGraphicsItem * item);
impl<'a> /*trait*/ QGraphicsItemGroup_removeFromGroup for (&'a mut QGraphicsItem) {
  fn removeFromGroup(self, rsthis: &mut QGraphicsItemGroup)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QGraphicsItemGroup15removeFromGroupEP13QGraphicsItem()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN18QGraphicsItemGroup15removeFromGroupEP13QGraphicsItem(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsItemGroup {
  pub fn addToGroup<T: QGraphicsItemGroup_addToGroup>(&mut self, value: T)  {
     value.addToGroup(self);
    // return 1;
  }
}

pub trait QGraphicsItemGroup_addToGroup {
  fn addToGroup(self, rsthis: &mut QGraphicsItemGroup) ;
}

// proto:  void QGraphicsItemGroup::addToGroup(QGraphicsItem * item);
impl<'a> /*trait*/ QGraphicsItemGroup_addToGroup for (&'a mut QGraphicsItem) {
  fn addToGroup(self, rsthis: &mut QGraphicsItemGroup)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QGraphicsItemGroup10addToGroupEP13QGraphicsItem()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN18QGraphicsItemGroup10addToGroupEP13QGraphicsItem(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsItemGroup {
  pub fn opaqueArea<T: QGraphicsItemGroup_opaqueArea>(&mut self, value: T) -> QPainterPath {
    return value.opaqueArea(self);
    // return 1;
  }
}

pub trait QGraphicsItemGroup_opaqueArea {
  fn opaqueArea(self, rsthis: &mut QGraphicsItemGroup) -> QPainterPath;
}

// proto:  QPainterPath QGraphicsItemGroup::opaqueArea();
impl<'a> /*trait*/ QGraphicsItemGroup_opaqueArea for () {
  fn opaqueArea(self, rsthis: &mut QGraphicsItemGroup) -> QPainterPath {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QGraphicsItemGroup10opaqueAreaEv()};
    let mut ret = unsafe {_ZNK18QGraphicsItemGroup10opaqueAreaEv(rsthis.qclsinst)};
    let mut ret1 = QPainterPath{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto: void QGraphicsItemGroup::NewQGraphicsItemGroup(const QGraphicsItemGroup & );
impl<'a> /*trait*/ QGraphicsItemGroup_NewQGraphicsItemGroup for (&'a  QGraphicsItemGroup) {
  fn NewQGraphicsItemGroup(self) -> QGraphicsItemGroup {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QGraphicsItemGroupC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN18QGraphicsItemGroupC1ERKS_(qthis, arg0)};
    let rsthis = QGraphicsItemGroup{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

