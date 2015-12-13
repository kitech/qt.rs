// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qgraphicsitem::QGraphicsItem;
use super::qpainter::QPainter;
use super::qstyleoptiongraphicsitem::QStyleOptionGraphicsItem;
use super::qwidget::QWidget;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: bool QGraphicsItemGroup::isObscuredBy(const QGraphicsItem * item);
  fn _ZNK18QGraphicsItemGroup12isObscuredByEPK13QGraphicsItem(arg0: *const c_void) -> i32;
  // proto: void QGraphicsItemGroup::FreeQGraphicsItemGroup();
  fn _ZN18QGraphicsItemGroupD0Ev() -> i32;
  // proto: void QGraphicsItemGroup::NewQGraphicsItemGroup(QGraphicsItem * parent);
  fn _ZN18QGraphicsItemGroupC1EP13QGraphicsItem(qthis: *mut c_void, arg0: *mut c_void) -> i32;
  // proto: int QGraphicsItemGroup::type_();
  fn _ZNK18QGraphicsItemGroup4typeEv() -> i32;
  // proto: QRectF QGraphicsItemGroup::boundingRect();
  fn _ZNK18QGraphicsItemGroup12boundingRectEv() -> i32;
  // proto: void QGraphicsItemGroup::paint(QPainter * painter, const QStyleOptionGraphicsItem * option, QWidget * widget);
  fn _ZN18QGraphicsItemGroup5paintEP8QPainterPK24QStyleOptionGraphicsItemP7QWidget(arg0: *mut c_void, arg1: *const c_void, arg2: *mut c_void) -> i32;
  // proto: void QGraphicsItemGroup::removeFromGroup(QGraphicsItem * item);
  fn _ZN18QGraphicsItemGroup15removeFromGroupEP13QGraphicsItem(arg0: *mut c_void) -> i32;
  // proto: void QGraphicsItemGroup::addToGroup(QGraphicsItem * item);
  fn _ZN18QGraphicsItemGroup10addToGroupEP13QGraphicsItem(arg0: *mut c_void) -> i32;
  // proto: QPainterPath QGraphicsItemGroup::opaqueArea();
  fn _ZNK18QGraphicsItemGroup10opaqueAreaEv() -> i32;
  // proto: void QGraphicsItemGroup::NewQGraphicsItemGroup(const QGraphicsItemGroup & );
  fn _ZN18QGraphicsItemGroupC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
}

// body block begin
// class sizeof(QGraphicsItemGroup)=1
pub struct QGraphicsItemGroup {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QGraphicsItemGroup {
  pub fn isObscuredBy<T: QGraphicsItemGroup_isObscuredBy>(&mut self, value: T) -> i32 {
    value.isObscuredBy(self);
    return 1;
  }
}

pub trait QGraphicsItemGroup_isObscuredBy {
  fn isObscuredBy(self, this: &mut QGraphicsItemGroup) -> i32;
}

// proto: bool QGraphicsItemGroup::isObscuredBy(const QGraphicsItem * item);
impl<'a> /*trait*/ QGraphicsItemGroup_isObscuredBy for (&'a  QGraphicsItem) {
  fn isObscuredBy(self, this: &mut QGraphicsItemGroup) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QGraphicsItemGroup12isObscuredByEPK13QGraphicsItem()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK18QGraphicsItemGroup12isObscuredByEPK13QGraphicsItem(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsItemGroup {
  pub fn FreeQGraphicsItemGroup<T: QGraphicsItemGroup_FreeQGraphicsItemGroup>(&mut self, value: T) -> i32 {
    value.FreeQGraphicsItemGroup(self);
    return 1;
  }
}

pub trait QGraphicsItemGroup_FreeQGraphicsItemGroup {
  fn FreeQGraphicsItemGroup(self, this: &mut QGraphicsItemGroup) -> i32;
}

// proto: void QGraphicsItemGroup::FreeQGraphicsItemGroup();
impl<'a> /*trait*/ QGraphicsItemGroup_FreeQGraphicsItemGroup for () {
  fn FreeQGraphicsItemGroup(self, this: &mut QGraphicsItemGroup) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QGraphicsItemGroupD0Ev()};
    unsafe {_ZN18QGraphicsItemGroupD0Ev()};
    return 1;
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
    value.type_(self);
    return 1;
  }
}

pub trait QGraphicsItemGroup_type_ {
  fn type_(self, this: &mut QGraphicsItemGroup) -> i32;
}

