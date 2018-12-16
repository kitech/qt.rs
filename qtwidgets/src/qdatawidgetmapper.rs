

// mod ::widgets::QDataWidgetMapper
// package qtwidgets
// /usr/include/qt/QtWidgets/qdatawidgetmapper.h
// #include <qdatawidgetmapper.h>
// #include <QtWidgets>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 44
*/
// import "C"
// import "unsafe"
// import "reflect"
// import "fmt"
// import "log"
// import "github.com/kitech/qt.go/qtrt"
// import "github.com/kitech/qt.go/qtcore"
use qtcore::*; // super::super::%!s(MISSING)::*;
// import "github.com/kitech/qt.go/qtgui"
use qtgui::*; // super::super::%!s(MISSING)::*;
use std::default::Default;
use std::ops::Deref;
use qtrt; // super::super::qtrt;
use super::*;
//  ext block end

//  body block begin



/*

*/
#[derive(Default)] // class sizeof(QDataWidgetMapper)=16
pub struct QDataWidgetMapper {
  qbase: QObject,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QDataWidgetMapper_ITF interface {
//    qtcore.QObject_ITF
//    QDataWidgetMapper_PTR() *QDataWidgetMapper
//}
//func (ptr *QDataWidgetMapper) QDataWidgetMapper_PTR() *QDataWidgetMapper { return ptr }

impl /*struct*/ QDataWidgetMapper {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QDataWidgetMapper {
    return QDataWidgetMapper{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QDataWidgetMapper {
//  type Target = QDataWidgetMapperBASE;
//
//  fn deref(&self) -> &QDataWidgetMapperBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QDataWidgetMapperBASE> for QDataWidgetMapper {
//  fn as_ref(& self) -> & QDataWidgetMapperBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qdatawidgetmapper.h:57
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QDataWidgetMapper {
  pub fn metaObject_0<RetType, T: QDataWidgetMapper_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QDataWidgetMapper_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QDataWidgetMapper) -> RetType;
}
impl<'a> /*trait*/ QDataWidgetMapper_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QDataWidgetMapper) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK17QDataWidgetMapper10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qdatawidgetmapper.h:64
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QDataWidgetMapper(QObject *)

/*
Constructs a new QDataWidgetMapper with parent object parent. By default, the orientation is horizontal and the submit policy is AutoSubmit.

See also setOrientation() and setSubmitPolicy().
*/
// QDataWidgetMapper(QObject *) ctx.fn_proto_cpp
impl /*struct*/ QDataWidgetMapper {
  pub fn QDataWidgetMapper_0<T: QDataWidgetMapper_QDataWidgetMapper_0>(value: T) -> QDataWidgetMapper {
    let rsthis = value.QDataWidgetMapper_0();
    return rsthis;
    // return 1;
  }
}

pub trait QDataWidgetMapper_QDataWidgetMapper_0 {
  fn QDataWidgetMapper_0(self) -> QDataWidgetMapper;
}
// QDataWidgetMapper(QObject *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QDataWidgetMapper_QDataWidgetMapper_0 for (usize) {
  fn QDataWidgetMapper_0(self) -> QDataWidgetMapper {
    // unsafe{_ZN17QDataWidgetMapperC2EP7QObject()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN17QDataWidgetMapperC2EP7QObject", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QDataWidgetMapper{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qdatawidgetmapper.h:65
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QDataWidgetMapper()

/*

*/
pub fn DeleteQDataWidgetMapper(this :*mut QDataWidgetMapper) {
    // let rv = qtrt::InvokeQtFunc6("_ZN17QDataWidgetMapperD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 16)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtWidgets/qdatawidgetmapper.h:67
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setModel(QAbstractItemModel *)

/*
Sets the current model to model. If another model was set, all mappings to that old model are cleared.

See also model().
*/
impl /*struct*/ QDataWidgetMapper {
  pub fn setModel_0<RetType, T: QDataWidgetMapper_setModel_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setModel_0(self);
    // return 1;
  }
}
pub trait QDataWidgetMapper_setModel_0<RetType> {
  fn setModel_0(self , rsthis: & QDataWidgetMapper) -> RetType;
}
impl<'a> /*trait*/ QDataWidgetMapper_setModel_0<(/*void*/)> for (usize) {
  fn setModel_0(self , rsthis: & QDataWidgetMapper) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN17QDataWidgetMapper8setModelEP18QAbstractItemModel", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qdatawidgetmapper.h:68
// index:0
// Public Visibility=Default Availability=Available
// [8] QAbstractItemModel * model() const

/*
Returns the current model.

See also setModel().
*/
impl /*struct*/ QDataWidgetMapper {
  pub fn model_0<RetType, T: QDataWidgetMapper_model_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.model_0(self);
    // return 1;
  }
}
pub trait QDataWidgetMapper_model_0<RetType> {
  fn model_0(self , rsthis: & QDataWidgetMapper) -> RetType;
}
impl<'a> /*trait*/ QDataWidgetMapper_model_0<usize> for () {
  fn model_0(self , rsthis: & QDataWidgetMapper) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK17QDataWidgetMapper5modelEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qdatawidgetmapper.h:70
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setItemDelegate(QAbstractItemDelegate *)

/*
Sets the item delegate to delegate. The delegate will be used to write data from the model into the widget and from the widget to the model, using QAbstractItemDelegate::setEditorData() and QAbstractItemDelegate::setModelData().

The delegate also decides when to apply data and when to change the editor, using QAbstractItemDelegate::commitData() and QAbstractItemDelegate::closeEditor().

Warning: You should not share the same instance of a delegate between widget mappers or views. Doing so can cause incorrect or unintuitive editing behavior since each view connected to a given delegate may receive the closeEditor() signal, and attempt to access, modify or close an editor that has already been closed.

See also itemDelegate().
*/
impl /*struct*/ QDataWidgetMapper {
  pub fn setItemDelegate_0<RetType, T: QDataWidgetMapper_setItemDelegate_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setItemDelegate_0(self);
    // return 1;
  }
}
pub trait QDataWidgetMapper_setItemDelegate_0<RetType> {
  fn setItemDelegate_0(self , rsthis: & QDataWidgetMapper) -> RetType;
}
impl<'a> /*trait*/ QDataWidgetMapper_setItemDelegate_0<(/*void*/)> for (usize) {
  fn setItemDelegate_0(self , rsthis: & QDataWidgetMapper) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN17QDataWidgetMapper15setItemDelegateEP21QAbstractItemDelegate", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qdatawidgetmapper.h:71
// index:0
// Public Visibility=Default Availability=Available
// [8] QAbstractItemDelegate * itemDelegate() const

/*
Returns the current item delegate.

See also setItemDelegate().
*/
impl /*struct*/ QDataWidgetMapper {
  pub fn itemDelegate_0<RetType, T: QDataWidgetMapper_itemDelegate_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.itemDelegate_0(self);
    // return 1;
  }
}
pub trait QDataWidgetMapper_itemDelegate_0<RetType> {
  fn itemDelegate_0(self , rsthis: & QDataWidgetMapper) -> RetType;
}
impl<'a> /*trait*/ QDataWidgetMapper_itemDelegate_0<usize> for () {
  fn itemDelegate_0(self , rsthis: & QDataWidgetMapper) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK17QDataWidgetMapper12itemDelegateEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qdatawidgetmapper.h:73
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setRootIndex(const QModelIndex &)

/*
Sets the root item to index. This can be used to display a branch of a tree. Pass an invalid model index to display the top-most branch.

See also rootIndex().
*/
impl /*struct*/ QDataWidgetMapper {
  pub fn setRootIndex_0<RetType, T: QDataWidgetMapper_setRootIndex_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setRootIndex_0(self);
    // return 1;
  }
}
pub trait QDataWidgetMapper_setRootIndex_0<RetType> {
  fn setRootIndex_0(self , rsthis: & QDataWidgetMapper) -> RetType;
}
impl<'a> /*trait*/ QDataWidgetMapper_setRootIndex_0<(/*void*/)> for (usize) {
  fn setRootIndex_0(self , rsthis: & QDataWidgetMapper) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN17QDataWidgetMapper12setRootIndexERK11QModelIndex", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qdatawidgetmapper.h:74
// index:0
// Public Visibility=Default Availability=Available
// [24] QModelIndex rootIndex() const

/*
Returns the current root index.

See also setRootIndex().
*/
impl /*struct*/ QDataWidgetMapper {
  pub fn rootIndex_0<RetType, T: QDataWidgetMapper_rootIndex_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.rootIndex_0(self);
    // return 1;
  }
}
pub trait QDataWidgetMapper_rootIndex_0<RetType> {
  fn rootIndex_0(self , rsthis: & QDataWidgetMapper) -> RetType;
}
impl<'a> /*trait*/ QDataWidgetMapper_rootIndex_0<usize> for () {
  fn rootIndex_0(self , rsthis: & QDataWidgetMapper) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK17QDataWidgetMapper9rootIndexEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qdatawidgetmapper.h:76
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setOrientation(Qt::Orientation)

/*

*/
impl /*struct*/ QDataWidgetMapper {
  pub fn setOrientation_0<RetType, T: QDataWidgetMapper_setOrientation_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setOrientation_0(self);
    // return 1;
  }
}
pub trait QDataWidgetMapper_setOrientation_0<RetType> {
  fn setOrientation_0(self , rsthis: & QDataWidgetMapper) -> RetType;
}
impl<'a> /*trait*/ QDataWidgetMapper_setOrientation_0<(/*void*/)> for (i32) {
  fn setOrientation_0(self , rsthis: & QDataWidgetMapper) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN17QDataWidgetMapper14setOrientationEN2Qt11OrientationE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qdatawidgetmapper.h:77
