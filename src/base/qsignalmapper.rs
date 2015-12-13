// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qobject::QObject;
use super::qstring::QString;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  fn _ZN13QSignalMapper14removeMappingsEP7QObject(arg0: *mut c_void) -> i32;
  fn _ZN13QSignalMapper3mapEP7QObject(arg0: *mut c_void) -> i32;
  fn _ZN13QSignalMapper6mappedEi(arg0: c_int) -> i32;
  fn _ZN13QSignalMapperC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  fn _ZNK13QSignalMapper10metaObjectEv() -> i32;
  fn _ZN13QSignalMapper10setMappingEP7QObjectS1_(arg0: *mut c_void, arg1: *mut c_void) -> i32;
  fn _ZN13QSignalMapper6mappedEP7QObject(arg0: *mut c_void) -> i32;
  fn _ZNK13QSignalMapper7mappingEi(arg0: c_int) -> i32;
  fn _ZN13QSignalMapperC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void) -> i32;
  fn _ZN13QSignalMapperD0Ev() -> i32;
  fn _ZN13QSignalMapper6mappedERK7QString(arg0: *const c_void) -> i32;
  fn _ZN13QSignalMapper10setMappingEP7QObjecti(arg0: *mut c_void, arg1: c_int) -> i32;
  fn _ZNK13QSignalMapper7mappingERK7QString(arg0: *const c_void) -> i32;
  fn _ZN13QSignalMapper3mapEv() -> i32;
  fn _ZNK13QSignalMapper7mappingEP7QObject(arg0: *mut c_void) -> i32;
  fn _ZN13QSignalMapper10setMappingEP7QObjectRK7QString(arg0: *mut c_void, arg1: *const c_void) -> i32;
}

// body block begin
// class sizeof(QSignalMapper)=1
pub struct QSignalMapper {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QSignalMapper {
  pub fn removeMappings<T: QSignalMapper_removeMappings>(&mut self, value: T) -> i32 {
    value.removeMappings(self);
    return 1;
  }
}

pub trait QSignalMapper_removeMappings {
  fn removeMappings(self, this: &mut QSignalMapper) -> i32;
}

// proto: void QSignalMapper::removeMappings(QObject * sender);
impl<'a> /*trait*/ QSignalMapper_removeMappings for (&'a mut QObject) {
  fn removeMappings(self, this: &mut QSignalMapper) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QSignalMapper14removeMappingsEP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN13QSignalMapper14removeMappingsEP7QObject(arg0)};
    return 1;
  }
}

impl /*struct*/ QSignalMapper {
  pub fn map<T: QSignalMapper_map>(&mut self, value: T) -> i32 {
    value.map(self);
    return 1;
  }
}

pub trait QSignalMapper_map {
  fn map(self, this: &mut QSignalMapper) -> i32;
}

// proto: void QSignalMapper::map(QObject * sender);
impl<'a> /*trait*/ QSignalMapper_map for (&'a mut QObject) {
  fn map(self, this: &mut QSignalMapper) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QSignalMapper3mapEP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN13QSignalMapper3mapEP7QObject(arg0)};
    return 1;
  }
}

impl /*struct*/ QSignalMapper {
  pub fn mapped<T: QSignalMapper_mapped>(&mut self, value: T) -> i32 {
    value.mapped(self);
    return 1;
  }
}

pub trait QSignalMapper_mapped {
  fn mapped(self, this: &mut QSignalMapper) -> i32;
}

// proto: void QSignalMapper::mapped(int );
impl<'a> /*trait*/ QSignalMapper_mapped for (i32) {
  fn mapped(self, this: &mut QSignalMapper) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QSignalMapper6mappedEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN13QSignalMapper6mappedEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QSignalMapper {
  pub fn NewQSignalMapper<T: QSignalMapper_NewQSignalMapper>(value: T) -> QSignalMapper {
    let rsthis = value.NewQSignalMapper();
    return rsthis;
    // return 1;
  }
}

pub trait QSignalMapper_NewQSignalMapper {
  fn NewQSignalMapper(self) -> QSignalMapper;
}

// proto: void QSignalMapper::NewQSignalMapper(const QSignalMapper & );
impl<'a> /*trait*/ QSignalMapper_NewQSignalMapper for (&'a  QSignalMapper) {
  fn NewQSignalMapper(self) -> QSignalMapper {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QSignalMapperC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN13QSignalMapperC1ERKS_(qthis, arg0)};
    let rsthis = QSignalMapper{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QSignalMapper {
  pub fn metaObject<T: QSignalMapper_metaObject>(&mut self, value: T) -> i32 {
    value.metaObject(self);
    return 1;
  }
}

pub trait QSignalMapper_metaObject {
  fn metaObject(self, this: &mut QSignalMapper) -> i32;
}

// proto: const QMetaObject * QSignalMapper::metaObject();
impl<'a> /*trait*/ QSignalMapper_metaObject for () {
  fn metaObject(self, this: &mut QSignalMapper) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QSignalMapper10metaObjectEv()};
    unsafe {_ZNK13QSignalMapper10metaObjectEv()};
    return 1;
  }
}

