// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qtextdocument::QTextDocument;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  void QTextBlockGroup::QTextBlockGroup(const QTextBlockGroup & );
  fn _ZN15QTextBlockGroupC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  const QMetaObject * QTextBlockGroup::metaObject();
  fn _ZNK15QTextBlockGroup10metaObjectEv(qthis: *mut c_void);
  // proto:  void QTextBlockGroup::~QTextBlockGroup();
  fn _ZN15QTextBlockGroupD0Ev(qthis: *mut c_void);
  // proto:  void QTextBlockGroup::QTextBlockGroup(QTextDocument * doc);
  fn _ZN15QTextBlockGroupC1EP13QTextDocument(qthis: *mut c_void, arg0: *mut c_void);
}

// body block begin
// class sizeof(QTextBlockGroup)=1
pub struct QTextBlockGroup {
  pub qclsinst: *mut c_void,
}

  // proto:  void QTextBlockGroup::QTextBlockGroup(const QTextBlockGroup & );
impl /*struct*/ QTextBlockGroup {
  pub fn NewQTextBlockGroup<T: QTextBlockGroup_NewQTextBlockGroup>(value: T) -> QTextBlockGroup {
    let rsthis = value.NewQTextBlockGroup();
    return rsthis;
    // return 1;
  }
}

pub trait QTextBlockGroup_NewQTextBlockGroup {
  fn NewQTextBlockGroup(self) -> QTextBlockGroup;
}

  // proto:  void QTextBlockGroup::QTextBlockGroup(const QTextBlockGroup & );
impl<'a> /*trait*/ QTextBlockGroup_NewQTextBlockGroup for (QTextBlockGroup) {
  fn NewQTextBlockGroup(self) -> QTextBlockGroup {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTextBlockGroupC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN15QTextBlockGroupC1ERKS_(qthis, arg0)};
    let rsthis = QTextBlockGroup{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  const QMetaObject * QTextBlockGroup::metaObject();
impl /*struct*/ QTextBlockGroup {
  pub fn metaObject<RetType, T: QTextBlockGroup_metaObject<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QTextBlockGroup_metaObject<RetType> {
  fn metaObject(self , rsthis: &mut QTextBlockGroup) -> RetType;
}

  // proto:  const QMetaObject * QTextBlockGroup::metaObject();
impl<'a> /*trait*/ QTextBlockGroup_metaObject<()> for () {
  fn metaObject(self , rsthis: &mut QTextBlockGroup) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QTextBlockGroup10metaObjectEv()};
     unsafe {_ZNK15QTextBlockGroup10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QTextBlockGroup::~QTextBlockGroup();
impl /*struct*/ QTextBlockGroup {
  pub fn FreeQTextBlockGroup<RetType, T: QTextBlockGroup_FreeQTextBlockGroup<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQTextBlockGroup(self);
    // return 1;
  }
}

pub trait QTextBlockGroup_FreeQTextBlockGroup<RetType> {
  fn FreeQTextBlockGroup(self , rsthis: &mut QTextBlockGroup) -> RetType;
}

  // proto:  void QTextBlockGroup::~QTextBlockGroup();
impl<'a> /*trait*/ QTextBlockGroup_FreeQTextBlockGroup<()> for () {
  fn FreeQTextBlockGroup(self , rsthis: &mut QTextBlockGroup) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTextBlockGroupD0Ev()};
     unsafe {_ZN15QTextBlockGroupD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QTextBlockGroup::QTextBlockGroup(QTextDocument * doc);
impl<'a> /*trait*/ QTextBlockGroup_NewQTextBlockGroup for (QTextDocument) {
  fn NewQTextBlockGroup(self) -> QTextBlockGroup {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTextBlockGroupC1EP13QTextDocument()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN15QTextBlockGroupC1EP13QTextDocument(qthis, arg0)};
    let rsthis = QTextBlockGroup{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