// index:0
// Public Visibility=Default Availability=Available
// [4] Qt::Orientation orientation() const

/*

*/
impl /*struct*/ QDataWidgetMapper {
  pub fn orientation_0<RetType, T: QDataWidgetMapper_orientation_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.orientation_0(self);
    // return 1;
  }
}
pub trait QDataWidgetMapper_orientation_0<RetType> {
  fn orientation_0(self , rsthis: & QDataWidgetMapper) -> RetType;
}
impl<'a> /*trait*/ QDataWidgetMapper_orientation_0<i32> for () {
  fn orientation_0(self , rsthis: & QDataWidgetMapper) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK17QDataWidgetMapper11orientationEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qdatawidgetmapper.h:81
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setSubmitPolicy(QDataWidgetMapper::SubmitPolicy)

/*

*/
impl /*struct*/ QDataWidgetMapper {
  pub fn setSubmitPolicy_0<RetType, T: QDataWidgetMapper_setSubmitPolicy_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setSubmitPolicy_0(self);
    // return 1;
  }
}
pub trait QDataWidgetMapper_setSubmitPolicy_0<RetType> {
  fn setSubmitPolicy_0(self , rsthis: & QDataWidgetMapper) -> RetType;
}
impl<'a> /*trait*/ QDataWidgetMapper_setSubmitPolicy_0<(/*void*/)> for (i32) {
  fn setSubmitPolicy_0(self , rsthis: & QDataWidgetMapper) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN17QDataWidgetMapper15setSubmitPolicyENS_12SubmitPolicyE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qdatawidgetmapper.h:82