impl /*struct*/ QSignalMapper {
  pub fn setMapping<T: QSignalMapper_setMapping>(&mut self, value: T) -> i32 {
    value.setMapping(self);
    return 1;
  }
}

pub trait QSignalMapper_setMapping {
  fn setMapping(self, this: &mut QSignalMapper) -> i32;
}

// proto: void QSignalMapper::setMapping(QObject * sender, QObject * object);
impl<'a> /*trait*/ QSignalMapper_setMapping for (&'a mut QObject, &'a mut QObject) {
  fn setMapping(self, this: &mut QSignalMapper) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QSignalMapper10setMappingEP7QObjectS1_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN13QSignalMapper10setMappingEP7QObjectS1_(arg0, arg1)};
    return 1;
  }
}

// proto: void QSignalMapper::mapped(QObject * );
impl<'a> /*trait*/ QSignalMapper_mapped for (&'a mut QObject) {
  fn mapped(self, this: &mut QSignalMapper) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QSignalMapper6mappedEP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN13QSignalMapper6mappedEP7QObject(arg0)};
    return 1;
  }
}

impl /*struct*/ QSignalMapper {
  pub fn mapping<T: QSignalMapper_mapping>(&mut self, value: T) -> i32 {
    value.mapping(self);
    return 1;
  }
}

pub trait QSignalMapper_mapping {
  fn mapping(self, this: &mut QSignalMapper) -> i32;
}

// proto: QObject * QSignalMapper::mapping(int id);
impl<'a> /*trait*/ QSignalMapper_mapping for (i32) {
  fn mapping(self, this: &mut QSignalMapper) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QSignalMapper7mappingEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK13QSignalMapper7mappingEi(arg0)};
    return 1;
  }
}

// proto: void QSignalMapper::NewQSignalMapper(QObject * parent);
impl<'a> /*trait*/ QSignalMapper_NewQSignalMapper for (&'a mut QObject) {
  fn NewQSignalMapper(self) -> QSignalMapper {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QSignalMapperC1EP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN13QSignalMapperC1EP7QObject(qthis, arg0)};
    let rsthis = QSignalMapper{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QSignalMapper {
  pub fn FreeQSignalMapper<T: QSignalMapper_FreeQSignalMapper>(&mut self, value: T) -> i32 {
    value.FreeQSignalMapper(self);
    return 1;
  }
}

pub trait QSignalMapper_FreeQSignalMapper {
  fn FreeQSignalMapper(self, this: &mut QSignalMapper) -> i32;
}

// proto: void QSignalMapper::FreeQSignalMapper();
impl<'a> /*trait*/ QSignalMapper_FreeQSignalMapper for () {
  fn FreeQSignalMapper(self, this: &mut QSignalMapper) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QSignalMapperD0Ev()};
    unsafe {_ZN13QSignalMapperD0Ev()};
    return 1;
  }
}

// proto: void QSignalMapper::mapped(const QString & );
impl<'a> /*trait*/ QSignalMapper_mapped for (&'a  QString) {
  fn mapped(self, this: &mut QSignalMapper) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QSignalMapper6mappedERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN13QSignalMapper6mappedERK7QString(arg0)};
    return 1;
  }
}

// proto: void QSignalMapper::setMapping(QObject * sender, int id);
impl<'a> /*trait*/ QSignalMapper_setMapping for (&'a mut QObject, i32) {
  fn setMapping(self, this: &mut QSignalMapper) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QSignalMapper10setMappingEP7QObjecti()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    unsafe {_ZN13QSignalMapper10setMappingEP7QObjecti(arg0, arg1)};
    return 1;
  }
}

// proto: QObject * QSignalMapper::mapping(const QString & text);
impl<'a> /*trait*/ QSignalMapper_mapping for (&'a  QString) {
  fn mapping(self, this: &mut QSignalMapper) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QSignalMapper7mappingERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK13QSignalMapper7mappingERK7QString(arg0)};
    return 1;
  }
}

// proto: void QSignalMapper::map();
impl<'a> /*trait*/ QSignalMapper_map for () {
  fn map(self, this: &mut QSignalMapper) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QSignalMapper3mapEv()};
    unsafe {_ZN13QSignalMapper3mapEv()};
    return 1;
  }
}

// proto: QObject * QSignalMapper::mapping(QObject * object);
impl<'a> /*trait*/ QSignalMapper_mapping for (&'a mut QObject) {
  fn mapping(self, this: &mut QSignalMapper) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QSignalMapper7mappingEP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZNK13QSignalMapper7mappingEP7QObject(arg0)};
    return 1;
  }
}

// proto: void QSignalMapper::setMapping(QObject * sender, const QString & text);
impl<'a> /*trait*/ QSignalMapper_setMapping for (&'a mut QObject, &'a  QString) {
  fn setMapping(self, this: &mut QSignalMapper) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QSignalMapper10setMappingEP7QObjectRK7QString()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN13QSignalMapper10setMappingEP7QObjectRK7QString(arg0, arg1)};
    return 1;
  }
}

