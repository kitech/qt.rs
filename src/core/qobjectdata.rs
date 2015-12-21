// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  QMetaObject * QObjectData::dynamicMetaObject();
  fn _ZNK11QObjectData17dynamicMetaObjectEv(qthis: *mut c_void);
  // proto:  void QObjectData::~QObjectData();
  fn _ZN11QObjectDataD0Ev(qthis: *mut c_void);
}

// body block begin
// class sizeof(QObjectData)=1
pub struct QObjectData {
  pub qclsinst: *mut c_void,
}

  // proto:  QMetaObject * QObjectData::dynamicMetaObject();
impl /*struct*/ QObjectData {
  pub fn dynamicMetaObject<RetType, T: QObjectData_dynamicMetaObject<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.dynamicMetaObject(self);
    // return 1;
  }
}

pub trait QObjectData_dynamicMetaObject<RetType> {
  fn dynamicMetaObject(self , rsthis: &mut QObjectData) -> RetType;
}

  // proto:  QMetaObject * QObjectData::dynamicMetaObject();
impl<'a> /*trait*/ QObjectData_dynamicMetaObject<()> for () {
  fn dynamicMetaObject(self , rsthis: &mut QObjectData) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QObjectData17dynamicMetaObjectEv()};
     unsafe {_ZNK11QObjectData17dynamicMetaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QObjectData::~QObjectData();
impl /*struct*/ QObjectData {
  pub fn FreeQObjectData<RetType, T: QObjectData_FreeQObjectData<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQObjectData(self);
    // return 1;
  }
}

pub trait QObjectData_FreeQObjectData<RetType> {
  fn FreeQObjectData(self , rsthis: &mut QObjectData) -> RetType;
}

  // proto:  void QObjectData::~QObjectData();
impl<'a> /*trait*/ QObjectData_FreeQObjectData<()> for () {
  fn FreeQObjectData(self , rsthis: &mut QObjectData) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QObjectDataD0Ev()};
     unsafe {_ZN11QObjectDataD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