// index:0
// Public Visibility=Default Availability=Available
// [4] QDataWidgetMapper::SubmitPolicy submitPolicy() const

/*

*/
impl /*struct*/ QDataWidgetMapper {
  pub fn submitPolicy_0<RetType, T: QDataWidgetMapper_submitPolicy_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.submitPolicy_0(self);
    // return 1;
  }
}
pub trait QDataWidgetMapper_submitPolicy_0<RetType> {
  fn submitPolicy_0(self , rsthis: & QDataWidgetMapper) -> RetType;
}
impl<'a> /*trait*/ QDataWidgetMapper_submitPolicy_0<i32> for () {
  fn submitPolicy_0(self , rsthis: & QDataWidgetMapper) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK17QDataWidgetMapper12submitPolicyEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qdatawidgetmapper.h:84
// index:0
// Public Visibility=Default Availability=Available
// [-2] void addMapping(QWidget *, int)

/*
Adds a mapping between a widget and a section from the model. The section is a column in the model if the orientation is horizontal (the default), otherwise a row.

For the following example, we assume a model myModel that has two columns: the first one contains the names of people in a group, and the second column contains their ages. The first column is mapped to the QLineEdit nameLineEdit, and the second is mapped to the QSpinBox ageSpinBox:


  QDataWidgetMapper *mapper = new QDataWidgetMapper();
  mapper->setModel(myModel);
  mapper->addMapping(nameLineEdit, 0);
  mapper->addMapping(ageSpinBox, 1);



Notes:


If the widget is already mapped to a section, the old mapping will be replaced by the new one.
Only one-to-one mappings between sections and widgets are allowed. It is not possible to map a single section to multiple widgets, or to map a single widget to multiple sections.


See also removeMapping(), mappedSection(), and clearMapping().
*/
impl /*struct*/ QDataWidgetMapper {
  pub fn addMapping_0<RetType, T: QDataWidgetMapper_addMapping_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.addMapping_0(self);
    // return 1;
  }
}
pub trait QDataWidgetMapper_addMapping_0<RetType> {
  fn addMapping_0(self , rsthis: & QDataWidgetMapper) -> RetType;
}
impl<'a> /*trait*/ QDataWidgetMapper_addMapping_0<(/*void*/)> for (usize,i32) {
  fn addMapping_0(self , rsthis: & QDataWidgetMapper) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN17QDataWidgetMapper10addMappingEP7QWidgeti", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qdatawidgetmapper.h:85