// proto: int QGraphicsItemGroup::type_();
impl<'a> /*trait*/ QGraphicsItemGroup_type_ for () {
  fn type_(self, this: &mut QGraphicsItemGroup) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QGraphicsItemGroup4typeEv()};
    unsafe {_ZNK18QGraphicsItemGroup4typeEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsItemGroup {
  pub fn boundingRect<T: QGraphicsItemGroup_boundingRect>(&mut self, value: T) -> i32 {
    value.boundingRect(self);
    return 1;
  }
}

pub trait QGraphicsItemGroup_boundingRect {
  fn boundingRect(self, this: &mut QGraphicsItemGroup) -> i32;
}

// proto: QRectF QGraphicsItemGroup::boundingRect();
impl<'a> /*trait*/ QGraphicsItemGroup_boundingRect for () {
  fn boundingRect(self, this: &mut QGraphicsItemGroup) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QGraphicsItemGroup12boundingRectEv()};
    unsafe {_ZNK18QGraphicsItemGroup12boundingRectEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsItemGroup {
  pub fn paint<T: QGraphicsItemGroup_paint>(&mut self, value: T) -> i32 {
    value.paint(self);
    return 1;
  }
}

pub trait QGraphicsItemGroup_paint {
  fn paint(self, this: &mut QGraphicsItemGroup) -> i32;
}

// proto: void QGraphicsItemGroup::paint(QPainter * painter, const QStyleOptionGraphicsItem * option, QWidget * widget);
impl<'a> /*trait*/ QGraphicsItemGroup_paint for (&'a mut QPainter, &'a  QStyleOptionGraphicsItem, &'a mut QWidget) {
  fn paint(self, this: &mut QGraphicsItemGroup) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QGraphicsItemGroup5paintEP8QPainterPK24QStyleOptionGraphicsItemP7QWidget()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    unsafe {_ZN18QGraphicsItemGroup5paintEP8QPainterPK24QStyleOptionGraphicsItemP7QWidget(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QGraphicsItemGroup {
  pub fn removeFromGroup<T: QGraphicsItemGroup_removeFromGroup>(&mut self, value: T) -> i32 {
    value.removeFromGroup(self);
    return 1;
  }
}

pub trait QGraphicsItemGroup_removeFromGroup {
  fn removeFromGroup(self, this: &mut QGraphicsItemGroup) -> i32;
}

// proto: void QGraphicsItemGroup::removeFromGroup(QGraphicsItem * item);
impl<'a> /*trait*/ QGraphicsItemGroup_removeFromGroup for (&'a mut QGraphicsItem) {
  fn removeFromGroup(self, this: &mut QGraphicsItemGroup) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QGraphicsItemGroup15removeFromGroupEP13QGraphicsItem()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN18QGraphicsItemGroup15removeFromGroupEP13QGraphicsItem(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsItemGroup {
  pub fn addToGroup<T: QGraphicsItemGroup_addToGroup>(&mut self, value: T) -> i32 {
    value.addToGroup(self);
    return 1;
  }
}

pub trait QGraphicsItemGroup_addToGroup {
  fn addToGroup(self, this: &mut QGraphicsItemGroup) -> i32;
}

// proto: void QGraphicsItemGroup::addToGroup(QGraphicsItem * item);
impl<'a> /*trait*/ QGraphicsItemGroup_addToGroup for (&'a mut QGraphicsItem) {
  fn addToGroup(self, this: &mut QGraphicsItemGroup) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QGraphicsItemGroup10addToGroupEP13QGraphicsItem()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN18QGraphicsItemGroup10addToGroupEP13QGraphicsItem(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsItemGroup {
  pub fn opaqueArea<T: QGraphicsItemGroup_opaqueArea>(&mut self, value: T) -> i32 {
    value.opaqueArea(self);
    return 1;
  }
}

pub trait QGraphicsItemGroup_opaqueArea {
  fn opaqueArea(self, this: &mut QGraphicsItemGroup) -> i32;
}

// proto: QPainterPath QGraphicsItemGroup::opaqueArea();
impl<'a> /*trait*/ QGraphicsItemGroup_opaqueArea for () {
  fn opaqueArea(self, this: &mut QGraphicsItemGroup) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QGraphicsItemGroup10opaqueAreaEv()};
    unsafe {_ZNK18QGraphicsItemGroup10opaqueAreaEv()};
    return 1;
  }
}

// proto: void QGraphicsItemGroup::NewQGraphicsItemGroup(const QGraphicsItemGroup & );
impl<'a> /*trait*/ QGraphicsItemGroup_NewQGraphicsItemGroup for (&'a  QGraphicsItemGroup) {
  fn NewQGraphicsItemGroup(self) -> QGraphicsItemGroup {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QGraphicsItemGroupC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN18QGraphicsItemGroupC1ERKS_(qthis, arg0)};
    let rsthis = QGraphicsItemGroup{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