// index:1
// Public Visibility=Default Availability=Available
// [-2] void addMapping(QWidget *, int, const QByteArray &)

/*
Adds a mapping between a widget and a section from the model. The section is a column in the model if the orientation is horizontal (the default), otherwise a row.

For the following example, we assume a model myModel that has two columns: the first one contains the names of people in a group, and the second column contains their ages. The first column is mapped to the QLineEdit nameLineEdit, and the second is mapped to the QSpinBox ageSpinBox:


  QDataWidgetMapper *mapper = new QDataWidgetMapper();
  mapper->setModel(myModel);
  mapper->addMapping(nameLineEdit, 0);
  mapper->addMapping(ageSpinBox, 1);



Notes:


If the widget is already mapped to a section, the old mapping will be replaced by the new one.
Only one-to-one mappings between sections and widgets are allowed. It is not possible to map a single section to multiple widgets, or to map a single widget to multiple sections.


See also removeMapping(), mappedSection(), and clearMapping().
*/
impl /*struct*/ QDataWidgetMapper {
  pub fn addMapping_1<RetType, T: QDataWidgetMapper_addMapping_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.addMapping_1(self);
    // return 1;
  }
}
pub trait QDataWidgetMapper_addMapping_1<RetType> {
  fn addMapping_1(self , rsthis: & QDataWidgetMapper) -> RetType;
}
impl<'a> /*trait*/ QDataWidgetMapper_addMapping_1<(/*void*/)> for (usize,i32,usize) {
  fn addMapping_1(self , rsthis: & QDataWidgetMapper) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN17QDataWidgetMapper10addMappingEP7QWidgetiRK10QByteArray", 3,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qdatawidgetmapper.h:86
// index:0
// Public Visibility=Default Availability=Available
// [-2] void removeMapping(QWidget *)

/*
Removes the mapping for the given widget.

See also addMapping() and clearMapping().
*/
impl /*struct*/ QDataWidgetMapper {
  pub fn removeMapping_0<RetType, T: QDataWidgetMapper_removeMapping_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.removeMapping_0(self);
    // return 1;
  }
}
pub trait QDataWidgetMapper_removeMapping_0<RetType> {
  fn removeMapping_0(self , rsthis: & QDataWidgetMapper) -> RetType;
}
impl<'a> /*trait*/ QDataWidgetMapper_removeMapping_0<(/*void*/)> for (usize) {
  fn removeMapping_0(self , rsthis: & QDataWidgetMapper) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN17QDataWidgetMapper13removeMappingEP7QWidget", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qdatawidgetmapper.h:87
// index:0
// Public Visibility=Default Availability=Available
// [4] int mappedSection(QWidget *) const

/*
Returns the section the widget is mapped to or -1 if the widget is not mapped.

See also addMapping() and removeMapping().
*/
impl /*struct*/ QDataWidgetMapper {
  pub fn mappedSection_0<RetType, T: QDataWidgetMapper_mappedSection_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mappedSection_0(self);
    // return 1;
  }
}
pub trait QDataWidgetMapper_mappedSection_0<RetType> {
  fn mappedSection_0(self , rsthis: & QDataWidgetMapper) -> RetType;
}
impl<'a> /*trait*/ QDataWidgetMapper_mappedSection_0<i32> for (usize) {
  fn mappedSection_0(self , rsthis: & QDataWidgetMapper) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK17QDataWidgetMapper13mappedSectionEP7QWidget", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qdatawidgetmapper.h:88
// index:0
// Public Visibility=Default Availability=Available
// [8] QByteArray mappedPropertyName(QWidget *) const

/*
Returns the name of the property that is used when mapping data to the given widget.

This function was introduced in  Qt 4.3.

See also mappedSection(), addMapping(), and removeMapping().
*/
impl /*struct*/ QDataWidgetMapper {
  pub fn mappedPropertyName_0<RetType, T: QDataWidgetMapper_mappedPropertyName_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mappedPropertyName_0(self);
    // return 1;
  }
}
pub trait QDataWidgetMapper_mappedPropertyName_0<RetType> {
  fn mappedPropertyName_0(self , rsthis: & QDataWidgetMapper) -> RetType;
}
impl<'a> /*trait*/ QDataWidgetMapper_mappedPropertyName_0<usize> for (usize) {
  fn mappedPropertyName_0(self , rsthis: & QDataWidgetMapper) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK17QDataWidgetMapper18mappedPropertyNameEP7QWidget", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qdatawidgetmapper.h:89
// index:0
// Public Visibility=Default Availability=Available
// [8] QWidget * mappedWidgetAt(int) const

/*
Returns the widget that is mapped at section, or 0 if no widget is mapped at that section.

See also addMapping() and removeMapping().
*/
impl /*struct*/ QDataWidgetMapper {
  pub fn mappedWidgetAt_0<RetType, T: QDataWidgetMapper_mappedWidgetAt_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mappedWidgetAt_0(self);
    // return 1;
  }
}
pub trait QDataWidgetMapper_mappedWidgetAt_0<RetType> {
  fn mappedWidgetAt_0(self , rsthis: & QDataWidgetMapper) -> RetType;
}
impl<'a> /*trait*/ QDataWidgetMapper_mappedWidgetAt_0<usize> for (i32) {
  fn mappedWidgetAt_0(self , rsthis: & QDataWidgetMapper) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK17QDataWidgetMapper14mappedWidgetAtEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qdatawidgetmapper.h:90
// index:0
// Public Visibility=Default Availability=Available
// [-2] void clearMapping()

/*
Clears all mappings.

See also addMapping() and removeMapping().
*/
impl /*struct*/ QDataWidgetMapper {
  pub fn clearMapping_0<RetType, T: QDataWidgetMapper_clearMapping_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.clearMapping_0(self);
    // return 1;
  }
}
pub trait QDataWidgetMapper_clearMapping_0<RetType> {
  fn clearMapping_0(self , rsthis: & QDataWidgetMapper) -> RetType;
}
impl<'a> /*trait*/ QDataWidgetMapper_clearMapping_0<(/*void*/)> for () {
  fn clearMapping_0(self , rsthis: & QDataWidgetMapper) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN17QDataWidgetMapper12clearMappingEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qdatawidgetmapper.h:92
// index:0
// Public Visibility=Default Availability=Available
// [4] int currentIndex() const

/*

*/
impl /*struct*/ QDataWidgetMapper {
  pub fn currentIndex_0<RetType, T: QDataWidgetMapper_currentIndex_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.currentIndex_0(self);
    // return 1;
  }
}
pub trait QDataWidgetMapper_currentIndex_0<RetType> {
  fn currentIndex_0(self , rsthis: & QDataWidgetMapper) -> RetType;
}
impl<'a> /*trait*/ QDataWidgetMapper_currentIndex_0<i32> for () {
  fn currentIndex_0(self , rsthis: & QDataWidgetMapper) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK17QDataWidgetMapper12currentIndexEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qdatawidgetmapper.h:95
// index:0
// Public Visibility=Default Availability=Available
// [-2] void revert()

/*
Repopulates all widgets with the current data of the model. All unsubmitted changes will be lost.

See also submit() and setSubmitPolicy().
*/
impl /*struct*/ QDataWidgetMapper {
  pub fn revert_0<RetType, T: QDataWidgetMapper_revert_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.revert_0(self);
    // return 1;
  }
}
pub trait QDataWidgetMapper_revert_0<RetType> {
  fn revert_0(self , rsthis: & QDataWidgetMapper) -> RetType;
}
impl<'a> /*trait*/ QDataWidgetMapper_revert_0<(/*void*/)> for () {
  fn revert_0(self , rsthis: & QDataWidgetMapper) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN17QDataWidgetMapper6revertEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qdatawidgetmapper.h:96
// index:0
// Public Visibility=Default Availability=Available
// [1] bool submit()

/*
Submits all changes from the mapped widgets to the model.

For every mapped section, the item delegate reads the current value from the widget and sets it in the model. Finally, the model's submit() method is invoked.

Returns true if all the values were submitted, otherwise false.

Note: For database models, QSqlQueryModel::lastError() can be used to retrieve the last error.

See also revert() and setSubmitPolicy().
*/
impl /*struct*/ QDataWidgetMapper {
  pub fn submit_0<RetType, T: QDataWidgetMapper_submit_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.submit_0(self);
    // return 1;
  }
}
pub trait QDataWidgetMapper_submit_0<RetType> {
  fn submit_0(self , rsthis: & QDataWidgetMapper) -> RetType;
}
impl<'a> /*trait*/ QDataWidgetMapper_submit_0<bool> for () {
  fn submit_0(self , rsthis: & QDataWidgetMapper) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN17QDataWidgetMapper6submitEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qdatawidgetmapper.h:98
// index:0
// Public Visibility=Default Availability=Available
// [-2] void toFirst()

/*
Populates the widgets with data from the first row of the model if the orientation is horizontal (the default), otherwise with data from the first column.

This is equivalent to calling setCurrentIndex(0).

See also toLast() and setCurrentIndex().
*/
impl /*struct*/ QDataWidgetMapper {
  pub fn toFirst_0<RetType, T: QDataWidgetMapper_toFirst_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toFirst_0(self);
    // return 1;
  }
}
pub trait QDataWidgetMapper_toFirst_0<RetType> {
  fn toFirst_0(self , rsthis: & QDataWidgetMapper) -> RetType;
}
impl<'a> /*trait*/ QDataWidgetMapper_toFirst_0<(/*void*/)> for () {
  fn toFirst_0(self , rsthis: & QDataWidgetMapper) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN17QDataWidgetMapper7toFirstEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qdatawidgetmapper.h:99
// index:0
// Public Visibility=Default Availability=Available
// [-2] void toLast()

/*
Populates the widgets with data from the last row of the model if the orientation is horizontal (the default), otherwise with data from the last column.

Calls setCurrentIndex() internally.

See also toFirst() and setCurrentIndex().
*/
impl /*struct*/ QDataWidgetMapper {
  pub fn toLast_0<RetType, T: QDataWidgetMapper_toLast_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toLast_0(self);
    // return 1;
  }
}
pub trait QDataWidgetMapper_toLast_0<RetType> {
  fn toLast_0(self , rsthis: & QDataWidgetMapper) -> RetType;
}
impl<'a> /*trait*/ QDataWidgetMapper_toLast_0<(/*void*/)> for () {
  fn toLast_0(self , rsthis: & QDataWidgetMapper) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN17QDataWidgetMapper6toLastEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qdatawidgetmapper.h:100
// index:0
// Public Visibility=Default Availability=Available
// [-2] void toNext()

/*
Populates the widgets with data from the next row of the model if the orientation is horizontal (the default), otherwise with data from the next column.

Calls setCurrentIndex() internally. Does nothing if there is no next row in the model.

See also toPrevious() and setCurrentIndex().
*/
impl /*struct*/ QDataWidgetMapper {
  pub fn toNext_0<RetType, T: QDataWidgetMapper_toNext_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toNext_0(self);
    // return 1;
  }
}
pub trait QDataWidgetMapper_toNext_0<RetType> {
  fn toNext_0(self , rsthis: & QDataWidgetMapper) -> RetType;
}
impl<'a> /*trait*/ QDataWidgetMapper_toNext_0<(/*void*/)> for () {
  fn toNext_0(self , rsthis: & QDataWidgetMapper) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN17QDataWidgetMapper6toNextEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qdatawidgetmapper.h:101
// index:0
// Public Visibility=Default Availability=Available
// [-2] void toPrevious()

/*
Populates the widgets with data from the previous row of the model if the orientation is horizontal (the default), otherwise with data from the previous column.

Calls setCurrentIndex() internally. Does nothing if there is no previous row in the model.

See also toNext() and setCurrentIndex().
*/
impl /*struct*/ QDataWidgetMapper {
  pub fn toPrevious_0<RetType, T: QDataWidgetMapper_toPrevious_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toPrevious_0(self);
    // return 1;
  }
}
pub trait QDataWidgetMapper_toPrevious_0<RetType> {
  fn toPrevious_0(self , rsthis: & QDataWidgetMapper) -> RetType;
}
impl<'a> /*trait*/ QDataWidgetMapper_toPrevious_0<(/*void*/)> for () {
  fn toPrevious_0(self , rsthis: & QDataWidgetMapper) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN17QDataWidgetMapper10toPreviousEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qdatawidgetmapper.h:102
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void setCurrentIndex(int)

/*

*/
impl /*struct*/ QDataWidgetMapper {
  pub fn setCurrentIndex_0<RetType, T: QDataWidgetMapper_setCurrentIndex_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setCurrentIndex_0(self);
    // return 1;
  }
}
pub trait QDataWidgetMapper_setCurrentIndex_0<RetType> {
  fn setCurrentIndex_0(self , rsthis: & QDataWidgetMapper) -> RetType;
}
impl<'a> /*trait*/ QDataWidgetMapper_setCurrentIndex_0<(/*void*/)> for (i32) {
  fn setCurrentIndex_0(self , rsthis: & QDataWidgetMapper) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN17QDataWidgetMapper15setCurrentIndexEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qdatawidgetmapper.h:103
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setCurrentModelIndex(const QModelIndex &)

/*
Sets the current index to the row of the index if the orientation is horizontal (the default), otherwise to the column of the index.

Calls setCurrentIndex() internally. This convenience slot can be connected to the signal currentRowChanged() or currentColumnChanged() of another view's selection model.

The following example illustrates how to update all widgets with new data whenever the selection of a QTableView named myTableView changes:


  QDataWidgetMapper *mapper = new QDataWidgetMapper();
  connect(myTableView->selectionModel(), SIGNAL(currentRowChanged(QModelIndex,QModelIndex)),
          mapper, SLOT(setCurrentModelIndex(QModelIndex)));



See also currentIndex().
*/
impl /*struct*/ QDataWidgetMapper {
  pub fn setCurrentModelIndex_0<RetType, T: QDataWidgetMapper_setCurrentModelIndex_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setCurrentModelIndex_0(self);
    // return 1;
  }
}
pub trait QDataWidgetMapper_setCurrentModelIndex_0<RetType> {
  fn setCurrentModelIndex_0(self , rsthis: & QDataWidgetMapper) -> RetType;
}
impl<'a> /*trait*/ QDataWidgetMapper_setCurrentModelIndex_0<(/*void*/)> for (usize) {
  fn setCurrentModelIndex_0(self , rsthis: & QDataWidgetMapper) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN17QDataWidgetMapper20setCurrentModelIndexERK11QModelIndex", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qdatawidgetmapper.h:106
// index:0
// Public Visibility=Default Availability=Available
// [-2] void currentIndexChanged(int)

/*
This signal is emitted after the current index has changed and all widgets were populated with new data. index is the new current index.

Note: Notifier signal for property currentIndex. 

See also currentIndex() and setCurrentIndex().
*/
impl /*struct*/ QDataWidgetMapper {
  pub fn currentIndexChanged_0<RetType, T: QDataWidgetMapper_currentIndexChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.currentIndexChanged_0(self);
    // return 1;
  }
}
pub trait QDataWidgetMapper_currentIndexChanged_0<RetType> {
  fn currentIndexChanged_0(self , rsthis: & QDataWidgetMapper) -> RetType;
}
impl<'a> /*trait*/ QDataWidgetMapper_currentIndexChanged_0<(/*void*/)> for (i32) {
  fn currentIndexChanged_0(self , rsthis: & QDataWidgetMapper) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN17QDataWidgetMapper19currentIndexChangedEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}


/*
This enum describes the possible submit policies a QDataWidgetMapper supports.


*/
pub type QDataWidgetMapper__SubmitPolicy = i32;
// Whenever a widget loses focus, the widget's current value is set to the item model.
pub const QDataWidgetMapper__AutoSubmit :QDataWidgetMapper__SubmitPolicy = 0;
// The model is not updated until submit() is called.
pub const QDataWidgetMapper__ManualSubmit :QDataWidgetMapper__SubmitPolicy = 1;
pub fn QDataWidgetMapper_SubmitPolicyItemName(val: i32) ->String {
  return qtrt::GetClassEnumItemName("QDataWidgetMapper", val);
}
pub fn QDataWidgetMapper_SubmitPolicyItemName_s(val: i32) ->String {
  //var nilthis *QDataWidgetMapper
  //return nilthis.SubmitPolicyItemName(val);
  return QDataWidgetMapper_SubmitPolicyItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
